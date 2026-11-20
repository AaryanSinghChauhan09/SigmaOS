#!/bin/bash
# Fix incomplete Phase 4 component repo scaffolds
set -e

BASE="/home/aaryansinghchauhan/SigmaOS/.agents/tasks/phase4-components"
CI_TEMPLATE="$BASE/_ci-template.yml"

# Component descriptions for repos that need README
declare -A DESCRIPTIONS=(
  ["SigmaOS-audio-tuner"]="System audio tuning and equalizer tool for SigmaOS desktop"
  ["SigmaOS-automate"]="Desktop automation and macro recording tool for SigmaOS"
  ["SigmaOS-backup-tool"]="System backup and restore utility for SigmaOS"
  ["SigmaOS-bluetooth"]="Bluetooth system integration and applet for SigmaOS desktop"
  ["SigmaOS-bluetooth-tool"]="Bluetooth management GUI for pairing and device control"
  ["SigmaOS-bootloader"]="System bootloader integration and configuration for SigmaOS"
  ["SigmaOS-brand-logo"]="Brand assets, logos, and visual identity for SigmaOS"
  ["SigmaOS-calculator"]="Desktop calculator application for SigmaOS"
  ["SigmaOS-chatroom"]="Community chat client for SigmaOS project communication"
  ["SigmaOS-chromium"]="Chromium browser integration and patches for SigmaOS"
  ["SigmaOS-daemon-settings"]="Settings daemon for desktop services and preferences"
  ["SigmaOS-desktop-configuration"]="Desktop environment configuration tool for SigmaOS"
  ["SigmaOS-desktop-environment"]="Full desktop environment stack for SigmaOS"
  ["SigmaOS-desktop-library"]="Shared library for SigmaOS desktop environment components"
  ["SigmaOS-desktop-portal"]="XDG desktop portal implementation for SigmaOS"
  ["SigmaOS-dev-tools"]="Developer tools and utilities for SigmaOS development"
  ["SigmaOS-display-manager"]="Graphical login and display manager for SigmaOS"
  ["SigmaOS-domain-blocker"]="Network domain blocking and filtering tool for SigmaOS"
  ["SigmaOS-file-writer"]="Text file editor and writer application for SigmaOS"
  ["SigmaOS-fish"]="Fish shell configuration and plugins for SigmaOS"
  ["SigmaOS-folder-color-switcher"]="Folder icon color customizer for SigmaOS file manager"
  ["SigmaOS-forum-themes"]="Community forum theme assets for SigmaOS"
  ["SigmaOS-github-action"]="Reusable GitHub Actions workflows for SigmaOS repositories"
  ["SigmaOS-greeter"]="Graphical login greeter for SigmaOS display manager"
  ["SigmaOS-GTK"]="GTK toolkit integration and patches for SigmaOS"
  ["SigmaOS-GTK-plus"]="GTK+ application suite integration for SigmaOS"
  ["SigmaOS-guide-developer"]="Developer contribution guide for SigmaOS"
  ["SigmaOS-guide-docs"]="Documentation portal and guide index for SigmaOS"
  ["SigmaOS-power-manager"]="Power management and battery tool for SigmaOS"
  ["SigmaOS-screen-recorder"]="Desktop screen recording tool for SigmaOS"
  ["SigmaOS-screen-saver"]="Desktop screensaver and lock screen for SigmaOS"
  ["SigmaOS-screen-shot"]="Screenshot capture tool for SigmaOS"
  ["SigmaOS-security"]="Security framework and hardening tools for SigmaOS"
  ["SigmaOS-session-manager"]="Desktop session management for SigmaOS"
  ["SigmaOS-setup"]="Initial system setup wizard for SigmaOS"
  ["SigmaOS-sticky-notes"]="Desktop sticky notes application for SigmaOS"
  ["SigmaOS-system"]="Core system utilities and configuration for SigmaOS"
  ["SigmaOS-systemd"]="Systemd integration and management tools for SigmaOS"
  ["SigmaOS-system-locale"]="System locale and language settings for SigmaOS"
  ["SigmaOS-term"]="Terminal emulator with multiple backend support for SigmaOS"
  ["SigmaOS-terminal-text-effects"]="Terminal text animation effects for SigmaOS"
  ["SigmaOS-theme-color"]="Color theme definitions and palettes for SigmaOS"
  ["SigmaOS-theme-cursor"]="Cursor theme collection for SigmaOS"
  ["SigmaOS-try-windows"]="Windows environment trial launcher for SigmaOS"
  ["SigmaOS-tte-go"]="Terminal text effects engine in Go for SigmaOS"
  ["SigmaOS-update"]="System package update manager for SigmaOS"
  ["SigmaOS-upgrade"]="System version upgrade tool for SigmaOS"
  ["SigmaOS-upgrade-info"]="Upgrade release information provider for SigmaOS"
  ["SigmaOS-vim"]="Vim and editor integration for SigmaOS"
  ["SigmaOS-vpn-pia"]="VPN client integration for SigmaOS"
  ["SigmaOS-wallpaper-engine"]="Dynamic wallpaper and background engine for SigmaOS"
  ["SigmaOS-webapp-manager"]="Web application manager and PWA launcher for SigmaOS"
  ["SigmaOS-webcam-recorder"]="Webcam capture and recording tool for SigmaOS"
  ["SigmaOS-website"]="Project website and documentation site for SigmaOS"
  ["SigmaOS-we-can-fix-everything"]="Community issue tracking and fix coordination for SigmaOS"
  ["SigmaOS-welcome-screen"]="First-run welcome screen and onboarding for SigmaOS"
  ["SigmaOS-wine-installer"]="Windows application compatibility installer for SigmaOS"
  ["SigmaOS-xapp-thumbnailers"]="File thumbnailer plugins for SigmaOS xapp"
  ["SigmaOS-xorg-server"]="Xorg display server integration for SigmaOS"
  ["SigmaOS-xreader"]="Document reader application for SigmaOS"
  ["SigmaOS-zsh"]="Zsh shell configuration and plugins for SigmaOS"
  ["Sigma"]="Core meta-package and system integration for SigmaOS"
)

