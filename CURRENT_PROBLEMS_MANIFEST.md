# SigmaOS: Active Problems Manifest

*Last updated: Phase C/D implementation pass. Full history in checklists.*

## ✅ Resolved — Phase C/D (this session)

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
| P-A4 | Safe-mode boot | **Partial** | Rollback gate + resilient fallback in `sigma_kernel_main.c`; bootloader UI pending |
| P-A5 | Native pod spec | **Partial** | `sigma-pod run-native` + orchestrator `spawnNativeContainer` |
| P-A6 | CLI customization | **Partial** | `userland/tools/sigma_cli.cpp` profiles/aliases |
| P-A7 | Personalization | **Partial** | `zenith_desktop/personalization/sigma_profile_engine.cpp` |
| P-A8 | Automation | **Partial** | `scripts/sigma_automation.sh`, `scripts/sigma_git_sync.sh` |

## High Priority (Kernel/Core)

- **[#1132] Ext4 Journal Checkpointing:** JBD2 journal (`fs/ext4_journal.c`) needs real commit transaction flushing.
- **[#1133] VFS / Block Dev Integration:** `fs/ext4.c` block reads must bind to NVMe/SATA AHCI drivers.
- **[#1134] Memory Fragmentation:** Buddy allocator rewrite for VMM shard churn.
- **[#1135] TCP/ARP completion:** TCP state machine + ARP resolution still stubbed in net stack.

## Medium Priority (Drivers/HAL)

- **[#844] Native KMS/GPU:** UEFI/VESA fallback works; AMD/Intel KMS drivers needed for accelerated Zenith compositing.
- **[#850] PCIe MSI-X:** Interrupt vector drops on NUMA under heavy network I/O.
- **[#851] Wi-Fi / Bluetooth:** No production sovereign WLAN/BT drivers yet (see `kernel/core/drivers/SovereignWLAN.cpp` stubs).

## Medium Priority (Orchestration / Packages)

- **[#900] Cgroup enforcement:** Native pod spec stored; CPU/mem/io limits need kernel cgroup shard binding.
- **[#901] Sovereign package registry:** Reproducible `.spkg` registry + community recipe pipeline incomplete.

## Low Priority (Userland/UI)

- **[#512] Zenith Compositor:** C++ compositor exists; JS prototypes remain in `web_ui/` — consolidate on native path.
- **[#520] Shell Globbing:** `sigma-sh` wildcard expansion still missing.
- **[#521] Recovery GUI:** Resilient serial shell exists; Rescuezilla-style GUI recovery not implemented.
- **[#522] Auto-tiling polish:** `sigma_tiling_wm.cpp` implemented; needs input/compositor integration tests.

## Documentation / Community

- **[#600] Wiki depth:** Phase A docs added; keep `wiki_repo/` synced via CI on every subsystem merge.
- **[#601] Branch matrix docs:** Document `release/*` branch expectations (microkernel, rtos, cloud, etc.).

---

*Found a bug? Open an issue with subsystem label (`net`, `boot`, `zenith`, `orchestrator`) and link to checklist item.*
