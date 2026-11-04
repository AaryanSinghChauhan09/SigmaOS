# Pull Request Template

## Summary
Brief summary of the changes proposed in this PR.

## Component
- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management
- [ ] IPC handler
- [ ] I/O component
- [ ] Storage component
- [ ] Shell component

## Motivation
Why is this change necessary?

## Implementation notes
Technical details and architecture impact.

## Reference study
Upstream study and reference behavior:
- Upstream study: Linux/BSD/Redox/xv6 reference
- What SigmaOS adopts

## Tests
- Unit tests
- Integration tests

## Verification
- cargo check --lib
- cargo test --lib
- ./run_sigma_tests.sh

## Capability status
- Prototype
- Integrated
- Supported

## Integration impact
- System integration notes

## Reviewer checklist
- [ ] Scope is narrow
- [ ] `unsafe` justified
- Kernel safety review / Reviewer checklist

## I/O component
- [ ] I/O policy
- [ ] Interrupt manager
- [ ] DMA manager
- [ ] Buffer manager
- [ ] I/O scheduler

## I/O safety review
- Verified buffer and interrupt lifecycle constraints.

## IPC handler
- IPC semantics
- Blocking behavior: supported
- Non-blocking behavior: supported
- EOF / closure behavior: handled
- Security review: Capability/namespace checks enforced

## Storage component
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review
- Double-completion prevented
- Resource limits enforced

## Shell component
- Shell safety review
- Safety and rollback
- Risks and rollback

## Shell safety review
- Verified shell execution constraints and rollback safety.
