---
name: OSS Idea Adoption / Competitor Feature Proposal
about: Adapt or prototype an idea from another open-source OS project (Redox, seL4, Tock, Fuchsia, Linux, BSD)
title: '[OSS-IDEA] '
labels: 'research, prototype'
assignees: ''
---

## Source Open-Source OS Project
- **Project Name**: (Redox / seL4 / Tock OS / Fuchsia / Linux / FreeBSD / OpenBSD)
- **Reference URL / Source Commit**:

## Feature Description
Describe the feature or architectural pattern to borrow and adapt for SigmaOS.

## Adaptation & Prototype Plan
- **Proposed Prototype Timebox**: (1–2 weeks)
- **Target SigmaOS Subsystem**:
- **Expected Benefits**: (Security / Latency / Footprint / Compatibility)

## SigmaOS Architecture Compliance Checklist
- [ ] Maintains `no_std` compliance in core microkernel crates.
- [ ] Respects `CapabilityToken` permission checks.
- [ ] Uses strict Paged vs NonPaged pool memory allocations.
- [ ] Prototype will include standalone `rustc --test` verification.
