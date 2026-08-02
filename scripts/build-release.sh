#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Read .env — single source of truth, no CLI overrides. Unlike samgraha,
# dharma has no build-time expiry lock; the only build-script setting is
# where to put the package.
OUTPUT_DIR=""

ENV_FILE="$ROOT_DIR/.env"
if [[ -f "$ENV_FILE" ]]; then
    while IFS='=' read -r key val; do
        key="${key#"${key%%[![:space:]]*}"}"; key="${key%"${key##*[![:space:]]}"}"
        val="${val#"${val%%[![:space:]]*}"}"; val="${val%"${val##*[![:space:]]}"}"
        val="${val#\"}"; val="${val%\"}"
        val="${val#\'}"; val="${val%\'}"
        [[ -z "$key" || "$key" =~ ^# ]] && continue
        case "$key" in
            OUTPUT_DIR) OUTPUT_DIR="$val" ;;
        esac
    done < <(grep -v '^\s*#' "$ENV_FILE" | grep '=')
fi

# Resolve output dir — prefer absolute path from .env
if [[ -z "$OUTPUT_DIR" ]]; then
    echo "WARNING: OUTPUT_DIR not set in .env — falling back to ./release. Set an absolute path in .env." >&2
    OUTPUT_DIR="./release"
fi
OUTPUT_DIR="${OUTPUT_DIR//\\//}"
if [[ "$OUTPUT_DIR" != /* ]]; then
    echo "WARNING: OUTPUT_DIR '$OUTPUT_DIR' is relative — resolving from project root. Use an absolute path in .env." >&2
    OUTPUT_DIR="$ROOT_DIR/${OUTPUT_DIR#./}"
fi
mkdir -p "$OUTPUT_DIR"
OUTPUT_DIR="$(cd "$OUTPUT_DIR" && pwd)"

# Build
echo "Building dharma-mcp + dharma (release)..."
cargo build --release --bin dharma-mcp --bin dharma --manifest-path "$ROOT_DIR/Cargo.toml"

# Package directory
PKG_DIR="$OUTPUT_DIR/dharma"
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR/bin"

# Copy binaries
cp "$ROOT_DIR/target/release/dharma-mcp" "$PKG_DIR/bin/"
cp "$ROOT_DIR/target/release/dharma" "$PKG_DIR/bin/"

# Strip debug info to reduce size
if command -v strip &>/dev/null; then
    strip "$PKG_DIR/bin/dharma-mcp" "$PKG_DIR/bin/dharma"
fi

# Example configs — one per repository role (proposal 11); no single
# "the" config the way samgraha ships one samgraha.toml, since a dharma
# deployment plays exactly one of four roles, not a fixed one.
mkdir -p "$PKG_DIR/config"
cp "$ROOT_DIR/config"/*.toml "$PKG_DIR/config/"
echo "  -> config/*.toml (example — copy the one matching your role, rename to dharma-*.toml)"

# Matching .env examples
mkdir -p "$PKG_DIR/env"
cp "$ROOT_DIR/env"/*.env.example "$PKG_DIR/env/"
echo "  -> env/*.env.example (matches config/, copy the one matching your role to .env)"

# Ship reference schema — not read at runtime (registry crate creates and
# migrates mcp.db/repo.db on demand via its own inline Rust migrations,
# per proposal 08's "schema/ is the canonical reference copy" constraint),
# just documentation for anyone integrating with a raw DB file directly.
mkdir -p "$PKG_DIR/schema/mcp" "$PKG_DIR/schema/repo"
cp "$ROOT_DIR/schema/mcp"/*.sql "$PKG_DIR/schema/mcp/"
cp "$ROOT_DIR/schema/repo"/*.sql "$PKG_DIR/schema/repo/"
echo "  -> schema/mcp/, schema/repo/ (reference schema)"

# Launcher scripts (Linux build: binaries have no .exe)
cat > "$PKG_DIR/run-mcp.sh" <<SHEOF
#!/usr/bin/env sh
# dharma-mcp — set DHARMA_MCP_DIR to control where mcp.db lives
# (defaults to \$HOME/.dharma if unset).
exec "\$(dirname "\$0")/bin/dharma-mcp" "\$@"
SHEOF
chmod +x "$PKG_DIR/run-mcp.sh"

cat > "$PKG_DIR/run-mcp.cmd" <<CMDEOF
@echo off
rem dharma-mcp — set DHARMA_MCP_DIR to control where mcp.db lives
rem (defaults to %USERPROFILE%\.dharma if unset).
"%~dp0bin\dharma-mcp.exe" %*
CMDEOF

# Checksums
if command -v sha256sum &>/dev/null; then
    sha256sum "$PKG_DIR/bin/dharma-mcp" "$PKG_DIR/bin/dharma" \
        | sed "s|$PKG_DIR/||" > "$PKG_DIR/SHA256SUMS"
elif command -v shasum &>/dev/null; then
    shasum -a 256 "$PKG_DIR/bin/dharma-mcp" "$PKG_DIR/bin/dharma" \
        | sed "s|$PKG_DIR/||" > "$PKG_DIR/SHA256SUMS"
fi

MCP_SIZE=$(wc -c < "$PKG_DIR/bin/dharma-mcp"); MCP_SIZE=$((MCP_SIZE / 1024))
CLI_SIZE=$(wc -c < "$PKG_DIR/bin/dharma"); CLI_SIZE=$((CLI_SIZE / 1024))
MCP_HASH=$(awk 'NR==1{print $1}' "$PKG_DIR/SHA256SUMS" 2>/dev/null || echo "n/a")
CLI_HASH=$(awk 'NR==2{print $1}' "$PKG_DIR/SHA256SUMS" 2>/dev/null || echo "n/a")

echo ""
echo "=== Release packaged ==="
echo "  Location:  $PKG_DIR"
echo "  dharma-mcp: ${MCP_SIZE}KB  ($MCP_HASH)"
echo "  dharma:     ${CLI_SIZE}KB  ($CLI_HASH)"
echo "  Use:        echo '{\"id\":1,\"method\":\"list_domain_systems\",\"params\":{}}' | ./run-mcp.sh"
