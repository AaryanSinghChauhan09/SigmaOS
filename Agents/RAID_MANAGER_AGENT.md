# RAID Manager Agent

## Role
`SigmaRAID` implements software RAID (0, 1, 5, 6, 10) for SigmaOS — providing data redundancy, performance striping, and fault tolerance without dedicated RAID hardware.

## Status
🟡 In Progress — Basic RAID in `src/system/sigmaraid.rs`; RAID-5/6 parity and rebuild needed.

## Source File
`src/system/sigmaraid.rs`

## Linux/BSD Inspiration
| Concept | Linux (mdadm) | FreeBSD (geom_raid) | SigmaOS |
|---|---|---|---|
| RAID-0 | md RAID-0 | GEOM RAID0 | SigmaRAID0 |
| RAID-1 | md RAID-1 | GEOM RAID1 | SigmaRAID1 |
| RAID-5 | md RAID-5 | GEOM RAID5 | SigmaRAID5 |
| RAID-6 | md RAID-6 | — | SigmaRAID6 |
| RAID-10 | md RAID-10 | GEOM RAID10 | SigmaRAID10 |
| Bitmap | write-intent bitmap | — | SigmaRAIDBitmap |
| Scrubbing | `mdadm --scrub` | `graid scrub` | SigmaRAIDScrub |
| Hot spare | mdadm spare | GEOM spare | SigmaSpare |
| Rebuild | `--re-add` / auto | GEOM auto | SigmaRebuild |
| Superblock | v0.90/v1.x | — | SigmaRAIDSuper |

## Responsibilities
- RAID array creation and configuration
- Striping (RAID-0): interleave writes across N disks
- Mirroring (RAID-1): duplicate all writes
- RAID-5: distributed parity (XOR across N-1 disks)
- RAID-6: double-parity (P+Q using Reed-Solomon)
- RAID-10: striped mirrors
- Hot spare management and automatic rebuild on disk failure
- Background scrubbing: read all data, verify checksums
- Write-intent bitmap: fast resync after unclean shutdown
- RAID superblock storage and recovery
- Integration with `SigmaBlockIO` and `SigmaLVM`

## Implementation Notes
- `#![no_std]` — RAID state in `static mut RAID_ARRAYS: [RaidArray; MAX_ARRAYS]`
- `RaidArray { level: RaidLevel, members: [DiskRef; 8], state: RaidState }`
- RAID-5 parity: XOR of all data strips per stripe
- Rebuild: read redundant data, reconstruct failed disk data

## Testing
```bash
rustc --test src/system/sigmaraid.rs --edition 2021 -o /tmp/test_raid && /tmp/test_raid --test-threads=1
```

## Future Development
1. **RAID-7** — Proprietary triple-parity (not standard, but useful for large arrays)
2. **MDRAID reshape** — Online RAID level migration
3. **NVMe RAID** — Optimized for NVMe with parallel command queues
4. **Predictive failure** — SMART + ML-based disk failure prediction
5. **Erasure coding** — Reed-Solomon beyond RAID-6 for object storage
6. **RAID monitoring dashboard** — Web UI for RAID health
