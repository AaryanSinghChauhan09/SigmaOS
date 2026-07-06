# multi-arch-ci-images

---
name: Multi-Arch CI Images
about: Add RISC-V and aarch64 QEMU test images and cross-toolchain to validate kernel boots
title: "[Phase 1] Add RISC-V and aarch64 CI Images"
labels: "Phase 1, hardware, ci, medium-priority"
assignees: ""
---

## Issue Description

Implement multi-architecture CI support for RISC-V and aarch64 using QEMU test images and cross-compilation toolchains to validate kernel boots across architectures.

## Background

Broad hardware support is a strategic pillar for SigmaOS. Adding RISC-V and aarch64 CI testing enables early validation of cross-platform compatibility and aligns with Phase 1 goals of hardware parity.

## Scope

### Primary Tasks

1. **Cross-Compilation Toolchain Setup**
   - Configure aarch64 cross-compiler (aarch64-linux-gnu)
   - Configure RISC-V cross-compiler (riscv64-linux-gnu)
   - Update CMake toolchain files for both architectures
   - Update Rust toolchain configuration for cross-compilation

2. **QEMU Test Images**
   - Create QEMU configurations for aarch64 (virt machine)
   - Create QEMU configurations for RISC-V (virt machine)
   - Add boot scripts for each architecture
   - Implement architecture-specific kernel configurations

3. **CI Matrix Configuration**
   - Create GitHub Actions matrix for x86_64, aarch64, riscv64
   - Add architecture-specific build steps
   - Implement QEMU boot smoke tests for each arch
   - Add cross-compile verification steps

### Files to Modify/Create

- `toolchain-aarch64-elf.cmake` - New aarch64 toolchain file

- `toolchain-riscv64-elf.cmake` - New RISC-V toolchain file

- `.github/workflows/multi-arch-ci.yml` - New CI workflow

- `qemu-boot-aarch64.sh` - New aarch64 boot script

- `qemu-boot-riscv64.sh` - New RISC-V boot script

- `configs/kernel-aarch64.config` - New aarch64 kernel config

- `configs/kernel-riscv64.config` - New RISC-V kernel config

- `rust-toolchain.toml` - Add cross-compilation targets

## Success Criteria

- [ ] CI builds kernel for aarch64 successfully

- [ ] CI builds kernel for RISC-V successfully

- [ ] QEMU boots aarch64 kernel to shell prompt

- [ ] QEMU boots RISC-V kernel to shell prompt

- [ ] All three architectures pass smoke tests in CI matrix

- [ ] Documentation updated for cross-compilation process

## Estimated Effort

**Difficulty**: Medium
**Time**: 1–3 weeks

## Dependencies

- None (can be implemented in parallel with other Phase 1 tasks)

## Related Issues

- Phase 1: Hardware parity & filesystem

- ROADMAP_NEW.md Phase 1 deliverables

## Implementation Notes

Key considerations:

- Use QEMU's virt machine type for both architectures

- Ensure consistent bootloader (UEFI for aarch64, OpenSBI for RISC-V)

- Test with minimal kernel config first, then expand

- Consider using Docker containers for cross-compilation environment

## Resources

- [QEMU System Emulation](https://www.qemu.org/docs/master/system/index.html)

- [Rust Cross-Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)

- [CMake Cross-Compiling](https://cmake.org/cmake/help/latest/manual/cmake-toolchains.7.html)
