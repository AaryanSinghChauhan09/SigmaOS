# SigmaOS — Active Problems Manifest
> Last updated: Phase 10 — Comprehensive OS Implementation (July 2026)
> Status: All Phase A–F resolved. Phase G–J tracking new work.

---

## ✅ Resolved — Phase A–F (All Prior Phases)

| ID | Area | Status | File |
|----|------|--------|------|
| A-01..A-08 | Networking TX/RX, Socket, Boot, CLI, Personalization | **Resolved** | Various |
| C-01..C-06 | PQC Kyber/Dilithium, Audit Trail, Zero-Trust, Neural UI | **Resolved** | `crypto/`, `kernel/security/` |
| D-01..D-02 | WASM/WASI Runtime, Linux ELF Compat | **Resolved** | `runtime/` |
| E-01..E-05 | NVMe, USB xHCI, ACPI, Crash Reporter, Ext4 Journal | **Resolved** | `drivers/`, `fs/` |
| F-01..F-07 | KMS/GPU, PCIe MSI-X, Cgroups, Package Registry, CRDT Sync, Perf Gov, HAL Boot | **Resolved** | Various |

---

## ✅ Resolved — Phase G–H (New Implementations This Session)

| ID | Area | Status | File |
|----|------|--------|------|
| G-01 | TCP/IP Stack (RFC 793/9293 full state machine) | **Resolved** | `kernel/core/sigma_tcp_stack.rs` |
| G-02 | Wi-Fi Stack (IEEE 802.11ax / WPA3-SAE) | **Resolved** | `kernel/core/sigma_wifi_stack.rs` |
| G-03 | Bluetooth 5.3 (HCI/L2CAP/GATT) | **Resolved** | `kernel/core/sigma_bluetooth.rs` |
| G-04 | Physical Memory Manager (Buddy Allocator) | **Resolved** | `kernel/core/sigma_pmm.rs` |
| G-05 | MLFQ + MCS Scheduler | **Resolved** | `kernel/core/sigma_mlfq_sched.rs` |
| G-06 | IRQ Controller (APIC/I-OAPIC/GIC-400) | **Resolved** | `kernel/core/sigma_irq_controller.rs` |
| G-07 | CryptFS Key Derivation (PBKDF2/HKDF) | **Resolved** | `crypto/sigma_key_derive.rs` |
| G-08 | eBPF Subsystem | **Resolved** | `kernel/bpf/sigma_ebpf.rs` |
| G-09 | epoll / io_uring equivalent | **Resolved** | `kernel/core/sigma_epoll.rs` |
| G-10 | VirtIO drivers | **Resolved** | `drivers/virtio/sigma_virtio.rs` |
| G-11 | India Stack (ABDM/UPI/GST/Aadhaar) | **Resolved** | `userland/india/sigma_india_stack.py` |
| G-12 | AI Daemon (sigma-ai) | **Resolved** | `sigmad/sigma_ai_daemon.py` |
| H-01 | IPC: Pipes, Message Queues, Shared Memory | **Resolved** | `kernel/core/sigma_ipc_pipe.rs` |
| H-02 | VFS + Ext4-compatible Filesystem | **Resolved** | `kernel/core/sigma_vfs_ext4.rs` |
| H-03 | Sound Subsystem (PipeWire-inspired, HDA) | **Resolved** | `kernel/core/sigma_sound.rs` |
| H-04 | USB Stack (xHCI/HID/Mass Storage) | **Resolved** | `kernel/core/sigma_usb_stack.rs` |
| H-05 | GPU/DRM/KMS (framebuffer, mode-setting, page-flip) | **Resolved** | `kernel/core/sigma_gpu_drm.rs` |
| H-06 | Full Network Stack (ARP/DHCP/DNS/ICMP) | **Resolved** | `kernel/core/sigma_network_stack.rs` |
| H-07 | OCI Container Runtime + CRI | **Resolved** | `kernel/core/sigma_container_runtime.rs` |
| H-08 | Power Management (ACPI/DVFS/Battery/Thermal) | **Resolved** | `kernel/core/sigma_power_mgmt.rs` |
| H-09 | Local LLM Inference Engine | **Resolved** | `kernel/core/sigma_local_llm.rs` |

---

## 🔴 Open — Phase I (High Priority, Next Sprint)

