# SigmaOS Roadmap: Built-In Network Emulator (sigma-netem)
Emulate network conditions (latency, loss, bandwidth) for testing.
## Goals
- Token bucket shaping for bandwidth limits
- Delay and loss injection in kernel network path
## Key Milestones
- [ ] Token bucket rate limiter
- [ ] Probabilistic packet drop
- [ ] CLI: sigma-netem --delay 100ms --loss 5%
"@

# â”€â”€â”€ New Domain: Storage Batch 2 â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
"Roadmap-Storage-04-Object-Storage.md" = @"
# SigmaOS Roadmap: S3-Compatible Object Storage (sigma-store)
Provide an S3-compatible object storage API for local and distributed data.
## Goals
- S3 REST API compatibility (PUT/GET/DELETE/LIST)
- Multi-part upload for large objects
## Key Milestones
- [ ] S3 request parser and router
- [ ] Object metadata in sigma_db
- [ ] Data stored in SovereignFS extents