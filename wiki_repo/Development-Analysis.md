# SigmaOS Development Analysis — 100+ Opportunities

> Complete gap analysis, implementation status, and prioritised roadmap.
> Every item maps to missing code and is weighted by impact.

---

## Status Legend

| Icon | Meaning |
|---|---|
| ✅ | Implemented and functional |
| 🔄 | In progress / partial |
| ⬜ | Planned, not started |
| 🆕 | Newly implemented (this sprint) |

---

## 1. Critical Kernel Infrastructure

### Scheduler

| Component | Status | File |
|---|---|---|
| MLFQ 4-queue with aging | 🆕 | `kernel/sched/sigma_mlfq.rs` |
| CFS red-black tree + vruntime | 🆕 | `kernel/sched/sigma_mlfq.rs` |
| EDF (earliest deadline first) | 🆕 | `kernel/sched/sigma_mlfq.rs` |
| RT fixed-priority | 🆕 | `kernel/sched/sigma_mlfq.rs` |
| Load balancing (SMP) | ⬜ | `kernel/sched/sigma_smp_lb.rs` (next) |
| Per-CPU queues | ⬜ | Phase B |
| AI-predictive pre-warming | ⬜ | Phase H |

### Memory Manager

| Component | Status | File |
|---|---|---|
| Buddy allocator (2^n blocks, O(log n)) | 🆕 | `kernel/memory/sigma_buddy.rs` |
| Slab allocator (object caches) | 🆕 | `kernel/memory/sigma_buddy.rs` |
| ASLR (42-bit entropy) | 🆕 | `kernel/memory/sigma_buddy.rs` |
| W^X enforcement | 🆕 | `kernel/memory/sigma_buddy.rs` |
| Page fault handler | 🔄 | `kernel/memory/sigma_paging.rs` |
| TLB shootdown | ⬜ | Phase B |
| NUMA support | ⬜ | Phase C |
| Huge pages (2MB/1GB) | ⬜ | Phase B |
| Swap/zswap | ⬜ | Phase C |

### Syscall Dispatch

| Component | Status | File |
|---|---|---|
| 50+ POSIX syscalls (complete table) | 🆕 | `kernel/syscalls/sigma_syscall_table.rs` |
| sigma_pledge syscall | 🆕 | `kernel/syscalls/sigma_syscall_table.rs` |
| sigma_unveil syscall | 🆕 | `kernel/syscalls/sigma_syscall_table.rs` |
| sigma-bus IPC syscalls | 🆕 | `kernel/syscalls/sigma_syscall_table.rs` |
| sigma-ai inference syscall | 🆕 | `kernel/syscalls/sigma_syscall_table.rs` |
| TPM2 attestation syscall | 🆕 | `kernel/syscalls/sigma_syscall_table.rs` |
| io_uring async I/O | ⬜ | Phase B |
| seccomp-BPF enforcement | 🔄 | `kernel/security/` |

---

## 2. Bootloader & Firmware

| Component | Status | Notes |
|---|---|---|
| sigma-boot.efi (UEFI PE/COFF) | ⬜ | Phase A blocker |
| Secure Boot (Dilithium-5 verify) | ⬜ | Phase A |
| TPM2 measured boot (PCR extend) | ⬜ | Phase A |
| GRUB2 fallback (legacy BIOS) | 🔄 | `arch/boot/` |
| Bootable ISO pipeline | 🔄 | `scripts/build-iso.sh` |
| QEMU CI pipeline | ✅ | `.github/workflows/sigma_qemu.yml` |
| Recovery boot mode | ⬜ | Phase B |

---

## 3. Device Drivers

### Networking

| Driver | Chip | Status | Priority |
|---|---|---|---|
| e1000 (Intel GbE) | Intel i210/i350 | ✅ | — |
| VirtIO-net | QEMU | ✅ | — |
| r8169 (Realtek Ethernet) | RTL8111/8168 | 🔄 | High |
| Wi-Fi framework (SDF) | All | 🆕 | **Critical** |
| iwlwifi (Intel Wi-Fi 6/6E) | AX200/AX201/AX210 | 🔄 | High |
| ath11k (Qualcomm Wi-Fi 6) | QCA6390/WCN6855 | ⬜ | High |
| mt76 (MediaTek Wi-Fi 6) | MT7921/MT7922 | ⬜ | High (Asia) |
| rtw89 (Realtek 802.11ax) | RTW8822CE/8852AE | ⬜ | Medium |

### Storage

| Driver | Status | Priority |
|---|---|---|
| NVMe (PCIe SSD) | ✅ | — |
| VirtIO-blk | ✅ | — |
| AHCI (SATA) | 🔄 | High |
| eMMC | ⬜ | High (ARM) |
| SD/SDIO | ⬜ | Medium |
| UAS (USB SSD) | ⬜ | Medium |

### GPU/Graphics

