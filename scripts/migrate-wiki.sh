#!/usr/bin/env bash
# migrate-wiki.sh — Helper script for GitHub Wiki migration
#
# This script automates the GitHub Wiki migration process using git.
# Following: TRANSFER & CONSOLIDATE ALL .MD/.TXT FILE DATA INTO REPO BASED GITHUB WIKI
#
# Usage:
#   ./scripts/migrate-wiki.sh
#
# Prerequisites:
#   - git must be installed
#   - GitHub personal access token with wiki write permissions
#   - GitHub Wiki must be enabled on the repository

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
WIKI_DIR="$ROOT_DIR/docs/wiki"
WIKI_CLONE_DIR="$ROOT_DIR/.wiki"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'
BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}[wiki-migrate]${RESET} $*"; }
success() { echo -e "${GREEN}[wiki-migrate]${RESET} $*"; }
warn()    { echo -e "${YELLOW}[wiki-migrate]${RESET} $*"; }
err()     { echo -e "${RED}[wiki-migrate]${RESET} $*" >&2; }

# Check prerequisites
check_prerequisites() {
    info "Checking prerequisites..."
    
    if ! command -v git &>/dev/null; then
        err "git is not installed. Install it with: sudo apt-get install git"
        exit 1
    fi
    
    if [[ ! -d "$WIKI_DIR" ]]; then
        err "Wiki directory not found: $WIKI_DIR"
        exit 1
    fi
    
    local wiki_count
    wiki_count=$(find "$WIKI_DIR" -name "*.md" | wc -l)
    info "Found $wiki_count wiki pages in $WIKI_DIR"
    
    if [[ $wiki_count -eq 0 ]]; then
        err "No wiki pages found to migrate"
        exit 1
    fi
}

# Clone the wiki repository
clone_wiki() {
    info "Cloning GitHub Wiki repository..."
    
    local repo_url
    repo_url=$(git -C "$ROOT_DIR" remote get-url origin 2>/dev/null || echo "")
    
    if [[ -z "$repo_url" ]]; then
        err "Could not determine repository URL"
        exit 1
    fi
    
    # Convert GitHub repo URL to wiki URL
    local wiki_url
    wiki_url="${repo_url%.git}.wiki.git"
    
    if [[ -d "$WIKI_CLONE_DIR" ]]; then
        info "Wiki repository already cloned, pulling latest..."
        git -C "$WIKI_CLONE_DIR" pull || warn "Pull failed, continuing..."
    else
        info "Cloning wiki from: $wiki_url"
        git clone "$wiki_url" "$WIKI_CLONE_DIR" || {
            err "Failed to clone wiki repository"
            err "Make sure GitHub Wiki is enabled in repository settings"
            err "Settings → Features → Wikis → Enable"
            exit 1
        }
    fi
    
    success "Wiki repository cloned to $WIKI_CLONE_DIR"
}

# Copy wiki pages
copy_wiki_pages() {
    info "Copying wiki pages..."
    
    local copied=0
    for page in "$WIKI_DIR"/*.md; do
        if [[ -f "$page" ]]; then
            local page_name
            page_name=$(basename "$page")
            cp "$page" "$WIKI_CLONE_DIR/"
            ((copied++))
        fi
    done
    
    success "Copied $copied wiki pages"
}

# Commit and push changes
commit_and_push() {
    info "Committing and pushing wiki changes..."
    
    git -C "$WIKI_CLONE_DIR" add .
    
    if git -C "$WIKI_CLONE_DIR" diff --staged --quiet; then
        warn "No changes to commit"
        return
    fi
    
    git -C "$WIKI_CLONE_DIR" commit -m "docs: Migrate wiki pages from docs/wiki/

Following TRANSFER & CONSOLIDATE ALL .MD/.TXT FILE DATA INTO REPO BASED GITHUB WIKI principle.

Generated with [Devin](https://devin.ai)"
    
    git -C "$WIKI_CLONE_DIR" push || {
        err "Failed to push wiki changes"
        err "Make sure you have push permissions to the wiki repository"
        exit 1
    }
    
    success "Wiki pages pushed to GitHub"
}

# Main
main() {
    echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════╗${RESET}"
    echo -e "${BOLD}${CYAN}║   SigmaOS GitHub Wiki Migration Helper      ║${RESET}"
    echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════╝${RESET}"
    echo ""
    
    check_prerequisites
    clone_wiki
    copy_wiki_pages
    commit_and_push
    
    echo ""
    success "Wiki migration complete!"
    info "View your wiki at: $(git -C "$ROOT_DIR" remote get-url origin 2>/dev/null | sed 's/\.git$/wiki/')"
}

main "$@"