# Component categories
declare -A CATEGORIES=(
  ["SigmaOS-audio-tuner"]="Desktop/UI"
  ["SigmaOS-automate"]="Utilities"
  ["SigmaOS-backup-tool"]="System Services"
  ["SigmaOS-bluetooth"]="System Services"
  ["SigmaOS-bluetooth-tool"]="Utilities"
  ["SigmaOS-bootloader"]="System Services"
  ["SigmaOS-brand-logo"]="Documentation"
  ["SigmaOS-calculator"]="Desktop/UI"
  ["SigmaOS-chatroom"]="Network"
  ["SigmaOS-chromium"]="Desktop/UI"
  ["SigmaOS-daemon-settings"]="System Services"
  ["SigmaOS-desktop-configuration"]="Desktop/UI"
  ["SigmaOS-desktop-environment"]="Desktop/UI"
  ["SigmaOS-desktop-library"]="Desktop/UI"
  ["SigmaOS-desktop-portal"]="Desktop/UI"
  ["SigmaOS-dev-tools"]="Developer Tools"
  ["SigmaOS-display-manager"]="Desktop/UI"
  ["SigmaOS-domain-blocker"]="Security"
  ["SigmaOS-file-writer"]="Desktop/UI"
  ["SigmaOS-fish"]="Utilities"
  ["SigmaOS-folder-color-switcher"]="Desktop/UI"
  ["SigmaOS-forum-themes"]="Documentation"
  ["SigmaOS-github-action"]="Developer Tools"
  ["SigmaOS-greeter"]="Desktop/UI"
  ["SigmaOS-GTK"]="Desktop/UI"
  ["SigmaOS-GTK-plus"]="Desktop/UI"
  ["SigmaOS-guide-developer"]="Documentation"
  ["SigmaOS-guide-docs"]="Documentation"
  ["SigmaOS-power-manager"]="System Services"
  ["SigmaOS-screen-recorder"]="Desktop/UI"
  ["SigmaOS-screen-saver"]="Desktop/UI"
  ["SigmaOS-screen-shot"]="Utilities"
  ["SigmaOS-security"]="Security"
  ["SigmaOS-session-manager"]="System Services"
  ["SigmaOS-setup"]="System Services"
  ["SigmaOS-sticky-notes"]="Desktop/UI"
  ["SigmaOS-system"]="System Services"
  ["SigmaOS-systemd"]="System Services"
  ["SigmaOS-system-locale"]="System Services"
  ["SigmaOS-term"]="Desktop/UI"
  ["SigmaOS-terminal-text-effects"]="Utilities"
  ["SigmaOS-theme-color"]="Desktop/UI"
  ["SigmaOS-theme-cursor"]="Desktop/UI"
  ["SigmaOS-try-windows"]="Utilities"
  ["SigmaOS-tte-go"]="Utilities"
  ["SigmaOS-update"]="Package Management"
  ["SigmaOS-upgrade"]="Package Management"
  ["SigmaOS-upgrade-info"]="Package Management"
  ["SigmaOS-vim"]="Developer Tools"
  ["SigmaOS-vpn-pia"]="Network"
  ["SigmaOS-wallpaper-engine"]="Desktop/UI"
  ["SigmaOS-webapp-manager"]="Desktop/UI"
  ["SigmaOS-webcam-recorder"]="Desktop/UI"
  ["SigmaOS-website"]="Documentation"
  ["SigmaOS-we-can-fix-everything"]="Utilities"
  ["SigmaOS-welcome-screen"]="Desktop/UI"
  ["SigmaOS-wine-installer"]="Package Management"
  ["SigmaOS-xapp-thumbnailers"]="Desktop/UI"
  ["SigmaOS-xorg-server"]="System Services"
  ["SigmaOS-xreader"]="Desktop/UI"
  ["SigmaOS-zsh"]="Utilities"
  ["Sigma"]="System Services"
)

