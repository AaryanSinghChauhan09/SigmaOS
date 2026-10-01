# Hardware and Distro Adaptation Proposal

**Status: proposal only.** The targets below are not evidence that the corresponding hardware drivers, operating-system compatibility, compliance controls, or performance levels are implemented. Verify each component against its source, runtime wiring, and recorded checks before describing it as supported.

## Proposed hardware targets

- Legacy PC devices: ISA and IDE controllers, VGA/VBE framebuffers, and PS/2 input.
- Modern systems: NVMe storage, xHCI USB, E1000-family networking, PCIe, and CXL.
- Architectures: x86-64, ARM64, and RISC-V, with explicit per-target boot and device support.

These targets require device discovery, validated register and DMA bounds, interrupt handling, resource ownership, reset and timeout paths, and tests that distinguish simulated devices from real hardware.

## Proposed compatibility targets

- Import and translate selected Linux/BSD package formats through isolated, bounded adapters.
- Treat package metadata as untrusted input; verify signatures with an audited provider and fail closed when it is unavailable.
- Measure performance and compatibility per format and architecture. Do not claim distro parity based on module presence or fixed scores.

## Security and compliance boundaries

Standards such as CIS, NIST, WCAG, GDPR, and FedRAMP are evaluation targets, not certifications. Document the applicable control, runtime enforcement path, and evidence separately. Do not claim encryption, post-quantum cryptography, hardware-backed key storage, sandbox enforcement, or compliance without an integrated and reviewed provider.

## Implementation and maintenance requirements for AI agents

1. Select one bounded target and document its threat model, hardware assumptions, and unsupported states.
2. Implement the runtime path with checked lengths, ownership, error propagation, and fail-closed behavior when a provider is absent.
3. Verify with focused tests and the relevant architecture or hardware runner; label emulation and model-only checks accurately.
4. Update `docs/` as the source of truth. Update the GitHub Wiki only after behavior is implemented and verified, and preserve one page per topic.
5. Record measured performance and compatibility results with the tested revision, target, and method. Never substitute estimates or fixed percentages for measurements.
