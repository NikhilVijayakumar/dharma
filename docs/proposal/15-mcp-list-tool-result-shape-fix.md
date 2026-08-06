# Proposal: MCP List-Tool Result Shape Fix

> Status: Draft — design-only, no code changed yet. Conforms to `docs/raw/proposal.md` standard.

## Purpose

Three MCP tools on the `dharma` server — `list_domain_systems`, `list_agent_systems`, `list_repos` — return a bare JSON array as their `structuredContent`, while every other tool on the server returns a JSON object. MCP clients that validate `structuredContent` against a `record`-shaped schema reject the response before its data ever reaches the caller, making these three tools unusable through any client that enforces the schema. This proposal defines the fix: wrap each tool's list under a named key, matching the object-shape convention every other tool on the server already follows.

## System Overview

### Overview

`crates/mcp/src/adapter.rs` implements one handler function per MCP tool. Handlers that return a collection (proposed capability manifests inside `tool_repo_status`, captured files inside `tool_recapture`, etc.) already wrap the collection in an object, e.g. `json!({"appended": appended})`. Three handlers instead call `Ok(json!(out))` directly on a `Vec<Value>`, producing a bare array:

- `tool_list_domain_systems` — `crates/mcp/src/adapter.rs:287-294`
- `tool_list_agent_systems` — `crates/mcp/src/adapter.rs:296-305`
- `tool_list_repos` — `crates/mcp/src/adapter.rs:353-371`

Reproduced live against this repository's own `dharma` MCP server (2026-08-04): calling any of the three returns

```text
MCP server "dharma" returned a malformed result that failed schema validation: [
  { "expected": "record", "code": "invalid_type", "path": ["structuredContent"], "message": "Invalid input: expected record, received array" }
]
```

### Structural Approach

No new component. The fix changes the return value of three existing handler functions from `Vec<Value>` wrapped directly, to `Vec<Value>` wrapped under one object key each — the same shape every other handler in `adapter.rs` already returns.

### Diagram

```text
Before:  tool_list_domain_systems() -> [ {...}, {...}, ... ]              ✗ fails structuredContent: record
After:   tool_list_domain_systems() -> { "domain_systems": [ {...}, ... ] } ✓
```

## Component Model

### `tool_list_domain_systems`
- **Responsibility:** Return every registered Domain System.
- **Ownership:** Output shape only; no change to `list_domain_systems` in `crates/services/src/registry.rs`.
- **Interfaces:** Returns `{"domain_systems": [...]}` instead of `[...]`.

### `tool_list_agent_systems`
- **Responsibility:** Return every registered Agent System.
- **Ownership:** Output shape only.
- **Interfaces:** Returns `{"agent_systems": [...]}` instead of `[...]`.

### `tool_list_repos`
- **Responsibility:** Return every registered repository.
- **Ownership:** Output shape only.
- **Interfaces:** Returns `{"repos": [...]}` instead of `[...]`.

## Communication

### Communication Paths

**MCP Client → `dharma` server**
- **Pattern:** Synchronous tool call.
- **Contract:** Client sends `list_domain_systems` / `list_agent_systems` / `list_repos` with no arguments; server returns an object whose value at the tool's collection key is the array of rows. Unchanged for every other tool already returning an object.

### Communication Diagram

```text
Client → dharma MCP : call(list_domain_systems)
dharma MCP → Client  : { "domain_systems": [ {id, name, version, description}, ... ] }
```

## Data Flow

### Data Paths

**List Path**
- **Entry point:** MCP tool call with no arguments.
- **Transformations:** `registry::list_*` reads rows from `mcp.db`; the handler maps each row to a `Value` (unchanged) and wraps the resulting `Vec<Value>` under one named key (changed).
- **Ownership boundary:** The handler owns the wrapping; the row-to-`Value` mapping and the underlying query are untouched.
- **Exit point:** An object whose value at the collection key is the array of rows.

### Data Ownership

| Data Entity | Owning Component |
|---|---|
| Row query + row-to-`Value` mapping | `crates/services/src/registry.rs`, unchanged |
| Response envelope (the wrapping key) | `crates/mcp/src/adapter.rs`, this proposal's change |

## Security

### Trust Boundaries

- **MCP Client ↔ `dharma` server:** Unchanged. This is a response-shape fix, not a change to what data is returned or who can request it.

### Threat Model

- Not applicable — no new data exposed, no new input accepted. The only behavior change is that these three calls stop failing client-side schema validation.

## Lifecycle

