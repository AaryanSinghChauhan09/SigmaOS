# OSS Absorption: OpenZFS — Next-Generation Filesystem

> **Status**: 📋 Planned | **Source Project**: OpenZFS | **Target Shard**: `SigmaFS Self-Healing Layer`

---

## 1. Executive Summary

OpenZFS is the world's most advanced filesystem, combining a volume manager and filesystem with end-to-end data integrity, instant snapshots, copy-on-write semantics, RAIDZ redundancy, and inline deduplication and compression. Originally created at Sun Microsystems, it is now maintained by the OpenZFS project and runs on Linux, FreeBSD, and macOS.

SigmaOS absorbs OpenZFS's **RAIDZ storage pools**, **instant snapshot/clone model**, and **inline compression** into SigmaFS's self-healing storage layer.

---

## 2. Key Features to Absorb

### 2.1 Storage Pools with RAIDZ

```bash
$ sigma pool create datapool raidz /dev/sdb /dev/sdc /dev/sdd
Σ [POOL] Created: datapool (RAIDZ-1, 3 drives, ~2TB usable)
  Compression:  zstd-3 (auto)
  Checksum:     BLAKE3
  Dedup:        on (block-level)

$ sigma pool status datapool
Σ [POOL] datapool: ONLINE
  sdb: ONLINE  sdc: ONLINE  sdd: ONLINE
  Scrub: last run 2h ago, 0 errors
```

### 2.2 Instant Snapshots and Clones

Snapshots are instantaneous and space-efficient (copy-on-write). Clones create writable branches from any snapshot.

```bash
$ sigma fs snapshot datapool/home@before-upgrade
Σ [FS] Snapshot created: datapool/home@before-upgrade (0 bytes initially)

# If the upgrade breaks something:
$ sigma fs rollback datapool/home@before-upgrade
Σ [FS] Rolled back to datapool/home@before-upgrade in 0.3s
```

### 2.3 Self-Healing Checksums

Every block has a BLAKE3 checksum. On read, if a checksum mismatch is detected and a redundant copy exists (RAIDZ), SigmaFS automatically repairs the bad block.

```bash
$ sigma pool scrub datapool
Σ [POOL] Scrub complete:
  Blocks checked:   2,847,392
  Errors detected:  1 (bit rot on /dev/sdb, sector 0x3a4f)
  Repaired:         1 (restored from RAIDZ parity)
```

---

## 3. References & Standards

- OpenZFS — `openzfs.org` (CDDL-1.0)
