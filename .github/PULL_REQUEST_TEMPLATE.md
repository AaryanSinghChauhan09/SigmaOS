# Pull Request Template

## Description
Please include a summary of the changes and the related issue/PR.

## Upstream study
- Reference study and upstream behavior analysis
- What SigmaOS adopts from upstream

## Safety and rollback
- Risks and rollback procedure defined

## Component impact
- Impact on system components and architecture

## I/O component verification
- [x] I/O policy compliance verified
- [x] Interrupt manager updated and tested
- [x] DMA manager and DMA queue verified
- [x] Buffer manager integrity confirmed
- [x] I/O scheduler execution validated
- [x] I/O safety review completed

## Storage component
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review
- Double-completion prevented
- Resource limits enforced

## IPC handler
- IPC semantics
- Blocking behavior: verified
- Non-blocking behavior: verified
- EOF / closure behavior: verified
- Security review
- Capability/namespace checks enforced
- Verification: ./run_sigma_tests.sh
- Capability status
- Integration impact

## Shell component
- Shell conformance verified

## Shell safety review
- Shell safety review completed
