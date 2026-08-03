# Roadmap

## Phase 1: Foundation (Current)

- [x] Microkernel with MLFQ scheduler
- [x] Memory management (PMM/VMM, buddy allocator, slab)
- [x] VFS with ext4, Btrfs, XFS support
- [x] SELinux + AppArmor + Sigma-MAC security
- [x] Capability token system (privilege escalation bug fixed)
- [x] TCP/IP stack (native, no lwIP)
- [x] DNS resolver with split-DNS + hosts file
- [x] 15-format package manager (SigmaPkg)
- [x] klib zero-std collections
- [x] Native crypto (AES, SHA-256, HMAC, PBKDF2, ChaCha20)
- [x] Dilithium PQC signatures
- [x] SSSD offline credential caching
- [x] 25+ Linux distro compatibility layers
- [x] AI subsystem (LLM, agents, orchestrator)
- [x] Runit service supervisor
- [x] Atomic A/B update system (OSTree-inspired)

## Phase 2: Hardware (In Progress)

- [ ] UEFI bootloader pointer-safety audit (raw ptr → safe wrappers)
- [ ] GPU driver (Vulkan/Mesa-inspired, self-healing recovery)
- [ ] USB host controller (EHCI/XHCI)
- [ ] NVMe / AHCI storage drivers
- [ ] Ethernet & Wi-Fi drivers (NDIS model)
- [ ] DMA engine safety wrappers
- [ ] TPM 2.0 full integration

## Phase 3: Desktop (Planned)

- [ ] Zenith Wayland compositor (full stability)
- [ ] Native window manager (tiling + floating)
- [ ] GPU-accelerated rendering pipeline
- [ ] Screen recorder (Bandicam parity)
- [ ] Screen reader (NVDA parity)
- [ ] Native file manager
- [ ] System settings (YaST-style)

## Phase 4: Applications (Planned)

- [ ] Sigma Office (Writer, Calc, Impress)
- [ ] Sigma Browser (WebKit or Servo integration)
- [ ] Sigma Email client
- [ ] Sigma Code editor (VS Code parity)
- [ ] Media player (VLC parity)
- [ ] Video editor

## Phase 5: Ecosystem (Future)

- [ ] App store (SigmaPkg hub)
- [ ] Cloud backup integration
- [ ] Remote desktop (RustDesk parity)
- [ ] Android app compatibility (Waydroid-inspired)
- [ ] Windows app compatibility (Wine/ReactOS inspired) — ongoing
- [ ] Linux app compatibility (S-COSMOS syscall layer) — ongoing
- [ ] Self-hosting build farm
- [ ] Reproducible builds (NixOS-style)

## Security Roadmap

- [ ] Full audit of bootloader raw pointer accesses
- [ ] Complete JS XSS remediation in web UI
- [ ] Formal verification of capability token bitmask logic
- [ ] Hardware memory tagging (MTE) support
- [ ] Post-quantum TLS 1.3 integration
- [ ] Secure enclave (SGX / TrustZone) support

## Performance Targets

| Metric | Target | Current |
|---|---|---|
| Boot to desktop | < 2s | ~5s (simulated) |
| Memory footprint | < 64 MB base | ~128 MB |
| Package install | < 500ms | N/A |
| Scheduler latency | < 1ms | Implemented |
| Crypto throughput | > 1 GB/s AES-256 | Native impl |

## Contributing to the Roadmap

Open a GitHub Discussion or PR to propose new items. Tag issues with milestone labels: `phase-1`, `phase-2`, etc.

See [Contributing](Contributing) for code standards.
