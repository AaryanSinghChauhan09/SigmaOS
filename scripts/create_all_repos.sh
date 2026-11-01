#!/usr/bin/env bash
# =============================================================================
# SigmaOS — Create All Repositories
# Account : AaryanSinghChauhan09
# Generated: 2026-11-01
#
# Usage:
#   chmod +x scripts/create_all_repos.sh
#   ./scripts/create_all_repos.sh
#
# Requirements:
#   - gh CLI installed and authenticated via keyring (gh auth login)
#   - git installed
#   - bash 4.0+ (for associative arrays)
# =============================================================================

set -euo pipefail

# ---------------------------------------------------------------------------
# 0. Auth — use keyring, NOT a GITHUB_TOKEN env var
# ---------------------------------------------------------------------------
unset GITHUB_TOKEN

OWNER="AaryanSinghChauhan09"
HOMEPAGE="https://github.com/${OWNER}"

# Counters for summary
CREATED=0
SKIPPED=0
FAILED=0
FAILED_REPOS=()

# Temp workspace — cleaned up on EXIT
WORKDIR="$(mktemp -d)"
trap 'rm -rf "$WORKDIR"' EXIT

# ---------------------------------------------------------------------------
# 1. REPO_MANIFEST  — repo name → description
# ---------------------------------------------------------------------------
declare -A REPO_MANIFEST=(
  ["SigmaOS-gtk-plus"]="GTK+ application suite: Bluetooth manager and media player"
  ["SigmaOS-nodejs-binary"]="Node.js binary distribution and runtime management"
  ["SigmaOS-JS-interpreter"]="JavaScript interpreter and runtime for SigmaOS desktop scripting"
  ["SigmaOS-LAN-fileshare"]="LAN file sharing with peer discovery and zero-config transfer"
  ["SigmaOS-NVIDIA"]="NVIDIA prime switching and proprietary driver integration"
  ["SigmaOS-actions"]="Custom desktop actions and right-click context integrations"
  ["SigmaOS-apturl"]="Install packages via apt:// URL scheme handler"
  ["SigmaOS-asdcontrol-git"]="ASD hardware control integration"
  ["SigmaOS-audio-tuner"]="System audio tuner and equalizer"
  ["SigmaOS-automate"]="Desktop automation and macro scripting tool"
  ["SigmaOS-bluetooth-tool"]="Bluetooth device pairing and management GUI"
  ["SigmaOS-calculator"]="Desktop calculator application"
  ["SigmaOS-chatroom"]="In-system chat and IRC-style messaging"
  ["SigmaOS-common"]="Shared common utilities and system constants"
  ["SigmaOS-configurator"]="System configurator and dotfiles manager"
  ["SigmaOS-control-center"]="Unified system control center and settings hub"
  ["SigmaOS-daemon-apt"]="APT daemon for background package operations"
  ["SigmaOS-daemon-settings"]="Settings daemon for persistent system preferences"
  ["SigmaOS-desktop-configuration"]="Desktop configuration profiles and defaults"
  ["SigmaOS-desktop-environment"]="Core Cinnamon-inspired desktop environment"
  ["SigmaOS-desktop-library"]="Desktop shared library and GTK integration layer"
  ["SigmaOS-desktop-portal"]="XDG desktop portal for sandboxed app integration"
  ["SigmaOS-dev-tools"]="Developer tools, build helpers, and CI utilities"
  ["SigmaOS-display-manager"]="Graphical display manager and login screen"
  ["SigmaOS-domain-blocker"]="Domain and host-based ad/tracker blocker"
  ["SigmaOS-dots-configuration"]="Dotfiles and user configuration management"
  ["SigmaOS-draw"]="Vector and raster drawing application"
  ["SigmaOS-file-manager-system"]="File manager with Nemo as core plus find integration"
  ["SigmaOS-file-writer"]="Lightweight plain-text and code file writer"
  ["SigmaOS-fish"]="Fish shell integration and configuration"
  ["SigmaOS-folder-color-switcher"]="Change folder colors in the file manager"
  ["SigmaOS-forum-themes"]="Forum and community site theme assets"
  ["SigmaOS-Github-Action"]="Reusable GitHub Actions workflows for SigmaOS CI/CD"
  ["SigmaOS-greeter"]="Graphical greeter and login screen UI"
  ["SigmaOS-GTK"]="GTK theme and widget customizations for SigmaOS"
  ["SigmaOS-guide-docs"]="Master documentation hub linking all guides"
  ["SigmaOS-guide-developer"]="Developer contribution guide and architecture docs"
  ["SigmaOS-guide-installation"]="Installation guide and setup walkthrough"
  ["SigmaOS-guide-translations"]="Translation guide and i18n infrastructure"
  ["SigmaOS-guide-troubleshooting"]="Troubleshooting guide and common fixes"
  ["SigmaOS-guide-user"]="End-user guide and manual"
  ["SigmaOS-herdr-runtime-live-coding-agents"]="Herdr runtime and live-coding AI agent integration"
  ["SigmaOS-indicator-sound"]="Sound indicator and volume control panel applet"
  ["SigmaOS-installer"]="System installer with partitioning and wizard"
  ["SigmaOS-kernel"]="SigmaOS kernel: microkernel core, scheduler, memory, syscall"
  ["SigmaOS-libappindicator"]="AppIndicator library for system tray support"
  ["SigmaOS-live-session"]="Live session bootstrap and persistence layer"
  ["SigmaOS-mac-run"]="macOS application compatibility and run layer"
  ["SigmaOS-menus"]="Application menus and launcher integration"
  ["SigmaOS-mesa3D"]="Mesa 3D graphics library and Vulkan integration"
  ["SigmaOS-mirror"]="Repository mirror sync and distribution"
  ["SigmaOS-mozjs"]="Mozilla JavaScript engine (SpiderMonkey) integration"
  ["SigmaOS-office"]="Office suite: documents, spreadsheets, presentations (Chromium-based)"
  ["SigmaOS-packages"]="Package manager core: resolve, install, sign, verify"
  ["SigmaOS-packages-kit"]="PackageKit integration layer for GUI package operations"
  ["SigmaOS-packages-tools"]="Package installation tools (GDebi, Captain)"
  ["SigmaOS-photo-viewer"]="Photo viewer and image browser"
  ["SigmaOS-plymouth"]="Plymouth boot splash screen and themes"
  ["SigmaOS-quickshell"]="Quick-access shell launcher and command palette"
  ["SigmaOS-radio-stream"]="Internet radio streaming client"
  ["SigmaOS-reel"]="Short video reel and media highlights tool"
  ["SigmaOS-remote-desktop"]="Remote desktop viewer and VNC/RDP client"
  ["SigmaOS-renamer-bulk"]="Bulk file renaming tool"
  ["SigmaOS-renamer-file"]="Single file renaming utility"
  ["SigmaOS-screen-saver"]="Screen saver and lock screen"
  ["SigmaOS-session-manager"]="Desktop session lifecycle management"
  ["SigmaOS-setup"]="Initial system setup and first-run wizard"
  ["SigmaOS-simple-scan"]="Document scanner and OCR tool"
  ["SigmaOS-slide-presenter"]="Presentation creation and slide show tool"
  ["SigmaOS-spreadsheet"]="Spreadsheet application"
  ["SigmaOS-ssh-port-forward-plugin"]="SSH port forwarding desktop plugin"
  ["SigmaOS-state-of-components"]="Component status dashboard and health tracker"
  ["SigmaOS-sticky-notes"]="Sticky notes and on-screen reminders"
  ["SigmaOS-stream"]="IPTV and video stream player"
  ["SigmaOS-system"]="Init, service manager and system orchestration"
  ["SigmaOS-system-admin"]="System administration tools and diagnostics"
  ["SigmaOS-system-locale"]="System locale, language, and region configurator"
  ["SigmaOS-systemd"]="Systemd integration and unit management"
  ["SigmaOS-term"]="Terminal emulator (Foot, Ghostty, Alacritty, Kitty, Ptyxis)"
  ["SigmaOS-terminal-text-effects"]="Terminal text effects and animation framework"
  ["SigmaOS-theme-color"]="Color scheme manager and palette generator"
  ["SigmaOS-theme-cursor"]="Cursor theme collection and switcher"
  ["SigmaOS-theme-desktop-environment"]="Desktop environment themes for Cinnamon"
  ["SigmaOS-theme-gtk3"]="GTK3 theme collection for SigmaOS"
  ["SigmaOS-theme-icon"]="Icon theme pack (Mint-L, Mint-X, Mint-Y)"
  ["SigmaOS-theme-marketplace"]="Theme marketplace and one-click installer"
  ["SigmaOS-theme-mdm"]="MDM display manager theme collection"
  ["SigmaOS-theme-registry"]="Central theme registry and metadata catalog"
  ["SigmaOS-theme-support"]="Libadwaita theme support and GTK4 adaptation"
  ["SigmaOS-theme-sync"]="Cross-component theme synchronization"
  ["SigmaOS-theme-y"]="Mint-Y style theme for SigmaOS"
  ["SigmaOS-theming-native"]="Native theming engine and Aether theme foundation"
  ["SigmaOS-timeshift"]="System snapshot and restore with BTRFS/RSYNC"
  ["SigmaOS-tray-icon"]="System tray icon and notification tray"
  ["SigmaOS-troubleshooting-tool"]="System health check and troubleshooting wizard"
  ["SigmaOS-try"]="Try SigmaOS in a container or VM without installing"
  ["SigmaOS-try-windows"]="Try SigmaOS from Windows without rebooting"
  ["SigmaOS-tte-go"]="Terminal text effects library in Go"
  ["SigmaOS-upgrade"]="Major version upgrade tool"
  ["SigmaOS-upgrade-info"]="Upgrade information and release notes viewer"
  ["SigmaOS-upgrade-legacy"]="Legacy upgrade path support for older versions"
  ["SigmaOS-upload"]="File upload manager and transfer tool"
  ["SigmaOS-video-cut"]="Video trimming and cutting tool"
  ["SigmaOS-vim"]="Vim/Neovim integration with LazyVim config"
  ["SigmaOS-vpn-pia"]="PIA VPN manager and network privacy tool"
  ["SigmaOS-wallpaper-engine"]="Dynamic wallpaper engine with animations"
  ["SigmaOS-we-can-fix-everything"]="Self-repair and auto-fix system utility"
  ["SigmaOS-webapp-manager"]="Web app manager and site-specific browser wrapper"
  ["SigmaOS-webcam-recorder"]="Webcam recording and video capture"
  ["SigmaOS-website"]="Official SigmaOS website and documentation site"
  ["SigmaOS-welcome-screen"]="First-run welcome wizard and onboarding screen"
  ["SigmaOS-wiki-extra"]="Extended wiki content and supplementary documentation"
  ["SigmaOS-window-management-library"]="Window management library with tiling and compositing"
  ["SigmaOS-wine-installer"]="Wine/Proton installer for Windows app compatibility"
  ["SigmaOS-zsh"]="Zsh shell integration and configuration"
)

