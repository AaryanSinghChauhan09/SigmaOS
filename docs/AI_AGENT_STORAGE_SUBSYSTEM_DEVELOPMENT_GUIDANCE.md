# AI Agent Storage Subsystem Development & Maintenance Guidance

## Executive Overview

This document provides architectural standards, diagnostic routines, and Copy-on-Write (CoW) guidelines for AI coding agents developing and maintaining the **SigmaOS Storage Subsystem**. The storage stack combines Btrfs/ZFS self-healing CoW datasets (`SovereignZfsPoolEngine`), bcachefs multi-tier storage (`SovereignBcachefsTieringEngine`), DragonFly HAMMER2 B-tree snapshots (`Hammer2MultiVersionEngine`), and FreeBSD FFS Soft Updates metadata ordering (`BsdSoftUpdatesEngine`).

---

## Storage Layer Hierarchy

```
                            +-----------------------------------+
                            |       VFS Abstract Layer          |
                            +-----------------------------------+
                                              |
                                              v
                            +-----------------------------------+
                            |  CoW Transactional File System    |
                            | (ZFS / Btrfs / HAMMER2 Engine)    |
                            +-----------------------------------+
                             /                 |               \
                            /                  |                \
        +-----------------------+  +-----------------------+  +-----------------------+
        | bcachefs Multi-Tier   |  | Soft Updates Engine   |  | RAID Self-Healing     |
        | Fast SSD / Slow HDD   |  | Metadata Dependencies |  | Scrub & Bit-Rot Heal  |
        +-----------------------+  +-----------------------+  +-----------------------+
```

---

## Engineering Guidelines for AI Agents

1. **Copy-on-Write Data Integrity**:
   - Never mutate committed block payloads in-place. Always write new transaction group (`txg`) blocks and update dataset root pointers atomically.
   - Verify Fletcher-4 or SHA-256 block checksums on every read. Trigger `SovereignRaidSelfHealer` when a bit-rot mismatch occurs.

2. **bcachefs Multi-Tier Migration**:
   - Track extent access frequency counts (`access_count`).
   - Promote extents with $\ge 5$ accesses to `StorageTier::FastSsd` and demote extents with $\le 1$ access to `StorageTier::SlowHdd`.

3. **Soft Updates Metadata Dependencies**:
   - Enforce strict dependency tree ordering in `BsdSoftUpdatesEngine` to prevent file system inconsistency during abrupt power loss.

---

## Diagnostic Verification Protocol

AI agents modifying storage drivers must verify clean execution:
1. Run `./run_sigma_tests.sh`.
2. Run standalone storage tests: `rustc --test --edition=2021 src/distro/linux_bsd_inspirations.rs -o build/test_storage` and ensure `test_sovereign_zfs_cow_snapshots_and_integrity` and `test_bcachefs_multi_tier_storage` pass cleanly.
