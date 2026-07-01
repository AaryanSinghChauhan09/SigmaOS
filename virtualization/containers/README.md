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
- [ ] `sigma-run` container runtime
- [ ] Shard bundle format (OCI-compatible import)
- [ ] Networking namespace analogue via mesh shards
