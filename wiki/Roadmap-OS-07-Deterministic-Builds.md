# SigmaOS Roadmap: Deterministic Reproducible Builds
Guarantee bit-for-bit identical build artefacts across machines.
## Goals
- Remove all timestamps and host-specific data from binaries
- Produce signed SBOM (Software Bill of Materials)
## Key Milestones
- [ ] SOURCE_DATE_EPOCH enforcement in build
- [ ] SBOM generation in SPDX format
- [ ] Reproducibility CI check