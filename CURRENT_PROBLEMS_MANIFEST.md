# SigmaOS: Active Problems Manifest

*Last updated: Phase F — Competitive Strategy integration. All Phase E targets resolved.*

## ✅ Resolved — Phase E (Gap-Closing)

| ID | Area | Status | File | 
| ---- | ------ | -------- | ------ | 
| E-01 | NVMe Driver | **Resolved** | `drivers/storage/sigma_nvme.cpp` | 
| E-02 | USB xHCI Driver | **Resolved** | `drivers/usb/sigma_xhci.cpp` | 
| E-03 | Power Management (ACPI) | **Resolved** | `kernel/power/sigma_power_manager.cpp` | 
| E-04 | Crash Reporter | **Resolved** | `kernel/diagnostics/sigma_crash_reporter.cpp` | 
| E-05 | Ext4 Journal (JBD2 rewrite) | **Resolved** | `fs/ext4_journal.c` | 

## ✅ Resolved — Phase C/D

| ID | Area | Status | File | 
| ---- | ------ | -------- | ------ | 
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
| ---- | ------ | -------- | ------- | 
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
| ---- | ------ | -------- | ------ | 
| F-01 | Native KMS/GPU (#844) | **Resolved** | `drivers/graphics/sigma_kms.cpp` | 
| F-02 | PCIe MSI-X HAL (#850) | **Resolved** | `hal/sigma_pci.cpp` | 
| F-03 | Cgroup Enforcement (#900) | **Resolved** | `kernel/core/orchestrator/sigma_cgroup.cpp` | 
| F-04 | Sovereign Package Registry (#901) | **Resolved** | `userland/pkg/sigma_registry.cpp` | 
| F-05 | Offline-First CRDT Sync | **Resolved** | `net/sigma_offline_sync.cpp` | 
| F-06 | Native Performance Governor | **Resolved** | `kernel/power/sigma_perf_governor.cpp` | 
| F-07 | HAL Boot Integration (PCI+KMS+PerfGov) | **Resolved** | `hal/SovereignHAL.cpp` | 

## ✅ Resolved — Phase E (Gap-Closing)

| ID | Area | Status | File | 
| ---- | ------ | -------- | ------ | 
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

## ✅ Resolved — Phase G (Kernel Completion)

| ID | Area | Status | File |
| ---- | ------ | -------- | ------ |
| G-01 | Kernel Scheduler (MLFQ+CFS+EDF) | **Resolved** | `kernel/core/sigma_sched.rs` |
| G-02 | Physical Memory Manager (Buddy+Slab) | **Resolved** | `kernel/core/sigma_mm.rs` |
| G-03 | Virtual Memory Manager (Page Table Walker) | **Resolved** | `kernel/mm/page_table_walker.rs` |
| G-04 | IRQ Controller (APIC+PIC+Timer) | **Resolved** | `kernel/core/sigma_irq.rs` |
| G-05 | Syscall Dispatch (30+ syscalls) | **Resolved** | `kernel/core/sigma_syscall_dispatch.rs` |
| G-06 | VESA/GOP Framebuffer Driver | **Resolved** | `drivers/display/sigma_vesa.zig` |
| G-07 | UEFI Bootloader (sigma-boot.efi) | **Resolved** | `sigma-boot/sigma_boot.zig` |
| G-08 | Bootable ISO Pipeline | **Resolved** | `Makefile` (iso target) |

## Open — Phase G Secondary Items

- **[#851-WLAN] Wi-Fi Stack:** IEEE 802.11ax (Wi-Fi 6) driver over xHCI transport.
- **[#851-BT] Bluetooth 5.3 Stack:** HCI/L2CAP/RFCOMM over USB transport.
- **[#1000] Developer SDK:** sigma-sdk CLI, debugger (sigma-gdb), profiler (sigma-perf trace).
- **[#1001] App Sandbox:** Fine-grained capability system (sandboxctl) for third-party apps.
- **[#1002] Multi-monitor KMS:** Extended desktop and clone mode across multiple KMS adapters.
- **[#1009] CryptFS real key derivation (Issue #44):** ✅ **Resolved** — `fs/sigma_cryptfs_derive.rs` implements PBKDF2-HMAC-SHA256 with 100,000 iterations.
- **[#1011] Package repository server:** ✅ **Resolved** — `userland/pkg/sigma_repo_server.rs` implements HTTP-based package repository server.
- **[#1012] TCP/UDP socket layer:** ✅ **Resolved** — `kernel/net/sigma_tcp_state.rs` implements full RFC 793 TCP state machine.
- **[#1013] ABDM FHIR API client:** ✅ **Resolved** — `userland/health/sigma_abdm_client.rs` implements ABDM FHIR R4 client.
- **[#1014] GST IRN API client:** ✅ **Resolved** — `userland/accounts/sigma_gst_client.rs` implements GST IRN and e-Way Bill client.
- **[#1015] Indian language IME:** ✅ **Resolved** — `userland/input/sigma_ime.rs` implements Inscript and Phonetic IME for 10 Indian languages.
- **[#1016] Local LLM backend (sigma-ai):** ✅ **Resolved** — `userland/ai/sigma_llm_backend.rs` implements llama.cpp backend integration.

## Open — Phase I India Profession Apps

- **[#2001] sigma-judicial:** ✅ **Resolved** — `userland/judicial/sigma_judicial.rs` implements eCourts Deep Integration.
- **[#2002] sigma-msme:** ✅ **Resolved** — `userland/msme/sigma_msme.rs` implements MSME Platform with Udyam, GeM, TReDS, OCEN.
- **[#2003] sigma-elections:** ✅ **Resolved** — `userland/elections/sigma_elections.rs` implements Voter Services with EPIC, booth finder, candidate affidavits.
- **[#2004] sigma-ayush:** ✅ **Resolved** — `userland/ayush/sigma_ayush.rs` implements AYUSH Healthcare with practitioner registry, AFI, Panchakarma, NABH.
- **[#2005] sigma-climate:** ✅ **Resolved** — `userland/climate/sigma_climate.rs` implements Environmental Compliance with CPCB, EC, Carbon Market, ESG, REC, AQI.
- **[#2006] sigma-media:** ✅ **Resolved** — `userland/media/sigma_media.rs` implements Broadcast & Press Compliance with MIB, OTT IT Rules, Press Registrar, PIB, TRAI DAS.
- **[#2007] sigma-water:** ✅ **Resolved** — `userland/water/sigma_water.rs` implements Water Resource Management with CWC, JJM, WRIS, Irrigation, CGWB, Flood Warning.
- **[#2008] sigma-prison:** ✅ **Resolved** — `userland/prison/sigma_prison.rs` implements Correctional Facility Management with ePrisons, BNSS, Bail, Rehabilitation, Arnesh Kumar.
- **[#2009] sigma-port:** ✅ **Resolved** — `userland/port/sigma_port.rs` implements Customs & Logistics with ICEGATE, PCS1x, Bill of Lading, FASTag, EXIM Bank, RODTEP.
- **[#2010] sigma-land:** ✅ **Resolved** — `userland/land/sigma_land.rs` implements Land Records & Survey with DILRMP, Mutation, Bhu-Naksha, LARR Act, SVAMITVA, Encumbrance.

## Documentation / Community

- **[#600] Wiki depth:** Phase F/G docs to be added to `wiki_repo/` — offline sync, perf governor.
- **[#601] Branch matrix docs:** Document `release/*` branch expectations in wiki.
- **[#602] CURRENT_PROBLEMS sync:** Keep this manifest updated per subsystem PR — required release gate.

---

## Phase H — India Stack & AI (Next after G)

These are blocked until Phase G kernel work completes:

| ID | Area | Blocked By | 
| ---- | ------ | ------------ | 
| H-01 | ABDM FHIR client live | #1013 — needs real API implementation | 
| H-02 | GST IRN + e-Way Bill API | #1014 — needs real API implementation | 
| H-03 | UPI Autopay / mandate | TCP stack (#1012) + API client work | 
| H-04 | Local LLM (sigma-ai) | #1016 — llama.cpp backend integration | 
| H-05 | Indian IME (Inscript + phonetic) | #1015 — desktop stack (#1010) required | 
| H-06 | sigma-bhashini offline models | #1016 + GPU/audio stack | 
| H-07 | Federated learning coordinator | sigma-ai (#1016) + network stack (#1012) | 
| H-08 | CBDC e-rupee wallet | UPI stack (H-03) prerequisite | 

---

*Found a bug? Open an issue with subsystem label (`net`, `boot`, `zenith`, `orchestrator`) and link to checklist item.*
