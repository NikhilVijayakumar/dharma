# Thin wrapper -- all packaging logic lives in crates/xtask (Rust, cross-platform).
# Config: dharma-build.toml + .env at repo root (see env/dharma-build.env.example).
$ErrorActionPreference = "Stop"
$root = (Resolve-Path (Split-Path -Parent $PSScriptRoot)).Path
& cargo run --quiet --manifest-path "$root\Cargo.toml" -p xtask -- release
exit $LASTEXITCODE
