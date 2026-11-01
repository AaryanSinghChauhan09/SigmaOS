#!/usr/bin/env bash
# =============================================================================
# sigma_delete_merged_branches.sh
# Delete branches that are fully merged into main on AaryanSinghChauhan09/SigmaOS.
#
# Usage:
#   ./sigma_delete_merged_branches.sh           # dry-run (default)
#   ./sigma_delete_merged_branches.sh --apply   # actually delete branches
# =============================================================================

set -euo pipefail

unset GITHUB_TOKEN

OWNER="AaryanSinghChauhan09"
REPO="SigmaOS"
FULL_REPO="${OWNER}/${REPO}"

# Branches that must never be deleted
PROTECTED_BRANCHES=(main docs develop staging)
PROTECTED_PREFIXES=(release/)

DRY_RUN=true
APPLY_FLAG="--apply"

# Parse args
for arg in "$@"; do
  if [[ "$arg" == "${APPLY_FLAG}" ]]; then
    DRY_RUN=false
  fi
done

count_deleted=0
count_retained=0
count_skipped_protected=0
count_failed=0

LOG_FILE="/tmp/sigma_delete_branches_$(date +%Y%m%d_%H%M%S).log"
WORK_DIR="/tmp/sigma_delete_work_$$"

log() {
  local msg="[$(date '+%Y-%m-%d %H:%M:%S')] $*"
  echo "$msg" | tee -a "$LOG_FILE"
}

cleanup() {
  rm -rf "${WORK_DIR}"
}
trap cleanup EXIT

log "=== SigmaOS Delete Merged Branches ==="
log "Repository : ${FULL_REPO}"
log "Mode       : $( $DRY_RUN && echo 'DRY-RUN (pass --apply to delete)' || echo 'APPLY — branches WILL be deleted' )"
log "Log file   : ${LOG_FILE}"

# ---------------------------------------------------------------------------
# Prerequisites
# ---------------------------------------------------------------------------
for cmd in gh git; do
  if ! command -v "$cmd" &>/dev/null; then
    log "ERROR: '${cmd}' not found."
    exit 1
  fi
done

if ! gh auth status &>/dev/null; then
  log "ERROR: Not authenticated with gh. Run: gh auth login"
  exit 1
fi

# ---------------------------------------------------------------------------
# Clone / fetch a bare mirror for merge-base checks
# ---------------------------------------------------------------------------
mkdir -p "${WORK_DIR}"
BARE_DIR="${WORK_DIR}/${REPO}.git"

log "Cloning bare mirror of ${FULL_REPO} for merge-base checks ..."
gh repo clone "${FULL_REPO}" "${BARE_DIR}" -- --bare --quiet 2>>"${LOG_FILE}"

# Resolve HEAD of main
MAIN_SHA=$(git -C "${BARE_DIR}" rev-parse "refs/heads/main" 2>/dev/null || true)
if [[ -z "${MAIN_SHA}" ]]; then
  log "ERROR: Could not resolve 'main' branch SHA."
  exit 1
fi
log "main HEAD : ${MAIN_SHA}"

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
is_protected() {
  local branch="$1"
  for b in "${PROTECTED_BRANCHES[@]}"; do
    [[ "$branch" == "$b" ]] && return 0
  done
  for prefix in "${PROTECTED_PREFIXES[@]}"; do
    [[ "$branch" == ${prefix}* ]] && return 0
  done
  return 1
}

is_ancestor() {
  local branch_sha="$1"
  # Returns 0 (true) if branch_sha is an ancestor of (or equal to) MAIN_SHA
  git -C "${BARE_DIR}" merge-base --is-ancestor "${branch_sha}" "${MAIN_SHA}" 2>/dev/null
}

delete_branch() {
  local branch="$1"
  # Use gh api to delete the remote ref
  gh api \
    --method DELETE \
    "/repos/${OWNER}/${REPO}/git/refs/heads/${branch}" \
    2>>"${LOG_FILE}"
}

