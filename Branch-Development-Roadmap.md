# SigmaOS — Per-Branch Development Roadmap

Detailed, file-level development plan for every active GitHub branch.
Each branch has a defined purpose, current state, concrete next tasks,
exit criteria, and merge target.

**Repo:** https://github.com/AaryanSinghChauhan09/SigmaOS
**Current working branch:** `tools-dev`

---

## Branch Map

| Branch | Purpose | Target release | Priority |
|--------|---------|----------------|----------|
| `main` | Stable integration target | v15.x current | 🔴 Critical |
| `master` | Legacy mirror of main | deprecated | — |
| `kernel-exp` | Kernel implementation lab | v16.0 Apex | 🔴 Critical |
| `drivers-dev` | SDF driver development | v16.0 Apex | 🔴 Critical |
| `fs-dev` | Filesystem work | v16.0 Apex | 🟠 High |
| `tools-dev` | CLI, docs, automation | v15.x ongoing | 🟠 High |
| `performance-optimized` | Scheduler + PGO tuning | v16.0 Apex | 🟠 High |
| `docs-update` | Wiki and API docs | v15.x ongoing | 🟡 Medium |
| `prepare-sigmaos-launch` | Launch checklist | v15.1 | 🟡 Medium |
| `gh-pages` | GitHub Pages website | live | 🟡 Medium |
| `release/standalone` | Full desktop profile | v15.1 | 🟠 High |
| `release/microkernel` | Minimal microkernel | v16.0 Apex | 🔴 Critical |
| `release/cloud` | Cloud/container profile | v17.0 Sovereign | 🟠 High |
| `release/distributed` | Multi-node cluster | v17.0 Sovereign | 🟡 Medium |
| `release/dual-boot` | Dual-boot coexistence | v16.0 Apex | 🟡 Medium |
| `release/rtos` | Real-time profile | v17.0 Sovereign | 🟡 Medium |
| `release/mobile` | ARM64 mobile profile | v17.0 Sovereign | 🟡 Medium |
| `release/browser` | Browser-hosted demo | v15.1 | 🟢 Low |
| `release/app` | App store demo | v15.1 | 🟢 Low |

---

## `main` — Stable Integration Target

**Purpose:** Accepted, tested work only. No direct development.
Every PR to `main` must pass `sigma_ci.yml` smoke tests.

### Current state
- Last merged: `tools-dev` with docs + compat headers
- CI: GitHub Actions running on push
- Wiki sync: automatic on push

### Next tasks for maintainers

| Task | File | Priority |
|------|------|----------|
| Wire QEMU boot test to CI | `.github/workflows/sigma_ci.yml` | 🔴 |
| Add `make check-abi` gate | `Makefile` | 🟠 |
| Enforce CURRENT_PROBLEMS_MANIFEST update on PR | `.github/PULL_REQUEST_TEMPLATE.md` | 🟠 |
| Add `sigma_wine_ci.yml` workflow | `.github/workflows/sigma_wine_ci.yml` | 🟡 |
| Merge `kernel-exp` once Phase 0 complete | — | 🔴 |

### Exit criteria
- `make iso` produces bootable image (blocked on `kernel-exp`)
- All CI checks green on every commit

---

## `kernel-exp` — Kernel Implementation Lab

**Purpose:** The most critical branch. Implements the real kernel bodies
that turn SigmaOS from a design document into a bootable OS.
**Everything else is blocked until this branch ships Phase 0.**

### Current state
- Headers complete for scheduler, MM, syscall, IRQ
- Bodies missing — all `.cpp` implementations are stubs

### Next tasks (ordered — do not skip ahead)

