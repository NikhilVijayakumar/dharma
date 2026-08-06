# MCP Configuration

This document covers configuring AI assistants and IDEs to use the
dharma-mcp server.

Unlike Samgraha's MCP server (anchored to whichever repository it's
launched from, discovered by walking up from `cwd` looking for
`.samgraha`/`samgraha.toml`), **dharma-mcp is a single global server** —
`mcp.db` is global (`DHARMA_MCP_DIR`, default `<home>/.dharma`), and every
tool call that targets a specific repository does so explicitly via a
`repo_path` field in the request (proposal 14), not via the server's own
working directory. There is nothing repo-specific to set up when
launching the server itself.

A release built with `[[release.providers]]` configured (proposal 16) also
ships a `data/mcp.db` next to `bin/`, pre-populated with whichever Domain/
Agent Systems their providers had published content for at build time. On
the very first launch — by any client below, since all of them invoke
`bin/dharma-mcp[.exe]` directly — if the global `mcp.db` doesn't exist yet,
that packaged database seeds it, once; after that it's the same single
global store described above, not a second package-local database. An
already-initialized global `mcp.db` is never touched by a later install.

## 1. Claude Code

**Development (source repo, any platform):**

```json
{
  "mcpServers": {
    "dharma": {
      "command": "cargo",
      "args": ["run", "--bin", "dharma-mcp"]
    }
  }
}
```

**Windows — release binary** (built with `scripts/build-release.ps1`):

Set `DHARMA_BUILD_OUTPUT_DIR` in `.env`, then build:

```powershell
# .env
DHARMA_BUILD_OUTPUT_DIR=E:\MCP\Dharma\release

# Build
.\scripts\build-release.ps1
```

The script prints the output location on completion:

```
Location: E:\MCP\Dharma\release\dharma
```

The binary is always at `<Location>\bin\dharma-mcp.exe`. Point Claude
Code at it:

```json
{
  "mcpServers": {
    "dharma": {
      "command": "E:\\MCP\\Dharma\\release\\dharma\\bin\\dharma-mcp.exe"
    }
  }
}
```

Replace `E:\\MCP\\Dharma\\release\\dharma` with the actual `Location`
path printed by the script (which is `DHARMA_BUILD_OUTPUT_DIR\dharma` from your
`.env`). Use double backslashes in JSON.

To point the server at a non-default `mcp.db` location, set
`DHARMA_MCP_DIR` in the server's environment (via the `env` field
below) rather than relying on the `<home>/.dharma` default:

```json
{
  "mcpServers": {
    "dharma": {
      "command": "E:\\MCP\\Dharma\\release\\dharma\\bin\\dharma-mcp.exe",
      "env": { "DHARMA_MCP_DIR": "E:\\MCP\\Dharma\\data" }
    }
  }
}
```

**Linux / Ubuntu — release binary** (built with `scripts/build-release.sh`):

Set `DHARMA_BUILD_OUTPUT_DIR` in `.env`, then build:

```bash
# .env
DHARMA_BUILD_OUTPUT_DIR=$HOME/mcp/dharma/release

# Build
bash scripts/build-release.sh
```

The script prints the output location on completion:

```
Location: /home/<you>/mcp/dharma/release/dharma
```

The binary is always at `<Location>/bin/dharma-mcp`. Point Claude Code
at it:

```json
{
  "mcpServers": {
    "dharma": {
      "command": "<Location>/bin/dharma-mcp"
    }
  }
}
```

Replace `<Location>` with the actual path printed by the script (which
is `DHARMA_BUILD_OUTPUT_DIR/dharma` from your `.env`).

Test prompts:

- "List the registered Domain Systems."
- "Register this repository against the `rust-dev-domain` Domain System."
- "What's the registration status of this repo?"

## 2. OpenCode

**Development (source repo, any platform):**

Configure MCP server in `opencode.json` (project root or global config):

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "dharma": {
      "type": "local",
      "command": ["cargo", "run", "--bin", "dharma-mcp"]
    }
  }
}
```

Verify the server is registered:

```bash
opencode mcp list
```

Expected: `dharma` listed with status.

**Windows — release binary** (built with `scripts/build-release.ps1`):

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "dharma": {
      "type": "local",
      "command": ["E:\\MCP\\Dharma\\release\\dharma\\bin\\dharma-mcp.exe"]
    }
  }
}
```

Replace the path with the actual `Location` printed by the build script.
Use double backslashes in JSON.

**Linux / Ubuntu — release binary** (built with `scripts/build-release.sh`):

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "dharma": {
      "type": "local",
      "command": ["<Location>/bin/dharma-mcp"]
    }
  }
}
```

Replace `<Location>` with the actual path printed by the build script.

Test prompts:

- "Use the dharma tools — list the registered Domain Systems."
- "Use dharma — what's the registration status of this repo?"

## 3. Antigravity IDE

**Development (source repo, any platform):**

Configure MCP server in `~/.gemini/config/mcp_config.json`:

```json
{
  "mcpServers": {
    "dharma": {
      "command": "cargo",
      "args": ["run", "--bin", "dharma-mcp"]
    }
  }
}
```

**Windows — release binary**:

```json
{
  "mcpServers": {
    "dharma": {
      "command": "E:\\MCP\\Dharma\\release\\dharma\\bin\\dharma-mcp.exe"
    }
  }
}
```

**Linux / Ubuntu — release binary**:

```json
{
  "mcpServers": {
    "dharma": {
      "command": "<Location>/bin/dharma-mcp"
    }
  }
}
```

## 4. Codex CLI / future IDE integrations

Codex CLI supports MCP servers via `mcp` in `codex.toml`. Configuration
pattern:

```toml
[mcp.dharma]
command = "<Location>/bin/dharma-mcp"
```

Replace `<Location>` with the actual path to the release or build
directory.

> **Status:** Untested. Codex CLI's MCP interface may differ from the
> above. Verify against the
> [Codex CLI documentation](https://github.com/openai/codex) before
> relying on this configuration.
