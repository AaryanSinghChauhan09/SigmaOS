#!/usr/bin/env bash
# Host wrapper for sigma-cli commands (maps to automation scripts on dev machine).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CMD="${1:-}"

case "$CMD" in
  update)
    "${ROOT}/scripts/sigma_automation.sh" update
    ;;
  branch-check)
    "${ROOT}/scripts/ci_branch_check.sh" "${@:2}"
    ;;
  automation)
    shift
    "${ROOT}/scripts/sigma_automation.sh" "${1:-}"
    ;;
  profile)
    echo "[sigma-cli] profiles: desktop minimal cloud"
    echo "[sigma-cli] example config: docs/examples/sigma_profile.example"
    ;;
  *)
    echo "Usage: sigma_cli_host.sh <update|branch-check|automation|profile> [args...]"
    exit 1
    ;;
esac
