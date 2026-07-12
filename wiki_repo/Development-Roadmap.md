# SigmaOS — Branch-Wise Development Roadmap

> ⚠️ **SUPERSEDED** — All branches unified into `main`. See **[ROADMAP.md](ROADMAP.md)** for the current phase-based plan.
> This file is retained for historical reference only.

> Last updated: July 2026 | Current version: v15.0.0 Zenith

---

## Overview

SigmaOS is a sovereign, AI-native, freestanding microkernel OS targeting 8 deployment profiles.
This roadmap covers every active branch, its purpose, current state, immediate tasks, and merge strategy.

**Repository:** https://github.com/AaryanSinghChauhan09/SigmaOS

---

## Branch Index

| Branch | Purpose | Target Version | Priority |
|--------|---------|----------------|----------|
| `main` | Stable integration target | v15.x | 🔴 Critical |
| `master` | Legacy mirror (deprecated) | — | — |
| `kernel-exp` | Real kernel implementation lab | v16.0 Apex | 🔴 Critical |
| `drivers-dev` | SDF hardware driver development | v16.0 Apex | 🔴 Critical |
| `fs-dev` | Filesystem layer (VFS, SigmaFS, Ext4) | v16.0 Apex | 🟠 High |
| `tools-dev` | CLI tools, automation, docs | v15.x ongoing | 🟠 High |
| `performance-optimized` | Scheduler tuning, SIMD, PGO | v16.0 Apex | 🟠 High |
| `docs-update` | Wiki, API docs, man pages | v15.x ongoing | 🟡 Medium |
| `prepare-sigmaos-launch` | v15.1 launch checklist | v15.1 | 🟡 Medium |
| `gh-pages` | Public website (GitHub Pages) | live | 🟡 Medium |
| `release/standalone` | Full desktop profile | v15.1 | 🟠 High |
| `release/microkernel` | Minimal microkernel profile | v16.0 Apex | 🔴 Critical |
| `release/cloud` | Cloud/container headless profile | v17.0 Sovereign | 🟠 High |
| `release/distributed` | Multi-node distributed cluster | v17.0 Sovereign | 🟡 Medium |
| `release/dual-boot` | Dual-boot coexistence | v16.0 Apex | 🟡 Medium |
| `release/rtos` | Hard real-time profile | v17.0 Sovereign | 🟡 Medium |
| `release/mobile` | ARM64/RISC-V mobile profile | v17.0 Sovereign | 🟡 Medium |
| `release/browser` | Browser-hosted WASM demo | v15.1 | 🟢 Low |
| `release/app` | App store demo | v15.1 | 🟢 Low |

---

## Critical Path

```
kernel-exp (Phase 0 — bootable kernel)
    ├── → drivers-dev   (SDF driver launch mechanism needed)
    │       ├── → release/standalone  (GPU + Wi-Fi)
    │       └── → release/mobile      (ARM64 BSP)
    ├── → fs-dev        (VFS layer)
    │       ├── → release/standalone  (profile VFS load)
    │       └── → release/cloud       (dm-verity)
    ├── → release/microkernel  (minimal kernel subset)
    └── → release/browser      (needs bootable ISO)

performance-optimized  → depends on kernel-exp scheduler
release/cloud          → depends on kernel-exp cgroup + namespace
release/distributed    → depends on release/cloud
release/rtos           → depends on performance-optimized EDF + kernel-exp IRQ
release/dual-boot      → depends on kernel-exp sigma-boot.efi + fs-dev
```

---

## `main` — Stable Integration Target

**Policy:** No direct development on `main`. Every PR must pass `sigma_ci.yml`.

### Current State

- v15.0.0 Zenith released

- CI: GitHub Actions on push

- Wiki auto-sync on push

### Immediate Tasks

| Task | File | Priority |
|------|------|----------|
| Wire QEMU boot test to CI | `.github/workflows/sigma_ci.yml` | 🔴 |
| Add `make check-abi` gate | `Makefile` | 🟠 |
| Enforce CURRENT_PROBLEMS_MANIFEST update on PR | `.github/PULL_REQUEST_TEMPLATE.md` | 🟠 |
| Merge `kernel-exp` once Phase 0 complete | — | 🔴 |
| Merge `drivers-dev` VESA/VirtIO-GPU | — | 🔴 |

### Exit Criteria

