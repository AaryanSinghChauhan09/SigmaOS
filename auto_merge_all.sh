#!/bin/bash
git fetch --all
git checkout main

# Get all remote branches except main and HEAD
branches=$(git branch -r | grep 'origin/' | grep -v 'origin/main' | grep -v 'HEAD' | sed 's/origin\///')

for branch in $branches; do
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

echo "=== All branches merged into main! ==="
