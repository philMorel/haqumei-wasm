#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WORK_DIR="${1:-$ROOT/.work/haqumei}"
OUT_DIR="${2:-$ROOT/pkg}"
RUNTIME_DIR="$ROOT/runtime/v0.12.0-wasm.3"

if [ ! -d "$WORK_DIR/.git" ]; then
  "$ROOT/scripts/prepare-upstream.sh" "$WORK_DIR"
fi
command -v wasm-bindgen >/dev/null
expected_bindgen="$(awk '/name = "wasm-bindgen"/ { getline; gsub(/"/, "", $3); print $3; exit }' "$WORK_DIR/Cargo.lock")"
actual_bindgen="$(wasm-bindgen --version | awk '{print $2}')"
test -n "$expected_bindgen"
test "$actual_bindgen" = "$expected_bindgen"

cargo build --manifest-path "$WORK_DIR/Cargo.toml" \
  -p haqumei-wasm --locked --release --target wasm32-unknown-unknown
rm -rf "$OUT_DIR"
wasm-bindgen --target web --out-dir "$OUT_DIR" \
  "$WORK_DIR/target/wasm32-unknown-unknown/release/haqumei_wasm.wasm"

cmp "$OUT_DIR/haqumei_wasm_bg.wasm" "$RUNTIME_DIR/haqumei_wasm_bg.wasm"
cmp "$OUT_DIR/haqumei_wasm.js" "$RUNTIME_DIR/haqumei_wasm.js"
cmp "$OUT_DIR/haqumei_wasm.d.ts" "$RUNTIME_DIR/haqumei_wasm.d.ts"
node "$ROOT/scripts/verify-runtime.mjs"
echo "Web build matches committed runtime assets."