# ---------------------------------------------------------------------------
# 2. TOPICS_MAP — repo name → comma-separated topic list
#    (GitHub topics must be lowercase, alphanumeric + hyphens)
# ---------------------------------------------------------------------------
declare -A TOPICS_MAP=(
  ["SigmaOS-gtk-plus"]="gtk,bluetooth,media-player,desktop"
  ["SigmaOS-nodejs-binary"]="nodejs,runtime,binary,packaging"
  ["SigmaOS-JS-interpreter"]="javascript,interpreter,cjs,mozjs"
  ["SigmaOS-LAN-fileshare"]="lan,fileshare,zeroconf,warpinator"
  ["SigmaOS-NVIDIA"]="nvidia,gpu,prime,drivers"
  ["SigmaOS-actions"]="actions,desktop,context-menu"
  ["SigmaOS-apturl"]="apt,url-handler,packages"
  ["SigmaOS-asdcontrol-git"]="hardware,control,asd"
  ["SigmaOS-audio-tuner"]="audio,equalizer,pulseaudio,pipewire"
  ["SigmaOS-automate"]="automation,macros,scripting"
  ["SigmaOS-bluetooth-tool"]="bluetooth,pairing,gui"
  ["SigmaOS-calculator"]="calculator,math,desktop-app"
  ["SigmaOS-chatroom"]="chat,irc,messaging"
  ["SigmaOS-common"]="common,utilities,shared"
  ["SigmaOS-configurator"]="configurator,dotfiles,settings"
  ["SigmaOS-control-center"]="control-center,settings,system"
  ["SigmaOS-daemon-apt"]="apt,daemon,packages,background"
  ["SigmaOS-daemon-settings"]="settings,daemon,dbus,preferences"
  ["SigmaOS-desktop-configuration"]="desktop,configuration,profiles"
  ["SigmaOS-desktop-environment"]="desktop,cinnamon,wayland,x11"
  ["SigmaOS-desktop-library"]="desktop,library,gtk,integration"
  ["SigmaOS-desktop-portal"]="xdg,portal,sandbox,flatpak"
  ["SigmaOS-dev-tools"]="dev-tools,build,ci,developer"
  ["SigmaOS-display-manager"]="display-manager,login,sddm,mdm"
  ["SigmaOS-domain-blocker"]="dns,blocker,privacy,hosts"
  ["SigmaOS-dots-configuration"]="dotfiles,configuration,home-manager"
  ["SigmaOS-draw"]="drawing,vector,paint,svg"
  ["SigmaOS-file-manager-system"]="file-manager,nemo,finder,files"
  ["SigmaOS-file-writer"]="editor,text-editor,lightweight"
  ["SigmaOS-fish"]="fish,shell,terminal,configuration"
  ["SigmaOS-folder-color-switcher"]="folder,colors,themes,file-manager"
  ["SigmaOS-forum-themes"]="forum,themes,community,web"
  ["SigmaOS-Github-Action"]="github-actions,ci,cd,automation"
  ["SigmaOS-greeter"]="greeter,login,lightdm,slick"
  ["SigmaOS-GTK"]="gtk,themes,widgets,styling"
  ["SigmaOS-guide-docs"]="docs,guides,hub,documentation"
  ["SigmaOS-guide-developer"]="docs,developer,contribution,architecture"
  ["SigmaOS-guide-installation"]="docs,installation,guide,setup"
  ["SigmaOS-guide-translations"]="i18n,l10n,translations,guide"
  ["SigmaOS-guide-troubleshooting"]="docs,troubleshooting,debug,help"
  ["SigmaOS-guide-user"]="docs,user-guide,manual,help"
  ["SigmaOS-herdr-runtime-live-coding-agents"]="ai,live-coding,agents,runtime"
  ["SigmaOS-indicator-sound"]="sound,indicator,volume,panel"
  ["SigmaOS-installer"]="installer,setup,partitioning,iso"
  ["SigmaOS-kernel"]="kernel,microkernel,rust,systems"
  ["SigmaOS-libappindicator"]="appindicator,systray,library"
  ["SigmaOS-live-session"]="live,iso,session,boot"
  ["SigmaOS-mac-run"]="macos,compat,wine,cross-platform"
  ["SigmaOS-menus"]="menus,launcher,applications,desktop"
  ["SigmaOS-mesa3D"]="mesa,graphics,vulkan,opengl"
  ["SigmaOS-mirror"]="mirror,cdn,distribution,packages"
  ["SigmaOS-mozjs"]="javascript,mozjs,spidermonkey,engine"
  ["SigmaOS-office"]="office,libreoffice,documents,productivity"
  ["SigmaOS-packages"]="packages,apt,pacman,package-manager"
  ["SigmaOS-packages-kit"]="packagekit,apt,packages,gui"
  ["SigmaOS-packages-tools"]="gdebi,packages,installer,tools"
  ["SigmaOS-photo-viewer"]="photos,viewer,images,gallery"
  ["SigmaOS-plymouth"]="plymouth,boot,splash,themes"
  ["SigmaOS-quickshell"]="launcher,quickshell,command-palette"
  ["SigmaOS-radio-stream"]="radio,streaming,internet-radio,audio"
  ["SigmaOS-reel"]="video,reels,media,sharing"
  ["SigmaOS-remote-desktop"]="remote-desktop,vnc,rdp,screen-sharing"
  ["SigmaOS-renamer-bulk"]="rename,bulk,files,utility"
  ["SigmaOS-renamer-file"]="rename,file,utility"
  ["SigmaOS-screen-saver"]="screensaver,lock,xfce,security"
  ["SigmaOS-session-manager"]="session,login,lifecycle,display"
  ["SigmaOS-setup"]="setup,wizard,first-run,onboarding"
  ["SigmaOS-simple-scan"]="scanner,ocr,documents,scanning"
  ["SigmaOS-slide-presenter"]="presentations,slides,office"
  ["SigmaOS-spreadsheet"]="spreadsheet,office,data,tables"
  ["SigmaOS-ssh-port-forward-plugin"]="ssh,port-forward,networking,security"
  ["SigmaOS-state-of-components"]="status,dashboard,components,health"
  ["SigmaOS-sticky-notes"]="sticky-notes,notes,reminders,desktop"
  ["SigmaOS-stream"]="iptv,streaming,video,tv"
  ["SigmaOS-system"]="init,systemd,service-manager,system"
  ["SigmaOS-system-admin"]="sysadmin,tools,administration"
  ["SigmaOS-system-locale"]="locale,i18n,language,region"
  ["SigmaOS-systemd"]="systemd,init,service,units"
  ["SigmaOS-term"]="terminal,alacritty,kitty,ghostty"
  ["SigmaOS-terminal-text-effects"]="terminal,text-effects,animation,cli"
  ["SigmaOS-theme-color"]="themes,colors,palette,mint-y"
  ["SigmaOS-theme-cursor"]="cursor,themes,xcursor"
  ["SigmaOS-theme-desktop-environment"]="themes,cinnamon,desktop,gtk"
  ["SigmaOS-theme-gtk3"]="gtk3,themes,css,styling"
  ["SigmaOS-theme-icon"]="icons,themes,hicolor,icon-theme"
  ["SigmaOS-theme-marketplace"]="themes,marketplace,cinnamon,mint"
  ["SigmaOS-theme-mdm"]="mdm,themes,display-manager,login"
  ["SigmaOS-theme-registry"]="themes,registry,catalog,metadata"
  ["SigmaOS-theme-support"]="gtk4,libadwaita,themes,support"
  ["SigmaOS-theme-sync"]="themes,sync,consistency,styling"
  ["SigmaOS-theme-y"]="mint-y,themes,gtk,styling"
  ["SigmaOS-theming-native"]="theming,native,aether,engine"
  ["SigmaOS-timeshift"]="timeshift,backup,snapshot,btrfs"
  ["SigmaOS-tray-icon"]="tray,systray,notifications,panel"
  ["SigmaOS-troubleshooting-tool"]="troubleshooting,diagnostics,health,report"
  ["SigmaOS-try"]="try,demo,container,live"
  ["SigmaOS-try-windows"]="windows,wsl,try,crossplatform"
  ["SigmaOS-tte-go"]="go,terminal,effects,cli"
  ["SigmaOS-upgrade"]="upgrade,migration,major-version"
  ["SigmaOS-upgrade-info"]="upgrade,info,release-notes"
  ["SigmaOS-upgrade-legacy"]="upgrade,legacy,migration,compatibility"
  ["SigmaOS-upload"]="upload,transfer,cloud,files"
  ["SigmaOS-video-cut"]="video,cut,trim,editor"
  ["SigmaOS-vim"]="vim,neovim,lazyvim,editor"
  ["SigmaOS-vpn-pia"]="vpn,pia,privacy,network"
  ["SigmaOS-wallpaper-engine"]="wallpaper,dynamic,desktop,animations"
  ["SigmaOS-we-can-fix-everything"]="repair,auto-fix,recovery,maintenance"
  ["SigmaOS-webapp-manager"]="webapp,browser,pwa,sites"
  ["SigmaOS-webcam-recorder"]="webcam,recording,video,capture"
  ["SigmaOS-website"]="website,docs,landing-page,marketing"
  ["SigmaOS-welcome-screen"]="welcome,onboarding,first-run,wizard"
  ["SigmaOS-wiki-extra"]="wiki,docs,extra,supplementary"
  ["SigmaOS-window-management-library"]="wm,tiling,muffin,hyprland"
  ["SigmaOS-wine-installer"]="wine,proton,windows-compat,gaming"
  ["SigmaOS-zsh"]="zsh,shell,oh-my-zsh,terminal"
)