| Driver | Status | Notes |
|---|---|---|
| VirtIO-GPU (QEMU) | 🔄 | `drivers/gpu/` |
| i915 (Intel) | ⬜ | Phase C |
| amdgpu | ⬜ | Phase C |
| NVIDIA Nouveau | ⬜ | Phase D |
| DRM/KMS mode setting | ⬜ | Phase C |

### USB

| Component | Status |
|---|---|
| xHCI host controller | ✅ |
| HID (keyboard/mouse) | 🔄 |
| Mass storage (USB disk) | ⬜ |
| USB hub management | ⬜ |
| USB-C Power Delivery | ⬜ |

---

## 4. Filesystem

| Component | Status | File |
|---|---|---|
| VFS layer | ✅ | `fs/sigma_vfs.zig` |
| Ext4 read/write | ✅ | — |
| Tmpfs | ✅ | — |
| FAT32 (read) | ✅ | — |
| CryptFS — PBKDF2 key derivation | 🆕 | `fs/sigma_cryptfs.rs` |
| CryptFS — XTS-AES-256 encryption | 🆕 | `fs/sigma_cryptfs.rs` |
| CryptFS — TPM2 key sealing | 🆕 | `fs/sigma_cryptfs.rs` |
| CryptFS — secure key erasure | 🆕 | `fs/sigma_cryptfs.rs` |
| SigmaFS (native CoW) | ⬜ | Phase G |
| OverlayFS | ⬜ | Needed for containers |
| FAT32 write | ⬜ | EFI partition writes |
| NTFS | ⬜ | Dual-boot Windows |
| exFAT | ⬜ | Portable storage |

---

## 5. Network Stack

| Component | Status | File |
|---|---|---|
| IPv4/IPv6 framework | ✅ | `kernel/net/` |
| DNS/DoH/DNSSEC | ✅ | `net/dns/` |
| DHCP | ✅ | — |
| TLS 1.3 + Kyber hybrid | 🔄 | `net/tls/` |
| TCP state machine (RFC 793) | 🆕 | `kernel/net/sigma_tcp.rs` |
| TCP congestion (BBR/CUBIC) | 🆕 | `kernel/net/sigma_tcp.rs` |
| TCP SACK, timestamps, ECN | 🆕 | `kernel/net/sigma_tcp.rs` |
| TCP retransmission | 🆕 | `kernel/net/sigma_tcp.rs` |
| QUIC / HTTP/3 | ⬜ | Phase B |
| BPF packet filters | ⬜ | Phase B |

---

## 6. Security

| Component | Status | File |
|---|---|---|
| sigma_pledge (OpenBSD-style) | ✅ | `kernel/security/` |
| sigma_unveil (path restriction) | ✅ | `kernel/security/` |
| AVC O(1) MAC cache | ✅ | `security/sigma_avc.rs` |
| PQC: Kyber-1024 + Dilithium-5 | ✅ | `crypto/` |
| CryptFS real key derivation | 🆕 | `fs/sigma_cryptfs.rs` |
| TPM2 key sealing | 🆕 | `fs/sigma_cryptfs.rs` |
| sigma-agent security advisor | ✅ | `userland/agent/sigma_agent_security.nim` |
| SELinux-style MAC policy | ⬜ | Phase D |
| Fine-grained capabilities (41+) | ⬜ | Phase C |
| KASLR | ⬜ | Phase B |

---

## 7. Userland & Shell

| Component | Status | File |
|---|---|---|
| sigma-sh basic REPL | ✅ | `sigma-sh/src/` |
| Globbing (* ? [a-z]) | 🆕 | `userland/shell/sigma_sh_features.rs` |
| Parameter expansion (${VAR:-}) | 🆕 | `userland/shell/sigma_sh_features.rs` |
| Command substitution $(...) | 🆕 | `userland/shell/sigma_sh_features.rs` |
| Job control (bg/fg) | 🆕 | `userland/shell/sigma_sh_features.rs` |
| Shell functions | 🆕 | `userland/shell/sigma_sh_features.rs` |
| Here-documents | 🆕 | `userland/shell/sigma_sh_features.rs` |
| Pipeline execution | 🆕 | `userland/shell/sigma_sh_features.rs` |
| Pipe/redirect 2>&1 | 🔄 | `sigma-sh/src/executor.rs` |
| Fish-style autocomplete | ✅ | sigma-agent complete |
| Scripting language (Lua) | ⬜ | Phase E |

---

## 8. India Stack

