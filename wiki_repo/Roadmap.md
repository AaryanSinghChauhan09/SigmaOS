# SigmaOS Development Roadmap

9 phases over 60 months from "design document" to "1,000 villages running SigmaOS".
Full details: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Development-Roadmap

---

## Milestones

| Milestone | Month | Definition of Done |
| --- | --- | --- |
| M0: First Boot | 3 | `qemu-system-x86_64 -cdrom SigmaOS.iso` reaches shell |
| M1: Real Hardware | 6 | Boots on real x86 laptop, WiFi + packages work |
| M2: First Desktop | 9 | Zenith DE, DID login, Hindi IME, sigma-ai local |
| M3: India Stack Live | 14 | CA files GSTR, doctor writes ABDM prescription |
| M4: Security Audit | 18 | Zero critical CVEs, full PQ crypto, TPM2 boot chain |
| M5: ARM64 | 21 | Raspberry Pi 4 + sigma-ultra on Pi Zero |
| M6: AI Native | 24 | sigma-heal+sigma-lex+sigma-fedlearn all live |
| M7: Govt Pilot | 30 | 1,000 NIC machines, BharatOS pilot |
| M8: 1,000 Villages | 42 | sigma-RuralStack in 1,000 villages across 5 states |
| M9: Verified Kernel | 60 | Formally verified IPC + scheduler, Rust network stack |

---

## Phase Summary

### Phase 0 — Foundation (Months 1–3): MAKE IT BOOT

- kernel/core/{sched,mm,syscall_dispatch,irq}.cpp

- sigma-boot.efi (UEFI PE binary)

- VESA/GOP framebuffer

- `make iso` → bootable ISO

- QEMU CI in GitHub Actions

### Phase 1 — Networking & Packages (Months 3–6): MAKE IT CONNECT

- TCP/UDP/IPv6 stack

- Ethernet + WiFi SDF drivers (e1000, virtio-net, r8169, iwlwifi, mt7921)

- sigma-repo-server + India CDN mirror

- sigma-pkg end-to-end install

- Argon2id CryptFS (fix Issue #44)

### Phase 2 — Desktop & Identity (Months 6–9): MAKE IT USABLE

- GPU drivers (i915, amdgpu, virtio-gpu)

- Zenith compositor live on DRM/KMS

- DID-based login (QR scan → desktop)

- Indian language IME (Inscript + phonetic)

- sigma-bhashini offline models

- sigma-ai local LLM (Sarvam-1, 4GB RAM)

### Phase 3 — India Stack Live (Months 9–14): MAKE IT INDIAN

- ABDM FHIR client (Health ID, PHR, PMJAY)

- GST IRN + e-Way Bill + GSTR filing

- UPI Autopay + e-RUPI + AA consent

- NavIC integration

- sigma-accounts + sigma-health production ready

### Phase 4 — Security Hardening (Months 12–18): MAKE IT TRUSTED

- ML-KEM/ML-DSA/SLH-DSA (FIPS 203/204/205 final)

- Continuous auth live (biometric all signals)

- sigma-mac policy engine live

- sigma-ids + sigma-heal integration

- TPM2 full Secure Boot chain

### Phase 5 — Multi-Platform (Months 15–21): MAKE IT EVERYWHERE

- ARM64: Raspberry Pi 4/5, sigma-ultra on Pi Zero

- RISC-V build toolchain

- sigma-commnet: TRAI-certified, BharatNet PoP

- sigma-ultra USSD mode on 2G

### Phase 6 — AI & Advanced (Months 18–24): MAKE IT SMART

- Federated learning coordinator live

- sigma-heal AI crash analysis real

- sigma-lex Gazette parser live

- sigma-twin with real IoT sensors

- ZK proofs working (groth16)

### Phase 7 — Enterprise & Government (Months 24–36): MAKE IT OFFICIAL

- BharatOS pilot: 1,000 NIC machines

- sigma-fleet: 10,000+ device management

- Hardware partnerships: Lava/Micromax OEM

- STQC + MeitY empanelment

- sigma-defense DRDO evaluation

### Phase 8 — Rural India (Months 30–42): MAKE IT UNIVERSAL

- sigma-RuralStack: 1,000 pilot villages

- BharatNet PoP certification

- sigma-gram for all 250,000 panchayats

- MGNREGS payment latency < 24 hours

### Phase 9 — Research (Months 36–60): MAKE IT FOREVER

- Formal verification: IPC + scheduler (IIT/IISc collab)

- Rust migration complete: zero memory-safety CVEs

- sigma-telco: O-RAN 5G on SigmaOS

- sigma-zkvm: banks share risk models without raw data

- ONEST: 1M DID-signed skill credentials

---

## Critical Path (What Blocks What)

```
Kernel implementations (Phase 0)
    └── ISO build
        └── QEMU CI
            └── Real hardware boot
                └── Network stack (Phase 1)
                    └── Package repo
                        └── GPU drivers (Phase 2)
                            └── Zenith desktop
                                └── DID login
                                    └── IME + sigma-ai
                                        └── India Stack APIs (Phase 3)
                                            └── Production launch
```

The kernel implementations in Phase 0 are the single critical-path item.
Everything else is blocked until a real scheduler, MM, and syscall table exist.

---

## How to Contribute

### Phase 0 needs most urgently:

- C++ kernel engineers (scheduler, memory manager)

- UEFI/EDK2 engineers (bootloader)

- Build system engineers (ISO pipeline)

### Phase 2–3 needs:

- Rust engineers (network stack, VFS)

- India fintech engineers (GST, ABDM, UPI API clients)

- GPU/graphics engineers (DRM/KMS, Mesa)

- Language/NLP engineers (IME, LLM integration)

See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to get involved.
See the [wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Development-Roadmap) for full phase details.
