#!/usr/bin/env bash
# cleanup-local-repos.sh — Clean up local synchronized SigmaOS repositories
#
# Following principle: IF THE CODE FROM THE LOCAL DEVICE IS FULLY SYNCED TO THE
# ONLINE GITHUB REPO, THEN REMOVE IT FROM THE LOCAL DEVICE
#
# This script checks if local repositories are clean (no uncommitted changes)
# and all commits are pushed to GitHub. If synced, it offers to remove the local
# clone to free up disk space while preserving the ability to re-clone later.
#
# Usage:
#   ./scripts/cleanup-local-repos.sh [--dry-run] [--confirm]
#
# Options:
#   --dry-run    Show what would be removed without actually removing
#   --confirm    Remove without prompting (use with caution)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
FLEET_DIR="$(dirname "$ROOT_DIR")"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'
BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}[cleanup]${RESET} $*"; }
success() { echo -e "${GREEN}[cleanup]${RESET} $*"; }
warn()    { echo -e "${YELLOW}[cleanup]${RESET} $*"; }
err()     { echo -e "${RED}[cleanup]${RESET} $*" >&2; }

DRY_RUN=0
AUTO_CONFIRM=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --dry-run) DRY_RUN=1; shift ;;
        --confirm) AUTO_CONFIRM=1; shift ;;
        -h|--help)
            echo "Usage: $0 [--dry-run] [--confirm]"
            echo ""
            echo "Options:"
            echo "  --dry-run    Show what would be removed without actually removing"
            echo "  --confirm    Remove without prompting (use with caution)"
            exit 0
            ;;
        *) err "Unknown option: $1"; exit 1 ;;
    esac
done

# List of active SigmaOS repositories
ACTIVE_REPOS=(
    "SigmaOS"
    "SigmaOS-dev"
    "SigmaOS-utils"
    "SigmaOS-themes"
    "SigmaOS-bootloader"
    "SigmaOS-package-manager"
    "SigmaOS-update-manager"
    "SigmaOS-snapshot-manager"
    "SigmaOS-network-manager"
    "SigmaOS-power-manager"
    "SigmaOS-keyring-manager"
    "SigmaOS-notifications"
)

# Check if a repository is synced (clean and all commits pushed)
is_repo_synced() {
    local repo_path="$1"
    
    # Check if it's a git repository
    [[ -d "$repo_path/.git" ]] || return 1
    
    # Check if working directory is clean
    git -C "$repo_path" diff --quiet HEAD 2>/dev/null || return 1
    git -C "$repo_path" diff --cached --quiet 2>/dev/null || return 1
    
    # Check if all commits are pushed
    local local_remote local_branch
    local_remote=$(git -C "$repo_path" remote get-url origin 2>/dev/null || echo "")
    [[ -z "$local_remote" ]] && return 1
    
    local_branch=$(git -C "$repo_path" branch --show-current 2>/dev/null || echo "")
    [[ -z "$local_branch" ]] && return 1
    
    # Check if local branch is behind remote
    git -C "$repo_path" fetch origin "$local_branch" 2>/dev/null || return 1
    local ahead_behind
    ahead_behind=$(git -C "$repo_path" rev-list --left-right --count "origin/$local_branch...HEAD" 2>/dev/null || echo "0 0")
    local ahead_count
    ahead_count=$(echo "$ahead_behind" | awk '{print $2}')
    [[ "$ahead_count" -gt 0 ]] && return 1
    
    return 0
}

# Main cleanup logic
main() {
    echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════╗${RESET}"
    echo -e "${BOLD}${CYAN}║   SigmaOS Local Repository Cleanup       ║${RESET}"
    echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════╝${RESET}"
    echo ""
    
    if [[ $DRY_RUN -eq 1 ]]; then
        info "DRY RUN MODE - No repositories will be removed"
        echo ""
    fi
    
    local synced_count=0
    local dirty_count=0
    local missing_count=0
    local to_remove=()
    
    for repo in "${ACTIVE_REPOS[@]}"; do
        local repo_path="$FLEET_DIR/$repo"
        
        if [[ ! -d "$repo_path" ]]; then
            warn "$repo - not checked out locally"
            ((missing_count++))
            continue
        fi
        
        if is_repo_synced "$repo_path"; then
            success "$repo - synced, can be removed"
            to_remove+=("$repo_path")
            ((synced_count++))
        else
            warn "$repo - has uncommitted changes or unpushed commits"
            ((dirty_count++))
        fi
    done
    
    echo ""
    info "Summary:"
    info "  Synced (removable): $synced_count"
    info "  Dirty (not removable): $dirty_count"
    info "  Missing (not checked out): $missing_count"
    echo ""
    
    if [[ ${#to_remove[@]} -eq 0 ]]; then
        info "No repositories to remove"
        exit 0
    fi
    
    if [[ $AUTO_CONFIRM -eq 0 ]]; then
        echo -e "${YELLOW}The following repositories will be removed:${RESET}"
        for path in "${to_remove[@]}"; do
            echo "  - $path"
        done
        echo ""
        read -p "Continue? [y/N] " -n 1 -r
        echo ""
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            info "Aborted"
            exit 0
        fi
    fi
    
    # Remove repositories
    for path in "${to_remove[@]}"; do
        if [[ $DRY_RUN -eq 1 ]]; then
            info "Would remove: $path"
        else
            info "Removing: $path"
            rm -rf "$path"
            success "Removed: $path"
        fi
    done
    
    echo ""
    success "Cleanup complete"
    info "Repositories can be re-cloned anytime using:"
    info "  ./scripts/sigma_hub.sh connect <name>"
}

main "$@"
