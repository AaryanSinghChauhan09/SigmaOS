# Pull Request

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Documentation update
- [ ] Architectural Decision Record (ADR)
- [ ] Distro parity PR

## Reference study / Upstream study
- Reference study: Linux/BSD/Redox/xv6 reference
- What SigmaOS adopts

## Risks and rollback / Safety and rollback
- Risks and rollback plan

## Reviewer checklist
- Kernel safety review / I/O safety review completed

## I/O component checklist (if applicable)
- [ ] I/O policy compliance verified
- [ ] Interrupt manager verified
- [ ] DMA manager / DMA queue verified
- [ ] Buffer manager verified
- [ ] I/O scheduler verified
- [ ] I/O safety review completed

## Storage component
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review
- Double-completion prevented
- Resource limits enforced

## IPC handler checklist (if applicable)
- IPC handler verification
- IPC semantics
  - Blocking behavior: verified
  - Non-blocking behavior: verified
  - EOF / closure behavior: verified
- Security review
  - Capability/namespace checks enforced
- Verification
  - Ran ./run_sigma_tests.sh
- Capability status verified
- Integration impact assessed

## Description
Summary of changes and motivation.

## Evidence & Testing
- Unit and integration tests passed.

## Shell component
- Shell parser and grammar

## Shell safety review
- Exit codes verified
