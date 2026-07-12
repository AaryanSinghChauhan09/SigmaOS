# SigmaOS FAQ

## General

### Q: What is SigmaOS?

A: SigmaOS is a sovereign, AI-native operating system with a freestanding microkernel (no glibc), post-quantum cryptography, and 8 deployment profiles — from bare-metal desktop to embedded RTOS to cloud containers.

### Q: Is SigmaOS based on Linux?

A: No. SigmaOS has its own kernel, written from scratch in C++ and C. It has a Linux ELF compatibility layer (`runtime/containers/sigma_linux_compat.cpp`) but doesn't use the Linux kernel.

### Q: Can I install SigmaOS on real hardware today?

A: Not yet. A bootable ISO (`make iso`) is Phase G, targeted for v16.0 Apex (Q1 2027). Currently you can run the kernel stub in QEMU.

### Q: What architectures does SigmaOS support?

A: x86_64 (primary), ARM64 (Phase G — Raspberry Pi 4/5, JioBook), RISC-V RV64GC (Phase H). HAL stubs exist for all three.

### Q: What is a "shard"?

A: A shard is an atomic, independently-deployable capability module. SigmaOS has 600+ shards (numbered `S001–S500+`) organised in `suites/`. Build profiles select which shards to include via CMake feature flags.

---

## Development

### Q: Where do I start contributing?

A: Check [Contributing](Contributing) and look for `good first issue` labels. The most impactful work is in `kernel-exp` branch (kernel bodies — see [Branch-Development-Roadmap](Branch-Development-Roadmap)).

### Q: Why are most kernel `.cpp` files empty stubs?

A: This is an honest architectural state: the design and headers are complete, but Phase G implementation is in progress. `make check-stubs` lists all unimplemented bodies.

### Q: What language is used?

A: C++ (kernel, drivers, UI), C (low-level paths), Rust (`lib/libsigma_safe.rs`), Python/Shell (scripts), Go (planned for daemons), Assembly (boot, context switch, SIMD).

### Q: How do I add a hardware driver?

A: See [Driver-Development](Driver-Development). Use the SDF template, implement `probe()`/`init()`/`shutdown()`, register with `SIGMA_SDF_REGISTER_DRIVER`.

---

## Security

### Q: What cryptography does SigmaOS use?

A: Post-quantum: Kyber-1024 KEM (FIPS 203) + Dilithium-5 signatures (FIPS 204). Classical: AES-256-GCM, BLAKE3, Argon2id. TLS 1.3 with X25519/Kyber-1024 hybrid.

### Q: Is disk encryption implemented?

A: CryptFS framework exists. However, Issue #1009: `derive_key()` currently returns 32 zero bytes — real key derivation is Phase G. Don't rely on it for real data yet.

**Q: What is sigma_pledge / sigma_unveil?**
A: OpenBSD-inspired process isolation. `pledge` restricts which syscalls a process can make; `unveil` restricts which filesystem paths it can see. See [Security-Model](Security-Model).

---

## Packages

### Q: What is the package format?

A: `.spkg` (Sovereign Package) — content-addressed, Dilithium-5 signed, BLAKE3 hashed, reproducibly built. Similar to NixOS but with cryptographic sovereignty.

### Q: Is there an app store?

A: The `release/app` branch has a demo app store UI. The backend package registry server is Phase G (#1011).

### Q: Can I run Flatpak/Snap/AppImage apps?

A: A compatibility layer is in `tools/sigma_flatpak.cpp` and `tools/sigma_snap.cpp`. Full runtime support is Phase G.

---

## India Stack

### Q: What India-specific features are planned?

A: ABDM FHIR health records, GST IRN + e-Way Bill, UPI Autopay, Indian IME (all 22 scheduled languages via Inscript + phonetic), sigma-bhashini offline speech, CBDC e-rupee wallet. All are Phase H (v17-18), blocked on kernel boot (Phase G).

---

*See also: [Home](Home) · [Getting-Started](Getting-Started) · [CURRENT_PROBLEMS_MANIFEST](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CURRENT_PROBLEMS_MANIFEST.md)*
