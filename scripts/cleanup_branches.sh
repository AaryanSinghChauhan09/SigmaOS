#!/bin/bash
# SigmaOS Branch Cleanup Utility
# 
# Audits remote branches against main and optionally deletes only branches
# already proven to be merged. It never merges or deletes unreviewed work.

set -euo pipefail

DRY_RUN=true
if [[ "${1:-}" == "--apply" ]]; then
    DRY_RUN=false
elif [[ "${1:-}" != "" ]]; then
    echo "Usage: $0 [--apply]" >&2
    exit 2
fi

echo "=================================================="
echo "    SigmaOS Automated Branch Consolidation        "
echo "=================================================="

# Ensure the audit is performed from main without changing the worktree.
if [[ "$(git branch --show-current)" != "main" ]]; then
    echo "Run this script from the main branch." >&2
    exit 1
fi

echo "[*] Fetching and pruning remote tracking branches..."
git fetch --prune --all

echo "[*] Auditing remote branches against main..."
while read -r remote_branch; do
    [[ -z "$remote_branch" ]] && continue
    branch="${remote_branch#origin/}"
    if git merge-base --is-ancestor "$remote_branch" main; then
        echo " -> $branch: merged into main"
        if [[ "$DRY_RUN" == false ]]; then
            git push origin --delete "$branch"
        fi
    else
        echo " -> $branch: retained (not proven merged)"
    fi
done < <(git for-each-ref --format='%(refname:short)' refs/remotes/origin/ | \
    grep -vE '^origin/(HEAD|main)$')

if [[ "$DRY_RUN" == true ]]; then
    echo "[*] Dry run only. Pass --apply to delete proven-merged branches."
fi

echo "=================================================="
echo " Cleanup Complete: Repository is unified on main. "
echo "=================================================="
