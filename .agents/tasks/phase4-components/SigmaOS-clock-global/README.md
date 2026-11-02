# SigmaOS-clock-global

World clock widget and timezone-aware clock service displaying multiple time zones with notifications and alarms. Integrates with the SigmaOS desktop shell.

**Upstream inspiration:** https://github.com/linuxmint/cinnamon-screensaver

**Dependencies:** `gtk4`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-clock-global /usr/bin/sigma-clock-global
```
