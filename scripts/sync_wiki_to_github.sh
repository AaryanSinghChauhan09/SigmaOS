#!/bin/bash
# Sync wiki/ directory to GitHub Wiki
# This script uses the GitHub API to upload wiki pages

set -e

REPO="AaryanSinghChauhan09/SigmaOS"
WIKI_DIR="wiki"

# Check if GitHub token is available
if [ -z "$GITHUB_TOKEN" ]; then
    echo "Error: GITHUB_TOKEN environment variable not set"
    echo "Please set GITHUB_TOKEN to enable wiki synchronization"
    exit 1
fi

echo "Syncing wiki pages to GitHub Wiki for $REPO"

# List all markdown files in wiki/
for file in "$WIKI_DIR"/*.md; do
    if [ -f "$file" ]; then
        filename=$(basename "$file")
        echo "Uploading $filename..."
        
        # Get page content
        content=$(cat "$file")
        
        # Check if page exists
        existing_sha=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
            "https://api.github.com/repos/$REPO/wiki/pages/$filename" \
            | jq -r '.sha // empty')
        
        if [ "$existing_sha" != "null" ] && [ -n "$existing_sha" ]; then
            # Update existing page
            echo "Updating existing page: $filename"
            curl -s -X PUT \
                -H "Authorization: token $GITHUB_TOKEN" \
                -H "Content-Type: application/json" \
                -d "{\"message\":\"Update $filename\",\"content\":\"$(echo "$content" | base64 -w 0)\",\"sha\":\"$existing_sha\"}" \
                "https://api.github.com/repos/$REPO/wiki/pages/$filename"
        else
            # Create new page
            echo "Creating new page: $filename"
            curl -s -X POST \
                -H "Authorization: token $GITHUB_TOKEN" \
                -H "Content-Type: application/json" \
                -d "{\"message\":\"Create $filename\",\"title\":\"$filename\",\"content\":\"$(echo "$content" | base64 -w 0)\"}" \
                "https://api.github.com/repos/$REPO/wiki/pages"
        fi
    fi
done

echo "Wiki synchronization complete"
