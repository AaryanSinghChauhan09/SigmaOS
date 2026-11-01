# SigmaOS-asdcontrol-git

ASD (Audio/Screen/Display) control daemon that coordinates hardware-level audio and display adjustments across input devices and system events. Runs as a background service with D-Bus API exposure.

**Upstream inspiration:** https://github.com/ASD-control

**Dependencies:** `alsa-lib`, `libdrm`, `dbus`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-asdcontrol /usr/sbin/sigma-asdcontrol
sudo systemctl enable sigma-asdcontrol
```
