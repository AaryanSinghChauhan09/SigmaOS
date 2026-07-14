# SigmaOS 90-Day Sprint Plan

## Sprint Overview

**Timeline**: Days 1-90
**Focus**: Foundation stabilization, CI infrastructure, and initial driver support
**Primary Goals**:

1. Establish reproducible build pipeline

2. Implement QEMU multi-arch CI

3. Complete kernel-exp Phase 0 boot path

4. Deliver sigpkg MVP

## Sprint 1: Foundation & CI (Days 1-30)

### Week 1-2: CI Infrastructure

**Priority**: Critical
**Branch**: main
**Files**: .github/workflows/, qemu-boot.sh

**Tasks**:

- [ ] Implement QEMU multi-arch matrix CI workflow (x86_64, aarch64, riscv64)

- [ ] Add kernel boot smoke tests to CI

- [ ] Implement reproducible build verification step

- [ ] Add build provenance generation and publishing

- [ ] Create benchmark suite skeleton (boot time, memory footprint)

**Acceptance Criteria**:

- CI runs on all three architectures successfully

- Boot smoke tests pass on all architectures

- Reproducible builds verified with hash comparison

- Build provenance JSON published for every release

**Owners**: Core Team
**Effort**: 2 weeks

### Week 3-4: Reproducible Builds

**Priority**: Critical
**Branch**: main
**Files**: build/, Dockerfile, toolchain-x86_64-elf.cmake

**Tasks**:

- [ ] Standardize build environment with Docker

- [ ] Implement deterministic build flags

- [ ] Add source timestamp normalization

- [ ] Create build provenance metadata format

- [ ] Integrate signtool for artifact signing

**Acceptance Criteria**:

- 100% of official artifacts reproducible across different runners

- Build provenance published with every artifact

- All artifacts signed with Dilithium-5

**Owners**: Build Team
**Effort**: 2 weeks

## Sprint 2: Kernel & Drivers (Days 31-60)

### Week 5-6: Kernel Phase 0 Completion

**Priority**: Critical
**Branch**: kernel-exp → main
**Files**: kernel/, kernel-exp/

**Tasks**:

- [ ] Complete scheduler implementation (lock-free runqueues)

- [ ] Finish memory management (VMM, page allocator)

- [ ] Implement syscall dispatch with capability checks

- [ ] Complete APIC/SMP support

- [ ] Add basic power management

**Acceptance Criteria**:

- Kernel boots determinably on x86_64

- All Phase 0 tasks from roadmap completed

- Basic scheduler and MM functional

- Syscall dispatch with pledge/unveil support

**Owners**: Kernel Team
**Effort**: 2 weeks

### Week 7-8: Critical Drivers

**Priority**: High
**Branch**: drivers-dev
**Files**: drivers/

**Tasks**:

- [ ] Implement VirtIO network driver (virtio-net)

- [ ] Implement VirtIO block driver (virtio-blk)

- [ ] Implement VirtIO GPU driver (virtio-gpu)

- [ ] Complete NVMe driver (sigma_nvme.cpp)

- [ ] Add USB xHCI driver

- [ ] Implement basic input driver (keyboard/mouse)

**Acceptance Criteria**:

- VirtIO drivers functional in QEMU

- NVMe driver validated on real hardware

- USB xHCI driver functional

- Input drivers working for basic desktop

**Owners**: Drivers Team
**Effort**: 2 weeks

## Sprint 3: Packaging & UX (Days 61-90)

### Week 9-10: sigpkg MVP

**Priority**: High
**Branch**: sigma-pkg
**Files**: sigma-pkg/, sigma_pkg_registry/

**Tasks**:

- [ ] Implement sigpkg package manager core

- [ ] Create package format specification

- [ ] Implement package signing with Dilithium-5

- [ ] Build package registry backend

- [ ] Create web UI for package store

- [ ] Package first 50 curated applications

**Acceptance Criteria**:

- sigpkg can install, remove, and update packages

- All packages signed and verified

- Web UI functional for browsing and installing

- 50 curated packages available

**Owners**: Packaging Team
**Effort**: 2 weeks

### Week 11-12: Desktop Demo & Polish

**Priority**: Medium
**Branch**: release/standalone
**Files**: release/standalone, userland/

**Tasks**:

- [ ] Integrate VirtIO-GPU with Zenith desktop

- [ ] Create bootable ISO with desktop demo

- [ ] Implement basic installer

- [ ] Add dual-boot support

- [ ] Create user onboarding guide

- [ ] Polish boot experience (splash screen, progress)

**Acceptance Criteria**:

- Zenith desktop boots in QEMU with VirtIO-GPU

- Installer can install to disk

- Dual-boot configuration working

- User can complete basic desktop tasks

**Owners**: UX Team
**Effort**: 2 weeks

## Parallel Workstreams

### Documentation (Ongoing)

