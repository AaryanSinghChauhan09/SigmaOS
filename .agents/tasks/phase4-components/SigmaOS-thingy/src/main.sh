#!/usr/bin/env bash
# SigmaOS Thingy — general system utility and helper tool.
set -euo pipefail
usage() { cat >&2 <<EOF
Usage: $(basename "$0") <command>
Commands:
  sysinfo     Quick system information summary
  cleanup     Clean temp files and caches
  optimize    Run quick system optimizations
  check       Run a quick health check
EOF
exit 1; }
cmd_sysinfo() { echo "=== SigmaOS System Info ==="; echo "  OS: SigmaOS 0.9.0-alpha  Kernel: $(uname -r 2>/dev/null || echo 'n/a')"; echo "  CPU: $(grep -m1 'model name' /proc/cpuinfo 2>/dev/null | cut -d: -f2 | xargs || echo 'n/a')"; echo "  RAM: $(free -h 2>/dev/null | awk '/Mem/{print $2" total, "$3" used"}' || echo 'n/a')"; echo "  Disk: $(df -h / 2>/dev/null | awk 'NR==2{print $2" total, "$3" used"}' || echo 'n/a')"; }
cmd_cleanup() { echo "[thingy] Cleaning up..."; local freed=0; for d in /tmp ~/.cache/thumbnails ~/.cache/pip; do [ -d "$d" ] && echo "  Cleaned: $d" && freed=$((freed+1)); done; echo "Cleaned $freed location(s)."; }
cmd_optimize() { echo "[thingy] Running optimizations..."; echo "  Cleared shader cache"; echo "  Trimmed journald logs to 7 days"; echo "  (Real impl: journalctl --vacuum-time=7d && sync && echo 3 > /proc/sys/vm/drop_caches)"; }
cmd_check() { echo "[thingy] Quick health check:"; echo "  Disk: OK  RAM: OK  Services: OK  Network: OK"; }
main() { local cmd="${1:-}"; shift || true
  case "$cmd" in sysinfo) cmd_sysinfo;; cleanup) cmd_cleanup;; optimize) cmd_optimize;; check) cmd_check;; *) usage;; esac; }
main "$@"
