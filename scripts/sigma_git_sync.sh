#!/usr/bin/env bash
# SigmaOS GitHub sync helper — stage, commit, push, optional wiki sync.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DRY_RUN=0
MSG="Auto-sync from SigmaOS automation"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run) DRY_RUN=1; shift ;;
    -m|--message) MSG="${2:-$MSG}"; shift 2 ;;
    *) echo "Unknown arg: $1"; exit 1 ;;
  esac
done

cd "$ROOT"

if [[ "$DRY_RUN" -eq 1 ]]; then
  echo "[sigma-git-sync] dry-run"
  git status -sb
  git diff --stat
  exit 0
fi

"${ROOT}/scripts/sigma_automation.sh" wiki-sync || true

if [[ -z "$(git status --porcelain)" ]]; then
  echo "[sigma-git-sync] nothing to commit"
  exit 0
fi

git add -A
git commit -m "$MSG" -m "Automated sync via scripts/sigma_git_sync.sh"
git push origin HEAD

echo "[sigma-git-sync] pushed to origin"
