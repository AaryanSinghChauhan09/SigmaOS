# Secondary Memory Policy

The lifecycle of secondary memory devices proceeds through:
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery

## System Classes
- Block Layer Class
- Device Class
- Cache Class
- Filesystem Class

## Subsystem Policies
- Mount Policy
- Flush Policy
- Reclaim Policy
- Error Policy

## Architectures & Hardware Events
- NUMA Behavior
- Behavior Under Storage Hotplug
