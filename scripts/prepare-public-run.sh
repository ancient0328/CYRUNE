#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PUBLIC_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FREE_ROOT="$PUBLIC_ROOT/free/v0.1/0"
STATE_ROOT="$FREE_ROOT/target/public-run"
CYRUNE_HOME="$STATE_ROOT/home"

cd "$FREE_ROOT"
rm -rf "$STATE_ROOT"
cargo build --quiet --release --manifest-path "$FREE_ROOT/Cargo.toml" --bin cyr --bin cyrune-daemon
install -d "$STATE_ROOT/bin" "$STATE_ROOT/home"
install -m 0755 "$FREE_ROOT/target/release/cyr" "$STATE_ROOT/bin/cyr"
install -m 0755 "$FREE_ROOT/target/release/cyrune-daemon" "$STATE_ROOT/bin/cyrune-daemon"