- `make iso` produces a bootable image

- All CI checks pass on every push

---

## `kernel-exp` — Kernel Implementation Lab

**Status:** 🔴 CRITICAL — blocks all other branches

### Current State

- Headers complete for scheduler, MM, syscall, IRQ

- All `.cpp` bodies are stubs — no real hardware boot

### Phase 0 Tasks (ordered — do not skip)

| # | Task | File | Exit Test |

|---|------|------|-----------|
| 1 | Round-robin scheduler (64 tasks) | `kernel/core/sigma_sched.cpp` | QEMU: 2 tasks interleave |
| 2 | Buddy physical allocator | `kernel/core/sigma_mm.cpp` | alloc/free 100 pages, no leak |
| 3 | Slab allocator (kmalloc) | `kernel/core/sigma_mm.cpp` | alloc/free 10 000 objects |
| 4 | x86-64 page table walker | `kernel/mm/sigma_vmm.cpp` | map 1 MB region, read back |
| 5 | APIC + PIC init | `kernel/core/sigma_irq.cpp` | timer IRQ fires in QEMU |
| 6 | HPET/APIC timer → jiffies | `kernel/core/sigma_timer.cpp` | `sleep(100ms)` works |
| 7 | 30-syscall dispatch table | `kernel/core/sigma_syscall_dispatch.cpp` | `write(1,"hi\n",3)` from userland |
| 8 | VESA/GOP framebuffer | `drivers/display/sigma_vesa.cpp` | pixels in QEMU |
| 9 | sigma-boot.efi UEFI loader | `sigma-boot/sigma_boot.c` | QEMU boots to kernel |
| 10 | `make iso` → bootable ISO | `Makefile` | `qemu -cdrom SigmaOS.iso` → shell |

### Scheduler Upgrade Path

```
Round 1: round-robin (unblocks boot)
Round 2: MLFQ (4 queues, aging)
Round 3: CFS clone (vruntime, red-black tree)
Round 4: NUMA-aware placement (ACPI SRAT)
Round 5: EDF (earliest-deadline-first, RT tasks)
Round 6: sigma-ai predictive pre-warming
```

### Merge Target

`main` after `make iso` + QEMU boot CI passing.

---

## `drivers-dev` — Hardware Driver Development

**Depends on:** `kernel-exp` Phase 0

### Current State

- NVMe: `drivers/storage/sigma_nvme.cpp` ✅

- USB xHCI: `drivers/usb/sigma_xhci.cpp` ✅

- e1000 NIC: `kernel/core/drivers/SovereignE1000.cpp` ✅

- KMS/GPU: framework only; Wi-Fi: not started

### Next Tasks

| Priority | Task | File | Target HW |
|----------|------|------|-----------|
| 🔴 | VESA/GOP framebuffer SDF driver | `drivers/display/sigma_vesa_sdf.cpp` | All UEFI |
| 🔴 | VirtIO-GPU (QEMU accelerated) | `drivers/display/sigma_virtio_gpu.cpp` | QEMU |
| 🔴 | DRM/KMS layer | `drivers/graphics/sigma_kms.cpp` | All GPU |
| 🔴 | Intel i915 basic modesetting | `drivers/graphics/sigma_i915.cpp` | Intel iGPU |
| 🟠 | AMD amdgpu basic modesetting | `drivers/graphics/sigma_amdgpu.cpp` | AMD GPU |
| 🟠 | Intel iwlwifi 802.11ax | `drivers/net/sigma_iwlwifi.cpp` | Intel Wi-Fi 6 |
| 🟠 | Realtek rtl8xxxu USB Wi-Fi | `drivers/net/sigma_rtl8xxxu.cpp` | USB dongles |
| 🟠 | HDA audio controller | `drivers/audio/sigma_hda.cpp` | All x86 |
| 🟡 | Bluetooth HCI (USB) | `drivers/bt/sigma_hci_usb.cpp` | USB BT adapters |
| 🟡 | ARM64 BCM2711 BSP | `arch/arm64/sigma_bcm2711.cpp` | Raspberry Pi 4 |
| 🟡 | ARM64 BCM2712 BSP | `arch/arm64/sigma_bcm2712.cpp` | Raspberry Pi 5 |

### Merge Target

`main` per-driver after QEMU + physical hardware CI passes.

---

