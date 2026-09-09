---
name: Feature Request
about: Suggest an idea or enhancement for SigmaOS
title: '[FEAT] '
labels: 'enhancement'
assignees: ''
---

## Feature Summary
A clear description of the proposed feature or enhancement.

## Architecture & Subsystem Impact
- [ ] Kernel Core / Microkernel Shards
- [ ] Driver Subsystem (WDM / NT Object Model)
- [ ] Security / Capability System (`CapabilityToken`)
- [ ] Memory Management (Paged / NonPaged Pools)
- [ ] Zenith Desktop Compositor / GUI
- [ ] Compatibility Layer (POSIX / Linux / BSD / ReactOS)

## Rationale & Use Case
Why is this feature needed and what problem does it solve?

## Architectural Guidelines Compliance
- [ ] Enforces `no_std` compliance in bare-metal crates.
- [ ] Includes `verify_token` checks for any new syscall entrypoints.
- [ ] Provides explicit type annotations on public APIs.
- [ ] Includes unit tests callable via standalone `rustc --test`.
