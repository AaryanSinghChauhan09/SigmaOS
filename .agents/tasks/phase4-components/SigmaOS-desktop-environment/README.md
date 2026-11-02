# SigmaOS-desktop-environment

Core desktop environment for SigmaOS providing the compositor, window manager, panel, system tray, and session management. Inspired by the Cinnamon desktop architecture.

**Upstream inspiration:** https://github.com/linuxmint/cinnamon

**Dependencies:** `mutter`, `clutter`, `cogl`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-session /usr/bin/sigma-session
```