# ---------------------------------------------------------------------------
# 3. Ordered list so we process in a deterministic sequence
# ---------------------------------------------------------------------------
REPO_ORDER=(
  "SigmaOS-gtk-plus"
  "SigmaOS-nodejs-binary"
  "SigmaOS-JS-interpreter"
  "SigmaOS-LAN-fileshare"
  "SigmaOS-NVIDIA"
  "SigmaOS-actions"
  "SigmaOS-apturl"
  "SigmaOS-asdcontrol-git"
  "SigmaOS-audio-tuner"
  "SigmaOS-automate"
  "SigmaOS-bluetooth-tool"
  "SigmaOS-calculator"
  "SigmaOS-chatroom"
  "SigmaOS-common"
  "SigmaOS-configurator"
  "SigmaOS-control-center"
  "SigmaOS-daemon-apt"
  "SigmaOS-daemon-settings"
  "SigmaOS-desktop-configuration"
  "SigmaOS-desktop-environment"
  "SigmaOS-desktop-library"
  "SigmaOS-desktop-portal"
  "SigmaOS-dev-tools"
  "SigmaOS-display-manager"
  "SigmaOS-domain-blocker"
  "SigmaOS-dots-configuration"
  "SigmaOS-draw"
  "SigmaOS-file-manager-system"
  "SigmaOS-file-writer"
  "SigmaOS-fish"
  "SigmaOS-folder-color-switcher"
  "SigmaOS-forum-themes"
  "SigmaOS-Github-Action"
  "SigmaOS-greeter"
  "SigmaOS-GTK"
  "SigmaOS-guide-docs"
  "SigmaOS-guide-developer"
  "SigmaOS-guide-installation"
  "SigmaOS-guide-translations"
  "SigmaOS-guide-troubleshooting"
  "SigmaOS-guide-user"
  "SigmaOS-herdr-runtime-live-coding-agents"
  "SigmaOS-indicator-sound"
  "SigmaOS-installer"
  "SigmaOS-kernel"
  "SigmaOS-libappindicator"
  "SigmaOS-live-session"
  "SigmaOS-mac-run"
  "SigmaOS-menus"
  "SigmaOS-mesa3D"
  "SigmaOS-mirror"
  "SigmaOS-mozjs"
  "SigmaOS-office"
  "SigmaOS-packages"
  "SigmaOS-packages-kit"
  "SigmaOS-packages-tools"
  "SigmaOS-photo-viewer"
  "SigmaOS-plymouth"
  "SigmaOS-quickshell"
  "SigmaOS-radio-stream"
  "SigmaOS-reel"
  "SigmaOS-remote-desktop"
  "SigmaOS-renamer-bulk"
  "SigmaOS-renamer-file"
  "SigmaOS-screen-saver"
  "SigmaOS-session-manager"
  "SigmaOS-setup"
  "SigmaOS-simple-scan"
  "SigmaOS-slide-presenter"
  "SigmaOS-spreadsheet"
  "SigmaOS-ssh-port-forward-plugin"
  "SigmaOS-state-of-components"
  "SigmaOS-sticky-notes"
  "SigmaOS-stream"
  "SigmaOS-system"
  "SigmaOS-system-admin"
  "SigmaOS-system-locale"
  "SigmaOS-systemd"
  "SigmaOS-term"
  "SigmaOS-terminal-text-effects"
  "SigmaOS-theme-color"
  "SigmaOS-theme-cursor"
  "SigmaOS-theme-desktop-environment"
  "SigmaOS-theme-gtk3"
  "SigmaOS-theme-icon"
  "SigmaOS-theme-marketplace"
  "SigmaOS-theme-mdm"
  "SigmaOS-theme-registry"
  "SigmaOS-theme-support"
  "SigmaOS-theme-sync"
  "SigmaOS-theme-y"
  "SigmaOS-theming-native"
  "SigmaOS-timeshift"
  "SigmaOS-tray-icon"
  "SigmaOS-troubleshooting-tool"
  "SigmaOS-try"
  "SigmaOS-try-windows"
  "SigmaOS-tte-go"
  "SigmaOS-upgrade"
  "SigmaOS-upgrade-info"
  "SigmaOS-upgrade-legacy"
  "SigmaOS-upload"
  "SigmaOS-video-cut"
  "SigmaOS-vim"
  "SigmaOS-vpn-pia"
  "SigmaOS-wallpaper-engine"
  "SigmaOS-we-can-fix-everything"
  "SigmaOS-webapp-manager"
  "SigmaOS-webcam-recorder"
  "SigmaOS-website"
  "SigmaOS-welcome-screen"
  "SigmaOS-wiki-extra"
  "SigmaOS-window-management-library"
  "SigmaOS-wine-installer"
  "SigmaOS-zsh"
)

