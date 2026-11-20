#!/usr/bin/env bash
# SigmaOS Mesa3D — Mesa/GPU driver integration and profile manager scaffold
# Upstream: Mesa (https://mesa3d.org/), GLX/EGL, Vulkan ICD
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MESA_VERSION_MIN="24.0"
PROFILE_DIR="${SIGMA_MESA_PROFILE_DIR:-/etc/sigmaos/mesa}"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

SigmaOS Mesa3D integration manager

Subcommands:
  info                    Print detected Mesa and GPU info
  detect                  Re-detect GPU and select Mesa driver
  profile   <name>        Apply a Mesa performance profile
  profiles                List available profiles
  vulkan                  List Vulkan ICD drivers
  opengl                  Show OpenGL/EGL renderer info
  env       [--export]    Print Mesa environment variables
  benchmark [--quick]     Run a quick GPU benchmark (glxgears)
  version                 Print Mesa version info

Profiles: performance | balanced | powersave | debug

EOF
}

print_info() {
    echo "=== SigmaOS Mesa3D Integration ==="
    echo "Minimum Mesa version : $MESA_VERSION_MIN"
    echo "Profile directory    : $PROFILE_DIR"
    # glxinfo / vulkaninfo may not be present in scaffold env
    if command -v glxinfo &>/dev/null; then
        echo "OpenGL renderer      : $(glxinfo -B 2>/dev/null | grep 'OpenGL renderer' | cut -d: -f2 | xargs)"
    else
        echo "OpenGL renderer      : (glxinfo not available in this env)"
    fi
    if command -v vulkaninfo &>/dev/null; then
        echo "Vulkan               : $(vulkaninfo --summary 2>/dev/null | grep deviceName | head -1 | xargs || echo unavailable)"
    else
        echo "Vulkan               : (vulkaninfo not available in this env)"
    fi
}

cmd_detect() {
    echo "[mesa] Re-detecting GPU..."
    # Scaffold: simulate detection via /sys/class/drm
    local gpus
    gpus=$(ls /sys/class/drm/ 2>/dev/null | grep -v render | head -5 || true)
    if [ -z "$gpus" ]; then
        echo "[mesa] No DRM devices found (running in VM or container?)"
        echo "[mesa] Defaulting to llvmpipe (software rendering)"
    else
        echo "[mesa] Found DRM devices:"
        for g in $gpus; do echo "  $g"; done
        echo "[mesa] Mesa driver selection: automatic (via sigmaos-drivers)"
    fi
}

cmd_profile() {
    local profile="${1:?'profile name required'}"
    case "$profile" in
        performance)
            echo "[mesa] Applying performance profile:"
            echo "  MESA_NO_ERROR=1  GALLIUM_HUD=  mesa_glthread=true"
            echo "  (writes to /etc/environment.d/99-mesa-performance.conf)"
            ;;
        balanced)
            echo "[mesa] Applying balanced profile (Mesa defaults)"
            ;;
        powersave)
            echo "[mesa] Applying powersave profile:"
            echo "  MESA_LOADER_DRIVER_OVERRIDE=softpipe  (fallback reduced-power)"
            ;;
        debug)
            echo "[mesa] Applying debug profile:"
            echo "  LIBGL_DEBUG=verbose  MESA_DEBUG=1  EGL_LOG_LEVEL=debug"
            ;;
        *)
            echo "[mesa] Unknown profile: $profile" >&2
            echo "[mesa] Valid profiles: performance balanced powersave debug" >&2
            exit 1
            ;;
    esac
}

cmd_profiles() {
    cat <<'EOF'
Available Mesa performance profiles:
  performance   Maximum GPU performance (mesa_glthread, no error checking)
  balanced      Mesa defaults — best compatibility (recommended)
  powersave     Reduce GPU power (lower clocks, software fallback)
  debug         Verbose Mesa logging for driver debugging

Apply with: $(basename "$0") profile <name>
EOF
}

cmd_vulkan() {
    echo "Vulkan ICD drivers:"
    local icd_dir="/usr/share/vulkan/icd.d"
    if [ -d "$icd_dir" ]; then
        for icd in "$icd_dir"/*.json; do
            echo "  $(basename "$icd")"
        done
    else
        echo "  radeon_icd.x86_64.json  (AMD RADV — scaffold)"
        echo "  intel_icd.x86_64.json   (Intel ANV — scaffold)"
        echo "  (actual ICDs at $icd_dir)"
    fi
}

cmd_opengl() {
    echo "OpenGL/EGL info:"
    if command -v glxinfo &>/dev/null; then
        glxinfo -B 2>/dev/null | grep -E "OpenGL|GLX" | head -10
    else
        echo "  OpenGL vendor  : Mesa Project (scaffold)"
        echo "  OpenGL renderer: llvmpipe (scaffold — no GPU in env)"
        echo "  OpenGL version : 4.5 (Core Profile)"
        echo "  GLX version    : 1.4"
    fi
}

cmd_env() {
    local do_export="${1:-}"
    local prefix=""
    [ "$do_export" = "--export" ] && prefix="export "
    cat <<EOF
${prefix}LIBGL_DRIVERS_PATH=/usr/lib/dri
${prefix}VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/radeon_icd.x86_64.json
${prefix}MESA_GL_VERSION_OVERRIDE=4.5
${prefix}MESA_GLSL_VERSION_OVERRIDE=450
${prefix}mesa_glthread=true
EOF
}

cmd_benchmark() {
    local quick="${1:-}"
    if command -v glxgears &>/dev/null; then
        if [ "$quick" = "--quick" ]; then
            timeout 3 glxgears -info 2>&1 | tail -5 || true
        else
            echo "[mesa] Run: glxgears -info   (press Ctrl-C after a few seconds)"
            timeout 5 glxgears -info 2>&1 | tail -5 || true
        fi
    else
        echo "[mesa] glxgears not available (install mesa-utils)"
        echo "[mesa] Scaffold benchmark: 3000 fps (simulated)"
    fi
}

case "${1:-}" in
    info)       print_info ;;
    detect)     cmd_detect ;;
    profile)    cmd_profile "${2:-}" ;;
    profiles)   cmd_profiles ;;
    vulkan)     cmd_vulkan ;;
    opengl)     cmd_opengl ;;
    env)        cmd_env "${2:-}" ;;
    benchmark)  cmd_benchmark "${2:-}" ;;
    version)    echo "SigmaOS Mesa3D integration v0.5 (Mesa >= $MESA_VERSION_MIN)" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
