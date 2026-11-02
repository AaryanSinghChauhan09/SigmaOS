# SigmaOS-configurator

System configuration UI for display, input, network, sound, and account settings. Panels-based interface modeled on Cinnamon Control Center.

**Upstream inspiration:** https://github.com/linuxmint/cinnamon-control-center

**Dependencies:** `gtk4`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-configurator /usr/bin/sigma-configurator
```