| # | Task | File | Exit test |
|---|------|------|-----------|
| 1 | Round-robin scheduler (64 tasks) | `kernel/core/sigma_sched.cpp` | QEMU: 2 tasks interleave |
| 2 | Buddy physical allocator | `kernel/core/sigma_mm.cpp` | alloc/free 100 pages, no leak |
| 3 | Slab allocator (kmalloc) | `kernel/core/sigma_mm.cpp` | alloc/free 10000 objects |
| 4 | x86-64 page table walker | `kernel/mm/sigma_vmm.cpp` | map 1 MB region, read back |
| 5 | APIC + PIC init | `kernel/core/sigma_irq.cpp` | timer IRQ fires in QEMU |
| 6 | HPET/APIC timer → jiffies | `kernel/core/sigma_timer.cpp` | `sleep(100ms)` works |
| 7 | 30-syscall dispatch table | `kernel/core/sigma_syscall_dispatch.cpp` | `write(1,"hi\n",3)` from userland |
| 8 | VESA/GOP framebuffer | `drivers/display/sigma_vesa.cpp` | pixels appear in QEMU |
| 9 | sigma-boot.efi UEFI loader | `sigma-boot/sigma_boot.c` | QEMU boots to kernel |
| 10 | `make iso` → bootable ISO | `Makefile` | `qemu -cdrom SigmaOS.iso` → shell |

### Upgrade path: scheduler
```
Round 1: round-robin (simplest, unblock boot)
Round 2: MLFQ (4 queues, aging)
Round 3: CFS clone (vruntime, red-black tree runqueue)
Round 4: NUMA-aware placement (reads ACPI SRAT)
Round 5: EDF (earliest-deadline-first for RT tasks)
Round 6: sigma-ai predictive pre-warming
```

### Merge target
`main` after `make iso` + QEMU boot CI passing.

---

## `drivers-dev` — SDF Driver Development

**Purpose:** All hardware drivers using the Sovereign Driver Framework.
Depends on `kernel-exp` Phase 0 for the SDF Ring-3 launch mechanism.

### Current state
- NVMe driver: `drivers/storage/sigma_nvme.cpp` ✅
- USB xHCI: `drivers/usb/sigma_xhci.cpp` ✅
- e1000 NIC: `kernel/core/drivers/SovereignE1000.cpp` ✅
- KMS/GPU: `drivers/graphics/sigma_kms.cpp` — framework only
- Wi-Fi: not started

### Next tasks

| Priority | Task | File | Target HW |
|----------|------|------|-----------|
| 🔴 | VESA/GOP framebuffer SDF driver | `drivers/display/sigma_vesa_sdf.cpp` | All UEFI |
| 🔴 | VirtIO-GPU (QEMU accelerated) | `drivers/display/sigma_virtio_gpu.cpp` | QEMU |
| 🔴 | DRM/KMS layer | `drivers/graphics/sigma_kms.cpp` | All GPU |
| 🔴 | Intel i915 basic modesetting | `drivers/graphics/sigma_i915.cpp` | Intel iGPU |
| 🟠 | AMD amdgpu basic modesetting | `drivers/graphics/sigma_amdgpu.cpp` | AMD |
| 🟠 | Intel iwlwifi 802.11ax | `drivers/net/sigma_iwlwifi.cpp` | Intel Wi-Fi 6 |
| 🟠 | MediaTek mt7921 Wi-Fi | `drivers/net/sigma_mt7921.cpp` | JioBook |
| 🟠 | Realtek rtl8xxxu USB Wi-Fi | `drivers/net/sigma_rtl8xxxu.cpp` | USB dongles |
| 🟠 | HDA audio controller | `drivers/audio/sigma_hda.cpp` | All x86 |
| 🟡 | Bluetooth HCI (USB) | `drivers/bt/sigma_hci_usb.cpp` | USB BT adapters |
| 🟡 | ARM64 BCM2711 (RPi 4) BSP | `arch/arm64/sigma_bcm2711.cpp` | Raspberry Pi 4 |
| 🟡 | ARM64 BCM2712 (RPi 5) BSP | `arch/arm64/sigma_bcm2712.cpp` | Raspberry Pi 5 |

### SDF driver template
Every new driver follows this pattern:
```cpp
// drivers/subsystem/sigma_mydriver.cpp
class SigmaMyDriver : public SovereignDriverBase {
public:
    sigma_status probe(SigmaDeviceInfo* dev) override;  // detect hardware
    sigma_status init() override;                        // allocate resources
    sigma_status shutdown() override;                    // release resources
    // device-specific ops follow
};
SIGMA_SDF_REGISTER_DRIVER(SigmaMyDriver, "my_driver", VendorID, DeviceID);
```

### Merge target
`main` per-driver, after passing QEMU + physical hardware CI.

---

## `fs-dev` — Filesystem Development

**Purpose:** VFS layer, SigmaFS native filesystem, Ext4 integration,
and the unified buffer cache.

