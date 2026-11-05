# Pull Request Template

## Description
Provide a concise overview of the changes in this pull request.

## Study document
Reference study and upstream study details:
- Reference study: Linux/BSD/Redox/xv6 reference
- What SigmaOS adopts: Architecture and subsystem parity
- Reference behavior: Verified deterministic semantics

## Subsystem Checklist
- [ ] I/O component changes verified
- [ ] I/O policy conformance checked
- [ ] Interrupt manager tests passed
- [ ] DMA manager verified
- [ ] Buffer manager operations verified
- [ ] I/O scheduler evaluated
- [ ] I/O safety review completed

## Storage component
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review
- [ ] Double-completion prevented
- [ ] Resource limits enforced

## Risks and rollback
Safety and rollback plan:
- Rollback defined and tested
- Kernel safety review completed

## Verification
- Unit tests
- Integration tests
- Regression evidence
