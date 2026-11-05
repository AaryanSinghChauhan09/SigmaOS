# Pull Request Template

## Study document
- Upstream study: Reference study
- Reference study: Linux/BSD/Redox/xv6 reference
- What SigmaOS adopts: Architectural mechanisms
- Risks and rollback: Safety and rollback defined

## Reviewer checklist
- Kernel safety review
- Memory safety review
- I/O safety review: Completed

## I/O component
- I/O policy: Verified
- Interrupt manager: Verified
- DMA manager: Verified
- Buffer manager: Verified
- I/O scheduler: Verified
- I/O safety review: Completed

## Storage component
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review
- Double-completion prevented
- Resource limits enforced
