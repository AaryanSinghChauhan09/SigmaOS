# Sovereign Container Framework

Linux-independent container implementation that uses SigmaOS shard isolation
instead of Linux namespaces and cgroups.

## Comparison with Docker/OCI

| Feature | Docker (Linux) | SovereignContainers |
| --- | --- | --- |
| Isolation | Linux namespaces | Shard capability model |
| Resource limits | cgroups | Sovereign scheduler quotas |
| Image format | OCI tar layers | Shard bundles (CoW SFS extents) |
| Runtime | runc | sigma-run |

## Roadmap

- [x] `sigma-run` container runtime
- [x] Shard bundle format (OCI-compatible import)
- [x] Networking namespace analogue via mesh shards
- [ ] Container image registry
- [ ] Container orchestration API
- [ ] Resource quota enforcement
- [ ] Container health monitoring
- [ ] Container lifecycle management
- [ ] Container storage volumes
- [ ] Container security policies
