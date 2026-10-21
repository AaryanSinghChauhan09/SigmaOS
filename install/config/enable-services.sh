#!/bin/bash
# SigmaOS enable-services (install/config/enable-services.sh)
# Enables the core system services needed for a working desktop.
# Sources the shared helper to avoid duplication (DRY).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=/dev/null
. "${SCRIPT_DIR}/../helpers/enable-services.sh"

enable_sigmaos_services
