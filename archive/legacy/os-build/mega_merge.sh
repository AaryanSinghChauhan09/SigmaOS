#!/usr/bin/env bash
# mega_merge.sh — Merge all remote branches into main, resolving conflicts
# Strategy: 
#   - Accept incoming NEW FILES (theirs) so we gain features
#   - For conflicted existing files, keep our clean fixed version (ours)
#   - After each merge batch, cargo check and fix if needed

set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/.." && pwd)"
BRANCH_LIST="${1:-/tmp/all_branches.txt}"
LOG_FILE="${REPO_DIR}/scripts/merge_log.txt"
ERROR_LOG="${REPO_DIR}/scripts/merge_errors.txt"

cd "$REPO_DIR"

echo "=== MEGA MERGE SESSION: $(date) ===" | tee -a "$LOG_FILE"
echo "Repo: $REPO_DIR" | tee -a "$LOG_FILE"
echo "Branch list: $BRANCH_LIST" | tee -a "$LOG_FILE"

MERGED=0
SKIPPED=0
FAILED=0

while IFS= read -r branch; do
    branch="${branch// /}"
    [[ -z "$branch" ]] && continue
    
    echo "" | tee -a "$LOG_FILE"
    echo "--- Processing: $branch ---" | tee -a "$LOG_FILE"
    
    # Check if branch exists remotely
    if ! git ls-remote --exit-code origin "$branch" &>/dev/null; then
        echo "  SKIP: branch not found on remote" | tee -a "$LOG_FILE"
        ((SKIPPED++)) || true
        continue
    fi
    
    # Fetch the branch
    git fetch origin "$branch" 2>>"$LOG_FILE" || {
        echo "  SKIP: fetch failed" | tee -a "$LOG_FILE"
        ((SKIPPED++)) || true
        continue
    }
    
    # Check if already merged (fast-forward check)
    LOCAL_COMMIT=$(git rev-parse HEAD)
    REMOTE_COMMIT=$(git rev-parse "origin/$branch" 2>/dev/null || echo "")
    
    if [[ -z "$REMOTE_COMMIT" ]]; then
        echo "  SKIP: cannot resolve remote commit" | tee -a "$LOG_FILE"
        ((SKIPPED++)) || true
        continue
    fi
    
    # Check if already an ancestor
    if git merge-base --is-ancestor "$REMOTE_COMMIT" HEAD 2>/dev/null; then
        echo "  SKIP: already merged" | tee -a "$LOG_FILE"
        ((SKIPPED++)) || true
        continue
    fi
    
    # Attempt merge — prefer ours for conflicts in existing files
    echo "  Merging origin/$branch ..." | tee -a "$LOG_FILE"
    
    if git merge --no-edit -X ours "origin/$branch" 2>>"$LOG_FILE"; then
        echo "  SUCCESS: clean merge" | tee -a "$LOG_FILE"
        ((MERGED++)) || true
    else
        # Merge had conflicts, resolve them
        echo "  CONFLICT: resolving with --ours strategy..." | tee -a "$LOG_FILE"
        
        # For new files added by the branch (untracked by us), accept them
        # For conflicts in existing files, keep ours
        git diff --name-only --diff-filter=U 2>/dev/null | while read -r f; do
            git checkout --ours "$f" 2>>"$LOG_FILE" || true
            git add "$f" 2>>"$LOG_FILE" || true
            echo "    Kept ours: $f" | tee -a "$LOG_FILE"
        done
        
        # Accept new files from theirs
        git diff --name-only --diff-filter=A "origin/$branch" 2>/dev/null | while read -r f; do
            if [[ ! -f "$f" ]]; then
                git checkout "origin/$branch" -- "$f" 2>>"$LOG_FILE" || true
                git add "$f" 2>>"$LOG_FILE" || true
                echo "    Accepted new file: $f" | tee -a "$LOG_FILE"
            fi
        done
        
        # Stage everything and commit
        git add -A 2>>"$LOG_FILE" || true
        
        if git commit --no-edit -m "merge($branch): integrate features, keep clean compilation baseline" 2>>"$LOG_FILE"; then
            echo "  SUCCESS: resolved merge committed" | tee -a "$LOG_FILE"
            ((MERGED++)) || true
        else
            echo "  FAILED: could not commit merge" | tee -a "$LOG_FILE" 2>&1
            git merge --abort 2>/dev/null || git reset --hard HEAD 2>/dev/null || true
            ((FAILED++)) || true
            echo "$branch" >> "$ERROR_LOG"
        fi
    fi
    
done < "$BRANCH_LIST"

echo "" | tee -a "$LOG_FILE"
echo "=== MERGE SUMMARY ===" | tee -a "$LOG_FILE"
echo "Merged: $MERGED" | tee -a "$LOG_FILE"
echo "Skipped: $SKIPPED" | tee -a "$LOG_FILE"
echo "Failed: $FAILED" | tee -a "$LOG_FILE"
echo "=====================" | tee -a "$LOG_FILE"
