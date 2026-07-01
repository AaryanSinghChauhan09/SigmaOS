# SigmaOS Feature Matrix

Legend: ✅ Complete · 🔄 Partial/In Progress · ⬜ Planned · — Not applicable

## Core Kernel

| Feature | main | standalone | microkernel | cloud | rtos | mobile |
|---------|------|------------|-------------|-------|------|--------|
| MLFQ Scheduler | 🔄 | 🔄 | ⬜ | 🔄 | ⬜ | 🔄 |
| EDF Real-Time Class | ⬜ | — | — | — | ⬜ | — |
| Buddy Allocator | 🔄 | 🔄 | ⬜ | 🔄 | ⬜ | 🔄 |
| Slab Allocator (kmalloc) | 🔄 | 🔄 | — | 🔄 | — | 🔄 |
| 4-level paging + ASLR | ✅ | ✅ | ⬜ | ✅ | ⬜ | ⬜ |
| W^X enforcement | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| 30-syscall dispatch | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| APIC/GIC IRQ controller | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| Bootable ISO | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ | ⬜ |
| sigma-boot.efi | ⬜ | ⬜ | ⬜ | ⬜ | — | — |

## Security

| Feature | main | standalone | microkernel | cloud | rtos | mobile |
|---------|------|------------|-------------|-------|------|--------|
| sigma_pledge (syscall restriction) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| sigma_unveil (FS restriction) | ✅ | ✅ | ✅ | ✅ | — | ✅ |
| Namespace isolation | ✅ | ✅ | ⬜ | ✅ | — | ✅ |
| Cgroup enforcement | ✅ | ✅ | ⬜ | ✅ | — | — |
| Kyber-1024 KEM (PQC) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Dilithium-5 signatures | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Zero-trust enforcer | ✅ | ✅ | ⬜ | ✅ | — | ⬜ |
| TPM2 attestation | 🔄 | 🔄 | — | ✅ | — | — |
| Secure boot (UEFI) | ⬜ | ⬜ | — | ⬜ | — | ⬜ |
| AVC (SELinux-inspired) | ✅ | ✅ | ⬜ | ✅ | — | ✅ |

## Networking

| Feature | main | standalone | cloud | distributed | rtos | mobile |
|---------|------|------------|-------|-------------|------|--------|
| TCP/IP stack | 🔄 | 🔄 | ✅ | ✅ | ⬜ | 🔄 |
| TLS 1.3 + Kyber hybrid | ✅ | ✅ | ✅ | ✅ | ⬜ | ✅ |
| DNS/DoH + DNSSEC | ✅ | ✅ | ✅ | ✅ | — | ✅ |
| DHCP client | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| WPA3/SAE Wi-Fi | ⬜ | ⬜ | — | — | ⬜ | ⬜ |
| WireGuard VPN | 🔄 | 🔄 | ✅ | ✅ | — | 🔄 |
| Stateful firewall | ✅ | ✅ | ✅ | ✅ | — | 🔄 |
| CRDT offline sync | ✅ | ✅ | ✅ | ✅ | — | ✅ |
| Mesh networking | 🔄 | — | ✅ | ✅ | — | 🔄 |

## Filesystem

| Feature | main | standalone | cloud | distributed |
|---------|------|------------|-------|-------------|
| VFS layer | 🔄 | 🔄 | 🔄 | 🔄 |
| SigmaFS (native) | ⬜ | ⬜ | ⬜ | ⬜ |
| Ext4 read/write | 🔄 | 🔄 | 🔄 | — |
| FAT32 | ✅ | ✅ | — | — |
| Tmpfs | ⬜ | ⬜ | ⬜ | — |
| dm-verity | ⬜ | ⬜ | ⬜ | ⬜ |
| Immutable root A/B | — | — | ⬜ | ⬜ |
| OSTree atomic updates | — | ⬜ | ⬜ | ⬜ |

## Desktop (standalone / browser)

| Feature | standalone | browser |
|---------|------------|---------|
| Zenith C++ compositor | 🔄 | — |
| Zenith JS prototype | ✅ | ✅ |
| Auto-tiling WM | 🔄 | — |
| Theme engine | ✅ | ✅ |
| Accessibility (SSR) | 🔄 | 🔄 |
| Indian IME (Inscript) | ⬜ | — |
| sigma-ai LLM daemon | ⬜ | — |
| App store UI | 🔄 | ✅ |
| sigma-pkg GUI | 🔄 | ✅ |

## Hardware Drivers

| Driver | Status | Branch |
|--------|--------|--------|
| NVMe | ✅ | drivers-dev |
| USB xHCI | ✅ | drivers-dev |
| e1000 NIC | ✅ | drivers-dev |
| VirtIO-GPU | ⬜ | drivers-dev |
| Intel i915 KMS | ⬜ | drivers-dev |
| AMD amdgpu | ⬜ | drivers-dev |
| Intel iwlwifi Wi-Fi 6 | ⬜ | drivers-dev |
| HDA audio | ⬜ | drivers-dev |
| Bluetooth HCI | ⬜ | drivers-dev |
| ARM64 BCM2711 (RPi 4) | ⬜ | release/mobile |
| ARM64 BCM2712 (RPi 5) | ⬜ | release/mobile |

---

*Last updated: July 2026 — v15.0.0 Zenith baseline*
