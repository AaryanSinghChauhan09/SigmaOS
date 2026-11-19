#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# setup_dev_stacks.sh — Developer Runtime Stacks & Toolchain Provisioner
# Sets up Mise, Docker, Tmux, Lazygit, Node.js, Ruby on Rails, and Laravel.
#
# Usage:
#   scripts/setup_dev_stacks.sh [core|node|rails|laravel|all] [--dry-run]

set -euo pipefail

CYAN='\033[0;36m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; RED='\033[0;31m'
BOLD='\033[1m'; RESET='\033[0m'

info()    { echo -e "${CYAN}[stack]${RESET} $*"; }
success() { echo -e "${GREEN}[stack]${RESET} $*"; }
warn()    { echo -e "${YELLOW}[stack]${RESET} $*"; }
err()     { echo -e "${RED}[stack]${RESET} $*" >&2; }

DRY_RUN=0
STACK="${1:-core}"
shift 2>/dev/null || true

for arg in "$@"; do
  [[ "$arg" == "--dry-run" ]] && DRY_RUN=1
done

run_cmd() {
  if [ "$DRY_RUN" -eq 1 ]; then
    warn "[DRY-RUN] Would run: $*"
  else
    eval "$@"
  fi
}

install_core() {
  info "1. Provisioning Core Developer Toolchain (Mise, Tmux, Lazygit, Docker)..."
  if command -v pacman &>/dev/null; then
    run_cmd "sudo pacman -S --needed --noconfirm mise tmux lazygit docker docker-compose"
    run_cmd "sudo systemctl enable --now docker"
    run_cmd "sudo usermod -aG docker \$USER || true"
  elif command -v apt-get &>/dev/null; then
    run_cmd "sudo apt-get install -y tmux lazygit docker.io docker-compose"
  fi
  success "Core developer toolchain ready."
}

install_node() {
  info "2. Setting up Node.js LTS stack..."
  if command -v mise &>/dev/null; then
    run_cmd "mise use -g node@lts"
    run_cmd "mise exec -- npm install -g pnpm yarn typescript tsx"
  elif command -v pacman &>/dev/null; then
    run_cmd "sudo pacman -S --needed --noconfirm nodejs npm yarn"
  fi
  success "Node.js stack ready."
}

install_rails() {
  info "3. Setting up Ruby on Rails stack..."
  if command -v mise &>/dev/null; then
    run_cmd "mise use -g ruby@latest"
    run_cmd "mise exec -- gem install rails --no-document"
  elif command -v pacman &>/dev/null; then
    run_cmd "sudo pacman -S --needed --noconfirm ruby rubygems"
    run_cmd "gem install rails --no-document"
  fi
  success "Ruby on Rails stack ready."
}

install_laravel() {
  info "4. Setting up PHP & Laravel stack..."
  if command -v pacman &>/dev/null; then
    run_cmd "sudo pacman -S --needed --noconfirm php composer"
  elif command -v apt-get &>/dev/null; then
    run_cmd "sudo apt-get install -y php composer"
  fi
  success "PHP & Laravel stack ready."
}

echo -e "\n${BOLD}${CYAN}=== SigmaOS Developer Stacks Provisioner ===${RESET}\n"
[ "$DRY_RUN" -eq 1 ] && warn "DRY-RUN active. No system packages will be installed."

case "$STACK" in
  core)
    install_core
    ;;
  node)
    install_core
    install_node
    ;;
  rails)
    install_core
    install_rails
    ;;
  laravel)
    install_core
    install_laravel
    ;;
  all)
    install_core
    install_node
    install_rails
    install_laravel
    ;;
  *)
    err "Unknown stack: $STACK. Choose: core, node, rails, laravel, all"
    exit 1
    ;;
esac

echo ""
success "Developer environment setup complete."