### Current state
- VFS header: `kernel/include/kernel/sigma_vfs.h` ✅
- Ext4 journal rewrite: `fs/ext4_journal.c` ✅
- SigmaFS: early design
- UBC (Unified Buffer Cache): `kernel/fs/sigma_ubc.h` — header only

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🔴 | VFS open/read/write/close bodies | `kernel/vfs/sigma_vfs.cpp` |
| 🔴 | Tmpfs (RAM-backed filesystem) | `kernel/vfs/sigma_tmpfs.cpp` |
| 🟠 | SigmaFS mkfs + mount | `fs/sigmafs/sigma_mkfs.cpp` |
| 🟠 | SigmaFS directory + file ops | `fs/sigmafs/sigma_fs_ops.cpp` |
| 🟠 | Ext4 read-only mount | `fs/ext4/sigma_ext4_ro.cpp` |
| 🟠 | UBC implementation | `kernel/fs/sigma_ubc.cpp` |
| 🟠 | Pre-emptive read-ahead | `kernel/fs/sigma_readahead.cpp` |
| 🟡 | dm-verity block verifier | `kernel/fs/sigma_dmverity.cpp` |
| 🟡 | Relativistic journaling | `kernel/fs/sigmafs/sigma_journal.cpp` |
| 🟡 | FAT32 write support | `fs/fat/sigma_fat32_write.cpp` |

### Exit criteria
- `sigma-sh` can `open`, `read`, `write`, `ls` on tmpfs
- `sigma-pkg install` writes packages to SigmaFS partition

---

## `tools-dev` — CLI, Docs, Automation

**Purpose:** sigma-cli, sigma-sh, sigma_automation.sh, documentation,
wiki sync, compat layer headers and stubs.
*This is the current working branch.*

### Current state ✅
- sigma-cli profiles/aliases: `userland/tools/sigma_cli.cpp`
- sigma_automation.sh: `scripts/sigma_automation.sh`
- sigma_git_sync.sh: `scripts/sigma_git_sync.sh`
- ci_branch_check.sh: `scripts/ci_branch_check.sh`
- Windows compat headers: `include/compat/*.h` (5 headers complete)
- Windows compat skeletons: `runtime/compat/win32/*.cpp` (5 files complete)

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🟠 | sigma-sh env vars + history | `userland/shell/sigma_shell.cpp` |
| 🟠 | sigma-sh tab completion | `userland/shell/sigma_shell.cpp` |
| 🟠 | sigma_ntdll registry stubs | `runtime/compat/win32/registry/sigma_reg.cpp` |
| 🟠 | sigma-kernel32 file I/O | `runtime/compat/win32/kernel32/sigma_kernel32_file.cpp` |
| 🟠 | sigma-kernel32 process | `runtime/compat/win32/kernel32/sigma_kernel32_process.cpp` |
| 🟠 | sigma-msvcrt printf + malloc | `runtime/compat/win32/crt/sigma_msvcrt.cpp` |
| 🟡 | sigma-wine CI workflow | `.github/workflows/sigma_wine_ci.yml` |
| 🟡 | Man pages for sigma-cli tools | `docs/man/` |
| 🟡 | sigma-observatory stub | `userland/tools/sigma_observatory.cpp` |

### Merge target
`main` on every green CI run. Ongoing.

---

## `performance-optimized` — Scheduler + PGO Tuning

**Purpose:** Silicon-aware performance improvements. Depends on
`kernel-exp` baseline scheduler being merged first.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🔴 | NUMA topology reader (ACPI SRAT) | `kernel/sched/sigma_numa.cpp` |
| 🔴 | Lock-free CAS runqueue | `klib/sigma_lockfree.h` |
| 🟠 | CFS vruntime red-black tree | `kernel/sched/sigma_cfs.cpp` |
| 🟠 | AVX-512 Kyber-1024 NTT | `crypto/sigma_kyber_avx512.cpp` |
| 🟠 | ARM NEON Kyber NTT | `crypto/sigma_kyber_neon.cpp` |
| 🟠 | PGO Makefile targets | `Makefile` — `make PROFILE=pgo iso` |
| 🟠 | ACPI P-state governor | `kernel/power/sigma_perf_governor.cpp` |
| 🟡 | io_uring equivalent ring | `kernel/io/sigma_uring.cpp` |
| 🟡 | Vectorized matrix scaling (SIMD) | `zenith_desktop/sigma_simd_scale.cpp` |
| 🟡 | sigma-ai predictive scheduler | `kernel/sched/sigma_ai_sched.cpp` |

