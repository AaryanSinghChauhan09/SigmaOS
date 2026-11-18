#!/usr/bin/env bash
# ==============================================================================
# Launch SigmaOS in an authentic isolated nested X11 display session (:2)
# ==============================================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

echo -e "\033[1;36m==> Launching SigmaOS Isolated Desktop Environment (Xephyr :2)...\033[0m"

# Kill any existing :2 server
pkill -f "Xephyr :2" 2>/dev/null || true
sleep 1

# Launch nested X server with 1280x800 resolution
Xephyr -ac -br -noreset -screen 1280x800 :2 &
XEPHYR_PID=$!

# Wait for display server to initialize
for i in {1..20}; do
    if xset -q -display :2 >/dev/null 2>&1; then
        echo "[✓] Nested display :2 is online."
        break
    fi
    sleep 0.2
done

# Launch SigmaOS Native Desktop inside :2
DISPLAY=:2 python3 "$SCRIPT_DIR/sigmaos_native_app.py" &
APP_PID=$!

wait $XEPHYR_PID