## `fs-dev` — Filesystem Development

**Depends on:** `kernel-exp` Phase 0

### Current State

- VFS header: `kernel/include/kernel/sigma_vfs.h` ✅

- Ext4 journal rewrite: `fs/ext4_journal.c` ✅

- SigmaFS: early design; UBC: header only

### Next Tasks

| Priority | Task | File |
|----------|------|------|
| 🔴 | VFS open/read/write/close bodies | `kernel/vfs/sigma_vfs.cpp` |
| 🔴 | Tmpfs (RAM-backed filesystem) | `kernel/vfs/sigma_tmpfs.cpp` |
| 🟠 | SigmaFS mkfs + mount | `fs/sigmafs/sigma_mkfs.cpp` |
| 🟠 | SigmaFS directory + file ops | `fs/sigmafs/sigma_fs_ops.cpp` |
| 🟠 | Ext4 read-only mount | `fs/ext4/sigma_ext4_ro.cpp` |
| 🟠 | Unified Buffer Cache (UBC) | `kernel/fs/sigma_ubc.cpp` |
| 🟠 | Pre-emptive read-ahead | `kernel/fs/sigma_readahead.cpp` |
| 🟡 | dm-verity block verifier | `kernel/fs/sigma_dmverity.cpp` |
| 🟡 | FAT32 write support | `fs/fat/sigma_fat32_write.cpp` |

### Exit Criteria

- `sigma-sh` can open/read/write/ls on tmpfs

- `sigma-pkg install` writes packages to SigmaFS partition

---

## `tools-dev` — CLI, Docs, Automation (Current Working Branch)

### Current State ✅

- sigma-cli profiles/aliases: `userland/tools/sigma_cli.cpp`

- sigma_automation.sh: `scripts/sigma_automation.sh`

- Windows compat headers: `include/compat/*.h`

- ci_branch_check.sh: `scripts/ci_branch_check.sh`

### Next Tasks

| Priority | Task | File |
|----------|------|------|
| 🟠 | sigma-sh env vars + history | `userland/shell/sigma_shell.cpp` |
| 🟠 | sigma-sh tab completion | `userland/shell/sigma_shell.cpp` |
| 🟠 | sigma_ntdll registry stubs | `runtime/compat/win32/registry/sigma_reg.cpp` |
| 🟠 | sigma-kernel32 file I/O | `runtime/compat/win32/kernel32/sigma_kernel32_file.cpp` |
| 🟠 | sigma-msvcrt printf + malloc | `runtime/compat/win32/crt/sigma_msvcrt.cpp` |
| 🟡 | Man pages for sigma-cli tools | `docs/man/` |
| 🟡 | sigma-observatory stub | `userland/tools/sigma_observatory.cpp` |

### Merge Target

`main` on every green CI run. Ongoing.

---

## `performance-optimized` — Scheduler Tuning + SIMD

**Depends on:** `kernel-exp` baseline scheduler

### Next Tasks

| Priority | Task | File |
|----------|------|------|
| 🔴 | NUMA topology reader (ACPI SRAT) | `kernel/sched/sigma_numa.cpp` |
| 🔴 | Lock-free CAS runqueue | `klib/sigma_lockfree.h` |
| 🟠 | CFS vruntime red-black tree | `kernel/sched/sigma_cfs.cpp` |
| 🟠 | AVX-512 Kyber-1024 NTT | `crypto/sigma_kyber_avx512.cpp` |
| 🟠 | ARM NEON Kyber NTT | `crypto/sigma_kyber_neon.cpp` |
| 🟠 | ACPI P-state governor | `kernel/power/sigma_perf_governor.cpp` |
| 🟡 | io_uring equivalent ring | `kernel/io/sigma_uring.cpp` |
| 🟡 | sigma-ai predictive scheduler | `kernel/sched/sigma_ai_sched.cpp` |

### Benchmark Targets

| Metric | Target |
|--------|--------|
| Context switch | < 50 ns |
| Kyber-1024 (AVX-512) | ≥ 5.8 M ops/sec |
| Boot time (NVMe) | < 2 s |
| Idle RAM (full desktop) | < 150 MB |

---

## `docs-update` — Wiki and API Docs

### Next Tasks

