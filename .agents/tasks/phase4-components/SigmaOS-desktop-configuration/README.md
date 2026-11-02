# SigmaOS-desktop-configuration

Desktop configuration profiles managing themes, panel layouts, applet sets, and workspace arrangements. Allows saving and applying complete desktop configuration snapshots.

**Upstream inspiration:** https://github.com/linuxmint/cinnamon

**Dependencies:** `SigmaOS-desktop-environment`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-desktop-config /usr/bin/sigma-desktop-config
```
