#!/usr/bin/env bash
# =============================================================================
# SigmaOS GitHub Sync Script
# Syncs SigmaOS repo and wiki to GitHub, ensuring clean, atomic pushes
# Usage: ./scripts/sync_github.sh [--wiki-only] [--repo-only] [--dry-run] [-m "message"]
# =============================================================================
set -euo pipefail

# ── Colors ───────────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

# ── Paths ─────────────────────────────────────────────────────────────────────
REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIKI_DIR="$(dirname "$REPO_DIR")/SigmaOS.wiki"
REPO_WIKI_DIR="$REPO_DIR/wiki"

# ── Defaults ─────────────────────────────────────────────────────────────────
SYNC_WIKI=true
SYNC_REPO=true
DRY_RUN=false
COMMIT_MSG=""
BRANCH="main"

# ── Argument parsing ──────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
    case $1 in
        --wiki-only) SYNC_REPO=false; shift ;;
        --repo-only) SYNC_WIKI=false; shift ;;
        --dry-run) DRY_RUN=true; shift ;;
        -m|--message) COMMIT_MSG="$2"; shift 2 ;;
        -b|--branch) BRANCH="$2"; shift 2 ;;
        -h|--help)
            echo "Usage: $0 [--wiki-only] [--repo-only] [--dry-run] [-m 'commit message'] [-b branch]"
            exit 0 ;;
        *) echo -e "${RED}Unknown argument: $1${NC}" >&2; exit 1 ;;
    esac
done

# ── Helpers ───────────────────────────────────────────────────────────────────
log_info()    { echo -e "${BLUE}[INFO]${NC} $*"; }
log_ok()      { echo -e "${GREEN}[OK]${NC} $*"; }
log_warn()    { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error()   { echo -e "${RED}[ERROR]${NC} $*" >&2; }
log_section() { echo -e "\n${BOLD}${CYAN}══ $* ══${NC}"; }

run_cmd() {
    if [[ "$DRY_RUN" == "true" ]]; then
        echo -e "${YELLOW}[DRY-RUN]${NC} $*"
    else
        "$@"
    fi
}

# ── Timestamp for auto-commit messages ───────────────────────────────────────
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S %Z')
AUTO_MSG="chore: sync SigmaOS improvements [$TIMESTAMP]

Automated sync from SigmaOS development session.
- Updated source code modules (Rust, Nim, Zig, Shell)
- Added new components inspired by Omarchy & Linux Mint
- Updated wiki documentation for all components
- Improved low-level subsystems"

COMMIT_MSG="${COMMIT_MSG:-$AUTO_MSG}"

# ── Step 1: Sync repo/wiki → SigmaOS.wiki (local mirror) ─────────────────────
log_section "Syncing internal wiki mirror"
if [[ -d "$REPO_WIKI_DIR" && -d "$WIKI_DIR" ]]; then
    log_info "Copying wiki pages from repo/wiki → SigmaOS.wiki (overwrite newer)"
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "[DRY-RUN] Would rsync $REPO_WIKI_DIR/ → $WIKI_DIR/"
    else
        rsync -av --exclude='.git' --exclude='man' "$REPO_WIKI_DIR/" "$WIKI_DIR/" 2>/dev/null || true
        log_ok "Wiki mirror synced"
    fi
else
    log_warn "Wiki directories not found, skipping mirror sync"
fi

# ── Step 2: Push main repo ────────────────────────────────────────────────────
if [[ "$SYNC_REPO" == "true" ]]; then
    log_section "Syncing SigmaOS main repository"
    cd "$REPO_DIR"

    # Check for uncommitted changes
    if git diff --quiet && git diff --staged --quiet; then
        log_info "Repository is clean — checking for untracked files"
    fi

    # Stage all changes
    CHANGED=$(git status --short | wc -l)
    if [[ "$CHANGED" -gt 0 ]]; then
        log_info "Staging $CHANGED changed/new files"
        run_cmd git add -A
        run_cmd git commit -m "$COMMIT_MSG"
        log_ok "Committed to local repo"
    else
        log_info "No changes to commit in main repo"
    fi

    # Push to origin
    log_info "Pushing $BRANCH to origin..."
    run_cmd git push origin "$BRANCH"
    log_ok "Main repo pushed to GitHub ✓"
fi

# ── Step 3: Push wiki ─────────────────────────────────────────────────────────
if [[ "$SYNC_WIKI" == "true" ]]; then
    log_section "Syncing SigmaOS Wiki"
    if [[ ! -d "$WIKI_DIR/.git" ]]; then
        log_warn "SigmaOS.wiki is not a git repo — skipping wiki push"
        log_warn "Clone it first: git clone https://github.com/AaryanSinghChauhan09/SigmaOS.wiki.git"
    else
        cd "$WIKI_DIR"

        # Copy latest from repo wiki
        if [[ -d "$REPO_WIKI_DIR" ]]; then
            rsync -av --exclude='.git' --exclude='man' "$REPO_WIKI_DIR/" "$WIKI_DIR/" 2>/dev/null || true
        fi

        WIKI_CHANGED=$(git status --short | wc -l)
        if [[ "$WIKI_CHANGED" -gt 0 ]]; then
            log_info "Staging $WIKI_CHANGED wiki changes"
            run_cmd git add -A
            run_cmd git commit -m "docs: update wiki [$TIMESTAMP]

Updated wiki pages:
- Uniform format applied to all component pages
- New component pages added
- Omarchy & Linux Mint inspiration documented
- All source modules have corresponding wiki pages"
            run_cmd git push origin main
            log_ok "Wiki pushed to GitHub ✓"
        else
            log_info "Wiki is already up to date"
        fi
    fi
fi

# ── Summary ───────────────────────────────────────────────────────────────────
log_section "Sync Complete"
echo -e "${GREEN}${BOLD}"
echo "  ✅ SigmaOS GitHub sync finished"
echo "  📦 Repo: https://github.com/AaryanSinghChauhan09/SigmaOS"
echo "  📖 Wiki: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki"
if [[ "$DRY_RUN" == "true" ]]; then
    echo -e "${YELLOW}  ⚠️  DRY RUN — no actual pushes performed${NC}"
fi
echo -e "${NC}"
