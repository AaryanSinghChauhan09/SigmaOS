#!/usr/bin/env bash
# SigmaOS NVIDIA — NVIDIA GPU driver integration and PRIME switching scaffold
# Upstream: nvidia-prime (https://github.com/bosim/FunctionFlip),
#           optimus-manager (https://github.com/Askannz/optimus-manager)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PRIMUS_DRIVER="${SIGMA_GPU_DRIVER:-auto}"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

SigmaOS NVIDIA GPU integration manager

Subcommands:
  detect          Detect NVIDIA hardware and driver status
  install         Install NVIDIA driver (open or proprietary)
  status          Show driver and GPU state
  prime           Manage PRIME GPU switching (integrated ↔ NVIDIA)
  offload <cmd>   Run a command on the NVIDIA GPU (PRIME offload)
  modeset         Show/set kernel modesetting options
  power           Show GPU power profile
  info            Show NVIDIA GPU details
  version         Print manager version

Options:
  --open          Use NVIDIA open-source kernel module (nvidia-open)
  --proprietary   Use proprietary NVIDIA driver (default)

EOF
}

cmd_detect() {
    echo "[nvidia] Detecting NVIDIA hardware..."
    if command -v lspci &>/dev/null; then
        local gpus
        gpus=$(lspci 2>/dev/null | grep -i nvidia || true)
        if [ -n "$gpus" ]; then
            echo "[nvidia] Found:"
            echo "$gpus" | while IFS= read -r line; do echo "  $line"; done
        else
            echo "[nvidia] No NVIDIA GPU detected (lspci: no match)"
        fi
    else
        echo "[nvidia] lspci not available; checking /sys/bus/pci/devices..."
        local found=0
        for d in /sys/bus/pci/devices/*/; do
            local vendor
            vendor=$(cat "$d/vendor" 2>/dev/null || true)
            if [ "$vendor" = "0x10de" ]; then
                echo "  Found NVIDIA PCI device: $(basename "$d")"
                found=1
            fi
        done
        [ "$found" -eq 0 ] && echo "  No NVIDIA GPU found"
    fi
    echo "[nvidia] Driver status:"
    if lsmod 2>/dev/null | grep -q "^nvidia"; then
        echo "  nvidia module: LOADED"
    else
        echo "  nvidia module: NOT LOADED (use: $(basename "$0") install)"
    fi
}

cmd_install() {
    local variant="${1:---proprietary}"
    case "$variant" in
        --open)
            echo "[nvidia] Installing NVIDIA open kernel module..."
            echo "  sigmaos-pkg install nvidia-open nvidia-utils"
            echo "  (scaffold: command printed, not executed)"
            ;;
        --proprietary|*)
            echo "[nvidia] Installing proprietary NVIDIA driver..."
            echo "  sigmaos-pkg install nvidia nvidia-utils nvidia-settings"
            echo "  (scaffold: command printed, not executed)"
            ;;
    esac
    echo "[nvidia] After install, reboot and run: $(basename "$0") detect"
}

cmd_status() {
    echo "=== NVIDIA Driver Status ==="
    if command -v nvidia-smi &>/dev/null; then
        nvidia-smi --query-gpu=name,driver_version,memory.total,temperature.gpu \
            --format=csv,noheader 2>/dev/null | while IFS=, read -r name drv mem temp; do
            echo "  GPU     : $name"
            echo "  Driver  : $drv"
            echo "  VRAM    : $mem"
            echo "  Temp    : $temp"
        done
    else
        echo "  nvidia-smi not found (driver not installed or not running)"
        echo "  Scaffold status: NVIDIA GeForce RTX 4070 Ti | driver 545.29.06 | 12 GiB"
    fi
}

cmd_prime() {
    local action="${1:-status}"
    case "$action" in
        status)
            echo "[prime] Current GPU mode: auto (PRIME)"
            echo "[prime] Integrated: Intel Iris Xe  |  Discrete: NVIDIA GeForce RTX 4070 Ti"
            ;;
        integrated)
            echo "[prime] Switching to integrated GPU only"
            echo "  (writes PRIME mode to /etc/sigmaos/gpu.toml; reboot required)"
            ;;
        nvidia|discrete)
            echo "[prime] Switching to NVIDIA GPU only"
            echo "  (writes PRIME mode to /etc/sigmaos/gpu.toml; reboot required)"
            ;;
        hybrid)
            echo "[prime] Enabling hybrid PRIME offload mode"
            echo "  Use: $(basename "$0") offload <app>  to run apps on NVIDIA"
            ;;
        *)
            echo "[prime] Unknown action: $action  (valid: status integrated nvidia hybrid)" >&2
            exit 1 ;;
    esac
}

cmd_offload() {
    if [ $# -eq 0 ]; then
        echo "Usage: $(basename "$0") offload <command> [args...]" >&2; exit 1
    fi
    echo "[nvidia] Running on NVIDIA GPU (PRIME offload):"
    echo "  Command: $*"
    export __NV_PRIME_RENDER_OFFLOAD=1
    export __NV_PRIME_RENDER_OFFLOAD_PROVIDER=NVIDIA-G0
    export __GLX_VENDOR_LIBRARY_NAME=nvidia
    export __VK_LAYER_NV_optimus=NVIDIA_only
    echo "  Environment: NV_PRIME_RENDER_OFFLOAD=1  GLX_VENDOR=nvidia"
    echo "  (scaffold: env set; executing if binary exists)"
    command -v "$1" &>/dev/null && exec "$@" || echo "  (binary not found: $1)"
}

cmd_modeset() {
    echo "Kernel modesetting status:"
    if [ -f /sys/module/nvidia_drm/parameters/modeset ]; then
        echo "  nvidia_drm.modeset = $(cat /sys/module/nvidia_drm/parameters/modeset)"
    else
        echo "  nvidia_drm not loaded  (set via kernel param: nvidia-drm.modeset=1)"
    fi
    echo "  Recommended: add  nvidia-drm.modeset=1  to kernel cmdline"
}

cmd_power() {
    echo "GPU Power Profile:"
    if command -v nvidia-smi &>/dev/null; then
        nvidia-smi --query-gpu=power.draw,power.limit --format=csv,noheader 2>/dev/null \
            | while IFS=, read -r draw limit; do
            echo "  Draw  : $draw"; echo "  Limit : $limit"
        done
    else
        echo "  Mode  : auto (system-managed)"
        echo "  (nvidia-smi not available)"
    fi
}

cmd_info() {
    echo "NVIDIA GPU Information (scaffold):"
    echo "  Architecture : Ada Lovelace"
    echo "  Model        : GeForce RTX 4070 Ti"
    echo "  VRAM         : 12 GiB GDDR6X"
    echo "  Compute      : CUDA 8.9  |  SM 8.9"
    echo "  Vulkan       : 1.3  |  OpenGL: 4.6"
    echo "  Driver       : 545.29.06 (proprietary)"
}

case "${1:-}" in
    detect)   cmd_detect ;;
    install)  cmd_install "${2:-}" ;;
    status)   cmd_status ;;
    prime)    cmd_prime "${2:-status}" ;;
    offload)  shift; cmd_offload "$@" ;;
    modeset)  cmd_modeset ;;
    power)    cmd_power ;;
    info)     cmd_info ;;
    version)  echo "SigmaOS NVIDIA manager v0.5" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
