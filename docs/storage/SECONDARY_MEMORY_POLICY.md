# Secondary Memory Policy

The lifecycle of secondary memory in SigmaOS follows the sequence:
probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery

## Block Layer Class
Handles block devices, dispatch queues, request merging and bounded request allocations.

## Device Class
Hardware and virtual drivers including NVMe, virtio-blk, AHCI SATA, and RAM disks.

## Cache Class
Page and buffer caches providing writeback caching, dirty page limits, and LRU eviction.

## Filesystem Class
Storage backends including ext4, btrfs, ZFS, and FAT32.

## Mount Policy
Race-free mounting, mount table integrity, and rollback on failure.

## Flush Policy
Periodic sync daemon and explicit fsync guarantees.

## Reclaim Policy
Memory pressure watermark reclamation and dirty throttling.

## Error Policy
Handling device disconnection, media errors, and filesystem recovery.

## NUMA Behavior
NUMA-aware memory allocation for cache pages and block request rings.

## Behavior Under Storage Hotplug
Dynamic device arrival and departure with clean resource tear-down.
