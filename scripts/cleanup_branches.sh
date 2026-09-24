#!/bin/bash
# SigmaOS Branch Cleanup Utility
# 
# Automatically fetches all remote branches, attempts to safely merge 
# non-conflicting feature branches into main, and deletes the redundant 
# remote branches to maintain a clean Git repository.

set -e

echo "=================================================="
echo "    SigmaOS Automated Branch Consolidation        "
echo "=================================================="

# Ensure we're on main
git checkout main
git pull origin main --rebase

echo "[*] Fetching and pruning remote tracking branches..."
git fetch --prune --all

echo "[*] Processing remote branches..."
for branch in $(git branch -r | grep 'origin/' | grep -v 'HEAD' | grep -v 'origin/main'); do
    b=$(echo $branch | sed 's/origin\///' | xargs)
    echo " -> Merging $b into main..."
    if git merge "origin/$b" --no-edit -m "Merge branch '$b' into main"; then
        echo "    Successfully merged."
    else
        echo "    [!] Conflict detected. Aborting merge for $b..."
        git merge --abort
    fi
done

echo "[*] Pushing consolidated main branch..."
git push origin main

echo "[*] Deleting redundant remote branches..."
for branch in $(git branch -r | grep 'origin/' | grep -v 'HEAD' | grep -v 'origin/main'); do
    b=$(echo $branch | sed 's/origin\///' | xargs)
    echo " -> Deleting remote branch: $b"
    git push origin --delete "$b" || true
done

echo "=================================================="
echo " Cleanup Complete: Repository is unified on main. "
echo "=================================================="
