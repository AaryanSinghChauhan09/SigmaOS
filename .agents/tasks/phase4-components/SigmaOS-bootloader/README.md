# SigmaOS-bootloader

System bootloader integration managing GRUB configuration, kernel selection, boot entries, and multi-OS boot menus. Provides a Rust API for reading and updating bootloader state.

**Upstream inspiration:** https://www.gnu.org/software/grub/

**Dependencies:** `grub2`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-bootloader /usr/sbin/sigma-bootloader
```

## Upstream Reference

| Upstream | Concept absorbed |
|----------|-----------------|
| [systemd-boot / Boot Loader Specification](https://systemd.io/BOOT_LOADER_SPECIFICATION/) | `SystemdBootBackend`: `BootEntry`, `loader.conf`, per-entry `.conf` format |
| [GRUB](https://www.gnu.org/software/grub/) | Multi-entry menu model; default selection; `BootloaderConfig` trait |
| [BSD loader.conf](https://man.freebsd.org/loader.conf) | `BsdLoaderBackend`: key=value `loader.conf` for BSD kernel selection |

`src/lib.rs` adds `SystemdBootBackend` and `BsdLoaderBackend`, both implementing the `BootloaderConfig` trait. 13 inline tests cover add/remove/default/duplicate/update/render.