### Benchmark targets (exit criteria)

| Metric | Target |
|--------|--------|
| Context switch | < 50 ns |
| Kyber-1024 (AVX-512) | ≥ 5.8 M ops/sec |
| Boot time (NVMe) | < 2 s |
| Idle RAM (desktop) | < 150 MB |

---

## `docs-update` — Wiki and API Docs

**Purpose:** Doxygen API reference, wiki page maintenance, man pages.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🟠 | Wire Doxygen to CI | `.github/workflows/sigma_ci.yml` |
| 🟠 | Man pages (50 tools) | `docs/man/*.1` |
| 🟠 | Kernel developer handbook update | `wiki_repo/Kernel-Developer-Handbook.md` |
| 🟡 | Windows Compat Layer wiki page | `wiki_repo/Windows-Compatibility-Layer-Roadmap.md` ✅ |
| 🟡 | Per-branch roadmap wiki page | `wiki_repo/Branch-Development-Roadmap.md` ✅ |
| 🟡 | Auto-generate subsystem stubs from Doxygen | `scripts/doxygen_wiki_export.sh` |

---

## `prepare-sigmaos-launch` — v15.1 Launch Checklist

**Purpose:** Final pre-launch hardening for v15.1 release.

### Checklist

| Task | Status |
|------|--------|
| FEATURE_MATRIX.md up to date | `[x]` |
| All `release/*` branches at parity | `[~]` |
| CURRENT_PROBLEMS_MANIFEST.md reflects Phase G | `[x]` |
| sigma_automation.sh backup tested | `[x]` |
| sigma_git_sync.sh wiki sync tested | `[x]` |
| Windows compat headers committed | `[x]` |
| sigma-wine-loader skeleton committed | `[x]` |
| QEMU boot CI green | `[ ]` blocked on kernel-exp |
| Release notes written | `[ ]` |
| GitHub release tag created | `[ ]` |

---

## `gh-pages` — GitHub Pages Website

**Purpose:** Public-facing website at `aaryansinghchauhan09.github.io/SigmaOS`.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🟠 | Update roadmap.html with Phase 0 progress | `roadmap.html` |
| 🟠 | Add Windows compat layer section | `index.html` |
| 🟠 | Update version timeline widget | `site.js` |
| 🟡 | Add interactive branch status dashboard | `site.js` |
| 🟡 | Dark/light theme CSS fix | `site.css` |

---

## `release/standalone` — Full Desktop Profile

**Purpose:** The primary user-facing profile: Zenith desktop + all
profession apps + sigma-ai local LLM.

### Next tasks

| Priority | Task | Blocked by |
|----------|------|-----------|
| 🔴 | GPU DRM/KMS + Zenith on real framebuffer | `drivers-dev` i915/amdgpu |
| 🔴 | Compositor input event loop | `kernel-exp` VMM |
| 🟠 | Auto-tiling WM complete | `tools-dev` |
| 🟠 | `~/.sigma_profile` VFS load | `fs-dev` |
| 🟠 | DID-based login screen | `kernel-exp` + sigma-trustd |
| 🟠 | sigma-ai LLM daemon (llama.cpp) | `kernel-exp` boot |
| 🟠 | Indian IME (Inscript + phonetic) | `kernel-exp` + sigma-bhashini |
| 🟡 | sigma-bhashini offline bundle | package repo |
| 🟡 | Accessibility: Braille + switch access | Phase B |

### Exit criteria
- First-time user boots → Zenith desktop → types Hindi → runs sigma-ai

---

## `release/microkernel` — Minimal Microkernel Profile

**Purpose:** Smallest possible SigmaOS: boot + syscalls + sigma-bus IPC.
Target: under 512 KB kernel image, under 8 MB RAM footprint.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🔴 | Minimal scheduler (round-robin only) | `kernel/core/sigma_sched.cpp` |
| 🔴 | Minimal MM (no slab, just buddy) | `kernel/core/sigma_mm.cpp` |
| 🔴 | 15 essential syscalls only | `kernel/core/sigma_syscall_dispatch.cpp` |
| 🟠 | sigma-bus IPC end-to-end | `kernel/ipc/sigma_bus.cpp` |
| 🟠 | Capability token passing | `kernel/security/sigma_caps.cpp` |
| 🟡 | Formal verification proofs | Frama-C / Coq |

