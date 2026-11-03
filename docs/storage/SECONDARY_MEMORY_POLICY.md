# Secondary Memory Policy

probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery

## Block Layer Class
Block request handling and submission.

## Device Class
Device drivers and controllers.

## Cache Class
Block page cache and dirty writebacks.

## Filesystem Class
Filesystem structure and directory entries.

## Mount Policy
Mount points and namespace binding.

## Flush Policy
Dirty block synchronization and barriers.

## Reclaim Policy
Memory reclamation and page eviction.

## Error Policy
Storage errors and failover mechanisms.

## NUMA Behavior
Node-local buffers and allocation affinity.

## Behavior Under Storage Hotplug
Dynamic device attachment and removal.
