# Secondary Memory Policy

Lifecycle:
probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery

## Block Layer Class
Block device operations and request handling.

## Device Class
Hardware and virtual storage devices.

## Cache Class
Block and page caching mechanisms.

## Filesystem Class
File layout and metadata structure.

## Mount Policy
Rules for filesystem attachment.

## Flush Policy
Rules for cache writeback.

## Reclaim Policy
Memory freeing under pressure.

## Error Policy
Degradation and fail-safe handling.

## NUMA Behavior
Allocation affinity for NUMA nodes.

## Behavior Under Storage Hotplug
Dynamic device arrival and departure.