### Exit criteria
- Boots in QEMU with 8 MB RAM
- sigma-bus ping-pong IPC working
- No userland except sigma-sh

---

## `release/cloud` — Cloud / Container Profile

**Purpose:** Container-first, no GUI. For sovereign cloud deployments,
BharatOS government machines, and sigma-fleet managed nodes.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🔴 | sigma-pod kernel cgroup enforcement | `kernel/core/process/sigma_cgroup.c` |
| 🔴 | sigma-pod kernel namespace creation | `kernel/core/process/sigma_namespace.cpp` |
| 🟠 | SovereignContainer KVM hypervisor | `kernel/hypervisor/sigma_kvm.cpp` |
| 🟠 | dm-verity .spkg image verification | `kernel/fs/sigma_dmverity.cpp` |
| 🟠 | sigma-fleet agent (MDM) | `userland/tools/sigma_fleet_agent.cpp` |
| 🟠 | OpenTelemetry sigma-ids export | `userland/sigma_otel_export.cpp` |
| 🟡 | gRPC management API | `api/sigma.proto` + codegen |
| 🟡 | SovereignCloudFS | `net/sigma_cloudfs.cpp` |

### Exit criteria
- `sigma-pod run-native demo.spkg --cpu=250 --mem=128` enforces limits in kernel
- sigma-fleet agent registers with management console

---

## `release/distributed` — Multi-Node Cluster

**Purpose:** Distributed computing across SigmaOS nodes — sigma-mesh-compute
and SovereignCloudFS for national distributed grid vision.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🟡 | SovereignCloudFS multi-node sync | `net/sigma_cloudfs.cpp` |
| 🟡 | sigma-mesh-compute scheduler | `net/sigma_mesh.cpp` |
| 🟡 | sigma-blockchain-lite DLT | `net/sigma_blockchain.cpp` |
| 🟡 | CRDT offline-first sync | `net/sigma_offline_sync.cpp` |
| 🟡 | sigma-zkvm (ZK virtual machine) | `runtime/zkvm/sigma_zkvm.cpp` |

### Blocked by
`release/cloud` cgroup + namespace enforcement must be complete first.

---

## `release/dual-boot` — Dual-Boot Coexistence

**Purpose:** Install SigmaOS alongside Windows/Linux without destroying
existing data. EFI dual-boot configuration, partition detection.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🟡 | EFI boot entry registration | `sigma-boot/sigma_efi_entry.c` |
| 🟡 | Partition layout detector | `userland/installer/sigma_part_detect.cpp` |
| 🟡 | Windows NTFS read-only mount | `fs/ntfs/sigma_ntfs_ro.cpp` |
| 🟡 | GRUB chainload fallback | `sigma-boot/sigma_grub_chain.c` |
| 🟡 | Installer UI | `userland/installer/sigma_installer_ui.cpp` |

### Blocked by
`kernel-exp` bootloader (`sigma-boot.efi`) and `fs-dev` VFS.

---

## `release/rtos` — Real-Time Profile

**Purpose:** Hard real-time guarantees for industrial, robotics, and
embedded control applications (sigma-robotics, sigma-digital-twin).

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🟡 | EDF scheduler (earliest-deadline-first) | `kernel/sched/sigma_edf.cpp` |
| 🟡 | Priority inheritance protocol | `kernel/sched/sigma_pi.cpp` |
| 🟡 | Bounded IRQ latency (< 10 µs) | `kernel/core/sigma_irq.cpp` |
| 🟡 | ROS 2 DDS middleware port | `runtime/ros2/sigma_ros2_dds.cpp` |
| 🟡 | sigma-twin real IoT sensor path | `userland/twin/sigma_twin_iot.cpp` |
| 🟡 | PREEMPT_RT-style full preemption | `kernel/core/sigma_sched.cpp` |

### Blocked by
`performance-optimized` scheduler work and `kernel-exp` IRQ subsystem.

---

