#!/usr/bin/env bash
# Thin wrapper — all packaging logic lives in crates/xtask (Rust, cross-platform).
# Config: dharma-build.toml + .env at repo root (see env/dharma-build.env.example).
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
exec cargo run --quiet --manifest-path "$ROOT_DIR/Cargo.toml" -p xtask -- release
