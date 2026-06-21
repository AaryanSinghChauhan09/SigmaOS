# SigmaOS: Active Problems Manifest

*Last updated: Phase F — Competitive Strategy integration. All Phase E targets resolved.*

## ✅ Resolved — Phase E (Gap-Closing)

| ID | Area | Status | File |
|----|------|--------|------|
| E-01 | NVMe Driver | **Resolved** | `drivers/storage/sigma_nvme.cpp` |
| E-02 | USB xHCI Driver | **Resolved** | `drivers/usb/sigma_xhci.cpp` |
| E-03 | Power Management (ACPI) | **Resolved** | `kernel/power/sigma_power_manager.cpp` |
| E-04 | Crash Reporter | **Resolved** | `kernel/diagnostics/sigma_crash_reporter.cpp` |
| E-05 | Ext4 Journal (JBD2 rewrite) | **Resolved** | `fs/ext4_journal.c` |

## ✅ Resolved — Phase C/D

| ID | Area | Status | File |
|----|------|--------|------|
| C-01 | Post-Quantum Crypto (Kyber KEM) | **Resolved** | `crypto/SovereignKyber.cpp` |
| C-02 | Post-Quantum Crypto (Dilithium-5) | **Resolved** | `crypto/SovereignDilithium5.cpp` |
| C-03 | PQC Type Header | **Resolved** | `include/crypto/sigma_pqc.h` |
| C-04 | Immutable Audit Trail | **Resolved** | `kernel/security/sigma_immutable_audit_trail.cpp` |
| C-05 | Adaptive Zero-Trust Engine | **Resolved** | `kernel/security/sigma_adaptive_zerotrust.cpp` |
| C-06 | Neural UI (AVX-512) | **Resolved** | `zenith_desktop/neural/sigma_neural_ui.cpp` |
| D-01 | WASM/WASI Runtime | **Resolved** | `runtime/wasm/sigma_wasm_runtime.cpp` |
| D-02 | Linux ELF Compat Layer | **Resolved** | `runtime/containers/sigma_linux_compat.cpp` |

## Resolved / In Progress (Phase A)

| ID | Area | Status | Notes |
|----|------|--------|-------|
| P-A1 | Networking TX | **Resolved** | `nic_tx_packet` wired to `SovereignE1000` |
| P-A2 | Networking RX | **Resolved** | `nic_rx_deliver` → `sigma_net_receive_frame` |
| P-A3 | Socket syscall | **Resolved** | `SIGMA_SYS_SOCKET` in syscall gate |
| P-A4 | Safe-mode boot | **Partial** | Rollback gate + resilient fallback in `sigma_kernel_main.c` |
| P-A5 | Native pod spec | **Partial** | `sigma-pod run-native` + orchestrator |
| P-A6 | CLI customization | **Partial** | `userland/tools/sigma_cli.cpp` profiles/aliases |
| P-A7 | Personalization | **Partial** | `zenith_desktop/personalization/sigma_profile_engine.cpp` |
| P-A8 | Automation | **Partial** | `scripts/sigma_automation.sh` |

## ✅ Resolved — Phase F (Competitor Crusher)

| ID | Area | Status | File |
|----|------|--------|------|
| F-01 | Native KMS/GPU (#844) | **Resolved** | `drivers/graphics/sigma_kms.cpp` |
| F-02 | PCIe MSI-X HAL (#850) | **Resolved** | `hal/sigma_pci.cpp` |
| F-03 | Cgroup Enforcement (#900) | **Resolved** | `kernel/core/orchestrator/sigma_cgroup.cpp` |
| F-04 | Sovereign Package Registry (#901) | **Resolved** | `userland/pkg/sigma_registry.cpp` |
| F-05 | Offline-First CRDT Sync | **Resolved** | `net/sigma_offline_sync.cpp` |
| F-06 | Native Performance Governor | **Resolved** | `kernel/power/sigma_perf_governor.cpp` |
| F-07 | HAL Boot Integration (PCI+KMS+PerfGov) | **Resolved** | `hal/SovereignHAL.cpp` |

## ✅ Resolved — Phase E (Gap-Closing)

| ID | Area | Status | File |
|----|------|--------|------|
| E-01 | NVMe Driver | **Resolved** | `drivers/storage/sigma_nvme.cpp` |
| E-02 | USB xHCI Driver | **Resolved** | `drivers/usb/sigma_xhci.cpp` |
| E-03 | Power Management (ACPI) | **Resolved** | `kernel/power/sigma_power_manager.cpp` |
| E-04 | Crash Reporter | **Resolved** | `kernel/diagnostics/sigma_crash_reporter.cpp` |
| E-05 | Ext4 Journal (JBD2 rewrite) | **Resolved** | `fs/ext4_journal.c` |

## Open — Low Priority (Userland/UI)

- **[#512] Zenith Compositor:** C++ compositor exists; JS prototypes remain in `web_ui/` — consolidate on native path.
- **[#520] Shell Globbing:** `sigma-sh` wildcard expansion still missing.
- **[#521] Recovery GUI:** Resilient serial shell exists; Rescuezilla-style GUI recovery not implemented.
- **[#522] Auto-tiling polish:** `sigma_tiling_wm.cpp` implemented; needs input/compositor integration tests.

## Open — Phase G (Next Targets)

- **[#851-WLAN] Wi-Fi Stack:** IEEE 802.11ax (Wi-Fi 6) driver over xHCI transport.
- **[#851-BT] Bluetooth 5.3 Stack:** HCI/L2CAP/RFCOMM over USB transport.
- **[#1000] Developer SDK:** sigma-sdk CLI, debugger (sigma-gdb), profiler (sigma-perf trace).
- **[#1001] App Sandbox:** Fine-grained capability system (sandboxctl) for third-party apps.
- **[#1002] Multi-monitor KMS:** Extended desktop and clone mode across multiple KMS adapters.

## Documentation / Community

- **[#600] Wiki depth:** Phase F docs to be added to `wiki_repo/` — offline sync, perf governor.
- **[#601] Branch matrix docs:** Document `release/*` branch expectations.

---

*Found a bug? Open an issue with subsystem label (`net`, `boot`, `zenith`, `orchestrator`) and link to checklist item.*
