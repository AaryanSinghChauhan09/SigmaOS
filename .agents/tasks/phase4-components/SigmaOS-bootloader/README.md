# SigmaOS-bootloader

System bootloader integration managing GRUB configuration, kernel selection, boot entries, and multi-OS boot menus. Provides a Rust API for reading and updating bootloader state.

**Upstream inspiration:** https://www.gnu.org/software/grub/

**Dependencies:** `grub2`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-bootloader /usr/sbin/sigma-bootloader
```
