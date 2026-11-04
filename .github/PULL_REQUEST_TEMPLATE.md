## Summary
Summary of changes proposed in this Pull Request.

## Motivation
Motivation and context for this change.

## Implementation notes
Implementation details and architecture design notes.

## Upstream study
- Reference study: Linux / BSD / Redox / xv6 reference
- What SigmaOS adopts
- Reference behavior

## Component
- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management
- [ ] IPC handler
- [ ] Memory component
- [ ] I/O component
## Shell component
- [ ] Shell component

## Shell safety review
- Shell safety review verified

## Storage component
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review
- Double-completion prevented
- Resource limits enforced

## IPC semantics
- Blocking behavior: Specified
- Non-blocking behavior: Specified
- EOF / closure behavior: Specified

## Security review
- Capability/namespace checks enforced

## Risks and rollback
- Safety and rollback: Defined

## Tests
- Unit and integration tests added.

## Verification
- cargo check --lib
- cargo test --lib
- ./run_sigma_tests.sh

## Capability status
Status level: Proposed / Prototype / Integrated / Hardened / Supported

## Integration impact
Impact on dependent subsystems and components.

## Reviewer checklist
- [ ] Scope is narrow
- [ ] `unsafe` justified
- [ ] Kernel safety review
- [ ] Memory safety review
- [ ] I/O safety review
- [ ] Shell safety review

<!--
### I/O Subsystem Verification (if applicable)
- I/O component: Verification details
- I/O policy: Specification conformance
- Interrupt manager: Latency and registration check
- DMA manager / DMA queue: Descriptor safety check
- Buffer manager: Memory pooling check
- I/O scheduler: Queue priority check
- I/O safety review: Sign-off
-->
