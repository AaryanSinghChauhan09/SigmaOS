# SigmaOS Development Roadmap

> Current version: **v15.0.0 Zenith** | Next: **v16.0.0 Apex** | Repo: https://github.com/AaryanSinghChauhan09/SigmaOS

---

## Version Timeline

| Version | Codename | Target | Key Milestone |
|---------|----------|--------|---------------|
| v15.0.0 | Zenith | ✅ May 2026 | PQC, AI sched, KMS framework, 600-shard lattice |
| v15.1.0 | Zenith LTS | Aug 2026 | Launch checklist, gh-pages polish, Windows compat headers |
| v16.0.0 | Apex | Q1 2027 | Bootable ISO, real kernel, GPU drivers, Zenith desktop on HW |
| v17.0.0 | Sovereign | Q3 2027 | Cloud, mobile, RTOS, distributed, dual-boot profiles live |
| v18.0.0 | Transcendence | Q1 2028 | India Stack live, LLM backend, formal verification |

---

## Current Phase: G — Kernel Completion

Phase G converts SigmaOS from a design scaffold to a bootable operating system.

### G Blockers (ordered)

| # | Task | File | Status |
|---|------|------|--------|
| 1 | Round-robin scheduler | `kernel/core/sigma_sched.cpp` | ⬜ |
| 2 | Buddy physical allocator | `kernel/core/sigma_mm.cpp` | ⬜ |
| 3 | Slab allocator (kmalloc) | `kernel/core/sigma_mm.cpp` | ⬜ |
| 4 | x86-64 page table walker | `kernel/mm/sigma_vmm.cpp` | ⬜ |
| 5 | APIC + PIC init | `kernel/core/sigma_irq.cpp` | ⬜ |
| 6 | 30-syscall dispatch | `kernel/core/sigma_syscall_dispatch.cpp` | ⬜ |
| 7 | VESA/GOP framebuffer | `drivers/display/sigma_vesa.cpp` | ⬜ |
| 8 | sigma-boot.efi UEFI loader | `sigma-boot/sigma_boot.c` | ⬜ |
| 9 | `make iso` bootable ISO | `Makefile` | ⬜ |

---

## Branch Roadmap Summary

| Branch | Purpose | Target Version | Priority |
|--------|---------|----------------|----------|
| `main` | Stable integration | v15.x | 🔴 Critical |
| `kernel-exp` | Real kernel bodies | v16.0 Apex | 🔴 Critical |
| `drivers-dev` | SDF hardware drivers | v16.0 Apex | 🔴 Critical |
| `fs-dev` | VFS, SigmaFS, Ext4 | v16.0 Apex | 🟠 High |
| `tools-dev` | CLI, docs, automation | v15.x | 🟠 High |
| `performance-optimized` | SIMD, PGO, scheduler | v16.0 Apex | 🟠 High |
| `release/standalone` | Full desktop | v15.1 | 🟠 High |
| `release/microkernel` | Minimal kernel | v16.0 Apex | 🔴 Critical |
| `release/cloud` | Cloud/container | v17.0 | 🟠 High |
| `release/distributed` | Multi-node cluster | v17.0 | 🟡 Medium |
| `release/dual-boot` | Dual-boot | v16.0 Apex | 🟡 Medium |
| `release/rtos` | Hard real-time | v17.0 | 🟡 Medium |
| `release/mobile` | ARM64 mobile | v17.0 | 🟡 Medium |
| `release/browser` | WASM demo | v15.1 | 🟢 Low |
| `release/app` | App store demo | v15.1 | 🟢 Low |

---

## Completed Phases

### Phase F — Competitor Crusher ✅
- Native KMS/GPU framework
- PCIe MSI-X HAL
- Cgroup enforcement
- Sovereign Package Registry
- Offline-First CRDT Sync
- Native Performance Governor

### Phase E — Gap Closing ✅
- NVMe Driver
- USB xHCI Driver
- Power Management (ACPI)
- Crash Reporter
- Ext4 Journal (JBD2 rewrite)

### Phase C/D — Security + Runtime ✅
- Kyber-1024 KEM
- Dilithium-5 signatures
- Immutable audit trail
- Adaptive zero-trust engine
- Neural UI (AVX-512)
- WASM/WASI runtime
- Linux ELF compat layer

---

*Full details: [Branch-Development-Roadmap](Branch-Development-Roadmap) · [Feature-Roadmap](Feature-Roadmap) · [PHASE_G_ROADMAP](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/PHASE_G_ROADMAP.md)*
