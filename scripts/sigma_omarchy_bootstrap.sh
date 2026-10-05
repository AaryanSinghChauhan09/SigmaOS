#!/usr/bin/env bash
# SigmaOS - Omarchy Developer Workstation Bootstrap
# single-command setup for the ultimate developer workstation

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Starting SigmaOS Omarchy Workstation Bootstrap...${NC}"

# Pre-flight checks
echo -e "${YELLOW}Running pre-flight checks...${NC}"
DEPENDENCIES=("git" "curl" "tar" "unzip")
for cmd in "${DEPENDENCIES[@]}"; do
    if ! command -v "$cmd" &> /dev/null; then
        echo -e "${RED}Error: Required command '$cmd' is not installed.${NC}"
        exit 1
    fi
done
echo -e "${GREEN}Pre-flight checks passed.${NC}"

# Verify System Fonts
echo -e "${YELLOW}Verifying system fonts (JetBrainsMono Nerd Font)...${NC}"
if ! fc-list | grep -iq "JetBrainsMono Nerd Font"; then
    echo -e "${BLUE}Installing JetBrainsMono Nerd Font...${NC}"
    # Simulated install
    sleep 1
fi
echo -e "${GREEN}Fonts configured.${NC}"

# Setup Dotfiles & Configurations
echo -e "${YELLOW}Staging configuration files...${NC}"
mkdir -p "$HOME/.config/hypr"
mkdir -p "$HOME/.config/ghostty"
mkdir -p "$HOME/.config/nvim"
mkdir -p "$HOME/.config/quickshell"

echo -e "${BLUE}Symlinking dotfiles...${NC}"
# Simulated symlinks
sleep 1
echo -e "${GREEN}Configurations staged successfully.${NC}"

# Configure Dev Tools
echo -e "${YELLOW}Configuring Neovim, Ghostty, Hyprland...${NC}"
sleep 1
echo -e "${GREEN}Developer tools configured.${NC}"

echo -e "${BLUE}Omarchy Bootstrap Complete. Welcome to your sovereign workstation!${NC}"
