# Agent System Registration

How an Agent System (proposal 04) gets into `mcp.db`, via the `dharma`
CLI's `register-agent-system` command. Companion doc:
`domain-system-registration.md` covers the parallel Domain System flow
(read that first — the mechanics are near-identical); `build.md` covers
producing the `dharma-mcp`/`dharma` binaries.

## What gets registered

Registering an Agent System writes one row to `agent_system_registry`
(`mcp.db` table 01) — identity only: `name`, `concern`, `description`,
`is_privileged`. It does **not** parse or populate any of the Agent
content tables (`agent`, `agent_goal`, `skill`, `skill_prompt`,
`skill_script`, `skill_example`, `skill_template`,
`agent_skill_binding` — mcp.db 12-19). As with Domain Systems,
registration and content capture are two separate steps this one
command happens to chain when `--content-root` is given.

## Usage

```sh
dharma register-agent-system \
  --name rust-development \
  --concern rust-development \
  --description "Agents and skills for Rust crate development, review, and QA" \
  --content-root /path/to/provider/content
```

| Flag | Required | Default | Notes |
|---|---|---|---|
| `--name` | Yes | — | Must be globally unique (`UNIQUE` on `agent_system_registry.name`). |
| `--concern` | Yes | — | Also globally unique (separate `UNIQUE` constraint) — this is what `task_step.required_capability` resolves against (proposal 02/04), so two Agent Systems can't claim the same concern. |
| `--description` | No | `""` | |
| `--content-root` | No | — | Captured into `content_asset` exactly as described in `domain-system-registration.md`'s "What `--content-root` capture actually does" — same walk, same extension→`asset_kind` mapping, same append-only ledger semantics. |

Output:

```
agent_system_id=1 name=rust-development concern=rust-development captured_files=17
```

## Privilege is not settable from this command

The CLI always registers with `is_privileged = false` — there is
currently no `dharma` subcommand to mark an Agent System privileged
(Agent-Management / Default-Bootstrap, proposal 04). If your deployment
needs a privileged Agent System, that flag has to be set directly in
`mcp.db` for now; treat this as a known gap, not an intentional
CLI-only-via-database design.

## Inspecting what's registered

```sh
dharma list-agent-systems
# 1 rust-development concern=rust-development privileged=false
# 2 documentation-management concern=documentation-management privileged=false

dharma agent-system-info --name rust-development
# full JSON tree: agents, agent_goal, skill, skill_prompt, skill_script,
# skill_example, skill_template, agent_skill_binding
```

## Next step

An Agent System existing in `mcp.db` becomes available to a repository
only through the Default/Bootstrap Agent System's capability resolution
at registration (proposal 06) and an explicit `dharma repo-approve
--agent-system <name> --reviewed-by <you> --yes` — registering it here
does not grant it to any repository.

## Related

- `docs/proposal/04-agent-system-registry.md` — the design this command
  implements.
- `docs/proposal/03-skill-model.md` — the Skill shape (`skill`,
  `skill_prompt` mandatory, `skill_script`/`skill_template` optional,
  `skill_example` mandatory ≥1) that Agent System content is expected to
  follow once a parse step exists.
- `docs/proposal/14-mcp-tool-contract.md` — the MCP-tool equivalent
  (`register_agent_system`) for a client driving this over MCP instead
  of the CLI.
