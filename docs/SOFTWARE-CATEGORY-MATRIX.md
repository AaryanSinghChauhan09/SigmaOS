# SOFTWARE CATEGORY MATRIX — SigmaOS Multi-Repository Ecosystem

*Generated from `COMPONENTS.toml`, modelled on Linux Mint & Omarchy Linux sub-repository architecture (150 total repos, multiple focused repos instead of one monolithic repo).*

| Repo | Category | Engine | Language | Visibility | Upstream Inspirations |
|------|----------|--------|----------|------------|----------------------|
| SigmaOS-actions | Reusable GitHub Actions for SigmaOS repos | plugin_registry | shell | public | https://github.com/linuxmint/cinnamon-spices-actions; https://github.com/linuxmint/github-actions |
| SigmaOS-apturl | apt:// URL handler | sources_manager | shell | public | https://github.com/linuxmint/apturl |
| SigmaOS-asdcontrol-git | ASD control via git-tracked config | asdcontrol | shell | public | https://github.com/omacom/asdcontrol-git |
| SigmaOS-audio-tuner | Audio device tuner (EQ, profiles) | audio_tuner | shell | public | https://github.com/omacom/omarchy-audio-tuner |
| SigmaOS-automate | Task automation engine (hooks, rules, schedules) | automate | shell | public | https://github.com/linuxmint/automate |
| SigmaOS-backup-tool | Backup / restore with manifest & verification | backup | shell | public | https://github.com/linuxmint/mintbackup |
| SigmaOS-bluetooth | Bluetooth adapter & device manager | bluetooth | shell | public | https://github.com/linuxmint/cinnamon-bluetooth |
| SigmaOS-bluetooth-tool | Bluetooth pairing CLI/GUI (blueberry/blueferry) | bluetooth | shell | public | https://github.com/linuxmint/blueberry; https://github.com/erikkb/blueferry |
| SigmaOS-bootloader | Bootloader configuration & chainload manager | bootloader | shell | public | https://github.com/omacom/m1n1 |
| SigmaOS-brand-logo | SigmaOS brand logo & identity assets | brand_logo | shell | public | https://github.com/linuxmint/brand-logo |
| SigmaOS-calculator | Scientific calculator engine | calc | python | public | https://github.com/omacom/omacalc |
| SigmaOS-chatroom | LAN chat server & client | chat_server | shell | public | https://github.com/linuxmint/jargonaut |
| SigmaOS-common | Shared helpers + policy kit used by every applet | common_lib | shell | public | https://github.com/linuxmint/mintcommon |
| SigmaOS-configurator | Omarchy-style declarative system configurator | configurator | shell | public | https://github.com/omacom/omarchy-configurator |
| SigmaOS-control-center | Central control centre (modules discover settings) | configurator | shell | public | https://github.com/linuxmint/cinnamon-control-center |
| SigmaOS-daemon-apt | Privileged apt D-Bus daemon | sources_manager | shell | public | https://github.com/linuxmint/aptdaemon |
| SigmaOS-daemon-settings | Settings daemon (fonts, theme, hotkeys) | settings_daemon | shell | public | https://github.com/linuxmint/cinnamon-settings-daemon |
| SigmaOS-desktop | Wayland compositor shell & desktop primitives | desktop_core | rust | private | native |
| SigmaOS-desktop-configuration | Desktop flavour configuration (browser, shell, WM) | configurator | shell | public | https://github.com/linuxmint/mintdesktop |
| SigmaOS-desktop-environment | SigmaOS desktop environment (panels, menus, WM glue) | desktop_lib | rust | public | https://github.com/linuxmint/cinnamon |
| SigmaOS-desktop-library | Shared desktop library (settings, icons, xapp glue) | desktop_lib | rust | public | https://github.com/linuxmint/cinnamon-desktop |
| SigmaOS-desktop-portal | XDG desktop portal backend (screenshot, files) | desktop_portal | shell | public | https://github.com/linuxmint/xdg-desktop-portal-xapp |
| SigmaOS-dev-tools | Developer toolchain helpers | dev_tools | shell | public | https://github.com/linuxmint/mint-dev-tools |
| SigmaOS-display-manager | Display manager & login greeter launcher | greeter | shell | public | https://github.com/omacom/sddm; https://github.com/linuxmint/mdm |
| SigmaOS-domain-blocker | Block domains system-wide via /etc/hosts | domain_blocker | shell | public | https://github.com/linuxmint/mintnanny |
| SigmaOS-dots-configuration | Dotfiles manager (omadots-style) | dots | shell | public | https://github.com/omacom/omadots |
| SigmaOS-draw | Simple raster drawing app | draw | shell | public | https://github.com/linuxmint/drawing |
| SigmaOS-draw2 | Vector drawing app | draw | shell | public | https://github.com/linuxmint/drawing |
| SigmaOS-drivers | Detect & install proprietary drivers | driver_manager | shell | public | https://github.com/linuxmint/mintdrivers; https://github.com/linuxmint/ubuntu-drivers-common |
| SigmaOS-file-manager-system | File manager (nemo-style, tabs, extensions) | file_manager | shell | public | https://github.com/linuxmint/nemo; https://github.com/omacom/omafind |
| SigmaOS-filesystem | Virtual filesystem layer, journaling & zero-copy paths | vfs_core | rust | private | native |
| SigmaOS-file-writer | Minimal distraction-free text writer | file_writer | shell | public | https://github.com/omacom/omawrite |
| SigmaOS-fish | Fish config & plugins | shell_config | shell | public | https://github.com/omacom/omarchy-fish |
| SigmaOS-folder-color-switcher | Colour-code folders in the file manager | folder_color | shell | public | https://github.com/linuxmint/folder-color-switcher |
| SigmaOS-forum-themes | Community forum theme pack | forum_themes | shell | public | https://github.com/linuxmint/forums-theme |
| SigmaOS-Github-Action | CI GitHub Actions collection | plugin_registry | shell | public | https://github.com/linuxmint/github-actions |
| SigmaOS-greeter | Slick login greeter with user list & a11y | greeter | shell | public | https://github.com/linuxmint/slick-greeter |
| SigmaOS-GTK | GTK build patches & theming glue | gtk_lib | shell | public | https://github.com/linuxmint/gtk |
| SigmaOS-GTK+ | Blueman & Celluloid GTK app wrappers | gtk_lib | shell | public | https://github.com/linuxmint/blueman; https://github.com/linuxmint/celluloid |
| SigmaOS-guide-developer | Developer guide | docs_site | shell | public | https://github.com/linuxmint/doc-developer-guide |
| SigmaOS-guide-docs | Aggregate documentation build | docs_site | shell | public | https://github.com/linuxmint/mintdoc |
| SigmaOS-guide-installation | Installation guide | docs_site | shell | public | https://github.com/linuxmint/doc-installation-guide |
| SigmaOS-guide-translations | Translation guide & tooling | translations | shell | public | https://github.com/linuxmint/cinnamon-translations; https://github.com/linuxmint/mint-translations |
| SigmaOS-guide-user | User guide | docs_site | shell | public | https://github.com/linuxmint/doc-user-guide |
| SigmaOS-herdr-runtime-live-coding-agents | Live-coding agent runtime (herdr) | herdr | shell | public | https://github.com/omacom/herdr |
| SigmaOS-indicator-sound | Sound indicator (volume, output switch) | indicator_sound | shell | public | https://github.com/linuxmint/indicator-sound |
| SigmaOS-indicator-sound2 | Sound indicator alt | indicator_sound | shell | public | https://github.com/linuxmint/indicator-sound |
| SigmaOS-install2 | Alternate installer (ubiquity-style) | installer | shell | public | https://github.com/linuxmint/ubiquity |
| SigmaOS-installer | Graphical live installer (partition, copy, bootloader) | installer | shell | public | https://github.com/linuxmint/live-installer; https://github.com/omacom/omarchy-mac-installer |
| SigmaOS-ISO | Build the SigmaOS live ISO from a manifest | iso_builder | shell | public | https://github.com/omacom/omarchy-iso |
| SigmaOS-JS-interpreter | Embedded JS interpreter (cjs/mullion) | js_interpreter | shell | public | https://github.com/linuxmint/cjs; https://github.com/hr1ddh0/omarchy-mullion |
| SigmaOS-kernel | SigmaOS microkernel core (scheduler, VFS, syscall gate, memory) | kernel_core | rust | private | https://github.com/omacom/linux-aurora |
| SigmaOS-LAN-fileshare | Warpinator-style LAN send/receive | lan_share | shell | public | https://github.com/linuxmint/warpinator; https://github.com/t4t5/owly |
| SigmaOS-libappindicator | Sys-tray appindicator library | xapp_lib | shell | public | https://github.com/linuxmint/libappindicator |
| SigmaOS-live-session | Live-session bootstrap for the try-before-install ISO | session_manager | shell | public | https://github.com/linuxmint/mint-live-session |
| SigmaOS-mac-run | Run SigmaOS tooling on macOS host | setup | shell | public | https://github.com/omacom/omarchy-mac |
| SigmaOS-menus | Application menu & menu database | menus | shell | public | https://github.com/linuxmint/cinnamon-menus; https://github.com/linuxmint/mintmenu |
| SigmaOS-mesa3D | Mesa 3D build & VA-API/V4L2 request support | graphics | shell | public | https://github.com/omacom/mesa; https://github.com/omacom/libva-v4l2_request |
| SigmaOS-mirror | Mirror speed tester & automatic selector | mirror | shell | public | https://github.com/omacom/omarchy-mirror |
| SigmaOS-mozjs | SpiderMonkey (mozjs) build wrapper | js_interpreter | shell | public | https://github.com/linuxmint/mozjs102 |
| SigmaOS-network | Networking stack: TCP/IP, TLS, mDNS discovery | net_core | rust | private | native |
| SigmaOS-network-discovery | Discover hosts & services on the LAN | net_discovery | shell | public | https://github.com/linuxmint/network-discovery |
| SigmaOS-Node.js-Binary | Node.js binary distribution manager | node_binary | shell | public | https://github.com/linuxmint/distributions |
| SigmaOS-notification | Notification daemon (d-Bus org.freedesktop.Notifications) | notify | shell | public | https://github.com/omacom/omapager |
| SigmaOS-notification-center-plugin | Notification centre plugin (history, do-not-disturb) | notify | shell | public | https://github.com/omacom/omarchy-notification-center-plugin |
| SigmaOS-NVIDIA | NVIDIA prime / offload GPU management | driver_manager | shell | public | https://github.com/linuxmint/nvidia-prime |
| SigmaOS-nvim | Neovim distribution (aether/lumon/pixel) | nvim_distro | shell | public | https://github.com/omacom/aether.nvim; https://github.com/omacom/lumon.nvim |