## `release/mobile` — ARM64 Mobile Profile

**Purpose:** SigmaOS on ARM64 devices — Raspberry Pi 4/5, JioBook,
sigma-ultra USSD mode on Pi Zero.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🟠 | ARM64 GIC interrupt controller | `arch/arm64/sigma_gic.cpp` |
| 🟠 | ARM64 MMU page table walker | `arch/arm64/sigma_mmu.cpp` |
| 🟠 | BCM2711 BSP (RPi 4) | `arch/arm64/sigma_bcm2711.cpp` |
| 🟠 | BCM2712 BSP (RPi 5) | `arch/arm64/sigma_bcm2712.cpp` |
| 🟠 | Neon-accelerated Kyber | `crypto/sigma_kyber_neon.cpp` |
| 🟠 | sigma-ultra USSD text mode | `userland/sigma_ultra.cpp` |
| 🟡 | RISC-V PLIC + MMU | `arch/riscv64/sigma_plic.cpp` |
| 🟡 | JioPhone KaiOS-compat build | `profiles/sigma_jio.config` |

### Blocked by
`kernel-exp` core kernel — the ARM64 port layers on top of the same MM/sched.

---

## `release/browser` — Browser-Hosted Demo

**Purpose:** Run SigmaOS inside a browser using WASM/QEMU-in-browser for
demos at conferences and on the website.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🟢 | QEMU-in-browser integration | `sigma-web/sigma_qemu_web.js` |
| 🟢 | sigma-web API 24-driver update | `sigma-web/` |
| 🟢 | Browser demo landing page | `browser/index.html` |
| 🟢 | Service worker for offline demo | `browser/sw.js` |

### Blocked by
`kernel-exp` producing a bootable ISO.

---

## `release/app` — App Store Demo

**Purpose:** Demonstrates the SigmaOS app ecosystem — app store UI,
sigma-pkg install flow, and profession app cards.

### Next tasks

| Priority | Task | File |
|----------|------|------|
| 🟢 | App store UI update | `app_store.html` |
| 🟢 | sigma-pkg web API integration | `sigma-web/` |
| 🟢 | Profession app card gallery | `app_store.html` |
| 🟢 | App install simulator | `site.js` |

---

## Cross-Branch Dependencies (Critical Path)

```
kernel-exp (Phase 0 kernel)
    ├── → drivers-dev (SDF driver launch mechanism)
    │       ├── → release/standalone (GPU + Wi-Fi needed)
    │       └── → release/mobile (ARM64 BSP)
    ├── → fs-dev (VFS layer)
    │       ├── → release/standalone (profile engine VFS load)
    │       └── → release/cloud (dm-verity)
    ├── → release/microkernel (minimal kernel subset)
    └── → tools-dev/release/browser (need bootable ISO)

performance-optimized
    └── depends on kernel-exp scheduler being merged first

release/cloud
    └── depends on kernel-exp cgroup + namespace in kernel path

release/distributed
    └── depends on release/cloud

release/rtos
    └── depends on performance-optimized EDF + kernel-exp IRQ

release/dual-boot
    └── depends on kernel-exp sigma-boot.efi + fs-dev
```

---

## Merge Order (Recommended)

```
1. kernel-exp → main          (Phase 0: boot + QEMU CI passing)
2. drivers-dev → main         (VESA + e1000 + VirtIO-GPU)
3. fs-dev → main              (VFS + tmpfs + basic SigmaFS)
4. tools-dev → main           (ongoing — after each green CI)
5. performance-optimized → main (after kernel-exp merged)
6. release/microkernel ← main (branch from stable main)
7. release/standalone ← main  (after drivers-dev GPU merged)
8. release/cloud ← main       (after cgroup enforcement)
9. release/mobile ← main      (after ARM64 BSP done)
10. release/rtos ← main       (after EDF scheduler)
11. release/dual-boot ← main  (after sigma-boot.efi)
12. release/distributed ← release/cloud
```

---

*See also: [Windows Compatibility Layer Roadmap](Windows-Compatibility-Layer-Roadmap) · [Development Roadmap](Development-Roadmap) · [Gap Analysis](Gap-Analysis) · [Feature Matrix](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_MATRIX.md) · [Phase A Execution Checklist](Phase-A-Execution-Checklist)*
