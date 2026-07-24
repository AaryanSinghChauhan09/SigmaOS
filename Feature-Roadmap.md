# SigmaOS Feature Roadmap

## Implemented Features (v15.0.0 Zenith)

### Kernel & Security
- Post-quantum crypto: Kyber-1024 KEM + Dilithium-5 signatures
- sigma_pledge (per-process syscall restriction)
- sigma_unveil (per-process filesystem restriction)
- Namespace isolation (unshare/pivot_root/seccomp)
- ASLR 42-bit per-region + W^X enforcement
- AVC — O(1) MAC policy cache
- Zero-trust SPIFFE workload identities
- Immutable audit trail
- TPM2 key unsealing (CryptFS)

### Networking
- TLS 1.3 + X25519/Kyber-1024 hybrid key exchange
- DNS resolver with DoH + DNSSEC + LRU cache
- DHCP client (full RFC 2131/2132 state machine)
- WPA3/SAE dragonfly key exchange
- Stateful firewall + NAT + conntrack
- Offline-First CRDT sync

### System Daemons
- sigma-healthd (structured health, CoreOS-inspired)
- sigma-watchdog (hardware WDT + daemon liveness)
- sigma-metrics (Prometheus-compatible)
- sigma-telemetry (opt-in, PII-scrubbed)
- sigma-cloudsync (E2E encrypted, Argon2id)

### Runtime
- WASM/WASI runtime
- Linux ELF compatibility layer
- Container orchestration framework

### Desktop
- Zenith JS prototype (browser-based desktop)
- Theme engine + dark mode
- AI-driven UI (AVX-512 neural acceleration)
- Accessibility (Sovereign Screen Reader)

---

## Planned Features — Phase G (v16.0 Apex)

### Kernel (Critical Path)
- [ ] Round-robin scheduler → MLFQ → CFS → NUMA → EDF
- [ ] Buddy physical allocator + slab (kmalloc)
- [ ] x86-64 4-level page table walker
- [ ] APIC + PIC init + HPET timer
- [ ] 30-syscall dispatch table
- [ ] sigma-boot.efi UEFI loader
- [ ] Bootable ISO pipeline (`make iso`)

### Drivers
- [ ] VESA/GOP framebuffer SDF driver
- [ ] VirtIO-GPU (QEMU accelerated)
- [ ] Intel i915 basic modesetting
- [ ] AMD amdgpu basic modesetting
- [ ] Intel iwlwifi 802.11ax (Wi-Fi 6)
- [ ] HDA audio controller
- [ ] Bluetooth HCI over USB

### Filesystem
- [ ] VFS open/read/write/close bodies
- [ ] Tmpfs (RAM-backed)
- [ ] SigmaFS mkfs + mount
- [ ] Ext4 read-only mount
- [ ] Unified Buffer Cache (UBC)
- [ ] dm-verity block verifier

### Desktop
- [ ] Zenith native C++ compositor (replacing JS prototype)
- [ ] Sigma Shell full POSIX scripting + tab completion
- [ ] Graphical installer (Calamares equivalent)
- [ ] Signed `.spkg` registry with BLAKE2b + Dilithium3

---

## Planned Features — Phase H (v17-18, India Stack)

- [ ] ABDM FHIR API client
- [ ] GST IRN + e-Way Bill API live
- [ ] UPI Autopay / mandate
- [ ] Local LLM backend (llama.cpp / TinyLlama)
- [ ] Indian IME — Inscript + phonetic (all 22 scheduled languages)
- [ ] sigma-bhashini offline speech models
- [ ] Federated learning coordinator
- [ ] CBDC e-rupee wallet

---

*See also: [FEATURE_MATRIX.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_MATRIX.md) · [Development-Roadmap](Development-Roadmap)*
