//! dharma-mcp server: JSON-RPC-ish line protocol over stdin/stdout (mirrors
//! samgraha's mcp transport — each request is one JSON object per line on
//! stdin, each response one JSON object per line on stdout; tracing goes to
//! stderr so stdout stays protocol-clean).

mod adapter;
mod protocol;

use adapter::McpAdapter;
use anyhow::Context;
use protocol::{McpMessage, McpRequest, McpResponse, McpError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
            Ok(None) => None,
            Err(e) => Some(serde_json::json!({
                "id": null,
                "error": {"code": -32700, "message": format!("parse error: {e}")},
            })),
        };
        if let Some(reply) = reply {
            if serde_json::to_string(&reply).is_ok() {
                let text = reply.to_string();
                writeln!(stdout, "{text}")?;
                stdout.flush()?;
            }
        } else {
            // shutdown — drop the transport
            break;
        }
    }
    Ok(())
}

/// Translate a wire line into the reply JSON (or `None` for shutdown).
fn handle_line(adapter: &McpAdapter, line: &str) -> anyhow::Result<Option<Value>> {
    let request: JsonRpcRequest = serde_json::from_str(line)
        .map_err(|e| anyhow::anyhow!("invalid request: {e}"))?;

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
            id: id.clone(),
            result: Some(result),
            error: None,
        },
        Some(McpMessage::Error(McpError { id: _, code, message })) => JsonRpcResponse {
            id: id.clone(),
            result: None,
            error: Some(JsonRpcError { code, message }),
        },
        None => {
            // shutdown notification path isn't reached (requests always reply)
            return Ok(None);
        }
        _ => unreachable!("adapter replies only with Response or Error for requests"),
    };
    Ok(Some(serde_json::to_value(reply)?))
}
