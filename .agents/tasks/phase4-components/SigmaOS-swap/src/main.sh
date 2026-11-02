#!/usr/bin/env bash
# SigmaOS Swap Policy — swap/zswap configuration
# CachyOS-inspired: zswap + zram tuned for performance workloads.
set -euo pipefail

usage() {
    cat >&2 <<EOF
usage: $(basename "$0") <command> [options]

commands:
  status            Show current swap and zswap status
  enable-zswap      Enable zswap with tuned parameters
  enable-zram <mb>  Enable a zram swap device of <mb> MB
  disable           Disable zswap/zram and revert to swapfile
  -h, --help        Show this help
EOF
    exit 1
}

cmd_status() {
    echo "=== Swap devices ==="
    swapon --show 2>/dev/null || echo "  (no swap devices)"
    echo
    echo "=== Zswap ==="
    local enabled=$(cat /sys/module/zswap/parameters/enabled 2>/dev/null || echo "N")
    if [[ "$enabled" == "Y" ]]; then
        echo "  zswap: enabled"
    else
        echo "  zswap: disabled or not available"
    fi
    echo "  compressor: $(cat /sys/module/zswap/parameters/compressor 2>/dev/null || echo n/a)"
    echo "  zpool: $(cat /sys/module/zswap/parameters/zpool 2>/dev/null || echo n/a)"
}

cmd_enable_zswap() {
    echo "[swap] Enabling zswap (zstd + zsmalloc)..."
    echo "  (Real impl: echo zstd > /sys/module/zswap/parameters/compressor)"
    echo "  (Real impl: echo zsmalloc > /sys/module/zswap/parameters/zpool)"
    echo "  (Real impl: echo Y > /sys/module/zswap/parameters/enabled)"
    echo "[swap] zswap enabled."
}

cmd_enable_zram() {
    local mb="${1:-}"
    [[ -z "$mb" ]] && { echo "error: size in MB required" >&2; return 1; }
    echo "[swap] Setting up zram0 ($mb MB) with zstd compression..."
    echo "  (Real impl: modprobe zram && echo zstd > /sys/block/zram0/comp_algorithm)"
    echo "  (Real impl: echo $mb > /sys/block/zram0/disksize && mkswap /dev/zram0 && swapon -p 100 /dev/zram0)"
    echo "[swap] zram swap enabled at priority 100."
}

cmd_disable() {
    echo "[swap] Disabling zswap and zram..."
    echo "  (Real impl: swapoff /dev/zram0 && echo 1 > /sys/block/zram0/reset)"
    echo "  (Real impl: echo N > /sys/module/zswap/parameters/enabled)"
    echo "[swap] Reverted to swapfile policy."
}

main() {
    local cmd="${1:-}"; shift || true
    case "$cmd" in
        status) cmd_status ;;
        enable-zswap) cmd_enable_zswap ;;
        enable-zram) cmd_enable_zram "$@" ;;
        disable) cmd_disable ;;
        -h|--help|"") usage ;;
        *) echo "Unknown command: $cmd" >&2; usage ;;
    esac
}
main "$@"
