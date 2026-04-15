#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PUBLIC_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FREE_ROOT="$PUBLIC_ROOT/free/v0.1/0"
STATE_ROOT="$FREE_ROOT/target/public-run"
CYRUNE_HOME="$STATE_ROOT/home"
export CYRUNE_HOME

cd "$FREE_ROOT"
"$STATE_ROOT/bin/cyr" run --no-llm --input "ship-goal public first success"