| Priority | Task | File |
|----------|------|------|
| 🟠 | Wire Doxygen to CI | `.github/workflows/sigma_ci.yml` |
| 🟠 | Man pages (50 tools) | `docs/man/*.1` |
| 🟠 | Kernel developer handbook update | `wiki_repo/Kernel-Developer-Handbook.md` |
| 🟡 | Windows compat wiki page | `wiki_repo/Windows-Compatibility-Layer-Roadmap.md` |
| 🟡 | Per-branch roadmap wiki page | `wiki_repo/Branch-Development-Roadmap.md` |

---

## `prepare-sigmaos-launch` — v15.1 Launch Checklist

| Task | Status |
|------|--------|
| FEATURE_MATRIX.md up to date | ✅ |
| All `release/*` branches at parity | 🔄 |
| CURRENT_PROBLEMS_MANIFEST.md reflects Phase G | ✅ |
| sigma_automation.sh backup tested | ✅ |
| Windows compat headers committed | ✅ |
| QEMU boot CI green | ⬜ blocked on kernel-exp |
| Release notes written | ⬜ |
| GitHub release tag created | ⬜ |

---

## `gh-pages` — GitHub Pages Website

### Next Tasks

| Priority | Task | File |
|----------|------|------|
| 🟠 | Update roadmap.html with Phase 0 progress | `roadmap.html` |
| 🟠 | Add Windows compat layer section | `index.html` |
| 🟠 | Update version timeline widget | `site.js` |
| 🟡 | Interactive branch status dashboard | `site.js` |
| 🟡 | Dark/light theme CSS fix | `site.css` |

---

## `release/standalone` — Full Desktop Profile

### Next Tasks (blocked on drivers-dev + kernel-exp)

| Priority | Task |
|----------|------|
| 🔴 | GPU DRM/KMS + Zenith on real framebuffer |
| 🔴 | Compositor input event loop |
| 🟠 | Auto-tiling WM |
| 🟠 | `~/.sigma_profile` VFS load |
| 🟠 | DID-based login screen |
| 🟠 | sigma-ai LLM daemon (llama.cpp) |
| 🟠 | Indian IME (Inscript + phonetic) |

### Exit Criteria

User boots → Zenith desktop → types Hindi → runs sigma-ai.

---

## `release/microkernel` — Minimal Microkernel

### Target: < 512 KB kernel, < 8 MB RAM

| Priority | Task | File |
|----------|------|------|
| 🔴 | Minimal round-robin scheduler | `kernel/core/sigma_sched.cpp` |
| 🔴 | Buddy MM (no slab) | `kernel/core/sigma_mm.cpp` |
| 🔴 | 15 essential syscalls | `kernel/core/sigma_syscall_dispatch.cpp` |
| 🟠 | sigma-bus IPC end-to-end | `kernel/ipc/sigma_bus.cpp` |
| 🟠 | Capability token passing | `kernel/security/sigma_caps.cpp` |
| 🟡 | Formal verification proofs (Coq) | — |

---

## `release/cloud` — Cloud/Container Profile

| Priority | Task | File |
|----------|------|------|
| 🔴 | sigma-pod cgroup enforcement | `kernel/core/process/sigma_cgroup.c` |
| 🔴 | sigma-pod namespace creation | `kernel/core/process/sigma_namespace.cpp` |
| 🟠 | SovereignContainer KVM hypervisor | `kernel/hypervisor/sigma_kvm.cpp` |
| 🟠 | dm-verity .spkg image verification | `kernel/fs/sigma_dmverity.cpp` |
| 🟠 | sigma-fleet agent (MDM) | `userland/tools/sigma_fleet_agent.cpp` |
| 🟡 | gRPC management API | `api/sigma.proto` + codegen |

---

## `release/distributed` — Multi-Node Cluster

**Blocked by:** `release/cloud` cgroup + namespace

| Priority | Task | File |
|----------|------|------|
| 🟡 | SovereignCloudFS multi-node sync | `net/sigma_cloudfs.cpp` |
| 🟡 | sigma-mesh-compute scheduler | `net/sigma_mesh.cpp` |
| 🟡 | CRDT offline-first sync | `net/sigma_offline_sync.cpp` |

---

## `release/dual-boot` — Dual-Boot Coexistence

**Blocked by:** `kernel-exp` sigma-boot.efi + `fs-dev` VFS

