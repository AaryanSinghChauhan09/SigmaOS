# Pull Request Template

## Description
Please include a summary of the changes and the related issue/PR.

## Upstream study
- Reference study and upstream behavior analysis
- What SigmaOS adopts from upstream
- Reference behavior and Linux/BSD/Redox/xv6 reference notes

## Safety and rollback
- Risks and rollback procedure defined

## I/O component verification
- [x] I/O policy compliance verified
- [x] Interrupt manager updated and tested
- [x] DMA manager and DMA queue verified
- [x] Buffer manager integrity confirmed
- [x] I/O scheduler execution validated
- [x] I/O safety review completed

## Storage component
- [ ] Block device manager
- [ ] Cache policy conformance
- [ ] Mount manager lifecycle verified
- [ ] Secondary memory ADR alignment

## Storage safety review
- Double-completion prevented
- Resource limits enforced
