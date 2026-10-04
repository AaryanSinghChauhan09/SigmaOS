# Future Development: Storage and Filesystems

**Status:** Proposal. Filesystem and storage ideas below are research and implementation targets, not a statement that every named format or feature is production-ready.

## Scope

Improve data integrity, crash recovery, snapshots, device discovery, and storage performance. Current source entry points include [`vfs.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/filesystem/vfs.rs), [`sigma_fs.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/filesystem/sigma_fs.rs), [`cow_snapshot.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/filesystem/cow_snapshot.rs), [`zfs_arc.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/filesystem/zfs_arc.rs), and [`nvme_driver.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/drivers/nvme_driver.rs).

## Linux and BSD ideas to evaluate

| Project | Design idea | SigmaOS question |
|---|---|---|
| Linux ext4/JBD2 | Ordered metadata journaling and replay after interruption | Which on-disk invariants must hold after every interrupted write? |
| Btrfs | Copy-on-write trees, checksums, and subvolume snapshots | Can snapshots preserve clear transaction and space-accounting guarantees? |
| FreeBSD ZFS/GEOM | Layered storage providers, checksummed data, and pool health | Which provider interfaces and integrity checks fit SigmaOS's resource limits? |
| NetBSD | VFS and device-independent subsystem boundaries | Can filesystem implementations share a narrow, testable VFS contract? |
| Alpine Linux | Small base system and simple recovery media | Can recovery remain usable with limited RAM and no network? |

## Proposed work sequence

1. **Specify the VFS contract.** Define path handling, mount ownership, permissions, error codes, and concurrent operation semantics.
2. **Establish crash-consistency rules.** Document write ordering, journal or log records, replay limits, and behavior on corrupt metadata.
3. **Build one reliable snapshot path.** Define snapshot creation, rollback, deletion, and space reclamation for a single supported filesystem before adding more formats.
4. **Connect device discovery to storage.** Validate capacity, sector size, alignment, DMA limits, and device removal for NVMe and virtio block devices.
5. **Add integrity and recovery tools.** Specify checksums and offline verification. Never treat a generated manifest or placeholder digest as proof of stored-data integrity.
6. **Bound the cache.** Account ARC capacity in bytes, define replacement behavior for variable-size blocks, and bypass entries larger than the cache without evicting resident data.

## Completion criteria

- Interrupted writes recover to a documented consistent state or return a clear unrecoverable error.
- Snapshot rollback is atomic from the caller's perspective and does not silently discard unrelated state.
- Device ranges and arithmetic are checked for overflow and alignment.
- Corrupt, truncated, or unsupported on-disk data fails closed without out-of-bounds access.
- Recovery procedures are documented and can run without assumptions about a network connection.
- Cache accounting never exceeds its configured capacity, including for oversized entries and cache configurations with zero capacity.

## Maintenance

Keep format support separate from operational readiness. Document the supported on-disk version, recovery limitations, and tested devices for each implementation.
