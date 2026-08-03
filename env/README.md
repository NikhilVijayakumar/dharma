# Dharma Env — example `.env` per repository role

Proposal state — examples, not loaded by any runtime yet, except
`dharma-build.env.example`: copied to `.env` at the dharma repository's own
root, it's live (`crates/xtask`, `crates/mcp/build.rs`). Matches the four
files in `config.example/`; design rationale lives in `docs/proposal/11-
provider-config-and-repo-sync.md`.

| File | Matches |
|---|---|
| `dharma-build.env.example` | `dharma-build.toml` (dharma's own root) |
| `dharma-domain.env.example` | `config.example/dharma-domain.toml` |
| `dharma-agent.env.example` | `config.example/dharma-agent.toml` |
| `dharma-repo.env.example` | `config.example/dharma-repo.toml` |

A repository only needs the one `.env.example` matching whichever single
role it plays — copy it to `.env` at that repository's root, same convention
Samgraha uses (`cp .env.example .env`).
