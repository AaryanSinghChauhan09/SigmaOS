# Secondary Memory Policy
Lifecycle stages: probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery.
## Block Layer Class
Policies for block devices.
## Device Class
Policies for device layers.
## Cache Class
Policies for cache layers.
## Filesystem Class
Policies for filesystem structures.
## Mount Policy
Mount requirements and isolation.
## Flush Policy
Flushing dirty blocks to backing stores.
## Reclaim Policy
Reclaiming blocks safely.
## Error Policy
Error handling and recovery paths.
## NUMA Behavior
NUMA memory node placement and alignment.
## Behavior Under Storage Hotplug
Handling storage detachment and hotplug.
