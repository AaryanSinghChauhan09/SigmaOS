#!/usr/bin/env bash
# SigmaOS automation engine — updates, backups, recovery checks, wiki/doc sync hooks.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_DIR="${ROOT}/.sigma/logs"
BACKUP_DIR="${ROOT}/.sigma/backups"
mkdir -p "$LOG_DIR" "$BACKUP_DIR"

log() { echo "[sigma-automation] $*"; }

cmd_backup() {
  local stamp
  stamp="$(date +%Y%m%d-%H%M%S)"
  local out="${BACKUP_DIR}/sigmaos-${stamp}.tar.gz"
  log "Creating backup archive: $out"
  tar -czf "$out" \
    --exclude='./.git' \
    --exclude='./node_modules' \
    --exclude='./build' \
    -C "$ROOT" .
  log "Backup complete."
}

cmd_update_check() {
  log "Checking repository status..."
  git -C "$ROOT" fetch --all --prune || true
  git -C "$ROOT" status -sb
  log "Run 'make iso' after merging updates."
}

cmd_meta_check() {
  log "Meta-distro subsystem file scan..."
  local ok=0
  for f in \
    kernel/subsystems/sigma_meta_distro.rs \
    kernel/subsystems/sigma_game_layer.rs \
    kernel/scheduler/sigma_sched.rs \
    kernel/scheduler/sigma_sched_profiles.rs \
    kernel/core/boot/sigma_immutable_root.rs \
    kernel/recovery/sigma_recovery.rs \
    kernel/recovery/sigma_recovery_gui.rs \
    sigma_pkg_registry/Cargo.toml \
    desktop/zenith/sigma_zenith.rs \
    include/sigma_meta_distro.rs; do
    if [[ -f "${ROOT}/${f}" ]]; then
      log "OK  ${f}"
    else
      log "MISSING ${f}"
      ok=1
    fi
  done
  return "$ok"
}

cmd_recovery_check() {
  log "Recovery readiness scan..."
  local ok=0
  for f in \
    kernel/resilience/sigma_rollback.rs \
    kernel/resilience/sigma_micro_fallback.rs \
    kernel/core/sovereign_kernel_main.rs; do
    if [[ -f "${ROOT}/${f}" ]]; then
      log "OK  ${f}"
    else
      log "MISSING ${f}"
      ok=1
    fi
  done
  return "$ok"
}

cmd_wiki_sync() {
  log "Syncing docs into wiki_repo/ mirrors..."
  mkdir -p "${ROOT}/wiki_repo"
  cp -f "${ROOT}/docs/COMPETITOR_COMPARISON.md" "${ROOT}/wiki_repo/Competitor-Comparison.md" 2>/dev/null || true
  cp -f "${ROOT}/PHASE_A_EXECUTION_CHECKLIST.md" "${ROOT}/wiki_repo/Phase-A-Execution-Checklist.md" 2>/dev/null || true
  cp -f "${ROOT}/docs/SIGMAOS_DIFFERENTIATION_BLUEPRINT.md" "${ROOT}/wiki_repo/SigmaOS-Differentiation-Blueprint.md" 2>/dev/null || true
  cp -f "${ROOT}/docs/PHASE_7_8_ROADMAP.md" "${ROOT}/wiki_repo/Phase-7-8-Roadmap.md" 2>/dev/null || true
  cp -f "${ROOT}/FEATURE_MATRIX.md" "${ROOT}/wiki_repo/Feature-Matrix.md" 2>/dev/null || true
  cp -f "${ROOT}/PHASE_B_EXECUTION_CHECKLIST.md" "${ROOT}/wiki_repo/Phase-B-Execution-Checklist.md" 2>/dev/null || true
  cp -f "${ROOT}/PHASE_C_EXECUTION_CHECKLIST.md" "${ROOT}/wiki_repo/Phase-C-Execution-Checklist.md" 2>/dev/null || true
  cp -f "${ROOT}/docs/META_DISTRO_UNIFIED_ENGINE.md" "${ROOT}/wiki_repo/Meta-Distro-Unified-Engine.md" 2>/dev/null || true
  "${ROOT}/scripts/doxygen_wiki_export.sh" || true
  "${ROOT}/scripts/sigma_branch_sync.sh" --report 2>/dev/null || true
  log "wiki_repo mirrors updated (push main to trigger wiki-sync workflow)."
}

cmd_update() {
  cmd_update_check
  cmd_wiki_sync
  log "Phase 7–8: run ./scripts/ci_branch_check.sh before merge."
}

cmd_quality_check() {
  log "Running sigma-quality gate checks..."
  "${ROOT}/scripts/sigma_quality_check.sh" "${@}"
}

usage() {
  cat <<EOF
Usage: sigma_automation.sh <command>
Commands:
  backup          Create timestamped source backup tarball
  update          Fetch status + mirror wiki_repo/ (Phase 7–8)
  update-check    Show git status after fetch
  recovery-check  Verify rollback/resilience files exist
  meta-check      Verify Phase C meta-distro subsystem files
  wiki-sync       Mirror key docs into wiki_repo/
  quality-check   Run all quality gates (stubs, SPDX, creds, problems)
  quality-check --strict  Same but fail on warnings too
EOF
}

main() {
  local cmd="${1:-}"
  case "$cmd" in
    backup) cmd_backup ;;
    update) cmd_update ;;
    update-check) cmd_update_check ;;
    recovery-check) cmd_recovery_check ;;
    meta-check) cmd_meta_check ;;
    wiki-sync) cmd_wiki_sync ;;
    quality-check) cmd_quality_check "${@:2}" ;;
    *) usage; exit 1 ;;
  esac
}

main "$@"
