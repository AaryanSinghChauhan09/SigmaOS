#!/usr/bin/env bash
# build-iso-helper.sh — Helper script for ISO build with prerequisite checks
#
# This script validates prerequisites and provides clear instructions for
# building the SigmaOS ISO using Docker or native Arch Linux.
# Following: REPRODUCIBLE BUILDS, DETERMINISTIC TESTING, FAIL-FAST
#
# Usage:
#   sudo ./scripts/build-iso-helper.sh [--docker] [--native]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; CYAN='\033[0;36m'
BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}[iso-helper]${RESET} $*"; }
success() { echo -e "${GREEN}[iso-helper]${RESET} $*"; }
warn()    { echo -e "${YELLOW}[iso-helper]${RESET} $*"; }
err()     { echo -e "${RED}[iso-helper]${RESET} $*" >&2; }

# Check if running as root
check_root() {
    if [[ "$(id -u)" != "0" ]]; then
        err "This script requires root privileges for Docker privileged mode"
        err "Run with: sudo $0 $*"
        exit 1
    fi
}

# Check Docker availability
check_docker() {
    info "Checking Docker availability..."
    
    if ! command -v docker &>/dev/null; then
        err "Docker is not installed"
        err "Install it with: sudo apt-get install -y docker.io"
        err "Then enable and start: sudo systemctl enable --now docker"
        exit 1
    fi
    
    if ! docker info &>/dev/null; then
        err "Docker daemon is not running or current user lacks access"
        err "Start Docker: sudo systemctl start docker"
        err "Add user to docker group: sudo usermod -aG docker \$USER"
        err "Then log out and log back in"
        exit 1
    fi
    
    success "Docker is available and running"
}

# Check mkarchiso availability (for native build)
check_mkarchiso() {
    info "Checking mkarchiso availability..."
    
    if ! command -v mkarchiso &>/dev/null; then
        err "mkarchiso is not installed"
        err "This requires an Arch Linux host"
        err "Install it with: sudo pacman -S archiso"
        exit 1
    fi
    
    success "mkarchiso is available"
}

# Validate ISO profile
validate_profile() {
    info "Validating ISO profile..."
    
    local profile_dir="$ROOT_DIR/iso/archiso"
    
    if [[ ! -d "$profile_dir" ]]; then
        err "ISO profile directory not found: $profile_dir"
        exit 1
    fi
    
    local required_files=(
        "$profile_dir/profiledef.sh"
        "$profile_dir/packages.x86_64"
        "$profile_dir/airootfs/usr/local/bin/sigmaos-install"
    )
    
    for f in "${required_files[@]}"; do
        if [[ ! -f "$f" ]]; then
            err "Required file missing: $f"
            exit 1
        fi
    done
    
    local pkg_count
    pkg_count=$(grep -v '^#' "$profile_dir/packages.x86_64" | grep -v '^$' | wc -l)
    success "ISO profile validated: $pkg_count packages configured"
}

# Build using Docker
build_docker() {
    info "Building ISO using Docker..."
    
    check_root
    check_docker
    validate_profile
    
    "$SCRIPT_DIR/build_archiso.sh" --docker
}

# Build using native Arch
build_native() {
    info "Building ISO using native mkarchiso..."
    
    check_root
    check_mkarchiso
    validate_profile
    
    "$SCRIPT_DIR/build_archiso.sh" --native
}

# Main
main() {
    local mode=""
    
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --docker) mode="docker"; shift ;;
            --native) mode="native"; shift ;;
            -h|--help) 
                echo "Usage: sudo $0 [--docker|--native]"
                echo ""
                echo "Options:"
                echo "  --docker  Build using Docker (works on any Linux host)"
                echo "  --native  Build using mkarchiso (requires Arch Linux host)"
                exit 0
                ;;
            *) err "Unknown option: $1"; exit 1 ;;
        esac
    done
    
    echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════╗${RESET}"
    echo -e "${BOLD}${CYAN}║   SigmaOS ISO Build Helper                 ║${RESET}"
    echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════╝${RESET}"
    echo ""
    
    if [[ -z "$mode" ]]; then
        # Auto-detect
        if command -v mkarchiso &>/dev/null; then
            mode="native"
        elif command -v docker &>/dev/null && docker info &>/dev/null 2>&1; then
            mode="docker"
        else
            err "Neither mkarchiso nor Docker available"
            err "Install Docker: sudo apt-get install -y docker.io"
            err "Or use Arch Linux with mkarchiso"
            exit 1
        fi
    fi
    
    case "$mode" in
        docker) build_docker ;;
        native) build_native ;;
    esac
}

main "$@"
