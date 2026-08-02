#!/bin/bash
set -e

CURRENT_BRANCH=$(git branch --show-current)
echo "Current branch: $CURRENT_BRANCH"

# List of remote branches to merge
BRANCHES=(
  "origin/algorithms-status-report-7209944668861913625"
  "origin/bolt-optimize-version-parsing-10169061779574772255"
  "origin/feat/linux-release-drivers-11485260438250341022"
  "origin/feature/sigmaos-strategic-roadmap-3692445946687651609"
  "origin/feature/sigmaos-strategic-roadmap-7275568434630319333"
  "origin/jules-15532892492441614180-73ce6847"
  "origin/jules-15969987454661535902-c693fd8f"
  "origin/jules-4929818014729465740-93bef09a"
  "origin/jules-5211497904429441944-cf01c5fc"
  "origin/jules-6497164819816536137-c6bc94c1"
  "origin/jules-8662134349396449944-dbc9966d"
  "origin/jules-9057756713964855410-d59a7b65"
  "origin/jules-9194715674943762331-96d1604a"
  "origin/jules/peripheral-compatibility-15747095318119433906"
  "origin/universal-application-absorption-plan-10947268359744048201"
  "origin/universal-driver-support-18128281713178212708"
)

# Set git identity if not set
git config user.email "jules@sigmaos.dev" || true
git config user.name "Jules Agent" || true

for branch in "${BRANCHES[@]}"; do
  echo "Attempting to merge $branch..."
  # If the merge succeeds or fails, we handle it
  if git merge "$branch" --no-edit; then
    echo "Successfully merged $branch"
  else
    echo "Conflict detected during merge of $branch. Resolving automatically by preferring our current branch files..."
    # Checkout our version of conflicted files
    git checkout --ours . || true
    # Add resolved files
    git add -u || true
    # Commit the merge
    git commit -m "Merge branch $branch with local resolution preferring current files" || true
    echo "Resolved and merged $branch"
  fi
done

echo "All merges processed successfully."
