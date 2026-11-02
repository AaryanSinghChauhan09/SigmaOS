# SigmaOS-automate

Task automation engine for SigmaOS that lets users define, schedule, and execute recurring system and application workflows. Inspired by NiceHash QuickMiner automation patterns.

**Upstream inspiration:** https://github.com/nicehash/NiceHashQuickMiner

**Dependencies:** `SigmaOS-common`, `cron`, `dbus`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-automate /usr/bin/sigma-automate
```
