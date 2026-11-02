# SigmaOS-control-center

Unified control panel aggregating all system configuration panels into a single searchable interface. Acts as the launcher and container for individual configurator modules.

**Upstream inspiration:** https://github.com/linuxmint/cinnamon-control-center

**Dependencies:** `gtk4`, `SigmaOS-configurator`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-control-center /usr/bin/sigma-control-center
```
