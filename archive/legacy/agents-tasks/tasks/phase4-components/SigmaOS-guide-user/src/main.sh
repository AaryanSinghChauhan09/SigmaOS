#!/usr/bin/env bash
# SigmaOS Guide — End-User Guide CLI scaffold
# Upstream: Arch Wiki (https://wiki.archlinux.org/), Ubuntu Help (https://help.ubuntu.com/)
set -euo pipefail

GUIDE_VERSION="0.5"
GUIDE_NAME="SigmaOS User Guide"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand>

$GUIDE_NAME v$GUIDE_VERSION

Subcommands:
  desktop       Desktop environment overview
  apps          Installing and managing applications
  settings      Accessing and using settings
  network       Connecting to networks
  files         File manager usage
  keyboard      Keyboard shortcuts reference
  theme         Changing themes and appearance
  updates       Keeping SigmaOS up to date
  help          Getting help and support
  list          List all topics
  version       Print guide version

EOF
}

topic_desktop() { cat <<'EOF'
Desktop Overview
=================
SigmaOS uses a Wayland compositor (sway-based) with a curated set of defaults:
  • Panel    — top bar with clock, workspaces, system tray
  • Launcher — Super+Space to search and launch apps
  • Files    — Nautilus-compatible file manager

Workspace switching: Super+1..9
Open terminal      : Super+Return
Close window       : Super+Shift+Q
Full-screen        : Super+F
EOF
}

topic_apps() { cat <<'EOF'
Installing Applications
========================
GUI: Open Settings → Software Center
CLI: sigmaos-pkg install <package-name>

Examples:
  sigmaos-pkg install firefox
  sigmaos-pkg install libreoffice
  sigmaos-pkg search image viewer

Remove an app:
  sigmaos-pkg remove <package-name>

List installed:
  sigmaos-pkg list --installed
EOF
}

topic_settings() { cat <<'EOF'
Settings
=========
Open: Super+Space → "Settings"  or  sigmaos-control-center

Key panels:
  Display   — resolution, scale, refresh rate
  Sound     — output/input devices, volume
  Network   — Wi-Fi, wired, VPN
  Accounts  — users, passwords, auto-login
  Power     — battery, suspend policy
  Updates   — update channel, auto-update
  Theme     — dark/light mode, accent color, cursor
EOF
}

topic_network() { cat <<'EOF'
Network Connection
===================
Wi-Fi:
  1. Click the network icon in the panel  OR  run: nmcli device wifi connect <SSID>
  2. Enter the password when prompted

Wired (DHCP): automatic on cable connect
Wired (static): Settings → Network → Wired → IPv4 → Manual

VPN: Settings → Network → VPN → Add

CLI quick-reference:
  nmcli device status          # list network devices
  nmcli connection up <name>   # bring up a saved connection
EOF
}

topic_files() { cat <<'EOF'
File Manager
=============
Open: Super+Space → "Files"  or  sigmaos-files

Keyboard shortcuts:
  Ctrl+L   — type a path directly
  Ctrl+H   — show hidden files
  Ctrl+F   — search
  Delete   — move to trash
  Ctrl+Z   — undo last file operation

Bookmarks: drag a folder to the sidebar  or  Ctrl+D
EOF
}

topic_keyboard() { cat <<'EOF'
Keyboard Shortcuts Reference
==============================
Window management:
  Super+Space       Launch app (Launcher)
  Super+Return      Open terminal
  Super+Shift+Q     Close focused window
  Super+F           Toggle fullscreen
  Super+1..9        Switch workspace

System:
  Super+L           Lock screen
  Ctrl+Alt+Del      Power menu
  PrintScreen       Screenshot
  Ctrl+PrintScreen  Screenshot region

App shortcuts:
  Ctrl+C/V/X        Copy/Paste/Cut (standard)
  F11               Fullscreen in most apps
EOF
}

topic_theme() { cat <<'EOF'
Themes and Appearance
======================
CLI:
  sigmaos-theme --mode dark        # switch to dark mode
  sigmaos-theme --mode light       # switch to light mode
  sigmaos-theme --list             # list available themes
  sigmaos-theme --switch <name>    # apply a theme

GUI: Settings → Theme

Cursor, icons, fonts can also be set in Settings → Theme.
EOF
}

topic_updates() { cat <<'EOF'
System Updates
===============
Check for updates:  sigmaos-update --check
Apply updates:      sigmaos-update --apply
Auto-updates:       Settings → Updates → Auto-update

Channels:
  stable   — recommended for most users
  testing  — early access, may have rough edges

Rollback a bad update:
  sigmaos-update --rollback
EOF
}

topic_help() { cat <<'EOF'
Getting Help
=============
• Built-in guide:   sigmaos-guide-user <topic>
• Wiki:             https://wiki.sigmaos.org
• Community forum:  https://forum.sigmaos.org
• Bug reports:      https://github.com/SigmaOS/SigmaOS/issues
• IRC/Matrix:       #sigmaos on libera.chat

For hardware-specific issues: sigmaos-hardware --diagnostics
For package issues:           sigmaos-pkg --self-check
EOF
}

list_topics() {
    echo "Available user guide topics:"
    for t in desktop apps settings network files keyboard theme updates help; do
        echo "  $t"
    done
}

case "${1:-}" in
    desktop)     topic_desktop ;;
    apps)        topic_apps ;;
    settings)    topic_settings ;;
    network)     topic_network ;;
    files)       topic_files ;;
    keyboard)    topic_keyboard ;;
    theme)       topic_theme ;;
    updates)     topic_updates ;;
    help)        topic_help ;;
    list)        list_topics ;;
    version)     echo "$GUIDE_NAME v$GUIDE_VERSION" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown topic: $1" >&2; usage; exit 1 ;;
esac
