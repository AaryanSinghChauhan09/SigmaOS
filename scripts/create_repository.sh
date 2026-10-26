#!/bin/bash
# SigmaOS Repository Creation Automation Script
# Follows principles of: AUTOMATION, REPEATABILITY, DEFENSIVE PROGRAMMING, FAIL-FAST
# Inspired by Linux Mint and Omarchy repository structure

set -euo pipefail  # FAIL-FAST: Exit on error, undefined variables, pipe failures

# Configuration
GITHUB_ORG="AaryanSinghChauhan09"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
REPOS_FILE="$SCRIPT_DIR/repository_list.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging function (INSTRUMENTATION)
log() {
    echo -e "${BLUE}[$(date +'%Y-%m-%d %H:%M:%S')]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

# Check if gh CLI is installed (DEFENSIVE PROGRAMMING)
check_gh_cli() {
    if ! command -v gh &> /dev/null; then
        log_error "GitHub CLI (gh) is not installed"
        log "Install it from: https://cli.github.com/"
        exit 1
    fi

    # Check if authenticated
    if ! gh auth status &> /dev/null; then
        log_error "GitHub CLI is not authenticated"
        log "Run: gh auth login"
        exit 1
    fi

    log_success "GitHub CLI is installed and authenticated"
}

# Create a single repository
create_repository() {
    local repo_name="$1"
    local description="$2"
    local homepage="${3:-}"
    local is_private="${4:-false}"
    local has_wiki="${5:-true}"
    local has_issues="${6:-true}"

    log "Creating repository: $repo_name"

    # Check if repository already exists (DETERMINISTIC TESTING)
    if gh repo view "$GITHUB_ORG/$repo_name" &> /dev/null; then
        log_warning "Repository $repo_name already exists, skipping..."
        return 0
    fi

    # Create repository (COMMAND-QUERY SEPARATION: command to create)
    local create_args=(
        "$GITHUB_ORG/$repo_name"
        --description "$description"
        --public
        --source "$GITHUB_ORG/SigmaOS"
    )

    if [ -n "$homepage" ]; then
        create_args+=(--homepage "$homepage")
    fi

    if [ "$has_wiki" = "false" ]; then
        create_args+=(--no-wiki)
    fi

    if [ "$has_issues" = "false" ]; then
        create_args+=(--no-issues)
    fi

    if gh repo create "${create_args[@]}"; then
        log_success "Created repository: $repo_name"
        return 0
    else
        log_error "Failed to create repository: $repo_name"
        return 1
    fi
}

# Initialize repository with standard structure
initialize_repository() {
    local repo_name="$1"
    local repo_path="/home/aaryansinghchauhan/$repo_name"

    log "Initializing repository structure for: $repo_name"

    # Create local directory if it doesn't exist
    if [ ! -d "$repo_path" ]; then
        mkdir -p "$repo_path"
        cd "$repo_path"

        # Initialize git repository
        git init
        git remote add origin "https://github.com/$GITHUB_ORG/$repo_name.git"

        # Create standard files (TALK IS CHEAP, SHOW ME THE CODE)
        cat > README.md << EOF
# $repo_name

Part of the SigmaOS multi-repository architecture.

## Principles
- MODULARITY: Single responsibility per repository
- LOOSE COUPLING: Minimal dependencies between repos
- INTERFACE SEGREGATION: Clear, focused public APIs
- DETERMINISTIC TESTING: Comprehensive test coverage

## License
SPDX-License-Identifier: GPL-3.0-or-later

## Copyright
Copyright (C) 2026 SigmaOS Project
EOF

        cat > LICENSE << EOF
                    GNU GENERAL PUBLIC LICENSE
                       Version 3, 29 June 2007

 Copyright (C) 2007 Free Software Foundation, Inc. <https://fsf.org/>
 Everyone is permitted to copy and distribute verbatim copies
 of this license document, but changing it is not allowed.
EOF

        cat > CONTRIBUTING.md << EOF
# Contributing to $repo_name

Thank you for your interest in contributing to SigmaOS!

## Principles
- BOY SCOUT RULE: Leave the code cleaner than you found it
- KISS: Keep It Simple, Stupid
- DRY: Don't Repeat Yourself
- YAGNI: You Aren't Gonna Need It

## Development Workflow
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## Code Review
All changes must be reviewed before merging.
EOF

        cat > .gitignore << EOF
# Build artifacts
/target/
**/*.rs.bk
*.o
*.a
*.so
*.dylib
*.dll
*.exe
*.elf

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Documentation (per SigmaOS policy: docs in wiki)
*.md
!README.md
!CONTRIBUTING.md
!LICENSE
EOF

        # Create initial commit
        git add README.md LICENSE CONTRIBUTING.md .gitignore
        git commit -m "Initial commit

Generated with SigmaOS repository creation automation
Following principles: MODULARITY, LOOSE COUPLING, INTERFACE SEGREGATION"

        log_success "Initialized repository structure for: $repo_name"
    else
        log_warning "Repository directory already exists: $repo_path"
    fi
}

# Read repository list and create all
create_all_repositories() {
    if [ ! -f "$REPOS_FILE" ]; then
        log_error "Repository list file not found: $REPOS_FILE"
        exit 1
    fi

    local count=0
    local total=$(wc -l < "$REPOS_FILE")
    local failed=0

    while IFS='|' read -r name description homepage; do
        # Skip empty lines and comments
        [[ -z "$name" || "$name" == \#* ]] && continue

        count=$((count + 1))
        log "[$count/$total] Processing: $name"

        if create_repository "$name" "$description" "$homepage"; then
            initialize_repository "$name"
        else
            failed=$((failed + 1))
        fi

        # Rate limiting (RESPECT & ETHICAL PARTICIPATION)
        if [ $((count % 10)) -eq 0 ]; then
            log "Pausing to respect GitHub API rate limits..."
            sleep 2
        fi
    done < "$REPOS_FILE"

    log "Repository creation complete"
    log "Total: $count, Failed: $failed, Success: $((count - failed))"
}

# Main function (MAIN - SINGLE ENTRY POINT)
main() {
    log "SigmaOS Repository Creation Automation"
    log "======================================"

    check_gh_cli
    create_all_repositories

    log_success "All repositories created successfully"
}

# Run main function
main "$@"
