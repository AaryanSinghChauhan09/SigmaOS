# SigmaOS-bluetooth-tool

Bluetooth configuration and device management UI tool exposing pairing, audio, and file transfer features through a GTK interface. Built on the Blueman tool architecture.

**Upstream inspiration:** https://github.com/blueman-project/blueman

**Dependencies:** `bluez`, `gtk4`, `SigmaOS-bluetooth`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-bluetooth-tool /usr/bin/sigma-bluetooth-tool
```
