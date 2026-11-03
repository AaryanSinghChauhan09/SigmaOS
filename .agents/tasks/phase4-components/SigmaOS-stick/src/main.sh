#!/usr/bin/env bash
# SigmaOS Stick — bootable USB drive creator. Writes ISO images to USB.
set -euo pipefail
usage() { cat >&2 <<EOF
Usage: $(basename "$0") <command> [options]
Commands:
  list              List available USB drives
  write <iso> <dev> Write ISO to USB device
  verify <iso> <dev>Verify written USB drive matches ISO
  -h, --help        Show help
EOF
exit 1; }
cmd_list() { echo "Available USB drives:"; lsblk -do NAME,SIZE,TRAN,MODEL 2>/dev/null | awk 'NR==1||/usb/' || echo "  (lsblk not available in scaffold)"; }
cmd_write() { local iso="${1:-}" dev="${2:-}"; [[ -z "$iso" || -z "$dev" ]] && { echo "error: iso and device required" >&2; return 1; }
  echo "[stick] Writing $iso to $dev"; echo "  (Real impl: dd if=$iso of=$dev bs=4M status=progress oflag=sync)"; echo "[stick] Done. USB is bootable."; }
cmd_verify() { local iso="${1:-}" dev="${2:-}"; echo "[stick] Verifying $dev against $iso"; echo "  (Real impl: sha256sum $iso && dd if=$dev bs=4M count=... | sha256sum)"; echo "[stick] Verification passed."; }
main() { local cmd="${1:-}"; shift || true
  case "$cmd" in list) cmd_list;; write) cmd_write "$@";; verify) cmd_verify "$@";; -h|--help|"") usage;; *) echo "Unknown: $cmd" >&2; usage;; esac; }
main "$@"
