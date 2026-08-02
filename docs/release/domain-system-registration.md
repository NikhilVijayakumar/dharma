# Domain System Registration

How a Domain System (proposal 05) gets into `mcp.db`, via the `dharma`
CLI's `register-domain-system` command. Companion doc:
`agent-system-registration.md` covers the parallel Agent System flow;
`build.md` covers producing the `dharma-mcp`/`dharma` binaries.

## What gets registered

Registering a Domain System writes one row to `domain_system_registry`
(`mcp.db` table 00) — just identity: `name`, `version`, `description`.
It does **not** parse or populate any of the Domain content tables
(`domain`, `section`, `section_profile`, `epic`, `usecase`, `task`,
`task_step` — mcp.db 05-11). Registration and content capture are two
separate steps this one command happens to chain together when
`--content-root` is given.

## Usage

```sh
dharma register-domain-system \
  --name rust-dev-domain \
  --version 0.1.0 \
  --description "Rust development domain: crate architecture, engineering standard" \
  --content-root /path/to/provider/content
```

| Flag | Required | Default | Notes |
|---|---|---|---|
| `--name` | Yes | — | Must be globally unique in `mcp.db` (`UNIQUE` on `domain_system_registry.name`). Registering an existing name errors. |
| `--version` | No | `0.0.0` | Free-form version string. |
| `--description` | No | `""` | |
| `--content-root` | No | — | A local directory to capture into `content_asset` (mcp.db 02), attributed to this Domain System's `name`. Omit to register identity only, with content captured later via `recapture-domain-system` (once implemented — see Related). |

Output:

```
domain_system_id=1 name=rust-dev-domain version=0.1.0 captured_files=42
```

`captured_files=0` if `--content-root` was omitted.

## What `--content-root` capture actually does

Walks the given directory recursively (`common::fs::walk_files`),
skipping `.git`, `target`, `node_modules`, and `.dharma` — no other
exclusions and **no extension allowlist**: every remaining file is
captured, so the directory should contain only what belongs in this
Domain System's bundle. Each file must be valid UTF-8 text; a binary
file among them fails the whole capture.

For each file, one row is inserted into `content_asset` (never updated):
`source_system` = this Domain System's `name`, `file_path` = path
relative to `--content-root` (`\` normalized to `/`), `content_text` =
the full file, `content_hash` = its sha256, `asset_kind` derived from
extension:

| Extension | `asset_kind` |
|---|---|
| `.yaml`, `.yml` | `yaml` |
| `.md`, `.markdown` | `markdown` |
| `.py` | `python` |
| `.json` | `json` |
| anything else | `text` |

Capture only fills the ledger (`content_asset`) — it does **not** parse
a `section-map.yaml` into `section` rows or a `task.yaml` bundle into
`epic`/`usecase`/`task` rows. That parse step is separate (tracked as
not-yet-exposed via CLI — see Related).

## Inspecting what's registered

```sh
dharma list-domain-systems
# 1 rust-dev-domain 0.1.0
# 2 electron-dev-domain 0.1.0

dharma domain-system-info --name rust-dev-domain
# full JSON tree: domains, sections, section_profiles, epics, usecases, tasks
```

## Next step

A Domain System existing in `mcp.db` is what a consuming repository
selects at registration (`dharma repo-register --domain-system
rust-dev-domain`, proposal 06). Registering the Domain System here does
not itself do anything to any repository.

## Related

- `docs/proposal/05-domain-system-registration.md` — the design this
  command implements.
- `docs/proposal/13-bodha-section-format-reference.md` — the frozen
  Section Map/Profile format this Domain System's content is expected
  to follow, and the fields (`paper_type`/`supported_types`, map-level
  `validation`, profile `trigger`, the Profile Default rule-group
  structure) capture preserves losslessly but does not yet parse into
  rows.
- `docs/proposal/14-mcp-tool-contract.md` — the MCP-tool equivalent
  (`register_domain_system`) for a client driving this over MCP instead
  of the CLI.
