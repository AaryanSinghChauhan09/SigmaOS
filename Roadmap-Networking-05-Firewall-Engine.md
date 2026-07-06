# SigmaOS Roadmap: Stateful Firewall Engine
A capability-aware stateful packet filter in the network stack.
## Goals
- Connection tracking table (static array)
- Per-capability inbound/outbound rules
## Key Milestones
- [ ] Connection tracking hash table
- [ ] Rule match engine (BPF-like)
- [ ] CLI: sigma-fw allow/deny/list
"@

"Roadmap-Storage-01-ZFS-Integration.md" = @"
# SigmaOS Roadmap: OpenZFS Integration
Production-grade storage with checksums, compression, and snapshots.
## Goals
- ZFS pool (zpool) driver wrapping SovereignFS
- LZ4 / ZSTD transparent compression
## Key Milestones
- [ ] ZFS pool VDEV abstraction
- [ ] Block-level BLAKE3 checksums
- [ ] sigma-zfs snapshot | rollback | send CLI