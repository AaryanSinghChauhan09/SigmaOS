# SigmaOS-drivers

Hardware driver management utility that detects hardware, recommends proprietary or open-source drivers, and handles installation and switching. Based on the mintdrivers approach.

**Upstream inspiration:** https://github.com/linuxmint/mintdrivers

**Dependencies:** `SigmaOS-common`, `apt`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-drivers /usr/bin/sigma-drivers
```
