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

# Custom prompt
export PS1="\[\033[1;35m\]sigmaos\[\033[0m\]:\[\033[1;34m\]\w\[\033[0m\]\$ "

# Start subshell
exec bash --noprofile --norc
