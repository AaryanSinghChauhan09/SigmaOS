#!/usr/bin/env bash
# SigmaOS Component Repository Creation Script
#
# This script automates the creation of missing component repositories
# following the multi-repository architecture pattern (Omarchy-inspired)
#
# Usage: ./scripts/create_component_repo.sh <component-name> <description>
#
# Example: ./scripts/create_component_repo.sh SigmaOS-aur-helper "AUR helper for Arch Linux package management"

set -euo pipefail

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Print colored messages
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if gh CLI is installed
check_gh_cli() {
    if ! command -v gh &> /dev/null; then
        print_error "GitHub CLI (gh) is not installed"
        print_info "Install it from: https://cli.github.com/"
        exit 1
    fi
    print_success "GitHub CLI found"
}

# Check if user is authenticated
check_auth() {
    if ! gh auth status &> /dev/null; then
        print_error "Not authenticated with GitHub CLI"
        print_info "Run: gh auth login"
        exit 1
    fi
    print_success "Authenticated with GitHub"
}

# Create repository
create_repo() {
    local repo_name="$1"
    local description="$2"

    print_info "Creating repository: $repo_name"

    # Check if repo already exists
    if gh repo view "$repo_name" &> /dev/null; then
        print_warning "Repository $repo_name already exists"
        return 0
    fi

    # Create repository
    gh repo create "$repo_name" \
        --description "$description" \
        --public \
        --source=false \
        --gitignore=Rust \
        --license=MIT \
        --confirm

    print_success "Repository $repo_name created"
}

# Clone repository
clone_repo() {
    local repo_name="$1"
    local base_dir="$2"

    print_info "Cloning repository to $base_dir"

    if [ -d "$base_dir/$repo_name" ]; then
        print_warning "Directory $base_dir/$repo_name already exists"
        return 0
    fi

    gh repo clone "$repo_name" "$base_dir/$repo_name"

    print_success "Repository cloned"
}

# Initialize repository structure
init_repo_structure() {
    local repo_path="$1"
    local component_name="$2"

    print_info "Initializing repository structure"

    cd "$repo_path"

    # Create basic directory structure
    mkdir -p src tests docs .github/workflows

    # Create README.md
    cat > README.md << EOF
# $component_name

> SigmaOS component: $component_name

## Overview

This is a component of the SigmaOS operating system ecosystem.

## Development

See [SigmaOS Development Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Development) for general development guidelines.

## License

MIT OR GPL-2.0 — Free and Open Source Software.
© SigmaOS Contributors
EOF

    # Create Cargo.toml for Rust components
    cat > Cargo.toml << EOF
[package]
name = "$(echo $component_name | tr '[:upper:]' '[:lower:] | tr '-' '_')"
version = "0.1.0"
edition = "2021"
authors = ["SigmaOS Team"]
description = "SigmaOS component: $component_name"
license = "MIT OR GPL-2.0"
repository = "https://github.com/AaryanSinghChauhan09/$component_name"

[dependencies]

[dev-dependencies]

[profile.dev]
opt-level = 0
debug = false
incremental = false

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
EOF

    # Create .gitignore
    cat > .gitignore << EOF
# Rust
/target
**/*.rs.bk
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db
EOF

    # Create LICENSE
    cat > LICENSE << EOF
MIT License

Copyright (c) 2026 SigmaOS Contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF

    # Create src/lib.rs
    cat > src/lib.rs << EOF
//! $component_name
//!
//! SigmaOS component library

#![allow(warnings, unused)]

pub mod core;

pub use core::*;
EOF

    # Create src/core.rs
    cat > src/core.rs << EOF
//! Core functionality for $component_name

/// Core trait for $component_name
pub trait Component {
    /// Initialize the component
    fn init(&self) -> Result<(), Box<dyn std::error::Error>>;

    /// Execute the component
    fn execute(&self) -> Result<(), Box<dyn std::error::Error>>;
}

/// Default implementation
pub struct DefaultComponent;

impl Component for DefaultComponent {
    fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
EOF

    # Create .github/workflows/test.yml
    mkdir -p .github/workflows
    cat > .github/workflows/test.yml << EOF
name: CI/CD

on:
  push:
    branches: [ main, master, develop ]
  pull_request:
    branches: [ main, master, develop ]

env:
  CARGO_TERM_COLOR: always

jobs:
  check:
    name: Cargo Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
      - name: Run cargo check
        run: cargo check

  fmt:
    name: Code Formatting
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
          components: rustfmt
      - name: Run cargo fmt --check
        run: cargo fmt --check
EOF

    print_success "Repository structure initialized"
}

# Commit and push
commit_and_push() {
    local repo_path="$1"

    print_info "Committing and pushing initial structure"

    cd "$repo_path"

    git add .
    git commit -m "feat: initial commit for SigmaOS component

- Initialize repository structure
- Add basic Cargo.toml
- Add CI/CD workflow
- Add LICENSE and README

Generated with SigmaOS component creation script.

Co-Authored-By: Devin <158243242+devin-ai-integration[bot]@users.noreply.github.com>"

    git push origin main

    print_success "Initial commit pushed"
}

# Update COMPONENTS.toml
update_components_toml() {
    local repo_name="$1"
    local description="$2"
    local category="$3"
    local language="${4:-rust}"

    print_info "Updating COMPONENTS.toml"

    local components_file="/home/aaryansinghchauhan/SigmaOS/COMPONENTS.toml"

    # Add component to COMPONENTS.toml
    cat >> "$components_file" << EOF

[[component]]
name = "$repo_name"
description = "$description"
repo = "https://github.com/AaryanSinghChauhan09/$repo_name"
engine = "component"
category = "$category"
language = "$language"
inspirations = ["native"]
visibility = "public"
depends_on = []
EOF

    print_success "COMPONENTS.toml updated"
}

# Main function
main() {
    local repo_name="$1"
    local description="$2"
    local category="${3:-Tools}"
    local language="${4:-rust}"
    local base_dir="${5:-/home/aaryansinghchauhan}"

    print_info "Starting component repository creation"
    print_info "Repository: $repo_name"
    print_info "Description: $description"
    print_info "Category: $category"
    print_info "Language: $language"

    # Pre-flight checks
    check_gh_cli
    check_auth

    # Create repository
    create_repo "$repo_name" "$description"

    # Clone repository
    clone_repo "$repo_name" "$base_dir"

    # Initialize structure
    init_repo_structure "$base_dir/$repo_name" "$repo_name"

    # Commit and push
    commit_and_push "$base_dir/$repo_name"

    # Update COMPONENTS.toml
    update_components_toml "$repo_name" "$description" "$category" "$language"

    print_success "Component repository creation complete"
    print_info "Repository: https://github.com/AaryanSinghChauhan09/$repo_name"
}

# Check arguments
if [ $# -lt 2 ]; then
    print_error "Usage: $0 <repo-name> <description> [category] [language] [base-dir]"
    print_info "Example: $0 SigmaOS-aur-helper 'AUR helper for Arch Linux' 'Package Management' 'rust'"
    exit 1
fi

# Run main function
main "$@"
