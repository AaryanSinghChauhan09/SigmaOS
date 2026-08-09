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
# Only include branches that actually exist in the repository
BRANCHES=(
  "doc/absorb_agents_repos-5960621972319753074"
  "feature/sigmaos-strategic-roadmap-14297109383819106955"
  "feature/sigmaos-strategic-roadmap-8600563103004760725"
  "feature/sigmaos-bolt-palette-sentinel-parity-12861902970884901261"
  "feature/wireshark-distro-improvements-14948326477708832768"
  "improve-package-manager-and-containers-15562379424742924660"
  "improve-sigmaos-systemd-2776481363129221438"
  "jules-11025946340927745781-54b5bb09"
  "jules-12240612823825885289-d7cec605"
  "jules-13571719274074749109-6af93541"
  "jules-15532892492441614180-73ce6847"
  "jules-1677725830451787196-193e25ea"
  "jules-523778995335499834-002b2189"
  "jules-8362645389262009630-ccefedb8"
  "jules-8725025787677827882-82aa0a51"
  "jules-880081283500171861-1eb07604"
  "jules-driver-improvements-linux-inspired-5291856075380713095"
  "universal-driver-support-18128281713178212708"
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
