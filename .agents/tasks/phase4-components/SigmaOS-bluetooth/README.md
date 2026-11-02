# SigmaOS-bluetooth

Bluetooth stack integration layer providing BlueZ D-Bus bindings, device pairing, audio routing, and profile management for SigmaOS. Wraps the kernel Bluetooth subsystem and BlueZ daemon.

**Upstream inspiration:** https://git.kernel.org/pub/scm/bluetooth/bluez.git

**Dependencies:** `bluez`, `dbus`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-bluetooth /usr/lib/sigma/bluetooth
```
