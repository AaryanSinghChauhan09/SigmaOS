#!/bin/bash
# ==============================================================================
# Sovereign Arch & Omarchy Complete OS Readiness Verification Suite
# ==============================================================================
set -euo pipefail

PASS=0
FAIL=0

run_test() {
    local name="$1"
    local file="$2"
    echo -n "[TEST] $name ... "
    if rustc --edition 2021 --test "$file" -o /tmp/sigmaos_suite_test 2>/dev/null && /tmp/sigmaos_suite_test >/dev/null 2>&1; then
        echo "PASS"
        PASS=$((PASS + 1))
    else
        echo "FAIL"
        FAIL=$((FAIL + 1))
    fi
    rm -f /tmp/sigmaos_suite_test
}

echo "=== SigmaOS Architecture & Runtime Verification Suite ==="

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../src/distro" && pwd)"

run_test "Session Environment Variables" "$DIR/arch_omarchy_session_env_v35.rs"
run_test "Network Manager Abstraction"   "$DIR/arch_omarchy_network_manager_v35.rs"
run_test "User & PAM Provisioning"       "$DIR/arch_omarchy_useradd_pam_v35.rs"
run_test "GTK/Qt Theming & Fonts"        "$DIR/arch_omarchy_theme_font_v35.rs"
run_test "Update & Snapper Scheduler"    "$DIR/arch_omarchy_update_scheduler_v35.rs"
run_test "XDG MIME Associations"         "$DIR/arch_omarchy_xdg_mime_v35.rs"
run_test "Session Idle & Screen Lock"    "$DIR/arch_omarchy_session_lock_v35.rs"
run_test "Notification Daemon"           "$DIR/arch_omarchy_notification_daemon_v35.rs"
run_test "Keyboard Input & Keybinds"     "$DIR/arch_omarchy_keyboard_input_v35.rs"
run_test "Hotplug Display Layout"        "$DIR/arch_omarchy_hotplug_display_v35.rs"
run_test "Battery & Power Optimization"  "$DIR/arch_omarchy_battery_power_v35.rs"
run_test "Live TUI Dashboard"            "$DIR/arch_omarchy_live_tui_v35.rs"
run_test "XDG Desktop Portal & Wofi"     "$DIR/arch_omarchy_xdg_desktop_v35.rs"
run_test "Pacman Hooks & Snapper"        "$DIR/arch_pacman_hooks_snapper_v35.rs"
run_test "Wallust Palette & PipeWire"    "$DIR/omarchy_wallust_audio_v35.rs"
run_test "mkinitcpio & UKI Engine"       "$DIR/arch_mkinitcpio_v35.rs"
run_test "Live Installer Subvolume Engine" "$DIR/arch_omarchy_live_installer.rs"
run_test "SigmaOS Init System v1"        "$DIR/sigma_init_v1.rs"

echo "=========================================================="
echo "Results: $PASS Passed, $FAIL Failed"

if [ "$FAIL" -eq 0 ]; then
    echo "Status: ALL SUBSYSTEMS PRODUCTION READY"
    exit 0
else
    echo "Status: SOME TESTS FAILED"
    exit 1
fi
