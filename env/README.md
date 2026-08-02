# Dharma Env — example `.env` per repository role

Proposal state — examples, not loaded by any runtime yet. Matches the four
files in `config/`; design rationale lives in `docs/proposal/11-provider-
config-and-repo-sync.md`.

| File | Matches |
|---|---|
| `dharma-build.env.example` | `config/dharma-build.toml` |
| `dharma-domain.env.example` | `config/dharma-domain.toml` |
| `dharma-agent.env.example` | `config/dharma-agent.toml` |
| `dharma-repo.env.example` | `config/dharma-repo.toml` |

A repository only needs the one `.env.example` matching whichever single
role it plays — copy it to `.env` at that repository's root, same convention
Samgraha uses (`cp .env.example .env`).
