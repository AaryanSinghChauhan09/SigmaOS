# SigmaOS Sub-Repositories & Component Ecosystem

SigmaOS uses a **distributed multi-repository** architecture modelled on **Linux Mint** and **Omarchy Linux**. Every subsystem and userland tool has a specialised repository linked from the central `SigmaOS` umbrella repo. This catalog is generated from the single source of truth in the main repo's `COMPONENTS.toml`, so the repository and this wiki stay **uniform**.

**Total component repositories: 150**


## Complete Component Catalog

| Repository | Description | Upstream Inspirations |
| :--- | :--- | :--- |
| [`SigmaOS-kernel`](https://github.com/AaryanSinghChauhan09/SigmaOS-kernel) | SigmaOS microkernel core (scheduler, VFS, syscall gate, memory) | https://github.com/omacom/linux-aurora |
| [`SigmaOS-filesystem`](https://github.com/AaryanSinghChauhan09/SigmaOS-filesystem) | Virtual filesystem layer, journaling & zero-copy paths | native |
| [`SigmaOS-network`](https://github.com/AaryanSinghChauhan09/SigmaOS-network) | Networking stack: TCP/IP, TLS, mDNS discovery | native |
| [`SigmaOS-security`](https://github.com/AaryanSinghChauhan09/SigmaOS-security) | Privilege separation, LSM hooks, secure defaults | https://github.com/omacom/omarchy/security |
| [`SigmaOS-system`](https://github.com/AaryanSinghChauhan09/SigmaOS-system) | Init, service manager & system orchestration | native |
| [`SigmaOS-desktop`](https://github.com/AaryanSinghChauhan09/SigmaOS-desktop) | Wayland compositor shell & desktop primitives | native |
| [`SigmaOS-packages`](https://github.com/AaryanSinghChauhan09/SigmaOS-packages) | Package manager core (resolve, install, verify) | https://github.com/linuxmint/mintsources; https://github.com/linuxmint/repolib |
| [`SigmaOS-bootloader`](https://github.com/AaryanSinghChauhan09/SigmaOS-bootloader) | Bootloader configuration & chainload manager | https://github.com/omacom/m1n1 |
| [`SigmaOS-plymouth`](https://github.com/AaryanSinghChauhan09/SigmaOS-plymouth) | Boot splash / plymouth theme manager | https://github.com/linuxmint/plymouth |
| [`SigmaOS-systemd`](https://github.com/AaryanSinghChauhan09/SigmaOS-systemd) | Service unit manager (start/enable/mask/status) | https://github.com/linuxmint/systemd-betsy |
| [`SigmaOS-session-manager`](https://github.com/AaryanSinghChauhan09/SigmaOS-session-manager) | XDG session manager (start/stop/save sessions) | https://github.com/linuxmint/cinnamon-session |
| [`SigmaOS-live-session`](https://github.com/AaryanSinghChauhan09/SigmaOS-live-session) | Live-session bootstrap for the try-before-install ISO | https://github.com/linuxmint/mint-live-session |
| [`SigmaOS-display-manager`](https://github.com/AaryanSinghChauhan09/SigmaOS-display-manager) | Display manager & login greeter launcher | https://github.com/omacom/sddm; https://github.com/linuxmint/mdm |
| [`SigmaOS-greeter`](https://github.com/AaryanSinghChauhan09/SigmaOS-greeter) | Slick login greeter with user list & a11y | https://github.com/linuxmint/slick-greeter |
| [`SigmaOS-power-manager`](https://github.com/AaryanSinghChauhan09/SigmaOS-power-manager) | Battery / power-profile & idle management | https://github.com/linuxmint/xfce4-power-manager |
| [`SigmaOS-installer`](https://github.com/AaryanSinghChauhan09/SigmaOS-installer) | Graphical live installer (partition, copy, bootloader) | https://github.com/linuxmint/live-installer; https://github.com/omacom/omarchy-mac-installer |
| [`SigmaOS-ISO`](https://github.com/AaryanSinghChauhan09/SigmaOS-ISO) | Build the SigmaOS live ISO from a manifest | https://github.com/omacom/omarchy-iso |
| [`SigmaOS-software-manager`](https://github.com/AaryanSinghChauhan09/SigmaOS-software-manager) | GUI/CLI software store (search, install, remove) | https://github.com/linuxmint/mintinstall |
| [`SigmaOS-update`](https://github.com/AaryanSinghChauhan09/SigmaOS-update) | Update notifier & apply (apt backend) | https://github.com/linuxmint/mintupdate |
| [`SigmaOS-upgrade`](https://github.com/AaryanSinghChauhan09/SigmaOS-upgrade) | Major-release upgrade assistant | https://github.com/linuxmint/mintupgrade |
| [`SigmaOS-upgrade-info`](https://github.com/AaryanSinghChauhan09/SigmaOS-upgrade-info) | Release upgrade compatibility metadata | https://github.com/linuxmint/mint-upgrade-info |
| [`SigmaOS-upgrade-legacy`](https://github.com/AaryanSinghChauhan09/SigmaOS-upgrade-legacy) | Legacy upgrade path for older releases | https://github.com/linuxmint/mintupgrade-legacy |
| [`SigmaOS-timeshift`](https://github.com/AaryanSinghChauhan09/SigmaOS-timeshift) | Btrfs/rsync system snapshots & restore | https://github.com/linuxmint/timeshift |
| [`SigmaOS-backup-tool`](https://github.com/AaryanSinghChauhan09/SigmaOS-backup-tool) | Backup / restore with manifest & verification | https://github.com/linuxmint/mintbackup |
| [`SigmaOS-drivers`](https://github.com/AaryanSinghChauhan09/SigmaOS-drivers) | Detect & install proprietary drivers | https://github.com/linuxmint/mintdrivers; https://github.com/linuxmint/ubuntu-drivers-common |
| [`SigmaOS-NVIDIA`](https://github.com/AaryanSinghChauhan09/SigmaOS-NVIDIA) | NVIDIA prime / offload GPU management | https://github.com/linuxmint/nvidia-prime |
| [`SigmaOS-packages-kit`](https://github.com/AaryanSinghChauhan09/SigmaOS-packages-kit) | apt/dpkg transaction kit (aptkit-style) | https://github.com/linuxmint/aptkit; https://github.com/linuxmint/packagekit |
| [`SigmaOS-packages-tools`](https://github.com/AaryanSinghChauhan09/SigmaOS-packages-tools) | Package install/remove CLI (captain/gdebi-style) | https://github.com/linuxmint/captain; https://github.com/linuxmint/gdebi |
| [`SigmaOS-apturl`](https://github.com/AaryanSinghChauhan09/SigmaOS-apturl) | apt:// URL handler | https://github.com/linuxmint/apturl |
| [`SigmaOS-daemon-apt`](https://github.com/AaryanSinghChauhan09/SigmaOS-daemon-apt) | Privileged apt D-Bus daemon | https://github.com/linuxmint/aptdaemon |
| [`SigmaOS-mirror`](https://github.com/AaryanSinghChauhan09/SigmaOS-mirror) | Mirror speed tester & automatic selector | https://github.com/omacom/omarchy-mirror |
| [`SigmaOS-common`](https://github.com/AaryanSinghChauhan09/SigmaOS-common) | Shared helpers + policy kit used by every applet | https://github.com/linuxmint/mintcommon |
| [`SigmaOS-daemon-settings`](https://github.com/AaryanSinghChauhan09/SigmaOS-daemon-settings) | Settings daemon (fonts, theme, hotkeys) | https://github.com/linuxmint/cinnamon-settings-daemon |
| [`SigmaOS-control-center`](https://github.com/AaryanSinghChauhan09/SigmaOS-control-center) | Central control centre (modules discover settings) | https://github.com/linuxmint/cinnamon-control-center |
| [`SigmaOS-configurator`](https://github.com/AaryanSinghChauhan09/SigmaOS-configurator) | Omarchy-style declarative system configurator | https://github.com/omacom/omarchy-configurator |
| [`SigmaOS-desktop-configuration`](https://github.com/AaryanSinghChauhan09/SigmaOS-desktop-configuration) | Desktop flavour configuration (browser, shell, WM) | https://github.com/linuxmint/mintdesktop |
| [`SigmaOS-system-locale`](https://github.com/AaryanSinghChauhan09/SigmaOS-system-locale) | Locale & language selector | https://github.com/linuxmint/mintlocale |
| [`SigmaOS-system-admin`](https://github.com/AaryanSinghChauhan09/SigmaOS-system-admin) | System administration toolkit (services, disks, logs) | https://github.com/linuxmint/mintsysadm |
| [`SigmaOS-troubleshooting-tool`](https://github.com/AaryanSinghChauhan09/SigmaOS-troubleshooting-tool) | System report & troubleshooting generator | https://github.com/linuxmint/mintreport |
| [`SigmaOS-we-can-fix-everything`](https://github.com/AaryanSinghChauhan09/SigmaOS-we-can-fix-everything) | Guided fix-it support workflows | https://github.com/omacom/wecanfixeverything |
| [`SigmaOS-automate`](https://github.com/AaryanSinghChauhan09/SigmaOS-automate) | Task automation engine (hooks, rules, schedules) | https://github.com/linuxmint/automate |
| [`SigmaOS-state-of-components`](https://github.com/AaryanSinghChauhan09/SigmaOS-state-of-components) | Dashboard of component health & parity gaps | https://github.com/omacom/state-of-omarchy |
| [`SigmaOS-desktop-environment`](https://github.com/AaryanSinghChauhan09/SigmaOS-desktop-environment) | SigmaOS desktop environment (panels, menus, WM glue) | https://github.com/linuxmint/cinnamon |
| [`SigmaOS-desktop-library`](https://github.com/AaryanSinghChauhan09/SigmaOS-desktop-library) | Shared desktop library (settings, icons, xapp glue) | https://github.com/linuxmint/cinnamon-desktop |
| [`SigmaOS-window-management-library`](https://github.com/AaryanSinghChauhan09/SigmaOS-window-management-library) | Compositor / window-manager library | https://github.com/linuxmint/muffin; https://github.com/omacom/omarchy-hyprland-titlebars |
| [`SigmaOS-panel`](https://github.com/AaryanSinghChauhan09/SigmaOS-panel) | Taskbar / panel applet host | https://github.com/linuxmint/xfce-panel |
| [`SigmaOS-menus`](https://github.com/AaryanSinghChauhan09/SigmaOS-menus) | Application menu & menu database | https://github.com/linuxmint/cinnamon-menus; https://github.com/linuxmint/mintmenu |
| [`SigmaOS-indicator-sound`](https://github.com/AaryanSinghChauhan09/SigmaOS-indicator-sound) | Sound indicator (volume, output switch) | https://github.com/linuxmint/indicator-sound |
| [`SigmaOS-tray-icon`](https://github.com/AaryanSinghChauhan09/SigmaOS-tray-icon) | System tray icon host (XAppStatusIcon) | https://github.com/linuxmint/rhythmbox-tray-icon |
| [`SigmaOS-notification`](https://github.com/AaryanSinghChauhan09/SigmaOS-notification) | Notification daemon (d-Bus org.freedesktop.Notifications) | https://github.com/omacom/omapager |
| [`SigmaOS-notification-center-plugin`](https://github.com/AaryanSinghChauhan09/SigmaOS-notification-center-plugin) | Notification centre plugin (history, do-not-disturb) | https://github.com/omacom/omarchy-notification-center-plugin |
| [`SigmaOS-screen-saver`](https://github.com/AaryanSinghChauhan09/SigmaOS-screen-saver) | Lock screen & idle screensaver | https://github.com/linuxmint/cinnamon-screensaver |
| [`SigmaOS-session-manager2`](https://github.com/AaryanSinghChauhan09/SigmaOS-session-manager2) | Session save/restore | https://github.com/linuxmint/cinnamon-session |
| [`SigmaOS-calculator`](https://github.com/AaryanSinghChauhan09/SigmaOS-calculator) | Scientific calculator engine | https://github.com/omacom/omacalc |
| [`SigmaOS-chatroom`](https://github.com/AaryanSinghChauhan09/SigmaOS-chatroom) | LAN chat server & client | https://github.com/linuxmint/jargonaut |
| [`SigmaOS-draw`](https://github.com/AaryanSinghChauhan09/SigmaOS-draw) | Simple raster drawing app | https://github.com/linuxmint/drawing |
| [`SigmaOS-spreadsheet`](https://github.com/AaryanSinghChauhan09/SigmaOS-spreadsheet) | CSV spreadsheet engine with formulas | native |
| [`SigmaOS-slide-presenter`](https://github.com/AaryanSinghChauhan09/SigmaOS-slide-presenter) | Markdown slide presenter | https://github.com/omacom/hype |
| [`SigmaOS-sticky-notes`](https://github.com/AaryanSinghChauhan09/SigmaOS-sticky-notes) | Sticky notes applet | https://github.com/linuxmint/sticky |
| [`SigmaOS-file-manager-system`](https://github.com/AaryanSinghChauhan09/SigmaOS-file-manager-system) | File manager (nemo-style, tabs, extensions) | https://github.com/linuxmint/nemo; https://github.com/omacom/omafind |
| [`SigmaOS-file-writer`](https://github.com/AaryanSinghChauhan09/SigmaOS-file-writer) | Minimal distraction-free text writer | https://github.com/omacom/omawrite |
| [`SigmaOS-folder-color-switcher`](https://github.com/AaryanSinghChauhan09/SigmaOS-folder-color-switcher) | Colour-code folders in the file manager | https://github.com/linuxmint/folder-color-switcher |
| [`SigmaOS-renamer-bulk`](https://github.com/AaryanSinghChauhan09/SigmaOS-renamer-bulk) | Bulk regex renamer with live preview | https://github.com/linuxmint/bulky |
| [`SigmaOS-renamer-file`](https://github.com/AaryanSinghChauhan09/SigmaOS-renamer-file) | Single & batch file renamer (pyRenamer-style) | https://github.com/linuxmint/pyRenamer |
| [`SigmaOS-photo-viewer`](https://github.com/AaryanSinghChauhan09/SigmaOS-photo-viewer) | Image viewer with fullscreen & EXIF | https://github.com/linuxmint/xviewer; https://github.com/linuxmint/pix |
| [`SigmaOS-player`](https://github.com/AaryanSinghChauhan09/SigmaOS-player) | Video player (mpv-backed) | https://github.com/linuxmint/xplayer; https://github.com/debpalash/Opal |
| [`SigmaOS-player-library`](https://github.com/AaryanSinghChauhan09/SigmaOS-player-library) | Media playlist / metadata library | https://github.com/linuxmint/xplayer-plparser |
| [`SigmaOS-reel`](https://github.com/AaryanSinghChauhan09/SigmaOS-reel) | Vertical short-video reel player | https://github.com/omacom/omareel |
| [`SigmaOS-stream`](https://github.com/AaryanSinghChauhan09/SigmaOS-stream) | IPTV / m3u stream player | https://github.com/linuxmint/hypnotix; https://github.com/sam-blakeman/OmarchyIPTV |
| [`SigmaOS-radio-stream`](https://github.com/AaryanSinghChauhan09/SigmaOS-radio-stream) | Internet radio streamer | https://github.com/omacom/radio.omarchy.org |
| [`SigmaOS-screen-shot`](https://github.com/AaryanSinghChauhan09/SigmaOS-screen-shot) | Screenshot tool (area, window, full) | https://github.com/omacom/omasnap |
| [`SigmaOS-screen-recorder`](https://github.com/AaryanSinghChauhan09/SigmaOS-screen-recorder) | Screencast recorder | native |
| [`SigmaOS-webcam-recorder`](https://github.com/AaryanSinghChauhan09/SigmaOS-webcam-recorder) | Webcam photo / video recorder | https://github.com/omacom/monologue |
| [`SigmaOS-simple-scan`](https://github.com/AaryanSinghChauhan09/SigmaOS-simple-scan) | Scanner (SANE) frontend | https://github.com/linuxmint/simple-scan |
| [`SigmaOS-video-cut`](https://github.com/AaryanSinghChauhan09/SigmaOS-video-cut) | Lossless-ish video trimmer | https://github.com/omacom/omacut |
| [`SigmaOS-xreader`](https://github.com/AaryanSinghChauhan09/SigmaOS-xreader) | Document / eBook & comic reader | https://github.com/linuxmint/xreader; https://github.com/dlpwaters/omarchy-ebook-reader |
| [`SigmaOS-office`](https://github.com/AaryanSinghChauhan09/SigmaOS-office) | LibreOffice icon/style integration | https://github.com/linuxmint/libreoffice-style-mint |
| [`SigmaOS-webapp-manager`](https://github.com/AaryanSinghChauhan09/SigmaOS-webapp-manager) | Turn websites into desktop webapps | https://github.com/linuxmint/webapp-manager |
| [`SigmaOS-remote-desktop`](https://github.com/AaryanSinghChauhan09/SigmaOS-remote-desktop) | VNC/RDP remote desktop sharing | https://github.com/omacom/gliff |
| [`SigmaOS-upload`](https://github.com/AaryanSinghChauhan09/SigmaOS-upload) | File upload / share service client | https://github.com/linuxmint/mintupload |
| [`SigmaOS-stick`](https://github.com/AaryanSinghChauhan09/SigmaOS-stick) | Write ISOs to USB with hash verification | https://github.com/linuxmint/mintstick; https://github.com/balena-io/etcher |
| [`SigmaOS-wine-installer`](https://github.com/AaryanSinghChauhan09/SigmaOS-wine-installer) | Wine prefix setup & Windows app installer | https://github.com/linuxmint/wine-installer |
| [`SigmaOS-vpn-pia`](https://github.com/AaryanSinghChauhan09/SigmaOS-vpn-pia) | VPN manager (PIA-style profiles) | https://github.com/linuxmint/pia-manager |
| [`SigmaOS-draw2`](https://github.com/AaryanSinghChauhan09/SigmaOS-draw2) | Vector drawing app | https://github.com/linuxmint/drawing |
| [`SigmaOS-bluetooth`](https://github.com/AaryanSinghChauhan09/SigmaOS-bluetooth) | Bluetooth adapter & device manager | https://github.com/linuxmint/cinnamon-bluetooth |
| [`SigmaOS-bluetooth-tool`](https://github.com/AaryanSinghChauhan09/SigmaOS-bluetooth-tool) | Bluetooth pairing CLI/GUI (blueberry/blueferry) | https://github.com/linuxmint/blueberry; https://github.com/erikkb/blueferry |
| [`SigmaOS-network-discovery`](https://github.com/AaryanSinghChauhan09/SigmaOS-network-discovery) | Discover hosts & services on the LAN | https://github.com/linuxmint/network-discovery |
| [`SigmaOS-LAN-fileshare`](https://github.com/AaryanSinghChauhan09/SigmaOS-LAN-fileshare) | Warpinator-style LAN send/receive | https://github.com/linuxmint/warpinator; https://github.com/t4t5/owly |
| [`SigmaOS-ssh-port-forward-plugin`](https://github.com/AaryanSinghChauhan09/SigmaOS-ssh-port-forward-plugin) | SSH port-forward manager plugin | https://github.com/omacom/omarchy-port-forward-plugin |
| [`SigmaOS-domain-blocker`](https://github.com/AaryanSinghChauhan09/SigmaOS-domain-blocker) | Block domains system-wide via /etc/hosts | https://github.com/linuxmint/mintnanny |
| [`SigmaOS-theme-color`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-color) | Palette / accent colour engine | https://github.com/linuxmint/mint-y-colors |
| [`SigmaOS-theme-cursor`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-cursor) | Cursor theme pack & switcher | https://github.com/linuxmint/mint-cursor-themes; https://github.com/linuxmint/Bibata_Cursor |
| [`SigmaOS-theme-icon`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-icon) | Icon theme pack (mint-x/y/l) | https://github.com/linuxmint/mint-y-icons; https://github.com/linuxmint/mint-x-icons |
| [`SigmaOS-theme-y`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-y) | Mint-Y GTK theme | https://github.com/linuxmint/mint-y-theme |
| [`SigmaOS-theme-gtk3`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-gtk3) | GTK3 theme engine | https://github.com/linuxmint/mint-themes-gtk3 |
| [`SigmaOS-theme-desktop-environment`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-desktop-environment) | Desktop-shell theme (Cinnamon-style) | https://github.com/linuxmint/cinnamon-themes |
| [`SigmaOS-theme-mdm`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-mdm) | Greeter (MDM) theme pack | https://github.com/linuxmint/mdm-themes |
| [`SigmaOS-theme-marketplace`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-marketplace) | Community theme marketplace | https://github.com/linuxmint/cinnamon-spices-themes; https://github.com/omacom/omarchy-theme-marketplace |
| [`SigmaOS-theme-registry`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-registry) | Theme registry & index | https://github.com/linuxmint/mint-themes; https://github.com/omacom/omarchy-theme-registry |
| [`SigmaOS-theme-sync`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-sync) | Sync themes across machines | https://github.com/linuxmint/mint-themes; https://github.com/omacom/omarchy-theme-sync |
| [`SigmaOS-theme-support`](https://github.com/AaryanSinghChauhan09/SigmaOS-theme-support) | libadwaita/GTK theme bridge | https://github.com/linuxmint/libadwaita |
| [`SigmaOS-theming-native`](https://github.com/AaryanSinghChauhan09/SigmaOS-theming-native) | Native (CSD-less) theming engine | https://github.com/omacom/aether |
| [`SigmaOS-wallpaper-engine`](https://github.com/AaryanSinghChauhan09/SigmaOS-wallpaper-engine) | Wallpaper set / slideshow / randomiser | https://github.com/omacom/owe |
| [`SigmaOS-brand-logo`](https://github.com/AaryanSinghChauhan09/SigmaOS-brand-logo) | SigmaOS brand logo & identity assets | https://github.com/linuxmint/brand-logo |
| [`SigmaOS-dots-configuration`](https://github.com/AaryanSinghChauhan09/SigmaOS-dots-configuration) | Dotfiles manager (omadots-style) | https://github.com/omacom/omadots |
| [`SigmaOS-forum-themes`](https://github.com/AaryanSinghChauhan09/SigmaOS-forum-themes) | Community forum theme pack | https://github.com/linuxmint/forums-theme |
| [`SigmaOS-plugin-marketplace`](https://github.com/AaryanSinghChauhan09/SigmaOS-plugin-marketplace) | Applet/desklet/extension marketplace | https://github.com/linuxmint/cinnamon-spices-applets; https://github.com/omacom/omarchy-plugin-marketplace |
| [`SigmaOS-plugin-registry`](https://github.com/AaryanSinghChauhan09/SigmaOS-plugin-registry) | Plugin registry & manifest index | https://github.com/omacom/omarchy-plugin-registry |
| [`SigmaOS-actions`](https://github.com/AaryanSinghChauhan09/SigmaOS-actions) | Reusable GitHub Actions for SigmaOS repos | https://github.com/linuxmint/cinnamon-spices-actions; https://github.com/linuxmint/github-actions |
| [`SigmaOS-Github-Action`](https://github.com/AaryanSinghChauhan09/SigmaOS-Github-Action) | CI GitHub Actions collection | https://github.com/linuxmint/github-actions |
| [`SigmaOS-dev-tools`](https://github.com/AaryanSinghChauhan09/SigmaOS-dev-tools) | Developer toolchain helpers | https://github.com/linuxmint/mint-dev-tools |
| [`SigmaOS-nvim`](https://github.com/AaryanSinghChauhan09/SigmaOS-nvim) | Neovim distribution (aether/lumon/pixel) | https://github.com/omacom/aether.nvim; https://github.com/omacom/lumon.nvim |
| [`SigmaOS-vim`](https://github.com/AaryanSinghChauhan09/SigmaOS-vim) | Editor config (xed + lazyvim) | https://github.com/linuxmint/xed; https://github.com/omacom/omarchy-lazyvim |
| [`SigmaOS-zsh`](https://github.com/AaryanSinghChauhan09/SigmaOS-zsh) | Zsh config & plugins | https://github.com/omacom/omarchy-zsh |
| [`SigmaOS-fish`](https://github.com/AaryanSinghChauhan09/SigmaOS-fish) | Fish config & plugins | https://github.com/omacom/omarchy-fish |
| [`SigmaOS-term`](https://github.com/AaryanSinghChauhan09/SigmaOS-term) | Terminal launcher wrapper (foot/ghostty/kitty) | https://github.com/linuxmint/ptyxis-minted; https://github.com/omacom/omaterm |
| [`SigmaOS-terminal-text-effects`](https://github.com/AaryanSinghChauhan09/SigmaOS-terminal-text-effects) | Terminal text effects engine | https://github.com/omacom/ttfx; https://github.com/omacom/tte-go |
| [`SigmaOS-tte-go`](https://github.com/AaryanSinghChauhan09/SigmaOS-tte-go) | Terminal text effects (Go flavour) | https://github.com/omacom/tte-go |
| [`SigmaOS-quickshell`](https://github.com/AaryanSinghChauhan09/SigmaOS-quickshell) | Quick shell command runner | https://github.com/omacom/quickshell |
| [`SigmaOS-herdr-runtime-live-coding-agents`](https://github.com/AaryanSinghChauhan09/SigmaOS-herdr-runtime-live-coding-agents) | Live-coding agent runtime (herdr) | https://github.com/omacom/herdr |
| [`SigmaOS-asdcontrol-git`](https://github.com/AaryanSinghChauhan09/SigmaOS-asdcontrol-git) | ASD control via git-tracked config | https://github.com/omacom/asdcontrol-git |
| [`SigmaOS-audio-tuner`](https://github.com/AaryanSinghChauhan09/SigmaOS-audio-tuner) | Audio device tuner (EQ, profiles) | https://github.com/omacom/omarchy-audio-tuner |
| [`SigmaOS-JS-interpreter`](https://github.com/AaryanSinghChauhan09/SigmaOS-JS-interpreter) | Embedded JS interpreter (cjs/mullion) | https://github.com/linuxmint/cjs; https://github.com/hr1ddh0/omarchy-mullion |
| [`SigmaOS-mozjs`](https://github.com/AaryanSinghChauhan09/SigmaOS-mozjs) | SpiderMonkey (mozjs) build wrapper | https://github.com/linuxmint/mozjs102 |
| [`SigmaOS-Node.js-Binary`](https://github.com/AaryanSinghChauhan09/SigmaOS-Node.js-Binary) | Node.js binary distribution manager | https://github.com/linuxmint/distributions |
| [`SigmaOS-mesa3D`](https://github.com/AaryanSinghChauhan09/SigmaOS-mesa3D) | Mesa 3D build & VA-API/V4L2 request support | https://github.com/omacom/mesa; https://github.com/omacom/libva-v4l2_request |
| [`SigmaOS-xorg-server`](https://github.com/AaryanSinghChauhan09/SigmaOS-xorg-server) | Xorg server LTS build & config | https://github.com/linuxmint/xorg-server-lts-vivid |
| [`SigmaOS-xapp`](https://github.com/AaryanSinghChauhan09/SigmaOS-xapp) | XApp shared library (GTK widgets & helpers) | https://github.com/linuxmint/xapp; https://github.com/xapp-project |
| [`SigmaOS-xapp-python`](https://github.com/AaryanSinghChauhan09/SigmaOS-xapp-python) | Python bindings for XApp | https://github.com/linuxmint/python3-xapp |
| [`SigmaOS-xapp-thumbnailers`](https://github.com/AaryanSinghChauhan09/SigmaOS-xapp-thumbnailers) | Thumbnailers for XApp file managers | https://github.com/linuxmint/xapp-thumbnailers |
| [`SigmaOS-libappindicator`](https://github.com/AaryanSinghChauhan09/SigmaOS-libappindicator) | Sys-tray appindicator library | https://github.com/linuxmint/libappindicator |
| [`SigmaOS-desktop-portal`](https://github.com/AaryanSinghChauhan09/SigmaOS-desktop-portal) | XDG desktop portal backend (screenshot, files) | https://github.com/linuxmint/xdg-desktop-portal-xapp |
| [`SigmaOS-GTK`](https://github.com/AaryanSinghChauhan09/SigmaOS-GTK) | GTK build patches & theming glue | https://github.com/linuxmint/gtk |
| [`SigmaOS-GTK+`](https://github.com/AaryanSinghChauhan09/SigmaOS-GTK+) | Blueman & Celluloid GTK app wrappers | https://github.com/linuxmint/blueman; https://github.com/linuxmint/celluloid |
| [`SigmaOS-setup`](https://github.com/AaryanSinghChauhan09/SigmaOS-setup) | Post-install setup (omamac-style) | https://github.com/omacom/omamac |
| [`SigmaOS-mac-run`](https://github.com/AaryanSinghChauhan09/SigmaOS-mac-run) | Run SigmaOS tooling on macOS host | https://github.com/omacom/omarchy-mac |
| [`SigmaOS-try`](https://github.com/AaryanSinghChauhan09/SigmaOS-try) | Try SigmaOS in a container/VM | https://github.com/omacom/try-omarchy |
| [`SigmaOS-try-windows`](https://github.com/AaryanSinghChauhan09/SigmaOS-try-windows) | Try SigmaOS from Windows (WSL/boot) | https://github.com/linuxmint/mint4win; https://github.com/omacom/try-omarchy-windows |
| [`SigmaOS-install2`](https://github.com/AaryanSinghChauhan09/SigmaOS-install2) | Alternate installer (ubiquity-style) | https://github.com/linuxmint/ubiquity |
| [`SigmaOS-website`](https://github.com/AaryanSinghChauhan09/SigmaOS-website) | SigmaOS website & docs site generator | https://github.com/omacom/omarchy-site; https://github.com/omacom/omakub-site |
| [`SigmaOS-guide-docs`](https://github.com/AaryanSinghChauhan09/SigmaOS-guide-docs) | Aggregate documentation build | https://github.com/linuxmint/mintdoc |
| [`SigmaOS-guide-developer`](https://github.com/AaryanSinghChauhan09/SigmaOS-guide-developer) | Developer guide | https://github.com/linuxmint/doc-developer-guide |
| [`SigmaOS-guide-installation`](https://github.com/AaryanSinghChauhan09/SigmaOS-guide-installation) | Installation guide | https://github.com/linuxmint/doc-installation-guide |
| [`SigmaOS-guide-troubleshooting`](https://github.com/AaryanSinghChauhan09/SigmaOS-guide-troubleshooting) | Troubleshooting guide | https://github.com/linuxmint/doc-troubleshooting-guide |
| [`SigmaOS-guide-user`](https://github.com/AaryanSinghChauhan09/SigmaOS-guide-user) | User guide | https://github.com/linuxmint/doc-user-guide |
| [`SigmaOS-guide-translations`](https://github.com/AaryanSinghChauhan09/SigmaOS-guide-translations) | Translation guide & tooling | https://github.com/linuxmint/cinnamon-translations; https://github.com/linuxmint/mint-translations |
| [`SigmaOS-translations`](https://github.com/AaryanSinghChauhan09/SigmaOS-translations) | Translation catalogue manager | https://github.com/linuxmint/cinnamon-translations |
| [`SigmaOS-wiki-extra`](https://github.com/AaryanSinghChauhan09/SigmaOS-wiki-extra) | Supplementary wiki content | native |
| [`SigmaOS-welcome-screen`](https://github.com/AaryanSinghChauhan09/SigmaOS-welcome-screen) | First-run welcome screen | https://github.com/linuxmint/mintwelcome |
| [`SigmaOS-indicator-sound2`](https://github.com/AaryanSinghChauhan09/SigmaOS-indicator-sound2) | Sound indicator alt | https://github.com/linuxmint/indicator-sound |

## Design Principles Implemented

Every component repository is built to the following principles (see each repo's README for the ones applied to it):

**Abstraction**, **Architecture**, **Arrange-Act-Assert**, **Avoid Fragmentation**, **Blackbox**, **Booch Method**, **Boy Scout Rule**, **Cache Conscious Design**, **Clear Abstraction**, **Code Reusability**, **Maximise Cohesion**, **Collaboration & Shared Responsibility**, **Command-Query Separation**, **Community-Oriented Development**, **Component Isolation**, **Composition Over Inheritance**, **Continuous Improvement & Long Term Sustainability**, **Minimise Coupling**, **Cross Compilation**, **Deadlock Avoidance**, **Defensive Programming**, **Demeter Law**, **Design Pattern**, **Deterministic Testing**, **Deutsch Limit**, **Direct Device Control**, **Discoverability**, **Don't Repeat Yourself**, **Efficient Scheduling**, **Encapsulation**, **Fail-Fast System**, **FIRST**, **Free and Open Source Software**, **Gall's Law**, **GRASP**, **Governance and Do-cracy**, **If It Ain't Broke, Don't Fix It**, **Information Hiding**, **Inheritance**, **Instrumentation**, **Interface Segregation**, **Interrupt Handling Discipline**, **Inversion of Control**, **Keep It Simple Stupid**, **Layered Design**, **Least Astonishment**, **Least Privilege**, **Liskov Substitution**, **Locality of Reference**, **Loose Coupling**, **Low Level Control**, **Low Level Languages**, **Manual Memory Control**, **Memory Discipline**, **Memory Safety**, **Meritocracy and Inclusivity**, **Minimal Kernel Philosophy**, **Modular Kernel Design**, **Modularity**, **Multiple Repository**, **Ninety-Ninety Rule**, **Normalisation**, **Object-Oriented Programming**, **Offensive Programming**, **Open-Closed**, **Open Standard and Interoperability**, **Optimise for Deletion**, **Plug-in Driver Model**, **Polymorphism**, **Portability**, **Power of Ten**, **Practical Licensing Awareness**, **Pristine Sources**, **Process Isolation**, **Race-Free Design**, **Respect & Ethical Participation**, **Refactoring**, **Rule of Three**, **Safe Concurrency**, **Secure Defaults**, **Separation of Concerns**, **Separation of Mechanism & Policy**, **Single Responsibility, Transparency & Open Exchange**, **Spec-Driven Development**, **SOLID (Open/Closed, Liskov, Interface Segregation, Dependency Inversion)**, **Software Support & Updating Programs (like Linux distros)**, **Stable APIs**, **Single Repository (main umbrella)**, **Talk Is Cheap, Show Me The Code**, **THEIRS**, **Uniform Access**, **Uniform Information (Repo == Wiki)**, **User Freedom and Control**, **Version Control Practices**, **Virtual Memory Abstraction**, **Worse Is Better**, **XTHEIRS**, **You Aren't Gonna Need It**, **Zen of Python**, **Zero-Copy**, **Zero-One-Infinity**

---

_Generated by `registry.py` / `wiki_catalog.py` — do not edit by hand; edit `gen.SPEC` in the main repo and regenerate._
