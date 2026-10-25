#!/usr/bin/env bash
# SigmaOS ISO Image Builder Wrapper
# Delegates to build_iso.sh to build the validated bootable desktop ISO.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/build_iso.sh" "$@"
