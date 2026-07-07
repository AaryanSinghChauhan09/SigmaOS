# SigmaOS Feature Matrix

> SigmaOS v15.0 "Zenith" — Last Updated: 2026-07-06

## Status Legend

| Symbol | Meaning |
|---|---|
| ✅ | Implemented & tested |
| 🔧 | Implemented (stub/prototype) |
| 🚧 | In progress |
| ❌ | Not started |
| N/A | Not applicable to this profile |

---

## Core Kernel

| Feature | Status | File |
|---|---|---|
| MLFQ→CFS→EDF Scheduler | ✅ | `kernel/sched/sigma_sched.rs` |
| x86-64 Page Table Walker | ✅ | `kernel/mm/sigma_vmm.rs` |
| ASLR | ✅ | `kernel/mm/sigma_vmm.rs` |
| W^X Enforcement | ✅ | `kernel/mm/sigma_vmm.rs` |
| APIC/PIC IRQ Controller | ✅ | `kernel/core/sigma_irq.rs` |
| 30-Syscall Table | ✅ | `kernel/core/sigma_syscall_dispatch.rs` |
| Pledge Security Gates | ✅ | `kernel/core/sigma_syscall_dispatch.rs` |
| VFS (open/read/write/close) | ✅ | `kernel/vfs/sigma_vfs.rs` |
| TmpFS | ✅ | `kernel/vfs/sigma_tmpfs.rs` |
| Unified Buffer Cache | ✅ | `klib/sigma_ubc.rs` |
| Buddy Allocator | 🔧 | `kernel/mm/buddy_allocator.rs` |
| SigmaFS (mkfs) | ✅ | `fs/sigmafs/sigma_mkfs.rs` |
| Btrfs Snapshots/Rollback | 🔧 | `fs/btrfs/sigma_btrfs.rs` |
| UEFI EFI Boot Entry | ✅ | `sigma-boot/sigma_efi_entry.c` |

---

## Hardware Drivers

| Driver | Status | File |
|---|---|---|
| VirtIO GPU | ✅ | `kernel/drivers/gpu/sigma_virtio_gpu.rs` |
| Intel i915 (Gen9+) | ✅ | `kernel/drivers/gpu/sigma_i915.rs` |
| AMD GPU | 🔧 | `kernel/drivers/gpu/sigma_amdgpu.rs` |
| Intel Wi-Fi 6 (AX200/210) | ✅ | `kernel/drivers/net/sigma_iwlwifi.rs` |
| Realtek USB Wi-Fi | 🔧 | `kernel/drivers/net/sigma_rtl8xxxu.rs` |
| Intel HDA Audio | ✅ | `kernel/drivers/audio/sigma_hda.rs` |
| USB Bluetooth HCI | 🔧 | `kernel/drivers/bt/sigma_hci_usb.rs` |
| ARM GIC v2 | ✅ | `arch/arm64/sigma_gic.rs` |
| BCM2711 (RPi 4) | ✅ | `arch/arm64/sigma_bcm2711.rs` |
| USB 3.0 xHCI | ❌ | Planned Phase 2.1 |
| NVMe | ❌ | Planned Phase 2.1 |
| SATA AHCI | ❌ | Planned Phase 2.1 |

---

## Package Ecosystem

| Feature | Status | File |
|---|---|---|
| DPLL SAT Resolver | ✅ | `sigma-pkg/sigma_pkg_core.rs` |
| Generation Rollback | ✅ | `sigma-pkg/sigma_pkg_core.rs` |
| Dilithium-5 Verification | 🔧 | `sigma-pkg/sigma_pkg_core.rs` |
| Signed Repo + Mirrors | ✅ | `sigma-pkg/sigma_pkg_repo.rs` |
| GUI Front-end | 🔧 | `sigma-pkg/sigma_pkg_gui.rs` |
| AI Recommender | 🔧 | `sigma-pkg/sigma_pkg_ai_recommender.rs` |

---

## Desktop & UI

| Feature | Status | File |
|---|---|---|
| Theme Store | 🔧 | `zenith_desktop/sigma_theme_store.js` |
| Screen Reader | 🔧 | `zenith_desktop/accessibility/sigma_screen_reader.js` |
| Customization Hub | 🔧 | `zenith_desktop/sigma_customization_hub.html` |
| Indian Language Packs (10) | ✅ | `locales/*.toml` |
| GNOME Compatibility | 🔧 | `desktop/sigma_de_gnome_compat.rs` |
| KDE Compatibility | 🔧 | `desktop/sigma_de_kde_compat.rs` |
| Wayland Compositor (Zenith) | 🚧 | `userland/zenith/` |

---

## Security

| Feature | Status | File |
|---|---|---|
| IDS (Suricata-compat) | 🔧 | `kernel/security/sigma_ids.rs` |
| Fail2Ban | 🔧 | `kernel/security/sigma_fail2ban.rs` |
| Sigma Vault (KVv2 + Transit) | ✅ | `kernel/security/sigma_vault.rs` |
| UEFI Secure Boot | ✅ | `kernel/security/sigma_secboot.rs` |
| MAC (Mandatory Access Control) | 🔧 | `kernel/security/sigma_mac.rs` |
| Seccomp Filter | 🔧 | `kernel/security/sigma_seccomp.rs` |

---

## AI & Automation

| Feature | Status | File |
|---|---|---|
| NL→CLI Translator | ✅ | `agents/sigma_ai_agent.rs` |
| Log Analyzer | ✅ | `agents/sigma_ai_agent.rs` |
| Workflow Engine | ✅ | `agents/sigma_workflow_engine.rs` |
| Adaptive CLI | ✅ | `agents/sigma_adaptive_cli.rs` |
| Error Explainer | ✅ | `agents/sigma_error_explainer.rs` |

---

## Init System

| Feature | Status | File |
|---|---|---|
| Parallel Service Startup | 🔧 | `init/sigma_init.rs` |
| .service Parser | 🔧 | `init/sigma_service.rs` |
| Structured Logging | 🔧 | `init/sigma_journal.rs` |

---

## CI/CD & Tests

| Feature | Status | File |
|---|---|---|
| QEMU Boot CI | ✅ | `.github/workflows/sigma_ci.yml` |
| Driver CI | ✅ | `.github/workflows/sigma_driver_ci.yml` |
| Syscall Tests | 🔧 | `tests/kernel/test_syscall_dispatch.rs` |
| Scheduler Tests | 🔧 | `tests/kernel/test_scheduler.rs` |
| VFS Tests | 🔧 | `tests/fs/test_vfs.rs` |
| Makefile Targets | ✅ | `Makefile` |

---

## Deployment Profile Support

| Profile | Kernel | Desktop | AI | Drivers |
|---|---|---|---|---|
| Desktop | ✅ | ✅ | ✅ | ✅ |
| Server | ✅ | N/A | ✅ | ✅ |
| Edge | ✅ | N/A | 🔧 | 🔧 |
| Embedded | ✅ | N/A | ❌ | 🔧 |
| HPC | ✅ | N/A | ✅ | 🔧 |
| Cloud | ✅ | N/A | ✅ | ✅ |
| IoT | 🚧 | N/A | ❌ | 🚧 |
| Mobile | 🚧 | 🚧 | 🔧 | 🚧 |