fix_repo() {
  local name="$1"
  local dir="$BASE/$name"
  
  if [ ! -d "$dir" ]; then
    return 0
  fi
  
  local desc="${DESCRIPTIONS[$name]:-Component for SigmaOS}"
  local cat="${CATEGORIES[$name]:-Utilities}"
  
  # Create README.md
  if [ ! -f "$dir/README.md" ]; then
    cat > "$dir/README.md" << EOF
# $name

## Overview
$desc.

## Features
- Core functionality for $name
- Integration with SigmaOS desktop environment
- Configurable settings and preferences

## Build
\`\`\`bash
cargo build --release
\`\`\`

## License
MIT — See LICENSE file for details.
EOF
  fi
  
  # Create LICENSE
  if [ ! -f "$dir/LICENSE" ]; then
    cat > "$dir/LICENSE" << 'EOF'
MIT License

Copyright (c) 2024 SigmaOS Project

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
EOF
  fi
  
  # Create CI workflow
  if [ ! -f "$dir/.github/workflows/ci.yml" ]; then
    mkdir -p "$dir/.github/workflows"
    cp "$CI_TEMPLATE" "$dir/.github/workflows/ci.yml"
  fi
  
  # Create src/lib.rs
  if [ ! -f "$dir/src/lib.rs" ] && [ ! -f "$dir/src/main.py" ] && [ ! -f "$dir/src/main.sh" ]; then
    mkdir -p "$dir/src"
    cat > "$dir/src/lib.rs" << EOF
//! $name — $desc
//!
//! Part of the SigmaOS multi-repository ecosystem.
//! See https://github.com/AaryanSinghChauhan09/SigmaOS for details.

#![allow(dead_code)]

/// Module entry point for $name
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
EOF
  fi
  
  # Create COMPONENTS-entry.toml
  if [ ! -f "$dir/COMPONENTS-entry.toml" ]; then
    local safe_name=$(echo "$name" | sed 's/-/_/g' | tr '[:upper:]' '[:lower:]')
    cat > "$dir/COMPONENTS-entry.toml" << EOF
name = "$name"
description = "$desc"
category = "$cat"
engine = "unknown"
language = "rust"
license = "MIT"
visibility = "public"
depends_on = []
EOF
  fi
}

# Fix all repos that need completion
for d in "$BASE"/SigmaOS-* "$BASE"/Sigma; do
  if [ -d "$d" ]; then
    name=$(basename "$d")
    fix_repo "$name"
  fi
done

echo "All repos fixed."
