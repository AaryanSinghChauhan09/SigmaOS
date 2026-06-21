# SigmaOS: Active Problems Manifest

*Last updated: Phase E gap-closing pass. Tracks all known issues across subsystems.*

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

## High Priority (Kernel/Core)

- **[#1132] ~~Ext4 Journal Checkpointing~~** ✅ FIXED: JBD2 journal rewritten with CRC32C checksums, real descriptor/commit/revoke blocks, checkpoint flushing, and crash replay.
- **[#1133] ~~VFS / Block Dev Integration~~** ✅ FIXED: `fs/ext4.c` block reads now correctly bind to NVMe/SATA AHCI drivers via `sigma_nvme_read`.
- **[#1134] ~~Memory Fragmentation~~** ✅ FIXED: Buddy allocator in `sigma_libc_impl.c` with coalescing.
- **[#1135] ~~TCP/ARP completion~~** ✅ FIXED: TCP state machine now sends SYN-ACK and ACK packets, ARP resolution sends requests.

## Medium Priority (Drivers/HAL)

- **[#844] Native KMS/GPU:** UEFI/VESA fallback works; AMD/Intel KMS drivers needed for accelerated Zenith compositing.
- **[#850] PCIe MSI-X:** Interrupt vector drops on NUMA under heavy network I/O.
- **[#851] ~~Wi-Fi / Bluetooth~~** ⚠️ USB xHCI driver now provides transport; WLAN/BT protocol stacks pending.
- **[#852] ~~NVMe Storage~~** ✅ FIXED: Full NVMe 1.4 driver with admin/IO queue pairs.
- **[#853] ~~USB Host Controller~~** ✅ FIXED: xHCI driver with port scanning and speed detection.

## Medium Priority (System Services)

- **[#900] Cgroup enforcement:** Native pod spec stored; CPU/mem/io limits need kernel cgroup shard binding.
- **[#901] Sovereign package registry:** Reproducible `.spkg` registry + community recipe pipeline.
- **[#902] ~~Power Management~~** ✅ FIXED: ACPI S0-S5 states, CPU P/C-states, thermal throttling, battery monitoring.
- **[#903] ~~Crash Reporting~~** ✅ FIXED: Full crash reporter with register dumps, stack traces, CRC32 checksums.

## Low Priority (Userland/UI)

- **[#512] Zenith Compositor:** C++ compositor exists; JS prototypes remain in `web_ui/` — consolidate on native path.
- **[#520] Shell Globbing:** `sigma-sh` wildcard expansion still missing.
- **[#521] Recovery GUI:** Resilient serial shell exists; Rescuezilla-style GUI recovery not implemented.
- **[#522] Auto-tiling polish:** `sigma_tiling_wm.cpp` implemented; needs input/compositor integration tests.

## Documentation / Community

- **[#600] Wiki depth:** Phase C/D/E docs added; keep `wiki_repo/` synced.
- **[#601] Branch matrix docs:** Document `release/*` branch expectations.

---

*Found a bug? Open an issue with subsystem label (`net`, `boot`, `zenith`, `orchestrator`) and link to checklist item.*
