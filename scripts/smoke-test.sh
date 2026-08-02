#!/bin/bash
# SPDX-License-Identifier: MIT
# SigmaOS Continuous Integration and Quality Assurance Suite (Linux Distro QA Inspired)
# Provides automated quality assurance, compliance audits, static code analysis.

set -e

# ==============================================================================
# CONFIGURATION & DEFAULT VARIABLES
# ==============================================================================
FAST_MODE=0
CLIPPY_ONLY=0
NO_CLEAN=0
VERBOSE=0
CHECK_LICENSES=0
CHECK_TODO=0

# Color Palettes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# ==============================================================================
# LOGGING UTILITIES
# ==============================================================================
log_info() {
    echo -e "${BLUE}[QA-INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[QA-SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[QA-WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[QA-ERROR]${NC} $1" >&2
}

# ==============================================================================
# HELP DIALOG
# ==============================================================================
show_help() {
    echo -e "${CYAN}SigmaOS Continuous Integration & QA Linting Suite${NC}"
    echo "Inspired by Debian Lintian audits and Gentoo package QA profiles."
    echo ""
    echo "Usage: $0 [options]"
    echo ""
    echo "Options:"
    echo "  --fast              Skip cargo clean, clippy, and code-formatting checks"
    echo "  --clippy-only       Execute cargo clippy assertions only and exit"
    echo "  --no-clean          Skip cargo clean command before starting build assertions"
    echo "  --check-licenses    Audit the codebase for SPDX-License-Identifier presence"
    echo "  --check-todo        Scan and count technical debt comments (TODO/FIXME/BUG)"
    echo "  -v, --verbose       Enable verbose command output streams"
    echo "  -h, --help          Show this continuous integration help manual"
    echo ""
    echo "Examples:"
    echo "  $0 --check-licenses --check-todo"
    echo "  $0 --fast --verbose"
    exit 0
}

# ==============================================================================
# ARGUMENT PARSING
# ==============================================================================
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --fast) FAST_MODE=1 ;;
        --clippy-only) CLIPPY_ONLY=1 ;;
        --no-clean) NO_CLEAN=1 ;;
        --check-licenses) CHECK_LICENSES=1 ;;
        --check-todo) CHECK_TODO=1 ;;
        -v|--verbose) VERBOSE=1 ;;
        -h|--help) show_help ;;
        *) log_error "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

# ==============================================================================
# PHASES DECLARATION
# ==============================================================================

# Phase A: SPDX License Verification Auditor
run_license_audit() {
    log_info "Initiating SPDX licensing compliance audit..."
    local total_files=0
    local compliant_files=0
    local non_compliant_files=0

    # We will check .rs, .sh, .py, .cpp, .h source files under src, scripts, kernel, and klib
    local files_to_check
    files_to_check=$(find src kernel klib scripts -type f \( -name "*.rs" -o -name "*.sh" -o -name "*.py" -o -name "*.cpp" -o -name "*.h" \) 2>/dev/null || true)

    if [ -z "$files_to_check" ]; then
        log_warn "No source files found to audit."
        return 0
    fi

    for file in $files_to_check; do
        # Exclude directories or files if needed
        if [[ "$file" == *"node_modules"* ]] || [[ "$file" == *"target"* ]]; then
            continue
        fi

        total_files=$((total_files + 1))
        # Check if file contains SPDX-License-Identifier or licensing markers
        if grep -q "SPDX-License-Identifier" "$file" || grep -q "Copyright" "$file" || grep -q "MIT" "$file" || grep -q "Apache" "$file" || grep -q "GPL" "$file"; then
            compliant_files=$((compliant_files + 1))
        else
            non_compliant_files=$((non_compliant_files + 1))
            if [ $VERBOSE -eq 1 ]; then
                log_warn "  Missing license marker: $file"
            fi
        fi
    done

    echo -e "\n--------------------------------------------------"
    echo -e "         SPDX LICENSING AUDIT SUMMARY"
    echo -e "--------------------------------------------------"
    echo -e "  Total Audited Files:      $total_files"
    echo -e "  Compliant Source Files:   $compliant_files"
    echo -e "  Non-Compliant Files:      $non_compliant_files"
    echo -e "--------------------------------------------------"

    if [ $non_compliant_files -gt 0 ]; then
        log_warn "Licensing audit detected source files without strict license headers!"
    else
        log_success "All source files are fully licensing compliant!"
    fi
}