| Priority | Task | File |
|----------|------|------|
| 🟡 | EFI boot entry registration | `sigma-boot/sigma_efi_entry.c` |
| 🟡 | Partition layout detector | `userland/installer/sigma_part_detect.cpp` |
| 🟡 | Windows NTFS read-only mount | `fs/ntfs/sigma_ntfs_ro.cpp` |
| 🟡 | GRUB chainload fallback | `sigma-boot/sigma_grub_chain.c` |

---

## `release/rtos` — Hard Real-Time Profile

**Blocked by:** `performance-optimized` EDF + `kernel-exp` IRQ subsystem

| Priority | Task | File |
|----------|------|------|
| 🟡 | EDF scheduler (earliest-deadline-first) | `kernel/sched/sigma_edf.cpp` |
| 🟡 | Priority inheritance protocol | `kernel/sched/sigma_pi.cpp` |
| 🟡 | Bounded IRQ latency (< 10 µs) | `kernel/core/sigma_irq.cpp` |
| 🟡 | ROS 2 DDS middleware port | `runtime/ros2/sigma_ros2_dds.cpp` |

---

## `release/mobile` — ARM64/RISC-V Mobile Profile

**Blocked by:** `kernel-exp` core kernel

| Priority | Task | File |
|----------|------|------|
| 🟠 | ARM64 GIC interrupt controller | `arch/arm64/sigma_gic.cpp` |
| 🟠 | ARM64 MMU page table walker | `arch/arm64/sigma_mmu.cpp` |
| 🟠 | BCM2711 BSP (RPi 4) | `arch/arm64/sigma_bcm2711.cpp` |
| 🟠 | Neon-accelerated Kyber | `crypto/sigma_kyber_neon.cpp` |
| 🟡 | RISC-V PLIC + MMU | `arch/riscv64/sigma_plic.cpp` |

---

## `release/browser` — Browser-Hosted WASM Demo

| Priority | Task | File |
|----------|------|------|
| 🟢 | QEMU-in-browser integration | `sigma-web/sigma_qemu_web.js` |
| 🟢 | Browser demo landing page | `browser/index.html` |
| 🟢 | Service worker for offline demo | `browser/sw.js` |

---

## `release/app` — App Store Demo

| Priority | Task | File |
|----------|------|------|
| 🟢 | App store UI update | `app_store.html` |
| 🟢 | sigma-pkg web API integration | `sigma-web/` |
| 🟢 | Profession app card gallery | `app_store.html` |

---

## Recommended Merge Order

```

1. kernel-exp → main          (Phase 0: boot + QEMU CI)

2. drivers-dev → main         (VESA + e1000 + VirtIO-GPU)

3. fs-dev → main              (VFS + tmpfs + SigmaFS)

4. tools-dev → main           (ongoing, every green CI)

5. performance-optimized → main (after kernel-exp)

6. docs-update → main         (ongoing)

7. release/microkernel ← main (branch from stable main)

8. release/standalone ← main  (after drivers-dev GPU)

9. release/cloud ← main       (after cgroup enforcement)

10. release/mobile ← main      (after ARM64 BSP)

11. release/rtos ← main        (after EDF scheduler)

12. release/dual-boot ← main   (after sigma-boot.efi)

13. release/distributed ← release/cloud

14. release/browser / release/app ← main (after bootable ISO)
```

---

## Version Timeline

| Version | Codename | Target Date | Key Milestone |
|---------|----------|-------------|---------------|
| v15.0.0 | Zenith | May 2026 | ✅ Released — PQC, AI sched, KMS framework |
| v15.1.0 | Zenith LTS | Aug 2026 | Launch checklist, docs, gh-pages polish |
| v16.0.0 | Apex | Q1 2027 | Bootable ISO, real kernel, drivers, desktop |
| v17.0.0 | Sovereign | Q3 2027 | Cloud, mobile, RTOS, distributed, dual-boot |
| v18.0.0 | Transcendence | Q1 2028 | India Stack live, LLM backend, formal verification |

---

*See also: [CURRENT_PROBLEMS_MANIFEST.md](CURRENT_PROBLEMS_MANIFEST.md) · [FEATURE_MATRIX.md](FEATURE_MATRIX.md) · [CONTRIBUTOR_ROADMAP.md](CONTRIBUTOR_ROADMAP.md) · [STRATEGIC_VISION.md](STRATEGIC_VISION.md)*