TOTAL=${#REPO_ORDER[@]}

# ---------------------------------------------------------------------------
# 4. Helper: build JSON array of topics from comma-separated string
# ---------------------------------------------------------------------------
topics_to_json() {
  local csv="$1"
  local json="["
  local first=true
  IFS=',' read -ra topics <<< "$csv"
  for t in "${topics[@]}"; do
    t="$(echo "$t" | tr -d '[:space:]')"
    if [[ "$first" == true ]]; then
      json+="\"$t\""
      first=false
    else
      json+=",\"$t\""
    fi
  done
  json+="]"
  echo "$json"
}

# ---------------------------------------------------------------------------
# 5. Helper: check if repo exists on GitHub
# ---------------------------------------------------------------------------
repo_exists() {
  local repo="$1"
  gh api "repos/${OWNER}/${repo}" --silent 2>/dev/null
}

# ---------------------------------------------------------------------------
# 6. Helper: create initial main-branch commit
#    Files: LICENSE, .gitignore, src/.gitkeep  — NO .md / .txt on main
# ---------------------------------------------------------------------------
bootstrap_main() {
  local repo="$1"
  local repodir="${WORKDIR}/${repo}"

  git init -b main "$repodir" -q

  # .gitignore — generic
  cat > "${repodir}/.gitignore" <<'GITIGNORE'
# Compiled output
*.o
*.so
*.a
*.out
*.pyc
__pycache__/
node_modules/
dist/
build/
target/

# Editors
.vscode/
.idea/
*.swp
*~

# OS
.DS_Store
Thumbs.db
GITIGNORE

  # LICENSE — MIT
  cat > "${repodir}/LICENSE" <<LICENSE
MIT License

Copyright (c) $(date +%Y) AaryanSinghChauhan09

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
LICENSE

  # src/.gitkeep — placeholder source directory
  mkdir -p "${repodir}/src"
  touch "${repodir}/src/.gitkeep"

  git -C "$repodir" config user.email "sigmaos-bot@users.noreply.github.com"
  git -C "$repodir" config user.name  "SigmaOS Bot"

  git -C "$repodir" add .gitignore LICENSE src/.gitkeep -q
  git -C "$repodir" commit -m "chore: initial skeleton (main branch)" -q

  git -C "$repodir" remote add origin "https://github.com/${OWNER}/${repo}.git"
  git -C "$repodir" push -u origin main -q
}

# ---------------------------------------------------------------------------
# 7. Helper: create docs branch with README.md
# ---------------------------------------------------------------------------
bootstrap_docs() {
  local repo="$1"
  local desc="$2"
  local repodir="${WORKDIR}/${repo}"

  git -C "$repodir" checkout -b docs -q

  cat > "${repodir}/README.md" <<README
# ${repo}

> ${desc}

## Overview

This repository is part of the **SigmaOS** project.

## Getting Started

Refer to the [SigmaOS Developer Guide](https://github.com/${OWNER}/SigmaOS-guide-developer)
for contribution instructions, build requirements, and architecture docs.

## License

[MIT](LICENSE) © $(date +%Y) AaryanSinghChauhan09
README

  git -C "$repodir" add README.md -q
  git -C "$repodir" commit -m "docs: add README on docs branch" -q
  git -C "$repodir" push -u origin docs -q

  # Switch back to main — nothing else to do
  git -C "$repodir" checkout main -q
}

# ---------------------------------------------------------------------------
# 8. Helper: set topics via GitHub API
# ---------------------------------------------------------------------------
set_topics() {
  local repo="$1"
  local topics_json="$2"
  gh api \
    --method PUT \
    "repos/${OWNER}/${repo}/topics" \
    --field "names=${topics_json}" \
    --silent 2>/dev/null || true
}

# ---------------------------------------------------------------------------
# 9. Helper: ensure default branch is main
# ---------------------------------------------------------------------------
set_default_branch_main() {
  local repo="$1"
  gh api \
    --method PATCH \
    "repos/${OWNER}/${repo}" \
    --field default_branch=main \
    --silent 2>/dev/null || true
}

# ---------------------------------------------------------------------------
# 10. MAIN LOOP
# ---------------------------------------------------------------------------
echo "================================================================="
echo "  SigmaOS — Repository Creation Script"
echo "  Owner  : ${OWNER}"
echo "  Total  : ${TOTAL} repos to process"
echo "  Started: $(date '+%Y-%m-%d %H:%M:%S')"
echo "================================================================="
echo ""

IDX=0
for REPO in "${REPO_ORDER[@]}"; do
  IDX=$(( IDX + 1 ))
  DESC="${REPO_MANIFEST[$REPO]}"
  TOPICS_CSV="${TOPICS_MAP[$REPO]}"
  TOPICS_JSON="$(topics_to_json "$TOPICS_CSV")"

  printf "[%3d/%d] %-55s  " "$IDX" "$TOTAL" "$REPO"

  # --- Check existence ---
  if repo_exists "$REPO"; then
    echo "SKIP  (already exists)"
    SKIPPED=$(( SKIPPED + 1 ))
    continue
  fi

  # --- Create the repo ---
  if ! gh repo create "${OWNER}/${REPO}" \
        --public \
        --description "${DESC}" \
        --homepage "${HOMEPAGE}" \
        2>/dev/null; then
    echo "FAIL  (gh repo create)"
    FAILED=$(( FAILED + 1 ))
    FAILED_REPOS+=("$REPO")
    continue
  fi

  # Brief pause to let GitHub register the repo
  sleep 1

  # --- Bootstrap main branch ---
  if ! bootstrap_main "$REPO"; then
    echo "FAIL  (bootstrap main)"
    FAILED=$(( FAILED + 1 ))
    FAILED_REPOS+=("$REPO")
    continue
  fi

  # --- Bootstrap docs branch ---
  if ! bootstrap_docs "$REPO" "$DESC"; then
    echo "FAIL  (bootstrap docs)"
    FAILED=$(( FAILED + 1 ))
    FAILED_REPOS+=("$REPO")
    continue
  fi

  # --- Set topics ---
  set_topics "$REPO" "$TOPICS_JSON"

  # --- Ensure main is the default branch ---
  set_default_branch_main "$REPO"

  echo "OK    (created + bootstrapped)"
  CREATED=$(( CREATED + 1 ))

  # Clean up local clone to keep temp space lean
  rm -rf "${WORKDIR:?}/${REPO}"
done

# ---------------------------------------------------------------------------
# 11. SUMMARY
# ---------------------------------------------------------------------------
echo ""
echo "================================================================="
echo "  SigmaOS — Repository Creation Summary"
echo "  Finished: $(date '+%Y-%m-%d %H:%M:%S')"
echo "-----------------------------------------------------------------"
echo "  Total processed : ${TOTAL}"
echo "  Created         : ${CREATED}"
echo "  Skipped (exist) : ${SKIPPED}"
echo "  Failed          : ${FAILED}"
echo "================================================================="

if [[ ${#FAILED_REPOS[@]} -gt 0 ]]; then
  echo ""
  echo "  Failed repositories:"
  for r in "${FAILED_REPOS[@]}"; do
    echo "    - $r"
  done
  echo ""
  echo "  Re-run this script to retry failed repos (idempotent)."
fi

echo ""
echo "Done."
