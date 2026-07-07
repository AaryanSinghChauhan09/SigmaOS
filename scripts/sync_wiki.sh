#!/bin/sh
# POSIX-compliant script for syncing markdown documentation to the Wiki Repo.
# Completely bypasses heavy runtime dependencies (like PowerShell or Python).

SOURCE_DIR="."
WIKI_DIR="./wiki_repo"

mkdir -p "$WIKI_DIR"

echo "Syncing Markdown files to Wiki..."

find "$SOURCE_DIR" -type f -name "*.md" | while read -r filepath; do
    # Skip files already in wiki_repo
    case "$filepath" in
        *"$WIKI_DIR"*) continue ;;
    esac

    filename=$(basename "$filepath")
    # GitHub Wiki replaces spaces with dashes natively
    destname=$(echo "$filename" | tr ' ' '-')
    
    cp "$filepath" "$WIKI_DIR/$destname"
done

if [ -f "$SOURCE_DIR/README.md" ]; then
    cp "$SOURCE_DIR/README.md" "$WIKI_DIR/Home.md"
fi

echo "Wiki Sync COMPLETE."