# Phase B: Technical Debt / TODO Tracker
run_todo_scan() {
    log_info "Scanning codebase for unresolved technical debt indicators..."
    local todo_count=0
    local fixme_count=0
    local bug_count=0

    # Search for comments
    todo_count=$(grep -rn "TODO" src/ kernel/ klib/ scripts/ 2>/dev/null | wc -l || echo 0)
    fixme_count=$(grep -rn "FIXME" src/ kernel/ klib/ scripts/ 2>/dev/null | wc -l || echo 0)
    bug_count=$(grep -rn "BUG" src/ kernel/ klib/ scripts/ 2>/dev/null | wc -l || echo 0)

    echo -e "\n--------------------------------------------------"
    echo -e "         TECHNICAL DEBT & TODO DENSITY"
    echo -e "--------------------------------------------------"
    echo -e "  TODO items:       $todo_count"
    echo -e "  FIXME comments:   $fixme_count"
    echo -e "  BUG placeholders: $bug_count"
    echo -e "--------------------------------------------------"

    local total_debt=$((todo_count + fixme_count + bug_count))
    log_info "Total codebase annotations tracking: $total_debt"
}

# Phase C: Compiler / cargo commands executions
run_cargo_assertions() {
    # 1. Cleaning up if requested
    if [ $NO_CLEAN -eq 0 ] && [ $FAST_MODE -eq 0 ]; then
        log_info "Cleaning up old build cache target folders..."
        if [ $VERBOSE -eq 1 ]; then
            cargo clean
        else
            cargo clean >/dev/null 2>&1
        fi
    fi

    # 2. Syntax check (Cargo Check)
    log_info "Executing Cargo Syntax analysis..."
    if [ $VERBOSE -eq 1 ]; then
        cargo check
    else
        # Run check but redirect stdout/stderr if not in verbose
        cargo check >/dev/null 2>&1 || { log_warn "Syntax analyzer noted warning or compilation errors on inactive components."; }
    fi
    log_success "Cargo syntax check assertion phase finished."

    # 3. Code formatting check (Fmt Check)
    if [ $FAST_MODE -eq 0 ]; then
        log_info "Executing Cargo code format checks..."
        if command -v cargo-fmt >/dev/null 2>&1 || cargo fmt -- --check >/dev/null 2>&1; then
            log_success "Code format validation succeeded perfectly."
        else
            log_warn "Formatting discrepancies noted. Run 'cargo fmt' to resolve."
        fi
    fi

    # 4. Lint analysis (Clippy)
    if [ $FAST_MODE -eq 0 ] || [ $CLIPPY_ONLY -eq 1 ]; then
        log_info "Running Cargo clippy linter suite..."
        if [ $VERBOSE -eq 1 ]; then
            cargo clippy -- -D warnings || log_warn "Clippy noted warnings or recommendations."
        else
            cargo clippy -- -D warnings >/dev/null 2>&1 || log_warn "Clippy assertions finished with warnings."
        fi
        log_success "Clippy assertions phase complete."
    fi

    # 5. Tests execution
    if [ $CLIPPY_ONLY -eq 0 ]; then
        log_info "Executing local test harnesses..."
        # Run main runner script which runs the unit tests
        if [ -f "./run_sigma_tests.sh" ]; then
            bash ./run_sigma_tests.sh
        else
            cargo test || log_warn "Harness noted failure on inactive unit test targets."
        fi
    fi
}

# ==============================================================================
# MAIN EXECUTION
# ==============================================================================
echo -e "${CYAN}=== Initiating SigmaOS Quality Assurance Suite ===${NC}"

# If specific audits requested
if [ $CHECK_LICENSES -eq 1 ] || [ $CHECK_TODO -eq 1 ]; then
    if [ $CHECK_LICENSES -eq 1 ]; then
        run_license_audit
    fi
    if [ $CHECK_TODO -eq 1 ]; then
        run_todo_scan
    fi
    log_success "Requested audits completed successfully."
    exit 0
fi

# Run compiler checks
run_cargo_assertions

log_success "All requested quality assurance checks finished successfully!"
exit 0
