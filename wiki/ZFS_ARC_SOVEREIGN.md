# SigmaOS Sovereign ZFS Adaptive Replacement Cache (ARC)

## Overview

SigmaOS implements a **pure-Rust sovereign ZFS ARC (Adaptive Replacement Cache)** (`src/fs/zfs_arc_sovereign.rs`), absorbing the caching architecture developed by IBM (Nimrod Megiddo & Dharmendra S. Modha) and popularized by OpenZFS and FreeBSD ZFS.

Standard operating systems rely on simple LRU (Least Recently Used) page caches, which suffer severe performance degradation under sequential file scans or mixed random/sequential workloads. ARC dynamically self-tunes between recency and frequency.

## The Four Lists of ARC

- **$T_1$ (MRU - Most Recently Used)**: Pages accessed recently.
- **$T_2$ (MFU - Most Frequently Used)**: Pages accessed at least twice.
- **$B_1$ (MRU Ghost List)**: Evicted page identifiers from $T_1$ (tracks metadata only).
- **$B_2$ (MFU Ghost List)**: Evicted page identifiers from $T_2$ (tracks metadata only).

## Self-Tuning Adaptation ($p$)

The target size parameter $p \in [0, c]$ dictates the target capacity of $T_1$:
- **Hit in $B_1$**: Indicates the cache should have prioritized **recency**; $p$ is increased.
- **Hit in $B_2$**: Indicates the cache should have prioritized **frequency**; $p$ is decreased.

This guarantees optimal hit ratios across database, compiler, multimedia streaming, and general filesystem access patterns without manual tuning.

## Test Verification

6 standalone unit tests verified in test runner suite `[18]`:
- `test_zfs_arc_basic_hit_miss`: Initial miss followed by hit and promotion.
- `test_zfs_arc_ghost_adaptation_b1`: Eviction into $B_1$ ghost list and adaptive target adjustment.
- `test_zfs_arc_mfu_promotion`: Promotion from $T_1$ to $T_2$ upon repeated access.
- `test_zfs_arc_hit_ratio`: Real-time mathematical hit ratio calculation.
- `test_zfs_arc_capacity_invariant`: Strict adherence to cache boundary limits.
- `test_zfs_arc_empty_ratio`: Clean handling of cold start.
