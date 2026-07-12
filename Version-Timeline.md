# SigmaOS Version Timeline

## Released Versions

### v15.0.0 Zenith — May 2026 ✅

The sovereignty milestone. Full PQC stack, AI scheduling, KMS framework, and 600-shard modular lattice.

### Added:

- Kyber-1024 KEM + Dilithium-5 PQC signatures

- WASM/WASI runtime

- Linux ELF compatibility layer

- Neural UI with AVX-512 acceleration

- Native KMS/GPU framework

- PCIe MSI-X HAL

- Cgroup enforcement

- Sovereign Package Registry

- Offline-First CRDT sync

- Native Performance Governor

- 600-shard modular lattice

- CI/CD hardening

---

## Upcoming Versions

### v15.1.0 Zenith LTS — August 2026

Pre-launch polish and Windows compatibility bridge.

### Planned:

- Windows compat layer headers (`include/compat/`)

- sigma-wine-loader skeleton

- gh-pages redesign with Phase G progress dashboard

- Launch checklist completion

- Signed release artefacts

- ARM64 build validation

---

### v16.0.0 Apex — Q1 2027

First truly bootable SigmaOS on real hardware.

### Planned:

- Real kernel scheduler (round-robin → MLFQ)

- Buddy + slab physical memory manager

- x86-64 page table walker

- APIC + PIC + HPET timer

- 30-syscall dispatch

- sigma-boot.efi UEFI loader

- `make iso` → bootable ISO (QEMU CI green)

- VESA/VirtIO-GPU framebuffer

- Wi-Fi 6 (iwlwifi) + Bluetooth 5.3

- Full Zenith desktop on physical hardware

- ARM64 BCM2711/BCM2712 BSP (RPi 4/5)

- CryptFS real key derivation (Issue #44 fix)

- Developer SDK (sigma-sdk CLI + sigma-gdb)

---

### v17.0.0 Sovereign — Q3 2027

All 8 deployment profiles functional.

### Planned:

- `release/cloud`: sigma-pod with real cgroup/namespace

- `release/mobile`: ARM64 GIC + MMU + touch UI

- `release/rtos`: EDF scheduler < 10 µs IRQ latency

- `release/dual-boot`: EFI entry + partition detector

- `release/distributed`: SovereignCloudFS + mesh

- sigma-fleet MDM agent

- VFS + SigmaFS production-ready

- Package repository server live

---

### v18.0.0 Transcendence — Q1 2028

India Stack, LLM backend, and formal verification.

### Planned:

- ABDM FHIR live API client

- GST IRN + e-Way Bill integration

- UPI Autopay / mandate

- Local LLM backend (llama.cpp / TinyLlama)

- Indian IME — all 22 scheduled languages

- sigma-bhashini offline speech models

- Federated learning coordinator

- CBDC e-rupee wallet

- Formal verification of kernel modules (Coq/Frama-C)

- Raspberry Pi 5 production image

---

*See also: [Development-Roadmap](Development-Roadmap) · [CHANGELOG](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CHANGELOG.md) · [Release-Profiles](Release-Profiles)*
