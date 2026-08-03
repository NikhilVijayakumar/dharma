//! dharma-mcp server: real MCP (Model Context Protocol) over stdin/stdout —
//! JSON-RPC 2.0 handshake (`initialize`), tool discovery (`tools/list`) and
//! dispatch (`tools/call`), mirroring samgraha's mcp transport. Tracing goes
//! to stderr so stdout stays protocol-clean. `tools/call` bridges into the
//! flat method-per-tool dispatch of `adapter::route` (docs/proposal/14),
//! which remains reachable directly (legacy path) for non-MCP-client callers
//! of this same binary.

mod adapter;
mod protocol;
mod tools;

use adapter::McpAdapter;
use anyhow::Context;
use protocol::{McpMessage, McpRequest, McpResponse, McpError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    id: Option<Value>,
    method: String,
    #[serde(default)]
    params: serde_json::Map<String, Value>,
    /// Explicit per-call repository root (proposal 14 envelope field).
    #[serde(default)]
    repo_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

/// Refuse to start past the build's baked-in expiry (`DHARMA_EXPIRY`, set by
/// `crates/mcp/build.rs` from `config/dharma-build.toml` `[package]` +
/// `.env`). Absent env means the build was packaged with `expiry_days = -1`
/// (never expires) — a no-op.
fn check_expiry() {
    let Some(expiry) = option_env!("DHARMA_EXPIRY") else { return };
    let now = chrono::Utc::now();
    match chrono::DateTime::parse_from_rfc3339(expiry) {
        Ok(dt) if now > dt => {
            eprintln!("ERROR: This binary expired at {expiry} UTC. Build a new one.");
            std::process::exit(1);
        }
        Ok(_) => {}
        Err(_) => eprintln!("Warning: DHARMA_EXPIRY='{expiry}' is not RFC3339, ignored"),
    }
}

fn main() -> anyhow::Result<()> {
    check_expiry();

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    let db = registry::McpDb::open().context("Failed to open mcp.db")?;
    let adapter = Arc::new(McpAdapter::new(db));
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    let lines = stdin.lock().lines();

    for line in lines {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let reply = match handle_line(&adapter, line) {
            Ok(Some(value)) => Some(value),
            // notification (no `id`): execute, no reply — per JSON-RPC 2.0
            Ok(None) => None,
            Err(e) => Some(serde_json::json!({
                "jsonrpc": "2.0",
                "id": null,
                "error": {"code": -32700, "message": format!("parse error: {e}")},
            })),
        };
        if let Some(reply) = reply {
            let text = reply.to_string();
            writeln!(stdout, "{text}")?;
            stdout.flush()?;
        }
    }
    Ok(())
}

/// Translate a wire line into the reply JSON (or `None` for a notification,
/// which must not receive a reply per JSON-RPC 2.0).
fn handle_line(adapter: &McpAdapter, line: &str) -> anyhow::Result<Option<Value>> {
    let request: JsonRpcRequest = serde_json::from_str(line)
        .map_err(|e| anyhow::anyhow!("invalid request: {e}"))?;
    let has_id = request.id.is_some();

    match request.method.as_str() {
        "initialize" => {
            return Ok(Some(json!({
                "jsonrpc": "2.0",
                "id": request.id,
                "result": {
                    "protocolVersion": "2025-03-26",
                    "capabilities": {"tools": {}},
                    "serverInfo": {"name": "dharma-mcp", "version": env!("CARGO_PKG_VERSION")},
                },
            })));
        }
        "notifications/initialized" | "notifications/cancelled" => return Ok(None),
        "tools/list" => {
            return Ok(Some(json!({
                "jsonrpc": "2.0",
                "id": request.id,
                "result": {"tools": tools::tool_defs()},
            })));
        }
        "tools/call" => return Ok(Some(handle_tools_call(adapter, &request)?)),
        _ => {}
    }

    // Legacy flat method-per-tool dispatch (direct method call, not routed
    // through tools/call) — kept for non-MCP-client callers of this binary.
    let id = request.id.clone();
    let wire_id = match &id {
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::String(s)) => s.clone(),
        _ => request.method.clone(),
    };
    let msg = McpMessage::Request(McpRequest {
        id: wire_id,
        method: request.method,
        params: request.params.into_iter().collect(),
        repo_path: request.repo_path,
    });

    let reply = match adapter.handle_message(msg) {
        Some(McpMessage::Response(McpResponse { id: _, result })) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: id.clone(),
            result: Some(result),
            error: None,
        },
        Some(McpMessage::Error(McpError { id: _, code, message })) => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: id.clone(),
            result: None,
            error: Some(JsonRpcError { code, message }),
        },
        None => return Ok(None),
        _ => unreachable!("adapter replies only with Response or Error for requests"),
    };
    if !has_id {
        // executed for effect; a true notification gets no reply
        return Ok(None);
    }
    Ok(Some(serde_json::to_value(reply)?))
}

/// `tools/call {name, arguments}` bridges to the same flat dispatch the
/// legacy path uses, then wraps the result in MCP's tool-result envelope
/// (mirrors samgraha's shape: `content` + `isError` + `structuredContent`).
fn handle_tools_call(adapter: &McpAdapter, request: &JsonRpcRequest) -> anyhow::Result<Value> {
    let name = request
        .params
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let mut arguments = request
        .params
        .get("arguments")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let repo_path = arguments
        .remove("repo_path")
        .and_then(|v| v.as_str().map(String::from));

    let msg = McpMessage::Request(McpRequest {
        id: name.clone(),
        method: name,
        params: arguments.into_iter().collect(),
        repo_path,
    });

    let (result, is_error) = match adapter.handle_message(msg) {
        Some(McpMessage::Response(McpResponse { result, .. })) => (result, false),
        Some(McpMessage::Error(McpError { message, .. })) => (json!({"error": message}), true),
        _ => unreachable!("adapter replies only with Response or Error for requests"),
    };
    let text = serde_json::to_string_pretty(&result)?;
    Ok(json!({
        "jsonrpc": "2.0",
        "id": request.id,
        "result": {
            "content": [{"type": "text", "text": text}],
            "isError": is_error,
            "structuredContent": result,
        },
    }))
}
