# LVM Manager Agent

## Role
`SigmaLVM` implements Logical Volume Management for SigmaOS — providing flexible disk partitioning with volume groups, logical volumes, thin provisioning, and snapshots, inspired by Linux LVM2 and FreeBSD GEOM.

## Status
🟡 In Progress — Basic LVM in `src/system/sigmalvm.rs`; thin provisioning and snapshot needed.

## Source File
`src/system/sigmalvm.rs`

## Linux/BSD Inspiration
| Concept | Linux LVM2 | FreeBSD GEOM eli | SigmaOS |
|---|---|---|---|
| Physical Volume | `pvcreate` | GEOM provider | SigmaPV |
| Volume Group | `vgcreate` | GEOM class | SigmaVG |
| Logical Volume | `lvcreate` | GEOM consumer | SigmaLV |
| Thin pool | `lvcreate --thin` | — | SigmaThinPool |
| Snapshot | `lvcreate -s` | ZFS snapshot | SigmaLVMSnap |
| Resize | `lvresize` / `pvresize` | GEOM resize | SigmaLVResize |
| Stripe | `lvcreate --stripes N` | GEOM stripe | SigmaLVStripe |
| Mirror | `lvcreate --mirrors N` | GEOM mirror | SigmaLVMirror |
| Activation | `vgchange -ay` | — | SigmaVGActivate |
| Metadata | `/etc/lvm/backup/` | — | SigmaLVMMeta |

## Responsibilities
- Physical volume initialization and UUID assignment
- Volume group creation from multiple PVs
- Logical volume allocation (linear, striped, mirrored)
- Thin provisioning: overcommit disk space, allocate on demand
- LVM snapshots: CoW at chunk granularity
- Online resize: grow/shrink LV and underlying filesystem
- LVM cache: SSD caching tier for HDD (dm-cache)
- LVM RAID: integrated RAID via device-mapper
- Metadata backup and recovery
- Integration with `SigmaMkfs` and `SigmaMount`

## Implementation Notes
- `#![no_std]` — LVM metadata in `static mut LVM_STATE: LvmState`
- `PhysVol { uuid: [u8;32], size_pe: u64, free_pe: u64 }`
- `LogVol { name: [u8;64], vg: &VolumeGroup, segments: Vec<Segment> }`
- Thin pool: B-tree mapping logical→physical chunks

## Testing
```bash
rustc --test src/system/sigmalvm.rs --edition 2021 -o /tmp/test_lvm && /tmp/test_lvm --test-threads=1
```

## Future Development
1. **VDO integration** — Deduplication and compression layer
2. **Bcache/dm-writecache** — Write cache for hybrid storage
3. **LVM auto-activation** — `udev` triggered VG activation
4. **Stratis integration** — High-level storage management
5. **LUKS2 integration** — Full-disk encryption on LVM volumes
6. **LVM over NVMe-oF** — LVM on remote NVMe targets
