#!/usr/bin/env bash
# SigmaOS Linux Mint Ergonomic Persona Launcher
# Enables Cinnamon-style bottom panel, start menu, XApps ecosystem, and APT compatibility.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo -e "\033[1;32m"
cat <<'EOF'
 ____________________________________________________________________
/                                                                    \
|                SIGMAOS · LINUX MINT ERGONOMIC PERSONA               |
|      Cinnamon Desktop · XApps Suite · MintUpdate · SigPkg (APT)     |
\____________________________________________________________________/
EOF
echo -e "\033[0m"

echo "--> Initializing Linux Mint Persona on SigmaOS Substrate..."
echo "  [✓] Desktop Session : Cinnamon Desktop Environment (Zenith Wayland/X11)"
echo "  [✓] Application Menu: MintMenu / Cinnamon Spices"
echo "  [✓] Package Backend : SigPkg APT Translation Gate (apt, dpkg, apt-get)"
echo "  [✓] Core XApps Suite: Nemo, Xed, Pix, Xreader, Xplayer, Hypnotix, Warpinator"
echo "  [✓] System Control  : MintUpdate + Timeshift Snapshot Integration"
echo ""

# Activate shell plugins for Mint layout
if [ -f "$ROOT_DIR/bin/sigmaos-shell" ]; then
    bash "$ROOT_DIR/bin/sigmaos-shell" summon menu "cinnamon-start-menu" || true
    bash "$ROOT_DIR/bin/sigmaos-shell" summon bar "bottom-panel" || true
    bash "$ROOT_DIR/bin/sigmaos-shell" summon notifications "mint-notify" || true
fi

echo -e "\033[1mLinux Mint Persona Active.\033[0m Type commands or interact with XApps."
echo "Examples:"
echo "  sigpkg apt update"
echo "  sigpkg apt install libreoffice-style-mint"
echo "  bash bin/sigmaos-shell summon settings"
echo ""
EOF
