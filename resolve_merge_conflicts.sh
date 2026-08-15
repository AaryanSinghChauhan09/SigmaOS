#!/bin/bash

# Auto-resolve merge conflicts by accepting incoming changes (theirs)
# This script resolves all merge conflicts in favor of the jules branch optimizations

echo "Resolving merge conflicts in favor of incoming changes..."

# Get list of conflicted files
conflicted_files=$(git diff --name-only --diff-filter=U)

for file in $conflicted_files; do
    echo "Resolving conflict in: $file"
    
    # For most files, accept incoming changes
    if [[ "$file" == *.rs ]] || [[ "$file" == *.md ]] || [[ "$file" == *.js ]] || [[ "$file" == *.toml ]]; then
        git checkout --theirs "$file"
        git add "$file"
    else
        # For other files, try manual resolution
        git checkout --theirs "$file"
        git add "$file"
    fi
done

# Also handle "both added" files
git status --porcelain | grep "^AA" | cut -c4- | while read file; do
    echo "Resolving 'both added' conflict in: $file"
    git checkout --theirs "$file"
    git add "$file"
done

echo "All conflicts resolved. Files staged for commit."