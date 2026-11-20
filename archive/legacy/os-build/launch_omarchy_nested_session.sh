#!/usr/bin/env bash
# SigmaOS — Launch Omarchy Wayland / Hyprland Nested Session on active display
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo -e "\033[1;36m==> Launching SigmaOS Omarchy Desktop Experience...\033[0m"

# If a native Wayland session or nested compositor exists, use it
if command -v Hyprland >/dev/null 2>&1; then
    echo "[✓] Starting native Hyprland compositor..."
    exec Hyprland --config ~/.config/hypr/hyprland.conf
elif command -v Xephyr >/dev/null 2>&1; then
    echo "[✓] Launching Omarchy Desktop inside nested display server (:2)..."
    Xephyr -ac -br -noreset -screen 1280x800 :2 &
    XEPHYR_PID=$!
    sleep 1
    DISPLAY=:2 python3 "$ROOT_DIR/scripts/sigmaos_gui.py" &
    wait $XEPHYR_PID
else
    echo "[✓] Launching interactive SigmaOS Sovereign GUI Desktop on current DISPLAY..."
    exec python3 "$ROOT_DIR/scripts/sigmaos_gui.py"
fi
