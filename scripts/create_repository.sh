#!/usr/bin/env bash
# Compatibility entry point. The publisher is dry-run by default; pass --apply
# explicitly to create and push repositories.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "$SCRIPT_DIR/publish_registry_repositories.py" "$@"
