#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
VERIFIER="$SCRIPT_DIR/verify-beta-release-contract.py"

exec python3 "$VERIFIER" "$@"
