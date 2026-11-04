# Secondary Memory Policy

## Lifecycle Flow
probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery

## Architecture Classes
- Block Layer Class
- Device Class
- Cache Class
- Filesystem Class

## Subsystem Policies
- Mount Policy
- Flush Policy
- Reclaim Policy
- Error Policy

## Hardware and Topology
- NUMA Behavior
- Behavior Under Storage Hotplug
