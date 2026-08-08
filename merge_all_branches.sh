#!/bin/bash
# Comprehensive merge script for SigmaOS branches
# Merges all non-main branches into main, resolving conflicts by preferring incoming improvements

set -e

REPO_DIR="/home/aaryansinghchauhan/SigmaOS"
cd "$REPO_DIR"

echo "=== SigmaOS Branch Merge Script ==="
echo "Starting at: $(date)"
echo ""

# List of branches to merge (ordered by improvement priority)
BRANCHES=(
  "feature/improve-kernel-headers-linux-inspired-5018644282529671678"
  "improve-sshd-4453662879443076923"
  "improve-installer-script-9830616872725964915"
  "jules-driver-improvements-linux-inspired-5291856075380713095"
  "jules-13571719274074749109-6af93541"
  "jules-2781770876213150319-18a8b4ea"
  "jules-9523791895558632879-f4c1ad14"
  "jules-3204690558743606025-06e1d059"
  "jules-14967948003256892231-7e7b3d2e"
  "feature/distro-parity-organizational-frameworks-251993214289770317"
)

for branch in "${BRANCHES[@]}"; do
  echo ""
  echo "=== Merging: $branch ==="
  
  # Try merge
  if git merge --no-edit "origin/$branch" 2>&1; then
    echo "✓ Merged cleanly: $branch"
  else
    echo "! Conflicts detected - resolving by preferring incoming changes..."
    
    # Get all conflicted files
    CONFLICTED=$(git diff --name-only --diff-filter=U 2>/dev/null)
    
    if [ -n "$CONFLICTED" ]; then
      echo "Conflicted files:"
      echo "$CONFLICTED"
      
      # Accept incoming (theirs) for all conflicted files
      git checkout --theirs $CONFLICTED 2>/dev/null || true
      git add $CONFLICTED 2>/dev/null || true
    fi
    
    # Stage all remaining files
    git add -A 2>/dev/null || true
    
    # Commit the merge
    git commit -m "Merge $branch: resolve conflicts preferring OS improvements" --no-edit 2>/dev/null || \
    git commit -m "Merge $branch: resolve conflicts preferring OS improvements" 2>/dev/null || true
    
    echo "✓ Merged with conflict resolution: $branch"
  fi
done

echo ""
echo "=== All branches merged! ==="
echo "Completed at: $(date)"
