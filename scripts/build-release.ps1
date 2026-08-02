$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$root = (Resolve-Path $root).Path

# Read .env -- single source of truth, no CLI overrides. Unlike samgraha,
# dharma has no build-time expiry lock; the only build-script setting is
# where to put the package.
$outputDir = ""

$envFile = Join-Path $root ".env"
if (Test-Path $envFile) {
    Get-Content $envFile | ForEach-Object {
        $line = $_.Trim()
        if ($line -and $line -notlike '#*' -and $line -like '*=*') {
            $kv = $line.Split('=', 2)
            $k  = $kv[0].Trim()
            $v  = $kv[1].Trim().Trim('"', "'")
            if ($k -eq "OUTPUT_DIR") { $outputDir = $v }
        }
    }
}

# Resolve output dir -- prefer absolute path from .env
if (-not $outputDir) {
    Write-Warning "OUTPUT_DIR not set in .env -- falling back to .\release. Set an absolute path in .env."
    $outputDir = ".\release"
}
if (-not [System.IO.Path]::IsPathRooted($outputDir)) {
    Write-Warning "OUTPUT_DIR '$outputDir' is relative -- resolving from project root. Use an absolute path in .env."
    $outputDir = Join-Path $root $outputDir
}
$outputDir = (New-Item -ItemType Directory -Force $outputDir).FullName

# Build
Write-Host "Building dharma-mcp.exe + dharma.exe (release)..." -ForegroundColor Yellow
& cargo build --release --bin dharma-mcp --bin dharma --manifest-path "$root\Cargo.toml"
if ($LASTEXITCODE -ne 0) { throw "Build failed" }

# Package directory
$pkgDir = Join-Path $outputDir "dharma"
if (Test-Path $pkgDir) { Remove-Item -Recurse -Force $pkgDir }
New-Item -ItemType Directory -Force "$pkgDir\bin" | Out-Null

# Copy binaries
Copy-Item "$root\target\release\dharma-mcp.exe" "$pkgDir\bin\"
Copy-Item "$root\target\release\dharma.exe" "$pkgDir\bin\"

# Example configs -- one per repository role (proposal 11); no single
# "the" config the way samgraha ships one samgraha.toml, since a dharma
# deployment plays exactly one of four roles, not a fixed one.
New-Item -ItemType Directory -Force "$pkgDir\config" | Out-Null
Copy-Item "$root\config\*.toml" "$pkgDir\config\" -Force
Write-Host "  -> config/*.toml (example -- copy the one matching your role, rename to dharma-*.toml)" -ForegroundColor Cyan

# Matching .env examples
New-Item -ItemType Directory -Force "$pkgDir\env" | Out-Null
Copy-Item "$root\env\*.env.example" "$pkgDir\env\" -Force
Write-Host "  -> env/*.env.example (matches config/, copy the one matching your role to .env)" -ForegroundColor Cyan

# Ship reference schema -- not read at runtime (registry crate creates and
# migrates mcp.db/repo.db on demand via its own inline Rust migrations,
# per proposal 08's "schema/ is the canonical reference copy" constraint),
# just documentation for anyone integrating with a raw DB file directly.
New-Item -ItemType Directory -Force "$pkgDir\schema\mcp" | Out-Null
New-Item -ItemType Directory -Force "$pkgDir\schema\repo" | Out-Null
Copy-Item "$root\schema\mcp\*.sql" "$pkgDir\schema\mcp\" -Force
Copy-Item "$root\schema\repo\*.sql" "$pkgDir\schema\repo\" -Force
Write-Host "  -> schema/mcp/, schema/repo/ (reference schema)" -ForegroundColor Cyan

# Launcher scripts
$runCmdLines = @(
    '@echo off',
    'rem dharma-mcp -- set DHARMA_MCP_DIR to control where mcp.db lives',
    'rem (defaults to %USERPROFILE%\.dharma if unset).',
    '"%~dp0bin\dharma-mcp.exe" %*'
)
Set-Content -Path "$pkgDir\run-mcp.cmd" -Value ($runCmdLines -join "`r`n") -Encoding ASCII

$runShLines = @(
    '#!/usr/bin/env sh',
    '# dharma-mcp -- set DHARMA_MCP_DIR to control where mcp.db lives',
    '# (defaults to $HOME/.dharma if unset).',
    'exec "$(dirname "$0")/bin/dharma-mcp.exe" "$@"'
)
Set-Content -Path "$pkgDir\run-mcp.sh" -Value ($runShLines -join "`n") -Encoding ASCII

# Checksums
$mcpHash = (Get-FileHash "$pkgDir\bin\dharma-mcp.exe" -Algorithm SHA256).Hash.ToLower()
$cliHash = (Get-FileHash "$pkgDir\bin\dharma.exe" -Algorithm SHA256).Hash.ToLower()
$sumsLines = @(
    "$mcpHash  bin/dharma-mcp.exe",
    "$cliHash  bin/dharma.exe"
)
Set-Content -Path "$pkgDir\SHA256SUMS" -Value ($sumsLines -join "`r`n") -Encoding ASCII

$mcpSize = [int]((Get-Item "$pkgDir\bin\dharma-mcp.exe").Length / 1KB)
$cliSize = [int]((Get-Item "$pkgDir\bin\dharma.exe").Length / 1KB)

Write-Host "`n=== Release packaged ===" -ForegroundColor Green
Write-Host "  Location:   $pkgDir" -ForegroundColor Cyan
Write-Host ("  dharma-mcp: {0}KB  ({1})" -f $mcpSize, $mcpHash) -ForegroundColor Cyan
Write-Host ("  dharma:     {0}KB  ({1})" -f $cliSize, $cliHash) -ForegroundColor Cyan
Write-Host '  Use:        Get-Content input.json | .\run-mcp.cmd' -ForegroundColor Gray
