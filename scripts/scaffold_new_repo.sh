#!/usr/bin/env bash
# =============================================================================
# scaffold_new_repo.sh — SigmaOS component repository scaffolder
#
# Usage:
#   ./scripts/scaffold_new_repo.sh <repo-name> [github-org] [author] [year]
#
# Examples:
#   ./scripts/scaffold_new_repo.sh sigma-scheduler
#   ./scripts/scaffold_new_repo.sh sigma-ipc sigmaos "SigmaOS Contributors" 2026
#
# What this script does:
#   1. Clones the target repository (must exist and be empty, or be created)
#   2. Creates the `main` branch with source code scaffolding
#   3. Creates the `docs` branch with documentation scaffolding
#   4. Replaces COMPONENT_NAME placeholder throughout
#   5. Pushes both branches
#   6. Sets main as the default branch
#
# Requirements:
#   - git, sed, gh (GitHub CLI, optional but recommended)
#   - TEMPLATES_DIR: path to the templates/ directory (default: sibling of scripts/)
#   - GITHUB_TOKEN or gh auth login for push access
# =============================================================================

set -euo pipefail
IFS=$'\n\t'

# -----------------------------------------------------------------------------
# Helpers
# -----------------------------------------------------------------------------

BOLD="$(tput bold 2>/dev/null || echo '')"
RESET="$(tput sgr0 2>/dev/null || echo '')"
GREEN="$(tput setaf 2 2>/dev/null || echo '')"
RED="$(tput setaf 1 2>/dev/null || echo '')"
CYAN="$(tput setaf 6 2>/dev/null || echo '')"
YELLOW="$(tput setaf 3 2>/dev/null || echo '')"

info()    { echo "${CYAN}[scaffold]${RESET} $*"; }
success() { echo "${GREEN}[scaffold]${RESET} $*"; }
warn()    { echo "${YELLOW}[scaffold]${RESET} WARNING: $*" >&2; }
die()     { echo "${RED}[scaffold]${RESET} ERROR: $*" >&2; exit 1; }

# -----------------------------------------------------------------------------
# Argument parsing
# -----------------------------------------------------------------------------

usage() {
  cat <<EOF
${BOLD}Usage:${RESET}
  $0 <repo-name> [github-org] [author] [year]

${BOLD}Arguments:${RESET}
  repo-name     Name of the component repository (e.g., sigma-scheduler)
  github-org    GitHub organisation or user (default: sigmaos)
  author        Author string for LICENSE (default: SigmaOS Contributors)
  year          Copyright year (default: current year)

${BOLD}Environment Variables:${RESET}
  TEMPLATES_DIR   Path to the templates/ directory
                  (default: directory adjacent to this script)
  GIT_REMOTE_BASE Base URL for git remotes
                  (default: git@github.com:<org>)
  SKIP_PUSH       Set to 1 to skip git push (dry-run mode)
  SKIP_GH_CLI     Set to 1 to skip gh CLI calls

${BOLD}Examples:${RESET}
  $0 sigma-scheduler
  $0 sigma-ipc sigmaos "SigmaOS Contributors" 2026
  SKIP_PUSH=1 $0 sigma-net  # dry run, no push
EOF
}