# ---------------------------------------------------------------------------
# Fetch all branches via gh API (handles repos with 100+ branches)
# ---------------------------------------------------------------------------
log "Fetching all branches from ${FULL_REPO} ..."
mapfile -t ALL_BRANCHES < <(
  gh api \
    --paginate \
    "/repos/${OWNER}/${REPO}/branches" \
    --jq '.[].name' \
  2>>"${LOG_FILE}"
)

log "Total branches found: ${#ALL_BRANCHES[@]}"

# ---------------------------------------------------------------------------
# Evaluate each branch
# ---------------------------------------------------------------------------
log ""
log "--- Branch evaluation ---"

WOULD_DELETE=()
RETAINED=()

for branch in "${ALL_BRANCHES[@]}"; do

  if is_protected "$branch"; then
    log "  PROTECT  ${branch}"
    (( count_skipped_protected++ )) || true
    RETAINED+=("$branch (protected)")
    continue
  fi

  # Get branch SHA from the bare clone
  branch_sha=$(
    git -C "${BARE_DIR}" rev-parse "refs/heads/${branch}" 2>/dev/null || echo ""
  )

  if [[ -z "$branch_sha" ]]; then
    # Possibly the branch appeared after clone; try API
    branch_sha=$(
      gh api "/repos/${OWNER}/${REPO}/branches/${branch}" \
        --jq '.commit.sha' 2>/dev/null || echo ""
    )
  fi

  if [[ -z "$branch_sha" ]]; then
    log "  WARN     ${branch} — could not resolve SHA; skipping."
    (( count_retained++ )) || true
    RETAINED+=("$branch (sha-unknown)")
    continue
  fi

  if is_ancestor "$branch_sha"; then
    log "  MERGED   ${branch} (${branch_sha:0:8}) → would delete"
    WOULD_DELETE+=("$branch")
  else
    log "  UNMERGED ${branch} (${branch_sha:0:8}) → retain"
    (( count_retained++ )) || true
    RETAINED+=("$branch")
  fi

done

# ---------------------------------------------------------------------------
# Delete phase
# ---------------------------------------------------------------------------
log ""
if [[ ${#WOULD_DELETE[@]} -eq 0 ]]; then
  log "No merged branches to delete."
else
  if $DRY_RUN; then
    log "=== DRY-RUN: the following ${#WOULD_DELETE[@]} branch(es) WOULD be deleted ==="
    for b in "${WOULD_DELETE[@]}"; do
      log "  - ${b}"
    done
    log "(Re-run with --apply to actually delete them.)"
  else
    log "=== APPLY: deleting ${#WOULD_DELETE[@]} merged branch(es) ==="
    for branch in "${WOULD_DELETE[@]}"; do
      log "  Deleting ${branch} ..."
      if delete_branch "$branch"; then
        log "    OK — deleted ${branch}"
        (( count_deleted++ )) || true
      else
        log "    FAIL — could not delete ${branch}"
        (( count_failed++ )) || true
      fi
      sleep 1   # gentle rate-limit
    done
  fi
fi

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
log ""
log "============================================================"
log "  SUMMARY for ${FULL_REPO}"
log "  Mode                  : $( $DRY_RUN && echo 'DRY-RUN' || echo 'APPLY' )"
log "  Total branches        : ${#ALL_BRANCHES[@]}"
log "  Protected (skipped)   : ${count_skipped_protected}"
log "  Unmerged (retained)   : ${count_retained}"
if $DRY_RUN; then
log "  Would delete          : ${#WOULD_DELETE[@]}"
else
log "  Deleted               : ${count_deleted}"
log "  Delete failures       : ${count_failed}"
fi
log "  Full log              : ${LOG_FILE}"
log "============================================================"

if [[ $count_failed -gt 0 ]]; then
  exit 1
fi
exit 0
