#!/usr/bin/env bash
# =============================================================================
# sigma_merge_all_prs_gh.sh
# Merge all open PRs on AaryanSinghChauhan09/SigmaOS using the gh CLI.
# Idempotent: safe to re-run; already-merged/closed PRs are skipped.
# =============================================================================

set -euo pipefail

unset GITHUB_TOKEN

OWNER="AaryanSinghChauhan09"
REPO="SigmaOS"
FULL_REPO="${OWNER}/${REPO}"

# Rate-limit sleep between merge attempts (seconds)
SLEEP_BETWEEN=3

# Counters
count_merged=0
count_skipped=0
count_failed=0

LOG_FILE="/tmp/sigma_merge_prs_$(date +%Y%m%d_%H%M%S).log"

log() {
  local msg="[$(date '+%Y-%m-%d %H:%M:%S')] $*"
  echo "$msg" | tee -a "$LOG_FILE"
}

log "=== SigmaOS PR Merge Script ==="
log "Repository : ${FULL_REPO}"
log "Log file   : ${LOG_FILE}"

# Verify gh CLI is available and authenticated
if ! command -v gh &>/dev/null; then
  log "ERROR: 'gh' CLI not found. Install from https://cli.github.com/"
  exit 1
fi

if ! gh auth status &>/dev/null; then
  log "ERROR: Not authenticated. Run: gh auth login"
  exit 1
fi

# Fetch all open PR numbers in one call (up to 1000)
log "Fetching open PRs for ${FULL_REPO} ..."
mapfile -t PR_NUMBERS < <(
  gh pr list \
    --repo "${FULL_REPO}" \
    --state open \
    --limit 1000 \
    --json number \
    --jq '.[].number'
)

total=${#PR_NUMBERS[@]}
log "Found ${total} open PR(s)."

if [[ $total -eq 0 ]]; then
  log "Nothing to merge. Exiting."
  exit 0
fi

# ---------------------------------------------------------------------------
# Merge loop
# ---------------------------------------------------------------------------
for pr_number in "${PR_NUMBERS[@]}"; do
  log "--- PR #${pr_number} ---"

  # Re-check current state (idempotency: another run may have merged it)
  pr_state=$(
    gh pr view "${pr_number}" \
      --repo "${FULL_REPO}" \
      --json state \
      --jq '.state' 2>/dev/null || echo "UNKNOWN"
  )

  if [[ "${pr_state}" == "MERGED" || "${pr_state}" == "CLOSED" ]]; then
    log "  SKIP — PR #${pr_number} is already ${pr_state}."
    (( count_skipped++ )) || true
    sleep 1
    continue
  fi

  # Check mergeability
  mergeable=$(
    gh pr view "${pr_number}" \
      --repo "${FULL_REPO}" \
      --json mergeable \
      --jq '.mergeable' 2>/dev/null || echo "UNKNOWN"
  )

  if [[ "${mergeable}" == "CONFLICTING" ]]; then
    log "  SKIP — PR #${pr_number} has merge conflicts (mergeable=${mergeable})."
    (( count_skipped++ )) || true
    sleep "$SLEEP_BETWEEN"
    continue
  fi

  # Attempt squash merge first; fall back to regular merge
  merge_success=false

  if gh pr merge "${pr_number}" \
       --repo "${FULL_REPO}" \
       --squash \
       --delete-branch=false \
       --body "" \
       2>>"${LOG_FILE}"; then
    log "  OK    — PR #${pr_number} merged (squash)."
    merge_success=true
  else
    log "  WARN  — Squash failed for PR #${pr_number}. Trying --merge ..."
    if gh pr merge "${pr_number}" \
         --repo "${FULL_REPO}" \
         --merge \
         --delete-branch=false \
         --body "" \
         2>>"${LOG_FILE}"; then
      log "  OK    — PR #${pr_number} merged (--merge fallback)."
      merge_success=true
    else
      log "  FAIL  — Could not merge PR #${pr_number}."
    fi
  fi

  if $merge_success; then
    (( count_merged++ )) || true
  else
    (( count_failed++ )) || true
  fi

  sleep "$SLEEP_BETWEEN"
done

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
log ""
log "============================================================"
log "  SUMMARY for ${FULL_REPO}"
log "  Total PRs found : ${total}"
log "  Merged          : ${count_merged}"
log "  Skipped         : ${count_skipped}"
log "  Failed          : ${count_failed}"
log "  Full log        : ${LOG_FILE}"
log "============================================================"

if [[ $count_failed -gt 0 ]]; then
  exit 1
fi
exit 0
