#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
UPSTREAM_REPO="https://github.com/o24s/haqumei.git"
UPSTREAM_COMMIT="f31c4342f0d94ffba08ce4b5abeb19b07bac5ac4"
VIBRATO_VERSION="0.7.17"
WORK_DIR="${1:-$ROOT/.work/haqumei}"

rm -rf "$WORK_DIR"
mkdir -p "$(dirname "$WORK_DIR")"
git init -q "$WORK_DIR"
git -C "$WORK_DIR" remote add origin "$UPSTREAM_REPO"
git -C "$WORK_DIR" fetch -q --depth 1 origin "$UPSTREAM_COMMIT"
git -C "$WORK_DIR" checkout -q --detach FETCH_HEAD

git -C "$WORK_DIR" apply "$ROOT/port/upstream.patch"
cp -a "$ROOT/port/haqumei-wasm" "$WORK_DIR/haqumei-wasm"
cp "$ROOT/port/haqumei/src/nani_model_data.rs" "$WORK_DIR/haqumei/src/nani_model_data.rs"

mkdir -p "$WORK_DIR/vendor/vibrato-rkyv"
archive="$(mktemp)"
trap 'rm -f "$archive"' EXIT
curl -L --fail --retry 3 -sS \
  "https://static.crates.io/crates/vibrato-rkyv/vibrato-rkyv-${VIBRATO_VERSION}.crate" \
  -o "$archive"
tar -xzf "$archive" --strip-components=1 -C "$WORK_DIR/vendor/vibrato-rkyv"
(
  cd "$WORK_DIR"
  patch -s -p1 < "$ROOT/port/vibrato-rkyv-0.7.17.patch"
)

actual="$(git -C "$WORK_DIR" rev-parse HEAD)"
test "$actual" = "$UPSTREAM_COMMIT"
echo "Prepared Haqumei $actual in $WORK_DIR"
