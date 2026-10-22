# Pull Request Template

## PR Description
Provide a concise description of the changes proposed in this Pull Request.

## Component
- [ ] Kernel & Memory
- [ ] VFS & Storage
- [ ] Networking & IPC
- [ ] Userspace & Shell
- [ ] Security & Sandboxing
- [ ] Package Management & Updates
- [ ] Desktop & GUI
- [ ] Governance & Documentation

## Upstream study
Detailed analysis of upstream research and reference architectures.

## What SigmaOS adopts
Specific design patterns, invariants, or vertical slice capabilities incorporated into SigmaOS.

## Safety and rollback
Risks introduced by this change and the documented rollback or recovery procedure.

## Kernel safety review
- [ ] Code meets `#![no_std]` compliance where required.
- [ ] Unit tests and integration tests pass cleanly (`./run_sigma_tests.sh`).
- [ ] Memory boundaries, privilege levels, and sandbox constraints verified.
- [ ] Canonical component wiki / documentation updated with evidence links.
