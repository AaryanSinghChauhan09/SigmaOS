#!/usr/bin/env bash
# SigmaOS Try — try-before-install live environment launcher.
set -euo pipefail
usage() { cat >&2 <<EOF
Usage: $(basename "$0") <command>
Commands:
  start      Launch the SigmaOS live try environment
  status     Show current live session status
  install    Launch the installer from the try environment
  exit       Exit the live session cleanly
EOF
exit 1; }
cmd_start() { echo "[try] Starting SigmaOS live try environment..."; echo "  Profile: balanced  Backend: linux  Desktop: sigma-wayland"; echo "  (Real impl: sigma-live-session start --profile=balanced)"; echo "[try] Live session active. Type 'sigma-try install' to install permanently."; }
cmd_status() { echo "Live session: active"; echo "  User: liveuser  Home: /home/liveuser (tmpfs)"; echo "  Installer available: yes  (run: sigma-try install)"; }
cmd_install() { echo "[try] Launching installer..."; echo "  (Real impl: sigma-installer --mode=from-live)"; }
cmd_exit() { echo "[try] Exiting live session. Unsaved changes will be lost."; echo "  (Real impl: systemctl poweroff)"; }
main() { local cmd="${1:-}"; shift || true
  case "$cmd" in start) cmd_start;; status) cmd_status;; install) cmd_install;; exit) cmd_exit;; *) usage;; esac; }
main "$@"
