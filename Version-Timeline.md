# SigmaOS Version Timeline

Official release roadmap showing the progression from current build to long-term vision.

---

## Release Versions

### v15.0 Zenith — NOW (Current)
**Status: Active development**

- 600-shard boot lattice architecture
- QEMU boot validated
- CI/CD pipeline active (GitHub Actions)
- 50+ India profession app headers
- 24 Web API browser drivers (sigma-web)
- sigma-heal, sigma-commnet, sigma-bhashini headers
- PQC headers (Kyber + Dilithium — not yet full integration)

**Active branches:** `main`, `kernel-exp`, `drivers-dev`, `fs-dev`, `performance-optimized`, `tools-dev`, `release/standalone`, `release/microkernel`, `release/rtos`, `release/mobile`, `release/cloud`, `release/distributed`, `release/dual-boot`, `release/browser`, `release/app`, `docs-update`, `gh-pages`, `prepare-sigmaos-launch`, `master`

---

### v16.0 Apex — Q4 2026
**Status: Planned**

Target capabilities:
- Full TCP/UDP/IPv6 stack
- Stable sigma-pkg (SPM) with Dilithium3-signed packages
- Gaming layer alpha (Vulkan compositor foundation)
- ARM64 build (Raspberry Pi 4 boot)
- sigma-ai local LLM (Sarvam-1 at 4GB RAM)
- DID-based login (no username/password)

Exit criteria:
- Boots on real x86_64 hardware
- `sigma-pkg install firefox` works
- sigma-bhashini Hindi TTS working offline

---

### v17.0 Sovereign — Q2 2027
**Status: Planned**

Target capabilities:
- Sovereign containers GA (`sigma-pod` with full kernel enforcement)
- Enterprise branch (`release/enterprise`) with MDM, sigma-fleet
- BharatOS pilot preparation (NIC proposal)
- sigma-lex Gazette parser live
- ABDM FHIR client + GST IRN client production-ready
- sigma-commnet TRAI-certified

Exit criteria:
- `sigma-pod run-native` applies cgroup limits in kernel path
- CA files GSTR using sigma-ca
- Doctor writes ABDM-linked prescription using sigma-health

---

### v18.0 Singularity — 2028+
**Status: Future vision**

Target capabilities:
- AI-assisted scheduler (sigma-ai in kernel scheduling path)
- Post-quantum crypto everywhere by default (ML-KEM + ML-DSA final NIST standards)
- Self-healing kernel (sigma-heal + sigma-ai live integration)
- Formally verified IPC and scheduler (IIT/IISc collaboration)
- Rust network stack (zero memory-safety CVEs)
- sigma-telco (O-RAN 5G deployment on SigmaOS)
- 1 million DID-signed skill credentials (ONEST)

---

## Development Phases Overview

| Phase | Focus | Target | Status |
|---|---|---|---|
| **Phase 0** | Core boot — scheduler, MM, syscall, ISO | Month 3 | `[ ]` |
| **Phase 1** | Networking + packages — TCP, drivers, sigma-pkg | Month 6 | `[~]` |
| **Phase 2** | Desktop + identity — GPU, Zenith, DID, IME | Month 9 | `[~]` |
| **Phase 3** | India Stack live — ABDM, GST, UPI, NavIC | Month 14 | `[ ]` |
| **Phase 4** | Security hardening — PQC final, TPM2, sigma-mac | Month 18 | `[~]` |
| **Phase 5** | Multi-platform — ARM64, RISC-V, sigma-ultra | Month 21 | `[~]` |
| **Phase 6** | AI & advanced — fedlearn, sigma-lex, ZK proofs | Month 24 | `[ ]` |
| **Phase 7** | Enterprise & government — BharatOS pilot | Month 36 | `[ ]` |
| **Phase 8** | Rural India — 1,000 villages | Month 42 | `[ ]` |
| **Phase 9** | Research — formal verification, Rust | Month 60 | `[ ]` |

---

## Immediate Priorities (Next 90 days — Phase A)

These are the highest-leverage tasks that unblock everything downstream:

### 1. Get SigmaOS Booting (Weeks 1–4)
```
kernel/core/sigma_sched.cpp    — round-robin scheduler
kernel/core/sigma_mm.cpp       — buddy allocator + slab
kernel/core/sigma_syscall_dispatch.cpp — dispatch table
sigma-boot.efi                 — UEFI PE binary
make iso                       → bootable SigmaOS.iso
qemu-system-x86_64 -cdrom SigmaOS.iso → shell prompt
```
Record a 60-second video. Post on Hacker News. That post changes everything.

### 2. Networking (Weeks 3–6)
```
Complete TCP state machine      → kernel/net/sigma_net_tcp.cpp
Fix ARP stub                    → kernel/net/sigma_net_arp.cpp
Single socket ABI               → remove SovereignSocketAPI.cpp duplicate
```

### 3. Package Infrastructure (Weeks 5–8)
```
sigma-repo-server (Go)          → serve packages over HTTPS + Dilithium3
50 bootstrap packages           → bash, coreutils, curl, git, Python, GCC
India CDN mirror                → packages.sigmaos.dev + NIC mirror
```

### 4. Desktop (Weeks 7–12)
```
Compositor input event loop     → zenith_desktop/compositor/
Auto-tiling WM                  → sigma_tiling_wm.cpp
~/.sigma_profile VFS load       → sigma_profile_engine.cpp
```

---

## Competitive Surpass Timeline

```
NOW:       Design document with 600 shards + 50 profession apps
Month 3:   First QEMU boot → "SigmaOS is real hardware software"
Month 6:   Real x86 boot + WiFi → Indian devs can test it
Month 9:   Zenith desktop + DID login → demo-able product
Month 14:  GST + ABDM live → first CA/doctor user
Month 24:  AI native + ZK proofs → technically most advanced India OS
Month 36:  BharatOS pilot → government traction
Month 42:  1,000 villages → impact at scale
```

---

## How to Contribute

**Phase 0 — needs urgently:**
- C++ kernel engineers (scheduler, memory manager, IRQ)
- UEFI/EDK2 engineers (sigma-boot.efi)
- Build system engineers (`make iso` pipeline)

**Phase 2–3 — needs:**
- Rust engineers (network stack, VFS)
- India fintech engineers (ABDM FHIR, GST IRN, UPI)
- GPU/graphics engineers (DRM/KMS, Mesa, Vulkan)
- Language/NLP engineers (Inscript IME, LLM integration)

Open issues: [github.com/AaryanSinghChauhan09/SigmaOS/issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

---

*See also: [Development Roadmap](Development-Roadmap) · [Phase A Execution Checklist](Phase-A-Execution-Checklist) · [Gap Analysis](Gap-Analysis)*
