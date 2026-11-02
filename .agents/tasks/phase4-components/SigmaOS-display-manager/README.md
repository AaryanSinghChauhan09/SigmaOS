# SigmaOS-display-manager

Display and login manager for SigmaOS handling X11/Wayland session startup, user authentication, and greeter orchestration. Built on the LightDM architecture.

**Upstream inspiration:** https://github.com/canonical/lightdm

**Dependencies:** `pam`, `xorg`, `wayland`, `SigmaOS-greeter`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-display-manager /usr/sbin/sigma-display-manager
sudo systemctl enable sigma-display-manager
```