> Status: draft
> Draft commit: not yet committed
> Finalized commit: not yet finalized
> Implementation commit (final, verified): not yet implemented
> Archive commit: not yet archived

Finalized means: the three handlers are patched to wrap their output object, `register_and_list_domain_systems` (`crates/mcp/src/adapter.rs:1211-1228`) is updated to unwrap the new `domain_systems` key instead of indexing the result as a bare array, `cargo test` passes, and a live call to all three via the `dharma` MCP tools returns without a schema-validation error. (No `structuredContent`/output-schema declarations exist in this repo today — `crates/mcp/src/tools.rs` declares only `inputSchema` per tool, per `tool_defs()` at `tools.rs:43-47` — so there is no separate schema file to update; the MCP-client-side `record` check this proposal fixes is enforced by the calling client, not declared here.)

## Implementation

Three edits, all in `crates/mcp/src/adapter.rs`:

1. `tool_list_domain_systems` (line 293): `Ok(json!(out))` → `Ok(json!({"domain_systems": out}))`
2. `tool_list_agent_systems` (line 304): `Ok(json!(out))` → `Ok(json!({"agent_systems": out}))`
3. `tool_list_repos` (line 370): `Ok(json!(out))` → `Ok(json!({"repos": out}))`

Plus one test update: `register_and_list_domain_systems` (lines 1225-1227) changes from

```rust
let list = call(&a, "list_domain_systems", &[], None);
assert_eq!(list.as_array().unwrap().len(), 1);
assert_eq!(list[0]["name"], "base_dev");
```

to

```rust
let list = call(&a, "list_domain_systems", &[], None);
assert_eq!(list["domain_systems"].as_array().unwrap().len(), 1);
assert_eq!(list["domain_systems"][0]["name"], "base_dev");
```

Verification: `cargo test -p mcp` (crate package name is `mcp`, per `crates/mcp/Cargo.toml:2`; `dharma-mcp` is only the `[[bin]]` name), then a live call to `list_domain_systems`, `list_agent_systems`, and `list_repos` through the `dharma` MCP server, confirming each returns an object (not an array) and no client-side schema-validation error occurs.

## Rationale

### Wrap Under a Named Key, Not Change the Client Schema
- **Context:** Every other tool on the server already returns an object; these three are the outliers.
- **Decision:** Fix the three outlier handlers to match the established convention, rather than relaxing the client-side schema to accept a bare array.
- **Alternatives Considered:** Loosen `structuredContent` validation to accept either `record` or `array`.
- **Rejection Reason:** Would leave the server's own tool set internally inconsistent (some tools return arrays, most return objects) and push the inconsistency onto every client instead of fixing it once at the source.
- **Architectural Goal:** One consistent response envelope shape across every tool on the `dharma` MCP server.

## Constraints

### Hard Constraints
- The row-to-`Value` field mapping for each of the three tools stays byte-identical — only the outer envelope changes. Existing callers that already unwrap a known key (if any exist) must not have that key's shape altered.

### Soft Constraints
- Prefer a key name that is the plural of the resource (`domain_systems`, `agent_systems`, `repos`), matching how `tool_recapture` already names its collection key (`appended`).

## Traceability

### Derivation Chain

```text
MCP Tool Contract (14-mcp-tool-contract.md)
    │
    ▼
MCP List-Tool Result Shape Fix (this document) — supersedes 14's result-shape column
    for list_domain_systems, list_agent_systems, list_repos (see below)
    │
    ▼
crates/mcp/src/adapter.rs: tool_list_domain_systems, tool_list_agent_systems, tool_list_repos
```

**Supersession note:** `docs/proposal/archive/14-mcp-tool-contract.md` (archived, not editable per proposal 12) documents these three tools' `Result` column as bare arrays — `list_domain_systems` → `[{name, version, description}]` (14:46), `list_agent_systems` → `[{name, concern, description, is_privileged}]` (14:47), `list_repos` → `[{repo_uuid, name, domain_system, status}]` (14:60). This proposal supersedes those three `Result` cells only; every other row of 14's contract is unaffected. The corrected shape is `{"domain_systems": [...]}`, `{"agent_systems": [...]}`, `{"repos": [...]}` respectively, per Component Model above.

### Non-Contradiction Rule

No downstream change may reintroduce a bare-array `structuredContent` return from any tool on the `dharma` MCP server. Any future reader of 14's `Result` column for `list_domain_systems`, `list_agent_systems`, or `list_repos` must treat this document as the current source of truth for those three rows, not 14.
