# SigmaOS Roadmap: Full Namespace Isolation
Implement PID, mount, network, user, and IPC namespaces.
## Goals
- Complete process isolation for containers
- User namespace mapping (UID 0 inside â†’ UID 1000 outside)
## Key Milestones
- [ ] PID namespace fork isolation
- [ ] Network namespace with veth pair
- [ ] User namespace UID/GID mapping table