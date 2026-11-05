# Secondary Memory Policy

Lifecycle: probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery

## Block Layer Class
Generic block device layer handling request queuing and completion.

## Device Class
Hardware and virtual devices (NVMe, AHCI, VirtIO).

## Cache Class
Page and buffer cache management with bounded metadata.

## Filesystem Class
Filesystem abstractions, extent trees, and inode mappings.

## Mount Policy
Lifecycle and semantics of mount/unmount operations.

## Flush Policy
Dirty buffer flushing and writeback policies.

## Reclaim Policy
Memory reclamation, cache eviction, and trimming.

## Error Policy
Handling storage timeouts, read/write errors, and degradations.

## NUMA Behavior
NUMA-aware memory allocation for storage rings and buffer pools.

## Behavior Under Storage Hotplug
Dynamic device discovery, safe unmount, and teardown under hotplug events.
