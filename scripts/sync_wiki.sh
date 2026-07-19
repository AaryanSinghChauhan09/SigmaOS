#!/bin/bash
# SPDX-License-Identifier: MIT
# SigmaOS Wiki Sync Utility

set -e

echo "=== Synchronizing SigmaOS Wiki Documentation ==="

WIKI_DIR="WIKI"
TARGET_DIR="wiki_repo"

# Ensure target directory exists
mkdir -p "$TARGET_DIR"

# Copy README to Home.md
cp README.md "$WIKI_DIR/Home.md"

# Copy and sync markdown files from WIKI to local target directory
# Replacing spaces with dashes in filenames
for filepath in "$WIKI_DIR"/*.md; do
    if [ -f "$filepath" ]; then
        filename=$(basename "$filepath")
        new_filename=$(echo "$filename" | tr ' ' '-')
        echo "Syncing $filename -> $TARGET_DIR/$new_filename"
        cp "$filepath" "$TARGET_DIR/$new_filename"
    fi
done

echo "=== Wiki Synchronization Complete! ==="
