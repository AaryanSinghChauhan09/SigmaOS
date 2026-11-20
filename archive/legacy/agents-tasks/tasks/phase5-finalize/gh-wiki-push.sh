#!/usr/bin/env bash
#
# gh-wiki-push.sh — publish the SigmaOS GitHub Wiki pages.
#
# Clones the SigmaOS wiki repository, copies the Markdown pages from this
# phase's wiki/ directory into it, commits, and pushes.
#
# Usage:
#   ./gh-wiki-push.sh                 # use the default repo owner/branch
#   REPO_OWNER=foo ./gh-wiki-push.sh   # override owner
#
# Environment:
#   REPO_OWNER   GitHub owner of the SigmaOS repo (default: AaryanSinghChauhan09)
#   WIKI_BRANCH  wiki branch to push to (default: master)
#   GIT_REMOTE   optional named git remote already configured for the wiki
#                (e.g. "wiki-origin"); if set, the script fetches/uses it
#                instead of cloning fresh.
#
set -euo pipefail

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------
REPO_OWNER="${REPO_OWNER:-AaryanSinghChauhan09}"
REPO_NAME="SigmaOS"
WIKI_BRANCH="${WIKI_BRANCH:-master}"

# Directory this script lives in (phase5-finalize/).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WIKI_SRC_DIR="${SCRIPT_DIR}/wiki"

# Clone target — a sibling temp directory so we never touch the source tree.
CLONE_PARENT="$(mktemp -d)"
WIKI_CLONE_DIR="${CLONE_PARENT}/${REPO_NAME}.wiki"

WIKI_GIT_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}.wiki.git"

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
log() { printf '[gh-wiki-push] %s\n' "$*"; }
err() { printf '[gh-wiki-push] ERROR: %s\n' "$*" >&2; }

cleanup() {
  if [ -n "${CLONE_PARENT:-}" ] && [ -d "${CLONE_PARENT:-}" ]; then
    rm -rf "${CLONE_PARENT}"
  fi
}
trap cleanup EXIT

# ---------------------------------------------------------------------------
# Pre-flight checks
# ---------------------------------------------------------------------------
if [ ! -d "${WIKI_SRC_DIR}" ]; then
  err "wiki source directory not found: ${WIKI_SRC_DIR}"
  err "expected this script to sit next to a wiki/ directory of .md pages."
  exit 1
fi

if ! command -v git >/dev/null 2>&1; then
  err "git is required but not found on PATH."
  exit 1
fi

# Collect the pages to publish (only top-level .md files in wiki/).
mapfile -t WIKI_PAGES < <(find "${WIKI_SRC_DIR}" -maxdepth 1 -type f -name '*.md' | sort)
if [ "${#WIKI_PAGES[@]}" -eq 0 ]; then
  err "no Markdown pages found in ${WIKI_SRC_DIR}"
  exit 1
fi

log "publishing ${#WIKI_PAGES[@]} wiki page(s) from ${WIKI_SRC_DIR}"

# ---------------------------------------------------------------------------
# Obtain the wiki working tree
# ---------------------------------------------------------------------------
if [ -n "${GIT_REMOTE:-}" ]; then
  # Reuse an already-configured remote (e.g. wiki-origin) instead of a fresh clone.
  log "using existing git remote '${GIT_REMOTE}'"
  REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && cd .. && pwd)"
  if ! git -C "${REPO_ROOT}" remote get-url "${GIT_REMOTE}" >/dev/null 2>&1; then
    err "git remote '${GIT_REMOTE}' is not configured in ${REPO_ROOT}"
    exit 1
  fi
  # Clone from the named remote into the temp dir.
  git clone "remote:${GIT_REMOTE}" "${WIKI_CLONE_DIR}" 2>/dev/null \
    || git clone "$(git -C "${REPO_ROOT}" remote get-url "${GIT_REMOTE}")" "${WIKI_CLONE_DIR}"
else
  log "cloning wiki from ${WIKI_GIT_URL}"
  git clone "${WIKI_GIT_URL}" "${WIKI_CLONE_DIR}"
fi

cd "${WIKI_CLONE_DIR}"

# Ensure we are on the right branch; create it from origin if needed.
git fetch origin >/dev/null 2>&1 || true
if git show-ref --verify --quiet "refs/remotes/origin/${WIKI_BRANCH}"; then
  git checkout "${WIKI_BRANCH}" >/dev/null 2>&1 \
    || git checkout -b "${WIKI_BRANCH}" "origin/${WIKI_BRANCH}"
else
  # Empty wiki or branch absent — start from whatever we cloned onto.
  git checkout -B "${WIKI_BRANCH}" >/dev/null 2>&1 || true
fi

# ---------------------------------------------------------------------------
# Copy pages into the wiki working tree
# ---------------------------------------------------------------------------
# GitHub Wiki treats only top-level .md files as pages; Home.md is the landing
# page. Clean previously-managed pages first so deletions propagate.
log "staging wiki pages"
find . -maxdepth 1 -type f -name '*.md' -delete

for page in "${WIKI_PAGES[@]}"; do
  cp "${page}" .
  log "  + $(basename "${page}")"
done

# ---------------------------------------------------------------------------
# Commit and push
# ---------------------------------------------------------------------------
git add -A '*.md'

# If there is nothing to commit, report and exit cleanly.
if git diff --cached --quiet; then
  log "no changes — wiki is already up to date."
  exit 0
fi

COMMIT_MSG="docs(wiki): publish SigmaOS wiki pages (phase5)

Pages: $(printf '%s, ' "$(basename "${WIKI_PAGES[@]}")" | sed 's/, $//')

Generated as part of Phase 5 (finalize)."

git commit -m "${COMMIT_MSG}" >/dev/null

log "pushing to origin ${WIKI_BRANCH}"
git push origin "${WIKI_BRANCH}"

log "wiki published: ${#WIKI_PAGES[@]} page(s) to ${WIKI_GIT_URL} (${WIKI_BRANCH})"