if [[ $# -lt 1 ]]; then
  usage
  exit 2
fi

REPO_NAME="${1}"
GITHUB_ORG="${2:-sigmaos}"
AUTHOR="${3:-SigmaOS Contributors}"
YEAR="${4:-$(date +%Y)}"

# Validate repo name: lowercase alphanumeric and hyphens only
if ! [[ "${REPO_NAME}" =~ ^[a-z][a-z0-9-]*$ ]]; then
  die "Invalid repo name '${REPO_NAME}'. Use lowercase letters, numbers, and hyphens only."
fi

# Derive component name: hyphens → underscores (for Rust crate naming)
COMPONENT_NAME="${REPO_NAME//-/_}"

SKIP_PUSH="${SKIP_PUSH:-0}"
SKIP_GH_CLI="${SKIP_GH_CLI:-0}"

# Locate templates directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATES_DIR="${TEMPLATES_DIR:-"$(dirname "${SCRIPT_DIR}")/templates"}"

GIT_REMOTE_BASE="${GIT_REMOTE_BASE:-"git@github.com:${GITHUB_ORG}"}"
REMOTE_URL="${GIT_REMOTE_BASE}/${REPO_NAME}.git"

# -----------------------------------------------------------------------------
# Pre-flight checks
# -----------------------------------------------------------------------------

info "Pre-flight checks..."

command -v git >/dev/null 2>&1 || die "git not found in PATH"
command -v sed >/dev/null 2>&1 || die "sed not found in PATH"

[[ -d "${TEMPLATES_DIR}/main" ]]  || die "Template directory not found: ${TEMPLATES_DIR}/main"
[[ -d "${TEMPLATES_DIR}/docs" ]]  || die "Template directory not found: ${TEMPLATES_DIR}/docs"

if [[ "${SKIP_GH_CLI}" -eq 0 ]] && ! command -v gh >/dev/null 2>&1; then
  warn "gh CLI not found — skipping GitHub-specific operations (set SKIP_GH_CLI=1 to suppress)"
  SKIP_GH_CLI=1
fi

success "Pre-flight checks passed"

# -----------------------------------------------------------------------------
# Workspace setup
# -----------------------------------------------------------------------------

WORK_DIR="$(mktemp -d)"
trap 'info "Cleaning up workspace ${WORK_DIR}"; rm -rf "${WORK_DIR}"' EXIT

REPO_DIR="${WORK_DIR}/${REPO_NAME}"

info "Working directory: ${WORK_DIR}"
info "Repository name:   ${REPO_NAME}"
info "Component name:    ${COMPONENT_NAME}"
info "GitHub org:        ${GITHUB_ORG}"
info "Author:            ${AUTHOR}"
info "Year:              ${YEAR}"
info "Remote URL:        ${REMOTE_URL}"
[[ "${SKIP_PUSH}" -eq 1 ]] && warn "SKIP_PUSH=1 — changes will NOT be pushed"

# -----------------------------------------------------------------------------
# Clone or initialise repository
# -----------------------------------------------------------------------------

info "Cloning repository..."
if git clone "${REMOTE_URL}" "${REPO_DIR}" 2>/dev/null; then
  info "Cloned existing repository"
else
  warn "Clone failed — initialising a new local repository"
  warn "You will need to add the remote and push manually, or create the repo first."
  mkdir -p "${REPO_DIR}"
  git -C "${REPO_DIR}" init
  git -C "${REPO_DIR}" remote add origin "${REMOTE_URL}"
fi

# -----------------------------------------------------------------------------
# Utility: replace placeholders in a file (in-place, portable sed)
# -----------------------------------------------------------------------------

replace_placeholders() {
  local file="$1"
  # Use a temp file approach for portability (macOS vs GNU sed differ on -i)
  local tmp
  tmp="$(mktemp)"
  sed \
    -e "s|COMPONENT_NAME|${COMPONENT_NAME}|g" \
    -e "s|YEAR|${YEAR}|g" \
    -e "s|AUTHOR|${AUTHOR}|g" \
    -e "s|GITHUB_ORG|${GITHUB_ORG}|g" \
    "${file}" > "${tmp}"
  mv "${tmp}" "${file}"
}

# -----------------------------------------------------------------------------
# Utility: copy template files and replace placeholders
# -----------------------------------------------------------------------------

install_template_files() {
  local src_dir="$1"   # e.g., templates/main
  local dst_dir="$2"   # destination inside repo

  find "${src_dir}" -type f | while IFS= read -r src_file; do
    # Compute relative path within template dir
    local rel_path="${src_file#"${src_dir}/"}"
    local dst_file="${dst_dir}/${rel_path}"

    # Create parent directory
    mkdir -p "$(dirname "${dst_file}")"

    # Copy
    cp "${src_file}" "${dst_file}"

    # Replace placeholders
    replace_placeholders "${dst_file}"

    info "  installed: ${rel_path}"
  done
}

# -----------------------------------------------------------------------------
# main branch — source code scaffold
# -----------------------------------------------------------------------------

info "Setting up main branch..."

# Ensure we are on (or create) main
git -C "${REPO_DIR}" checkout -B main 2>/dev/null || \
  git -C "${REPO_DIR}" checkout --orphan main

# Remove everything (start clean, but keep .git)
find "${REPO_DIR}" -mindepth 1 -maxdepth 1 ! -name '.git' -exec rm -rf {} +

# Install main branch templates
install_template_files "${TEMPLATES_DIR}/main" "${REPO_DIR}"

# Rename the library crate-type name in Cargo.toml:
# The lib name uses underscore-separated component name (already done by placeholder)
# but double-check the lib name ends up as component name, not component_name_lib if undesired
# (Template deliberately uses COMPONENT_NAME_lib to avoid collision with binary name)

# Verify no .md/.txt files crept in (sigma-check equivalent)
VIOLATIONS="$(find "${REPO_DIR}" -not -path '*/.git/*' \( -name '*.md' -o -name '*.txt' -o -name '*.rst' \) 2>/dev/null || true)"
if [[ -n "${VIOLATIONS}" ]]; then
  die "sigma-check FAIL: documentation files found on main:\n${VIOLATIONS}\nRemove them before proceeding."
fi

# Stage and commit
git -C "${REPO_DIR}" add --all
git -C "${REPO_DIR}" \
  -c "user.name=SigmaOS Scaffold" \
  -c "user.email=scaffold@sigmaos.dev" \
  commit -m "chore(scaffold): initial main branch scaffolding

Scaffolded by scaffold_new_repo.sh for component: ${COMPONENT_NAME}
Templates: SigmaOS standard component templates
Author: ${AUTHOR}
Year: ${YEAR}

Includes:
  - .gitignore (Rust/C/Python/Shell)
  - LICENSE (MIT)
  - Makefile (build, test, lint, fmt, check, install, sigma-check)
  - Cargo.toml (edition 2021, serde, thiserror, log, env_logger, clap)
  - src/lib.rs (ComponentName type, Error enum, Result alias, tests)
  - src/main.rs (clap CLI, --verbose, --config, run/check subcommands)"

success "main branch ready"

# -----------------------------------------------------------------------------
# docs branch — documentation scaffold
# -----------------------------------------------------------------------------

info "Setting up docs branch..."

# Create an orphan docs branch (no shared history with main — docs are separate)
git -C "${REPO_DIR}" checkout --orphan docs

# Remove everything staged from main
git -C "${REPO_DIR}" rm -rf --cached . >/dev/null 2>&1 || true
find "${REPO_DIR}" -mindepth 1 -maxdepth 1 ! -name '.git' -exec rm -rf {} +

# Install docs branch templates
install_template_files "${TEMPLATES_DIR}/docs" "${REPO_DIR}"

# Stage and commit
git -C "${REPO_DIR}" add --all
git -C "${REPO_DIR}" \
  -c "user.name=SigmaOS Scaffold" \
  -c "user.email=scaffold@sigmaos.dev" \
  commit -m "docs(scaffold): initial docs branch scaffolding

Scaffolded by scaffold_new_repo.sh for component: ${COMPONENT_NAME}

Includes:
  - README.md  (Overview, Architecture, Dependencies, Building, Testing,
                Configuration, Contributing, License, Upstream Inspirations)
  - CONTRIBUTING.md (All 80 SigmaOS principles, code style, Conventional
                     Commits, PR process, branch naming, test requirements)
  - ARCHITECTURE.md (Purpose, context, modules, interfaces, data flow,
                     security model, performance targets, ADRs)
  - SECURITY.md (Vulnerability reporting, secure coding standards,
                 threat model, security checklist)"

success "docs branch ready"

# -----------------------------------------------------------------------------
# Push branches
# -----------------------------------------------------------------------------

if [[ "${SKIP_PUSH}" -eq 1 ]]; then
  warn "Skipping push (SKIP_PUSH=1). Branches exist locally in: ${REPO_DIR}"
  warn "To push manually:"
  warn "  cd ${REPO_DIR}"
  warn "  git push -u origin main"
  warn "  git push -u origin docs"
else
  info "Pushing main branch..."
  git -C "${REPO_DIR}" push -u origin main
  success "main branch pushed"

  info "Pushing docs branch..."
  git -C "${REPO_DIR}" push -u origin docs
  success "docs branch pushed"
fi

# -----------------------------------------------------------------------------
# Post-push GitHub configuration (requires gh CLI)
# -----------------------------------------------------------------------------

if [[ "${SKIP_GH_CLI}" -eq 0 ]] && [[ "${SKIP_PUSH}" -eq 0 ]]; then
  info "Configuring repository via GitHub CLI..."

  # Set main as the default branch
  if gh repo edit "${GITHUB_ORG}/${REPO_NAME}" --default-branch main 2>/dev/null; then
    success "Default branch set to: main"
  else
    warn "Could not set default branch via gh CLI (may need admin permissions)"
  fi

  # Set docs branch description (via topic or description — gh doesn't expose branch descriptions)
  # Instead, set the repo description and topics
  gh repo edit "${GITHUB_ORG}/${REPO_NAME}" \
    --description "SigmaOS component: ${COMPONENT_NAME}" \
    --add-topic "sigmaos" \
    --add-topic "rust" \
    --add-topic "systems" \
    2>/dev/null && success "Repository metadata updated" || warn "Could not update repo metadata"

  # Branch protection for main: require PR, no direct push
  if gh api \
    --method PUT \
    -H "Accept: application/vnd.github+json" \
    "/repos/${GITHUB_ORG}/${REPO_NAME}/branches/main/protection" \
    --field required_status_checks='{"strict":true,"contexts":["build","test","lint"]}' \
    --field enforce_admins=false \
    --field required_pull_request_reviews='{"required_approving_review_count":1}' \
    --field restrictions=null \
    2>/dev/null; then
    success "Branch protection configured for main"
  else
    warn "Could not configure branch protection (may need admin permissions)"
  fi
fi

# -----------------------------------------------------------------------------
# Summary
# -----------------------------------------------------------------------------

echo ""
echo "${BOLD}${GREEN}═══════════════════════════════════════════════════════${RESET}"
echo "${BOLD}${GREEN}  SigmaOS Scaffold Complete: ${REPO_NAME}${RESET}"
echo "${BOLD}${GREEN}═══════════════════════════════════════════════════════${RESET}"
echo ""
echo "  Component:    ${COMPONENT_NAME}"
echo "  Repository:   ${REMOTE_URL}"
echo "  main branch:  source code (Cargo.toml, src/, Makefile, LICENSE, .gitignore)"
echo "  docs branch:  documentation (README.md, CONTRIBUTING.md, ARCHITECTURE.md, SECURITY.md)"
echo ""
echo "${BOLD}Next steps:${RESET}"
echo "  1. Clone the repo: git clone ${REMOTE_URL}"
echo "  2. Implement the component in src/lib.rs"
echo "  3. Fill in ARCHITECTURE.md (docs branch) with real design detail"
echo "  4. Run: make sigma-check lint test"
echo "  5. Open your first PR!"
echo ""
echo "${CYAN}Remember:${RESET} main branch = source code only. Docs go on the docs branch."
echo ""
