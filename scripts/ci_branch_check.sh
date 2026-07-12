#!/usr/bin/env bash
# Verify branch feature parity — required files per FEATURE_MATRIX.md profiles.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
JSON=0
BRANCH="${GITHUB_HEAD_REF:-${GITHUB_REF_NAME:-$(git -C "$ROOT" rev-parse --abbrev-ref HEAD 2>/dev/null || echo unknown)}}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --json) JSON=1; shift ;;
    --branch) BRANCH="$2"; shift 2 ;;
    *) echo "Unknown option: $1"; exit 1 ;;
  esac
done

require_file() {
  local rel="$1"
  if [[ -f "${ROOT}/${rel}" ]]; then
    echo "OK   ${rel}"
    return 0
  fi
  echo "MISS ${rel}"
  return 1
}

profile_for_branch() {
  case "$1" in
    release/standalone|release/dual-boot) echo "desktop" ;;
    release/cloud|release/distributed) echo "cloud" ;;
    release/microkernel|release/rtos) echo "microkernel" ;;
    kernel-exp|kernel-dev) echo "kernel" ;;
    drivers-dev|fs-dev) echo "drivers" ;;
    *) echo "core" ;;
  esac
}

PROFILE="$(profile_for_branch "$BRANCH")"
FAIL=0

CORE_FILES=(
  kernel/net/sigma_net.c
  kernel/net/sigma_net_socket.cpp
  userland/tools/sigma_pod_cli.cpp
  kernel/core/boot/sigma_boot.c
  scripts/sigma_automation.sh
  scripts/sigma_git_sync.sh
  userland/tools/sigma_cli.cpp
  FEATURE_MATRIX.md
  PHASE_A_EXECUTION_CHECKLIST.md
  PHASE_B_EXECUTION_CHECKLIST.md
  scripts/sigma_branch_sync.sh
)

META_SCaffold=(
  kernel/subsystems/sigma_meta_distro.c
  kernel/subsystems/sigma_game_layer.c
  kernel/scheduler/sigma_sched.c
  kernel/scheduler/sigma_sched_profiles.c
  kernel/core/boot/sigma_immutable_root.c
  kernel/recovery/sigma_recovery.c
  kernel/recovery/sigma_recovery_gui.c
  sigma_pkg_registry/README.md
  zenith_desktop/zenith_unified_init.cpp
  include/sigma_meta_distro.h
)

DESKTOP_FILES=(
  zenith_desktop/compositor/sigma_compositor.cpp
  zenith_desktop/wm/sigma_tiling_wm.cpp
  zenith_desktop/personalization/sigma_profile_engine.cpp
  zenith_desktop/theme/sigma_theme_engine.cpp
)

CLOUD_FILES=(
  kernel/core/orchestrator/sigma_orchestrator.cpp
  kernel/core/process/sigma_cgroup.c
)

check_list() {
  local -n arr=$1
  for f in "${arr[@]}"; do
    require_file "$f" || FAIL=1
  done
}

if [[ "$JSON" -eq 1 ]]; then
  echo "{\"branch\":\"${BRANCH}\",\"profile\":\"${PROFILE}\"}"
else
  echo "[ci_branch_check] branch=${BRANCH} profile=${PROFILE}"
fi

check_list CORE_FILES

for f in "${META_SCaffold[@]}"; do
  require_file "$f" || true
done

case "$PROFILE" in
  desktop)
    check_list DESKTOP_FILES
    ;;
  cloud)
    check_list CLOUD_FILES
    ;;
  kernel)
    check_list CLOUD_FILES
    require_file kernel/include/sigma_socket_abi.h || FAIL=1
    ;;
  microkernel)
    require_file kernel/core/sigma_kernel_main.c || FAIL=1
    ;;
  drivers)
    require_file kernel/core/drivers/SovereignE1000.cpp || true
    ;;
esac

if [[ "$FAIL" -ne 0 ]]; then
  echo "[ci_branch_check] FAILED — see FEATURE_MATRIX.md"
  exit 1
fi

echo "[ci_branch_check] PASSED"
exit 0
