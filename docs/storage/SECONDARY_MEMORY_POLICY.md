# Secondary Memory Policy

Lifecycle:
probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery

## Subsystem Classes
- Block Layer Class
- Device Class
- Cache Class
- Filesystem Class

## Subsystem Policies
- Mount Policy
- Flush Policy
- Reclaim Policy
- Error Policy
- NUMA Behavior
- Behavior Under Storage Hotplug
