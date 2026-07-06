# SigmaOS — Future Development Roadmap
> Phases I–Z | Updated July 2026 | [Full detail: FUTURE_ROADMAP.md in repo](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FUTURE_ROADMAP.md)

## Phase I — Q3 2026 (Bootability)
- UEFI bootloader (`sigma-boot.efi`) — no GRUB dependency
- Bootable ISO pipeline (`make iso`)
- NVMe interrupt-driven async driver
- SATA AHCI driver
- virtio-GPU for QEMU/KVM
- Multi-monitor KMS

## Phase J — Q4 2026 (Architecture)
- ARM64 port (Raspberry Pi 5, Apple M-series stub)
- RISC-V 64 port (SBI/PLIC)
- eBPF JIT compiler (x86-64)
- Formal verification (Coq proofs)
- Linux binary compatibility (`binfmt_misc`)
- Wayland protocol (client-side)

## Phase K — Q1 2027 (Security)
- Quantum-safe TLS 1.3 (ML-KEM-768 + ML-DSA-65)
- TPM 2.0 measured boot + remote attestation
- FIDO2/WebAuthn
- Full MAC (AppArmor-inspired)
- KASLR, SMEP/SMAP, CET

## Phase L — Q2 2027 (Ecosystem)
- sigma-sdk CLI v2 (scaffold, debug, profile)
- sigma-pkg repository server (content-addressed)
- Zenith desktop v2 (tiling WM, virtual desktops, HiDPI)
- 10 bundled apps complete
- 500+ packages in registry

## Phase M — Q3 2027 (AI-Native)
- sigma-ai v2: Phi-3, Gemma-2B, DeepSeek-Coder on-device
- AI shell completion (NL → command)
- ML-guided adaptive scheduler
- Differential privacy telemetry
- ONNX model import

## Phase N — Q4 2027 (Cloud + Enterprise)
- sigma-pod v2: rootless, < 100ms startup, CRI for K8s
- Cloud images: AWS/GCP/Azure/DO
- sigma-deploy CLI
- FIPS 140-3 + PCI-DSS + SOC2 compliance

## Phase O–Z (2028+)
- India Stack: ABDM, UPI, GST, DigiLocker, ONDC
- Defence profile: MLS, air-gap, tamper-evident logs
- IoT: < 100MB, GPIO/I2C/SPI/CAN, 500ms boot
- Quantum: QPU driver, hybrid classical-quantum scheduler
- v2.0.0 Sovereign Release: production OS, 10K packages

## Driver Roadmap
| Driver | Phase | Priority |
|--------|-------|----------|
| SATA AHCI | I | 🔴 Critical |
| virtio-GPU | I | 🔴 Critical |
| RTL8125B 2.5GbE | I | 🟠 High |
| AMD GPU | J | 🟠 High |
| Intel GPU (xe) | J | 🟠 High |
| Broadcom Wi-Fi | K | 🟡 Medium |
| DisplayLink USB | L | 🟡 Medium |
| NVIDIA nouveau 2.0 | L | 🟡 Medium |

## Performance Targets
| Metric | Current | v1.0 | v2.0 |
|--------|---------|------|------|
| Boot time | ~8s | 2s | <1s |
| Syscall latency | 800ns | 300ns | <200ns |
| TCP throughput | 2 Gbps | 8 Gbps | 10 Gbps |
| NVMe IOPS | 200K | 800K | 1M |
| LLM tokens/sec | 0 | 15 | 20+ |

See full details in the [repo FUTURE_ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FUTURE_ROADMAP.md).