| ID | Area | Blocked By | Notes |
|----|------|-----------|-------|
| I-01 | UEFI bootloader binary (`sigma-boot.efi`) | Phase G kernel stable | Cannot dual-boot without EFI stub |
| I-02 | Bootable ISO pipeline (`make iso` → GPT image) | I-01 | GRUB2 fallback usable meanwhile |
| I-03 | NVMe interrupt-driven async driver (vs MMIO poll) | IRQ controller | Performance: 4× throughput gain |
| I-04 | Wi-Fi 6E (6 GHz band) + WPA3-Enterprise | G-02 base | Enterprise / government deployments |
| I-05 | Multi-monitor KMS (clone + extended) | H-05 base | Zenith desktop requires this |
| I-06 | SATA AHCI driver | PMM stable | Many embedded devices use SATA |
| I-07 | Virtio-GPU (GPU acceleration for VMs) | H-05 base | QEMU CI requires this |
| I-08 | Package repository server (`sigma-repo-server`) | net stack stable | sigma-pkg install needs a real server |
| I-09 | Display server crash recovery | H-05 + IPC | Zenith restarts on segfault |
| I-10 | Indian Language IME (Inscript + phonetic 22 langs) | H-05 + sound | BHASHINI integration |
| I-11 | Dependency reduction (Electron → native Zenith) | Zenith compositor | Reduce binary size by ~200MB |
| I-12 | Build tool replacement (Vite → sigma-build) | sigma-build design | Eliminate Node.js dependency |
| I-13 | Linter replacement (ESLint → sigma-lint) | sigma-lint implementation | ✅ Started - basic implementation done |
| I-14 | Formatter replacement (Prettier → sigma-format) | sigma-format implementation | ✅ Started - basic implementation done |
| I-15 | Intel GPU driver (i915-inspired) | GPU framework | ✅ Started - basic modesetting done |
| I-16 | Realtek network driver (r8169-inspired) | Network stack | ✅ Started - basic driver done |
| I-17 | Init system (systemd/OpenRC-inspired) | Service framework | ✅ Started - basic service manager done |
| I-18 | Package manager core (sigpkg) | Package ecosystem | ✅ Started - basic package handling done |
| I-19 | SELinux-inspired MAC system | Security framework | ✅ Started - basic policy engine done |
| I-20 | Graphical installer (Calamares-inspired) | Installer framework | ✅ Started - basic installer done |
| I-21 | UEFI bootloader (sigma-boot) | Bootloader framework | ✅ Started - basic bootloader done |
| I-22 | Display server (Wayland-inspired) | Desktop framework | ✅ Started - basic compositor done |
| I-23 | NLP engine (AI assistant) | AI framework | ✅ Started - basic NLP done |

---

## 🟠 Open — Phase J (Medium Priority)

| ID | Area | Notes |
|----|------|-------|
| J-01 | RISC-V 64 port (boot, IRQ, MMU) | Emerging server/embedded market |
| J-02 | ARM64 BSP (Raspberry Pi 5, Cortex-A76) | Mobile/IoT profile |
| J-03 | Formal verification (Coq proofs for scheduler + PMM) | FIPS/CC certification path |
| J-04 | FIDO2/WebAuthn hardware key support | Security profile |
| J-05 | Wayland protocol implementation (client-side) | Zenith third-party apps |
| J-06 | Linux binary compatibility (binfmt_misc + syscall compat) | App ecosystem |
| J-07 | sigma-pod rootless containers | Security hardening |
| J-08 | eBPF JIT for x86-64 (current: interpreter only) | 10× performance |
| J-09 | Zstd/LZ4 transparent filesystem compression | Storage efficiency |
| J-10 | TPM 2.0 measured boot + remote attestation | Enterprise/Govt deployments |

---

## 🟡 Open — Phase K (Lower Priority / Research)

| ID | Area | Notes |
|----|------|-------|
| K-01 | Quantum-safe TLS 1.3 (ML-KEM + ML-DSA) | Post-quantum readiness |
| K-02 | Federated learning coordinator | ML research market |
| K-03 | CBDC e-rupee wallet (RBI sandbox) | India fintech |
| K-04 | ABDM live FHIR API client | India health |
| K-05 | GST IRN + e-Way Bill API | India commerce |
| K-06 | sigma-bhashini offline STT/TTS (11 Indian languages) | Accessibility |
| K-07 | Hardware neural accelerator (NPU) driver | Edge AI |
| K-08 | Light detection (LiDAR) driver for robotics | Robotics profile |
| K-09 | CAN bus driver for automotive | Embedded/auto |
| K-10 | Deterministic replay for kernel debugging | Developer tools |

---

## Documentation Status

| Doc | Status | Location |
|-----|--------|----------|
| DEVELOPMENT_ROADMAP.md | ✅ Updated | repo root |
| FUTURE_ROADMAP.md | ✅ Created | wiki_repo/ |
| wiki Home.md | ✅ Updated | wiki_repo/ |
| wiki Kernel.md | ✅ Updated | wiki_repo/ |
| CURRENT_PROBLEMS_MANIFEST.md | ✅ This file | repo root |

---

*Found a bug? Open an issue with label `subsystem:kernel`, `subsystem:net`, `subsystem:zenith`, etc.*
*Reference the Phase ID above in your PR description.*
