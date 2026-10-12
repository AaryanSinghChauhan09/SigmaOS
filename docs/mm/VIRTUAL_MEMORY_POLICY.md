# Virtual Memory Manager Policy

## Scope

This document specifies user/kernel virtual address space management, Virtual Memory Area (VMA) region tracking, protection enforcement, and OOM handling for SigmaOS.

## Principles

1. **User/Kernel Isolation**: User VMAs must lie within `0x0000_0000_0000_1000` to `0x0000_7FFF_FFFF_FFFF`. Kernel space (`0xFFFF_8000_0000_0000` to `0xFFFF_FFFF_FFFF_FFFF`) is inaccessible to user-mode code.
2. **W^X Protection Policy**: VMAs cannot be both writable and executable. Any attempt to map or protect a VMA with `write = true` and `execute = true` is rejected with `MmError::ProtectionViolation`.
3. **Bounded Memory Allocations**: Every process address space has a configurable maximum VMA memory limit. Exceeding this limit returns `MmError::OomLimitExceeded`.
4. **Swap & Reclaim Integration**: VMAs support background page eviction and swap-out triggers under memory pressure.
