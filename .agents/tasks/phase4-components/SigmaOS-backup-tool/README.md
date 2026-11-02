# SigmaOS-backup-tool

System backup and restore utility using rsync snapshots with incremental scheduling. Provides BTRFS and rsync-based snapshot management with a simple CLI and optional GTK frontend.

**Upstream inspiration:** https://github.com/linuxmint/timeshift

**Dependencies:** `rsync`, `btrfs-progs`, `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-backup /usr/bin/sigma-backup
```
