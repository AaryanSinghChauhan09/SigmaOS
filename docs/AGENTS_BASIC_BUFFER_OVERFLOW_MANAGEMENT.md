# SigmaOS AI Agent Basic Buffer Overflow Management Guidelines

## 1. Overview
SigmaOS incorporates advanced memory safety and exploit mitigation controls operated by AI security agents (such as `BufferOverflowGuard`, `StackCanaryProtector`, `HardenedBsdPaxCfiEngine`, `OpenBsdRetguardSentinel`, and `AslrEntropyGovernor`). These guidelines define stack canary guard pages, Address Space Layout Randomization (ASLR), W^X (Write-XOR-Execute) memory protection, Control Flow Integrity (CFI), and bounds-checked safe buffers in SigmaOS.

## 2. Core Basic Buffer Overflow Management Principles

### 2.1 Stack Canary Protection & Guard Pages
- **Stack Canaries**: Function call prologues place a random 64-bit stack canary value (`__stack_chk_guard`) between local stack variables and return addresses. Epilogues verify canary integrity, triggering an immediate kernel crash (`SIGABRT`) upon canary mismatch.
- **Hardened Guard Pages**: Physical page frame allocators (`HardenedGuardPageAllocator` in `src/kernel/memory.rs`) insert unmapped guard pages (`PTE_PRESENT = 0`) before and after kernel stack and heap allocations to trap out-of-bounds sequential buffer overruns.

### 2.2 W^X (Write-XOR-Execute) & HardenedBSD PaX CFI
- **W^X Memory Policy**: Memory pages are never simultaneously writable and executable (`PTE_WRITE` and `PTE_EXEC` are mutually exclusive). Writable heap, stack, and data pages have execution bits disabled (`NX` / `XD` bit set).
- **HardenedBSD PaX CFI**: Indirect call and jump instructions verify Control Flow Integrity (CFI) target labels to block Return-Oriented Programming (ROP) and Jump-Oriented Programming (JOP) gadget chains.

### 2.3 OpenBSD Retguard & MAP_STACK Sentinels
- **Retguard Stack Protection**: Pushes XOR-encrypted return addresses onto kernel stacks to prevent stack frame smashing attacks.
- **MAP_STACK Enforcement**: System call execution checks that userland stack pointers (`RSP`/`SP`) point strictly inside memory regions explicitly mapped with `MAP_STACK` flags, aborting forged stack pivoting attacks.

### 2.4 ASLR & Safe Bounds-Checked Strings
- **ASLR Entropy**: Kernel text, userland stack, heap (`brk`), and shared library memory mappings are randomized with 32-bit/48-bit address space layout entropy at process startup.
- **Safe Vector & String Primitives**: All internal OS code utilizes bounds-checked Rust `SigmaString` / `Vec` types and zero-allocation `klib` primitives, eliminating unsafe C-style `strcpy` / `sprintf` buffer overflows.

---
*Maintained by the SigmaOS Security, Hardening & SIG-Security Steering Committee.*
