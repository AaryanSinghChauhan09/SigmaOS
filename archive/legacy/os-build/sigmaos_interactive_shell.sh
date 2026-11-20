#!/usr/bin/env bash
# SigmaOS Interactive OS Shell Environment
# Provides an interactive CLI session where users can directly test Arch & Omarchy commands.
set -u

# Ensure ~/.local/bin is in PATH
export PATH="${HOME}/.local/bin:${PATH}"

clear
echo -e "\033[1;36m"
cat << 'BANNER'
╔══════════════════════════════════════════════════════════════════════════════╗
║                     SIGMAOS LIVE INTERACTIVE SHELL                         ║
║         Unified Arch Linux Rolling Substrate & Omarchy Desktop Studio        ║
╚══════════════════════════════════════════════════════════════════════════════╝
BANNER
echo -e "\033[0m"

echo "Welcome to the live SigmaOS environment. You are running in an interactive session."
echo -e "Available direct commands:"
echo -e "  \033[1;32mpacman -Syu\033[0m              - Check and simulate rolling release upgrade"
echo -e "  \033[1;32mpacman -S <pkg>\033[0m          - Search / install package via ALPM bridge"
echo -e "  \033[1;32msigpkg aur <query>\033[0m       - Query Arch User Repository (AUR v5)"
echo -e "  \033[1;32momarchy doctor\033[0m           - Run Omarchy diagnostic health checks"
echo -e "  \033[1;32momarchy theme list\033[0m       - List dynamic Wallust wallpaper themes"
echo -e "  \033[1;32msigmaos --status\033[0m         - View kernel and launch status"
echo -e "  \033[1;33mexit\033[0m                     - Return to host shell"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Ensure ~/.local/bin and scripts are in PATH
export PATH="${HOME}/.local/bin:${ROOT_DIR}/scripts:${PATH}"

# Create interactive rcfile for pristine live environment
TEMP_RC="$(mktemp /tmp/sigmaos_rc.XXXXXX)"
cat << 'EOF' > "$TEMP_RC"
[ -f ~/.bashrc ] && source ~/.bashrc 2>/dev/null
export PATH="${HOME}/.local/bin:${PATH}"
export PS1="\[\033[1;35m\]󰣇 sigmaos\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]\$ "
export SIGMAOS_LIVE=1
alias pacman="sigpkg pacman"
alias yay="sigpkg"
alias paru="sigpkg"
alias aur="sigpkg aur"
alias update="sigpkg update"
alias doctor="omarchy doctor"
alias theme="omarchy theme"
alias fastfetch="neofetch 2>/dev/null || echo 'SigmaOS Sovereign Workstation'"
alias neofetch="neofetch 2>/dev/null || echo 'SigmaOS Sovereign Workstation'"
EOF

trap 'rm -f "$TEMP_RC"' EXIT
exec bash --rcfile "$TEMP_RC" -i
