# SigmaOS-daemon-settings

Settings daemon that applies user preferences for keyboard, mouse, display, power, and accessibility at login and on change. Runs as a per-session service.

**Upstream inspiration:** https://github.com/linuxmint/cinnamon-settings-daemon

**Dependencies:** `gtk4`, `dbus`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-daemon-settings /usr/lib/sigma/sigma-daemon-settings
```
