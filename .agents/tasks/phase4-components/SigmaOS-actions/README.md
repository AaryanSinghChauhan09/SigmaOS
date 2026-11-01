# SigmaOS-actions

System action dispatcher that routes file and application actions to the appropriate handler, including open-with menus, default app associations, and custom action scripts. Inspired by the Nemo file manager's action system.

**Upstream inspiration:** https://github.com/linuxmint/nemo

**Dependencies:** `SigmaOS-common`, `dbus`, `gio`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-actions /usr/bin/sigma-actions
```
