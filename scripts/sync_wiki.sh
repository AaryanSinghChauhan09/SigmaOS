#!/bin/bash
# SPDX-License-Identifier: MIT
# SigmaOS Wiki Sync Utility (Enhanced with OOP principles)
# Formulates sync engine states and documents as pseudo-classes for extreme scalability.

set -e

# ==============================================================================
# CLASS: WikiPage
# Represents a single wiki page document with self-formatting and path formatting.
# ==============================================================================

# Constructor: WikiPage_new <out_var> <filepath>
WikiPage_new() {
    local out_var="$1"
    local filepath="$2"

    local rand_id
    rand_id=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 8 | head -n 1)
    local self="WikiPage_${rand_id}"

    local filename
    filename=$(basename "$filepath")

    # Translate spaces to dashes for standard slug compatibility
    local slug
    slug=$(echo "$filename" | tr ' ' '-')

    eval "${self}_source_path=\"\$filepath\""
    eval "${self}_filename=\"\$filename\""
    eval "${self}_slug=\"\$slug\""

    eval "$out_var=\"\$self\""
}

# Method: WikiPage_sync <self> <target_dir>
WikiPage_sync() {
    local self="$1"
    local target_dir="$2"

    local source_path
    local slug
    eval "source_path=\"\$${self}_source_path\""
    eval "slug=\"\$${self}_slug\""

    echo "  [WikiPage::sync] Copying: $source_path -> $target_dir/$slug"
    cp "$source_path" "$target_dir/$slug"
}

# ==============================================================================
# CLASS: WikiSyncEngine
# Orchestrates multiple WikiPages, handling sync environments and batch processing.
# ==============================================================================

# Constructor: WikiSyncEngine_new <out_var> <wiki_dir> <target_dir>
WikiSyncEngine_new() {
    local out_var="$1"
    local wiki_dir="$2"
    local target_dir="$3"

    local rand_id
    rand_id=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 8 | head -n 1)
    local self="WikiSyncEngine_${rand_id}"

    eval "${self}_wiki_dir=\"\$wiki_dir\""
    eval "${self}_target_dir=\"\$target_dir\""
    eval "${self}_pages=\"\""

    eval "$out_var=\"\$self\""
}

# Method: WikiSyncEngine_add_page <self> <page_instance>
WikiSyncEngine_add_page() {
    local self="$1"
    local page_instance="$2"

    local current_pages
    eval "current_pages=\"\$${self}_pages\""
    if [ -z "$current_pages" ]; then
        current_pages="$page_instance"
    else
        current_pages="$current_pages $page_instance"
    fi
    eval "${self}_pages=\"\$current_pages\""
}

# Method: WikiSyncEngine_initialize_env <self>
WikiSyncEngine_initialize_env() {
    local self="$1"
    local target_dir
    eval "target_dir=\"\$${self}_target_dir\""

    echo "[WikiSyncEngine::initialize_env] Setting up target environment: $target_dir"
    mkdir -p "$target_dir"
}

# Method: WikiSyncEngine_run_sync <self>
WikiSyncEngine_run_sync() {
    local self="$1"
    local wiki_dir
    local target_dir
    local pages

    eval "wiki_dir=\"\$${self}_wiki_dir\""
    eval "target_dir=\"\$${self}_target_dir\""
    eval "pages=\"\$${self}_pages\""

    echo "[WikiSyncEngine::run_sync] Syncing files from '$wiki_dir' to '$target_dir'..."

    # 1. Re-sync README to Home.md in the WIKI directory first if WIKI exists
    if [ -f "README.md" ] && [ -d "$wiki_dir" ]; then
        echo "  [WikiSyncEngine::run_sync] Aligning README.md -> $wiki_dir/Home.md"
        cp README.md "$wiki_dir/Home.md"
    fi

    # Synchronize core documentation files from root and docs/ to target_dir
    local core_docs=("SECURITY.md" "INSTALL.md" "ARCHITECTURE.md" "ROADMAP.md" "docs/PACKAGE_MANAGER.md" "docs/DISTRO_COMPAT.md" "docs/KERNEL.md")
    for doc in "${core_docs[@]}"; do
        if [ -f "$doc" ]; then
            local base
            base=$(basename "$doc")
            echo "  [WikiSyncEngine::run_sync] Syncing core doc: $doc -> $target_dir/$base"
            cp "$doc" "$target_dir/$base"
        fi
    done

    # 2. Iterate and sync each page object
    local count=0
    for page_instance in $pages; do
        WikiPage_sync "$page_instance" "$target_dir"
        count=$((count + 1))
    done

    echo "[WikiSyncEngine::run_sync] Successfully synchronized $count wiki documentation assets!"
}

# ==============================================================================
# MAIN ENGINE EXECUTION
# ==============================================================================

main() {
    echo "=== Initiating SigmaOS OOP Wiki Sync Utility ==="

    local wiki_dir="WIKI"
    local target_dir="wiki_repo"

    # Instantiate central WikiSyncEngine object
    local engine
    WikiSyncEngine_new engine "$wiki_dir" "$target_dir"
    WikiSyncEngine_initialize_env "$engine"

    # Scan WIKI directory (if it exists) and dynamically construct WikiPage objects
    if [ -d "$wiki_dir" ]; then
        for filepath in "$wiki_dir"/*.md; do
            if [ -f "$filepath" ]; then
                local page
                WikiPage_new page "$filepath"
                WikiSyncEngine_add_page "$engine" "$page"
            fi
        done
    fi

    # Perform full batch sync
    WikiSyncEngine_run_sync "$engine"

    # Also synchronize to 'wiki' directory
    local engine_wiki
    WikiSyncEngine_new engine_wiki "$wiki_dir" "wiki"
    WikiSyncEngine_initialize_env "$engine_wiki"
    if [ -d "$wiki_dir" ]; then
        for filepath in "$wiki_dir"/*.md; do
            if [ -f "$filepath" ]; then
                local page
                WikiPage_new page "$filepath"
                WikiSyncEngine_add_page "$engine_wiki" "$page"
            fi
        done
    fi
    WikiSyncEngine_run_sync "$engine_wiki"

    echo "=== Wiki Synchronization Complete! ==="
}

main "$@"