| Component | Status | File |
|---|---|---|
| ABDM FHIR patient search | 🆕 | `userland/indiastack/sigma_india_stack.rs` |
| ABDM health record linking | 🆕 | `userland/indiastack/sigma_india_stack.rs` |
| UPI payment URI generation | 🆕 | `userland/indiastack/sigma_india_stack.rs` |
| UPI collect request | 🆕 | `userland/indiastack/sigma_india_stack.rs` |
| UPI transaction status | 🆕 | `userland/indiastack/sigma_india_stack.rs` |
| GST/IRN generation (NIC API) | 🆕 | `userland/indiastack/sigma_india_stack.rs` |
| e-RUPI voucher | 🆕 | `userland/indiastack/sigma_india_stack.rs` |
| Aadhaar e-KYC | ⬜ | Phase D |
| Indian language IME | ⬜ | Phase D |
| MeitY compliance | ⬜ | Phase D |

---

## 9. Virtualisation & Containers

| Component | Status | File |
|---|---|---|
| OCI container runtime (sigma-pod) | 🆕 | `virtualization/ocirunner/sigma_oci.rs` |
| Namespace isolation (PID/net/IPC) | 🆕 | `virtualization/ocirunner/sigma_oci.rs` |
| cgroup v2 resource limits | 🆕 | `virtualization/ocirunner/sigma_oci.rs` |
| OverlayFS layers | 🆕 | `virtualization/ocirunner/sigma_oci.rs` |
| OCI image pull (skopeo/docker) | 🆕 | `virtualization/ocirunner/sigma_oci.rs` |
| runc/crun delegation | 🆕 | `virtualization/ocirunner/sigma_oci.rs` |
| Docker Compose support | ⬜ | Phase B |
| KVM guest support | ⬜ | Phase C |
| VirtIO GPU | 🔄 | — |

---

## 10. AI & ML

| Component | Status | File |
|---|---|---|
| sigma-agent (35 modules) | ✅ | `userland/agent/` |
| llama.cpp backends (4 providers) | ✅ | `userland/agent/sigma_llm.rs` |
| RLHF feedback loop | ✅ | `userland/agent/sigma_agent_learn.nim` |
| DPO fine-tuning pipeline | ✅ | `userland/agent/sigma_agent_learn.nim` |
| Multi-agent routing (6 specialists) | ✅ | `userland/agent/sigma_agent_multi.nim` |
| Workflow automation (n8n-style) | ✅ | `userland/agent/sigma_agent_workflow.nim` |
| Corpus builder | ✅ | `userland/agent/sigma_agent_corpus.nim` |
| Local AI inference daemon | 🔄 | `userland/ai/sigma_ai.rs` |
| Federated learning coordinator | ⬜ | Phase F |
| sigma-heal crash analysis | ⬜ | Phase E |

---

## 11. Package Ecosystem

| Component | Status | File |
|---|---|---|
| sigma-pkg core | 🔄 | `pkg/sigma_pkg_core.nim` |
| .deb absorption | 🆕 | `pkg/sigma_pkg_absorb.nim` |
| .rpm absorption | 🆕 | `pkg/sigma_pkg_absorb.nim` |
| AppImage absorption | 🆕 | `pkg/sigma_pkg_absorb.nim` |
| Declarative recipes (NixOS-style) | 🆕 | `pkg/sigma_pkg_recipe.nim` |
| Linux compat layer | 🆕 | `userland/compat/sigma_linux_compat.nim` |
| Repository server | ⬜ | Phase B |
| Dependency resolver (DAG) | ⬜ | Phase B |
| Delta updates | ⬜ | Phase C |
| 1000+ packages | ⬜ | Community effort |

---

## Phase Roadmap

| Phase | Timeline | Focus | Key Deliverables |
|---|---|---|---|
| **A (Now)** | v15.x | AI agent complete, kernel foundations | sigma-agent 35 modules ✅, MLFQ ✅, buddy ✅, 50+ syscalls ✅ |
| **B** | v16.x | Kernel stability, Wi-Fi, TCP, pkg repo | sigma-boot.efi, iwlwifi, ath11k, pkg repository server |
| **C** | v17.x | GPU, desktop, containers | i915/amdgpu, Zenith DE, OCI runtime hardening |
| **D** | v18.x | Security hardening, India Stack, MAC | SELinux policy, ABDM/UPI/GST production, MeitY |
| **E** | v19.x | Community scale, 1000+ packages | Package ecosystem, contributor growth |
| **F** | v20.x | Enterprise, federated learning | Hardware partnerships, corporate adoption |

---

## Immediate Priorities (1-2 Contributors)

```bash

# Fix syscall dispatch (unblocks real hardware)

cd kernel/syscalls && cargo test

# Implement AHCI driver (SATA disk support)

# File: drivers/storage/sigma_ahci.rs

# Wi-Fi firmware loading (iwlwifi priority)

# File: drivers/net/sigma_wifi_driver.rs (firmware_name() already returns paths)

# sigma-boot.efi UEFI bootloader

# File: sigma-boot/sigma_boot.zig (extend existing stub)

```

---

*See also: [Architecture Overview](Architecture-Overview) · [OSS Absorption Strategy](OSS-Absorption-Strategy) · [sigma-agent](sigma-agent) · [Migration Guide](Migration-Guide)*
