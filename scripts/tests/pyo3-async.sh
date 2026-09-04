#!/usr/bin/env bash
set -Eeuo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$SCRIPT_DIR"

while [ ! -f "$ROOT/Cargo.toml" ] && [ "$ROOT" != "/" ]; do
    ROOT="$(dirname "$ROOT")"
done

export VIRTUAL_ENV="$ROOT/python/.venv"
PYTHON="$VIRTUAL_ENV/bin/python"

export PYTHONHOME="$("$PYTHON" -c 'import sys; print(sys.base_prefix)')"

export PYTHONPATH="$("$PYTHON" -c '
import sys
print(":".join(
    p for p in sys.path
    if "site-packages" in p or "/src" in p
))
')"

cd "$ROOT"

RUST_TEST_NOCAPTURE=1 cargo test \
    -p lyrics \
    --features python-tests \
    --test python-async
