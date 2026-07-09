# SigmaOS Roadmap: Distributed Storage (Ceph-like)
Spread data across multiple SigmaOS nodes with replication and erasure coding.
## Goals
- CRUSH-like placement algorithm
- 3x replication minimum
## Key Milestones
- [ ] Object placement ring (consistent hashing)
- [ ] Replication protocol over WireGuard mesh
- [ ] Erasure coding (Reed-Solomon)