**Priority**: Medium
**Files**: docs/

**Tasks**:

- [ ] Complete API documentation

- [ ] Write driver development guide

- [ ] Create contributor onboarding guide

- [ ] Document performance optimization techniques

- [ ] Create security best practices guide

**Acceptance Criteria**:

- All major components documented

- Contributor guide complete

- Security documentation published

**Owners**: Documentation Team
**Effort**: Ongoing

### Security Hardening (Ongoing)

**Priority**: High
**Files**: security/, kernel/security/

**Tasks**:

- [ ] Implement pledge/unveil system calls

- [ ] Add capability-based access control

- [ ] Integrate TPM2 for measured boot

- [ ] Implement secure boot chain

- [ ] Add post-quantum cryptography throughout

**Acceptance Criteria**:

- pledge/unveil functional for userland processes

- TPM2 measured boot working

- Secure boot chain verified

- All crypto uses post-quantum algorithms

**Owners**: Security Team
**Effort**: Ongoing

## Milestones

### Milestone 1: CI Foundation (Day 30)

- CI pipeline running on all architectures

- Reproducible builds verified

- Build provenance published

### Milestone 2: Kernel Boot (Day 60)

- Kernel boots determinably on all architectures

- Critical drivers functional

- Basic desktop demo in QEMU

### Milestone 3: Package Ecosystem (Day 90)

- sigpkg MVP complete

- 50 curated packages available

- Desktop ISO with installer

## Success Metrics

### Technical Metrics

- **CI Success Rate**: >95% across all architectures

- **Build Reproducibility**: 100% of official artifacts

- **Boot Time**: <5s to desktop in QEMU

- **Package Count**: 50 curated packages

- **Driver Coverage**: Top 5 NICs, top 3 GPUs

### Quality Metrics

- **Test Coverage**: >70% for core components

- **Security Vulnerabilities**: 0 critical CVEs

- **Documentation Coverage**: >80% of public APIs

### Adoption Metrics

- **Contributors**: 5+ active contributors

- **Stars**: 500+ GitHub stars

- **Issues**: <20 open issues, <5 critical

## Risk Management

### High-Risk Items

**Risk**: Kernel Phase 0 delays
**Mitigation**: Prioritize critical path items, defer non-essential features
**Contingency**: Extend Sprint 2 by 1 week if needed

**Risk**: Driver development complexity
**Mitigation**: Focus on VirtIO drivers first (easier to test in QEMU)
**Contingency**: Use existing open-source drivers as reference

**Risk**: sigpkg complexity
**Mitigation**: Start with minimal viable package manager
**Contingency**: Defer web UI to later sprint if needed

### Medium-Risk Items

**Risk**: Multi-arch CI infrastructure
**Mitigation**: Start with x86_64 only, add others incrementally
**Contingency**: Use GitHub Actions hosted runners initially

**Risk**: Reproducible build challenges
**Mitigation**: Use Docker for consistent build environment
**Contingency**: Accept partial reproducibility initially

## Resource Allocation

### Team Composition

- **Kernel Team**: 2 engineers

- **Drivers Team**: 2 engineers

- **Build/CI Team**: 1 engineer

- **Packaging Team**: 2 engineers

- **UX Team**: 1 engineer

- **Security Team**: 1 engineer

- **Documentation**: 1 engineer (part-time)

### Time Allocation

- **Sprint 1**: 100% CI/Build focus

- **Sprint 2**: 70% Kernel/Drivers, 30% Security

- **Sprint 3**: 60% Packaging, 40% UX/Polish

## Dependencies

### External Dependencies

- QEMU for testing

- Docker for reproducible builds

- GitHub Actions for CI

- Rust toolchain for userland

### Internal Dependencies

- Kernel Phase 0 → Drivers

- Drivers → Desktop Demo

- sigpkg → Package Ecosystem

- CI → All releases

## Communication Plan

### Weekly Standups

- Monday: Sprint planning

- Wednesday: Progress check

- Friday: Demo and retrospective

### Milestone Reviews

- Day 30: CI Foundation review

- Day 60: Kernel/Driver review

- Day 90: Package ecosystem review

### Stakeholder Updates

- Bi-weekly status reports

- Monthly demo to community

- Quarterly roadmap review

## Next Steps After 90 Days

### Immediate Next Quarter (Days 91-180)

- Complete filesystem implementation (SigmaFS, tmpfs)

- Expand driver coverage (Wi-Fi, more GPUs)

- Implement WASM runtime

- Add POSIX compatibility layer

### Medium Term (Days 181-365)

- Reach 1,000 curated packages

- Complete AI-assisted scheduler

- Implement secure update mechanism

- Add enterprise features (fleet orchestration)

---

**Last Updated**: 2026-07-05
**Sprint Owner**: SigmaOS Core Team
**Review Date**: Day 30, 60, 90
