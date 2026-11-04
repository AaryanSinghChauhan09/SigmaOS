# Secondary Memory Policy

Lifecycle: probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery

## Block Layer Class
Specifications for block layer abstraction.

## Device Class
Device classes covering NVMe, virtio-blk, and SATA.

## Cache Class
Block page caching semantics and writeback.

## Filesystem Class
Core filesystem structure and journaling invariants.

## Mount Policy
Mount namespace isolation and rollback behavior.

## Flush Policy
Coordinated barrier synchronization and page flushing.

## Reclaim Policy
LRU page eviction and memory reclaiming watermarks.

## Error Policy
Degraded mode handling and I/O error isolation.

## NUMA Behavior
NUMA node allocation affinity and page localization.

## Behavior Under Storage Hotplug
Dynamic device attachment and safe detachment synchronization.
