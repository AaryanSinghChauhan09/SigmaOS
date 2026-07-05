# SigmaOS Roadmap: Native Container Runtime (sigma-pod)
Run OCI-compliant containers natively on SigmaOS without Docker daemon.
## Goals
- OCI bundle extraction and namespace setup
- cgroup v2 resource limits via HAL
## Key Milestones
- [ ] OCI spec JSON parser
- [ ] Mount namespace and chroot setup
- [ ] cgroup CPU/memory limit enforcement