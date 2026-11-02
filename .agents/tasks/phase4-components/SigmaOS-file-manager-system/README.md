# SigmaOS-file-manager-system

System-level file management layer providing mount point handling, file operation daemons, and extended metadata indexing. Sits below the file manager UI, inspired by Nemo's backend.

**Upstream inspiration:** https://github.com/linuxmint/nemo

**Dependencies:** `gio`, `udisks2`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-file-manager-system /usr/lib/sigma/file-manager-system
```
