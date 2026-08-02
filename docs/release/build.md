# Build

How to produce a standalone, distributable Dharma release — the compiled
`dharma-mcp`/`dharma` binaries plus what they need at the destination
machine. Not about `dharma-*.toml`'s runtime configuration (see
`docs/proposal/11-provider-config-and-repo-sync.md` for that) — this is
specifically the release-packaging pipeline.

## Purpose

A release is a portable distribution that runs on any Windows or Linux
machine without a Rust toolchain or source checkout: the compiled MCP
server (`dharma-mcp`), the CLI (`dharma`), example configs for each of
the four repository roles, and a reference copy of the SQL schema.

**Not bundled**: `mcp.db` or any `repo.db`. Both are created fresh, on
demand, by `registry`'s own inline Rust migrations (`registry::McpDb::open`)
the first time each is opened — never by executing a `.sql` file from
disk. A release ships no database file at all.

Unlike Samgraha's release, there is **no build-time expiry lock** — dharma
has no equivalent mechanism, and this pipeline doesn't invent one.

## Configuration (`.env`)

The build script accepts no CLI arguments — `.env` (at the repo root,
read only by the build scripts) is the single source of truth for where
the package lands.

| Key | Default | Description |
|-----|---------|--------------|
| `OUTPUT_DIR` | *(required)* | Absolute path for the release package. Use absolute — `.env` is machine-specific. |

```env
# .env
OUTPUT_DIR=C:\releases\dharma
```

This `.env` is unrelated to any of the four `env/*.env.example` files
(`DHARMA_MCP_DIR`, `DHARMA_DIR`, `DHARMA_DOCS_DIR`, etc.) — those are read
at *runtime* by whichever role's `dharma-*.toml` a deployment uses (see
`docs/proposal/11-provider-config-and-repo-sync.md`). Nothing in this
build pipeline reads them.

## Running a build

Two build scripts, same logic, no arguments:

| Platform | Script |
|----------|--------|
| Windows  | `scripts\build-release.ps1` |
| Linux    | `scripts/build-release.sh` |

```powershell
# Windows — edit .env first, then run:
.\scripts\build-release.ps1
```

```sh
# Linux — edit .env first, then run:
./scripts/build-release.sh
```

Both scripts: read `.env` for `OUTPUT_DIR` (falling back to `.\release`
with a warning if unset), run
`cargo build --release --bin dharma-mcp --bin dharma`, then assemble the
package directory described below.

## Output Structure

Verified directly against `scripts/build-release.ps1`/`.sh` (both
scripts produce the identical layout):

```
<OUTPUT_DIR>/
  dharma/
    bin/
      dharma-mcp.exe    # MCP line-delimited JSON server (dharma-mcp on Linux)
      dharma.exe        # CLI tool (dharma on Linux)
    config/
      dharma-build.toml   # example -- self-tooling config (proposal 11)
      dharma-domain.toml  # example -- Domain System provider config
      dharma-agent.toml   # example -- Agent System provider config
      dharma-repo.toml    # example -- consuming repository config
    env/
      dharma-build.env.example
      dharma-domain.env.example
      dharma-agent.env.example
      dharma-repo.env.example
    schema/
      mcp/*.sql   # mcp.db reference schema (registry's inline migrations, not read from disk)
      repo/*.sql  # repo.db reference schema (same)
    run-mcp.cmd   # Windows launcher
    run-mcp.sh    # Linux launcher
    SHA256SUMS    # SHA-256 hashes of bin/dharma-mcp(.exe) and bin/dharma(.exe)
```

Unlike Samgraha (one `samgraha.toml`, one repository role), a dharma
deployment plays exactly one of four roles (proposal 11) — the package
ships all four example configs/envs so whichever role applies can be
copied into place (e.g. `config/dharma-repo.toml` → the consuming
repository's own root, renamed if needed, alongside a `.env` copied from
`env/dharma-repo.env.example`).

The `schema/*.sql` files are **not read by anything at runtime** — the
`registry` crate creates and migrates `mcp.db`/`repo.db` via its own
inline Rust migration constants, mirroring the same "canonical reference
copy, not loaded by any runtime" role `schema/README.md` already
describes for the source tree. They're shipped purely as a human-readable
reference for anyone integrating with a raw database file directly.

## Requirements

- Runtime: None. The binaries are static, no interpreter or VM
  dependency. (A Skill's own script may need an interpreter — Python,
  PowerShell, etc., per `common::env::script_command` — but that's the
  Skill's requirement, not the release binary's.)
- Disk: a few MB for the binaries plus a few hundred KB for the
  reference SQL schema.
- OS: Windows 10+ or Linux (x86-64).

## Where `mcp.db` and `repo.db` actually live

Neither is bundled or created by the build. At runtime:

- `mcp.db` — `DHARMA_MCP_DIR` if set, otherwise `<home>/.dharma/mcp.db`
  (`common::env::mcp_dir`/`mcp_db_path`). There is no exe-relative
  fallback — set `DHARMA_MCP_DIR` explicitly for a fully portable,
  no-home-directory deployment.
- `repo.db` — inside each registered repository's own `.dharma/`
  directory, created by the sync flow (proposal 11) once that
  repository's Capability Manifest is approved.

## Usage

Pipe line-delimited JSON requests into the launcher:

```powershell
# Windows
Get-Content request.json | .\run-mcp.cmd
# or
echo '{"id":1,"method":"list_domain_systems","params":{}}' | .\run-mcp.cmd
```

```sh
# Linux
echo '{"id":1,"method":"list_domain_systems","params":{}}' | ./run-mcp.sh
```

See `docs/proposal/14-mcp-tool-contract.md` for the full tool list and
request/response shapes.

## Verifying Checksums

```sh
# Linux
sha256sum -c SHA256SUMS
```

```powershell
# Windows (PowerShell)
Get-Content SHA256SUMS | ForEach-Object {
    $hash, $file = $_ -split '\s+', 2
    $actual = (Get-FileHash $file -Algorithm SHA256).Hash.ToLower()
    if ($actual -eq $hash) { "OK: $file" } else { "FAIL: $file" }
}
```
