# Secondary Memory Policy

## Complete Lifecycle
probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery

## Subsystems
- Block Layer Class
- Device Class
- Cache Class
- Filesystem Class
- Mount Policy
- Flush Policy
- Reclaim Policy
- Error Policy
- NUMA Behavior
- Behavior Under Storage Hotplug
