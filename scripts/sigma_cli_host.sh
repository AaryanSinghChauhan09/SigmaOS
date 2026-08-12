#!/usr/bin/env bash
# Host wrapper for sigma-cli commands (maps to automation scripts on dev machine).
# Usage: sigma_cli_host.sh <command> [args...]
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CMD="${1:-}"

# ANSI colour helpers
_cyan()  { printf '\033[1;36m%s\033[0m' "$*"; }
_green() { printf '\033[1;32m%s\033[0m' "$*"; }
_red()   { printf '\033[1;31m%s\033[0m' "$*"; }
_bold()  { printf '\033[1m%s\033[0m' "$*"; }

_usage() {
  echo "$(_cyan "Σ SigmaOS Host CLI") — dev machine wrapper"
  echo ""
  echo "$(_bold "Usage:") sigma_cli_host.sh <command> [args...]"
  echo ""
  echo "$(_bold "Commands:")"
  echo "  update              Pull latest sources and rebuild"
  echo "  backup              Snapshot the current workspace"
  echo "  sync    [args...]   Git sync (delegates to sigma_git_sync.sh)"
  echo "  branch-check [args] Verify branch naming/parity rules"
  echo "  automation <sub>    Run an automation sub-task"
  echo "  profile             List available build profiles"
  echo "  status              Show repo status summary"
  echo "  version             Show CLI version"
  echo ""
  echo "Run with --help after any command for more detail."
  exit "${1:-1}"
}

case "$CMD" in
  --help|-h|help)
    _usage 0
    ;;

  version)
    echo "$(_cyan "Σ SigmaOS Host CLI") v1.1.0"
    echo "  Root : $ROOT"
    echo "  Shell: $BASH_VERSION"
    ;;

  update)
    echo "$(_cyan "Σ") Updating SigmaOS workspace..."
    "${ROOT}/scripts/sigma_automation.sh" update
    echo "$(_green "✓") Update complete."
    ;;

  backup)
    echo "$(_cyan "Σ") Creating workspace backup..."
    "${ROOT}/scripts/sigma_automation.sh" backup
    echo "$(_green "✓") Backup complete."
    ;;

  sync)
    echo "$(_cyan "Σ") Syncing repository..."
    "${ROOT}/scripts/sigma_git_sync.sh" "${@:2}"
    ;;

  branch-check)
    "${ROOT}/scripts/ci_branch_check.sh" "${@:2}"
    ;;

  automation)
    if [[ -z "${2:-}" ]]; then
      echo "$(_red "error:") automation requires a sub-command." >&2
      echo "  Available: update, backup, clean, lint, test" >&2
      exit 1
    fi
    "${ROOT}/scripts/sigma_automation.sh" "${2}"
    ;;

  profile)
    echo "$(_bold "Available build profiles:")"
    echo "  desktop    — Full GUI + driver set (default)"
    echo "  minimal    — Kernel + essential userspace only"
    echo "  cloud      — Headless, optimised for server/VM deployment"
    echo "  embedded   — RTOS-style, stripped memory footprint"
    echo ""
    echo "Example: sigma build --profile minimal"
    echo "Config:  docs/examples/sigma_profile.example"
    ;;

  status)
    echo "$(_bold "Repository status:")"
    git -C "${ROOT}" status --short --branch 2>/dev/null || echo "  (git not available)"
    echo ""
    echo "$(_bold "Recent commits:")"
    git -C "${ROOT}" log --oneline -5 2>/dev/null || true
    ;;

  "")
    echo "$(_red "error:") no command given." >&2
    _usage 1
    ;;

  *)
    echo "$(_red "error:") unknown command '$CMD'." >&2
    _usage 1
    ;;
esac
