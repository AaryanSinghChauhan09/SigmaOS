# SigmaOS-desktop-portal

XDG desktop portal implementation enabling sandboxed applications to request file access, camera, screenshots, and notifications through a standardized D-Bus API.

**Upstream inspiration:** https://github.com/flatpak/xdg-desktop-portal

**Dependencies:** `dbus`, `pipewire`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-desktop-portal /usr/lib/sigma/xdg-desktop-portal-sigma
```
