# SigmaOS-domain-blocker

Network domain blocker applying DNS-based and hosts-file filtering to block ads, trackers, and malicious domains system-wide. Supports blocklist subscription and per-app rules.

**Upstream inspiration:** https://github.com/nicehash/NiceHashQuickMiner

**Dependencies:** `SigmaOS-common`

**Build/Install:**
```
cargo build --release
sudo cp target/release/sigma-domain-blocker /usr/sbin/sigma-domain-blocker
sudo systemctl enable sigma-domain-blocker
```
