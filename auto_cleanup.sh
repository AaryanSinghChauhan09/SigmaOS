#!/bin/bash
# Push main to origin
git push origin main

# Delete remote branches
branches=$(git branch -r | grep '^[[:space:]]*origin/' | grep -v 'origin/main' | grep -v 'HEAD' | sed 's/^[[:space:]]*origin\///')
for branch in $branches; do
    echo "Deleting remote branch: $branch"
    git push origin --delete "$branch" || echo "Failed to delete $branch"
done
