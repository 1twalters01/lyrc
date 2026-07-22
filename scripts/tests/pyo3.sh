#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

export VIRTUAL_ENV="$ROOT/.venv"
export PYTHONHOME="$($VIRTUAL_ENV/bin/python -c 'import sys; print(sys.base_prefix)')"

export PYTHONPATH="$(uv run python -c '
import sys
print(":".join(
    p for p in sys.path
    if "site-packages" in p or "/src" in p
))
')"

cd "$ROOT"


cargo test -p lyrics \
    --features=python-tests
    --test python-async \
