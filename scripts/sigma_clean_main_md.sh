#!/usr/bin/env bash
# =============================================================================
# sigma_clean_main_md.sh
# Remove .md and .txt files from the main branch of ALL SigmaOS-prefixed repos.
# Moves them to a 'docs' branch instead of discarding them.
# Idempotent: if files are already absent from main the repo is skipped.
# =============================================================================

set -euo pipefail

unset GITHUB_TOKEN

OWNER="AaryanSinghChauhan09"
REPO_PREFIX="SigmaOS"
WORK_DIR="/tmp/sigma_clean_work_$$"
DOCS_BRANCH="docs"
COMMIT_MSG="chore: move docs to docs branch, keep main code-only"

count_repos_processed=0
count_repos_skipped=0
count_repos_failed=0

LOG_FILE="/tmp/sigma_clean_md_$(date +%Y%m%d_%H%M%S).log"

log() {
  local msg="[$(date '+%Y-%m-%d %H:%M:%S')] $*"
  echo "$msg" | tee -a "$LOG_FILE"
}

cleanup() {
  rm -rf "${WORK_DIR}"
}
trap cleanup EXIT

log "=== SigmaOS Clean-Main-Docs Script ==="
log "Owner      : ${OWNER}"
log "Prefix     : ${REPO_PREFIX}"
log "Log file   : ${LOG_FILE}"

# Verify prerequisites
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

mkdir -p "${WORK_DIR}"

# ---------------------------------------------------------------------------
# Fetch all repos matching prefix (up to 200)
# ---------------------------------------------------------------------------
log "Fetching repositories for ${OWNER} matching prefix '${REPO_PREFIX}' ..."
mapfile -t REPOS < <(
  gh repo list "${OWNER}" \
    --limit 200 \
    --json name \
    --jq '.[].name' \
  | grep -i "^${REPO_PREFIX}" || true
)

log "Found ${#REPOS[@]} matching repo(s): ${REPOS[*]:-none}"

if [[ ${#REPOS[@]} -eq 0 ]]; then
  log "No repos found. Exiting."
  exit 0
fi

# ---------------------------------------------------------------------------
# Per-repo processing
# ---------------------------------------------------------------------------
process_repo() {
  local repo_name="$1"
  local full_repo="${OWNER}/${repo_name}"
  local clone_dir="${WORK_DIR}/${repo_name}"

  log ""
  log "=== Processing ${full_repo} ==="

  # Clone (shallow enough to be fast, but we need full history for branch ops)
  if [[ -d "${clone_dir}" ]]; then
    log "  Repo already cloned; fetching latest ..."
    git -C "${clone_dir}" fetch --all --prune 2>>"${LOG_FILE}"
  else
    log "  Cloning ${full_repo} ..."
    gh repo clone "${full_repo}" "${clone_dir}" -- --quiet 2>>"${LOG_FILE}"
  fi

  # Ensure we are on main
  git -C "${clone_dir}" checkout main 2>>"${LOG_FILE}"
  git -C "${clone_dir}" pull origin main --ff-only 2>>"${LOG_FILE}"

  # Collect .md and .txt files tracked on main
  mapfile -t doc_files < <(
    git -C "${clone_dir}" ls-files '*.md' '*.txt' 2>/dev/null || true
  )

  if [[ ${#doc_files[@]} -eq 0 ]]; then
    log "  SKIP — no .md/.txt files on main for ${repo_name}."
    (( count_repos_skipped++ )) || true
    return 0
  fi

  log "  Found ${#doc_files[@]} doc file(s): ${doc_files[*]}"

  # ------------------------------------------------------------------
  # Set up docs branch
  # ------------------------------------------------------------------
  local remote_has_docs=false
  if git -C "${clone_dir}" ls-remote --exit-code --heads origin "${DOCS_BRANCH}" &>/dev/null; then
    remote_has_docs=true
  fi

  if $remote_has_docs; then
    # Checkout existing docs branch
    git -C "${clone_dir}" checkout "${DOCS_BRANCH}" 2>>"${LOG_FILE}" \
      || git -C "${clone_dir}" checkout -b "${DOCS_BRANCH}" \
              "origin/${DOCS_BRANCH}" 2>>"${LOG_FILE}"
    git -C "${clone_dir}" pull origin "${DOCS_BRANCH}" --ff-only 2>>"${LOG_FILE}" || true
  else
    # Create docs branch off main
    git -C "${clone_dir}" checkout -b "${DOCS_BRANCH}" 2>>"${LOG_FILE}"
  fi

  # Copy doc files onto docs branch (they may already be there — idempotent)
  git -C "${clone_dir}" checkout main -- "${doc_files[@]}" 2>>"${LOG_FILE}"

  # Stage and commit on docs branch if there are changes
  local docs_changed
  docs_changed=$(git -C "${clone_dir}" status --porcelain 2>/dev/null | wc -l)
  if [[ $docs_changed -gt 0 ]]; then
    git -C "${clone_dir}" add -- "${doc_files[@]}" 2>>"${LOG_FILE}"
    git -C "${clone_dir}" \
      -c user.name="sigma-bot" \
      -c user.email="bot@sigmaos.local" \
      commit -m "docs: add/update documentation files from main" \
      2>>"${LOG_FILE}"
    git -C "${clone_dir}" push origin "${DOCS_BRANCH}" 2>>"${LOG_FILE}"
    log "  Pushed doc files to '${DOCS_BRANCH}' branch."
  else
    log "  docs branch already up-to-date."
  fi

  # ------------------------------------------------------------------
  # Remove doc files from main
  # ------------------------------------------------------------------
  git -C "${clone_dir}" checkout main 2>>"${LOG_FILE}"
  git -C "${clone_dir}" pull origin main --ff-only 2>>"${LOG_FILE}"

  # Re-check: files may have been removed by a previous run
  mapfile -t still_present < <(
    git -C "${clone_dir}" ls-files "${doc_files[@]}" 2>/dev/null || true
  )

  if [[ ${#still_present[@]} -eq 0 ]]; then
    log "  Doc files already absent from main — nothing to remove."
    (( count_repos_skipped++ )) || true
    return 0
  fi

  git -C "${clone_dir}" rm --cached -r -- "${still_present[@]}" 2>>"${LOG_FILE}"
  # Also remove from working tree
  for f in "${still_present[@]}"; do
    rm -f "${clone_dir}/${f}"
  done

  git -C "${clone_dir}" \
    -c user.name="sigma-bot" \
    -c user.email="bot@sigmaos.local" \
    commit -m "${COMMIT_MSG}" \
    2>>"${LOG_FILE}"

  git -C "${clone_dir}" push origin main 2>>"${LOG_FILE}"
  log "  Pushed cleaned main (removed ${#still_present[@]} doc file(s))."

  (( count_repos_processed++ )) || true
}

# ---------------------------------------------------------------------------
# Process each repo
# ---------------------------------------------------------------------------
for repo in "${REPOS[@]}"; do
  if process_repo "$repo"; then
    : # success logged inside
  else
    log "  ERROR — Failed to process ${repo}."
    (( count_repos_failed++ )) || true
  fi
done

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
log ""
log "============================================================"
log "  SUMMARY"
log "  Repos found      : ${#REPOS[@]}"
log "  Repos processed  : ${count_repos_processed}"
log "  Repos skipped    : ${count_repos_skipped}"
log "  Repos failed     : ${count_repos_failed}"
log "  Full log         : ${LOG_FILE}"
log "============================================================"

if [[ $count_repos_failed -gt 0 ]]; then
  exit 1
fi
exit 0
