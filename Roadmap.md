# SigmaOS Roadmap

> **Living document** — updated each phase. Track progress in [CURRENT_PROBLEMS_MANIFEST.md](CURRENT_PROBLEMS_MANIFEST.md).

---

## Vision

SigmaOS aims to be the first production-ready, **fully sovereign** operating system:
- No glibc/musl dependency — sovereign libc only
- Post-quantum security by default (Kyber-1024, Dilithium-5)
- Declarative configuration (NixOS-inspired, without Nix)
- Multiple deployment targets: desktop, server, IoT, cloud, browser (WASM)

---

## Release Timeline

### ✅ Phase E — Core Subsystems (Complete — Q2 2026)

| # | Feature | File | Status |
|---|---------|------|--------|
| E-01 | NVMe 1.4 Driver | `drivers/storage/sigma_nvme.cpp` | ✅ |
| E-02 | USB xHCI Driver | `drivers/usb/sigma_xhci.cpp` | ✅ |
| E-03 | ACPI Power Mgmt | `kernel/power/sigma_power_manager.cpp` | ✅ |
| E-04 | Crash Reporter | `kernel/diagnostics/sigma_crash_reporter.cpp` | ✅ |
| E-05 | Ext4 JBD2 Journal | `fs/ext4_journal.c` | ✅ |
| E-06 | KMS/GPU Stubs | `drivers/graphics/sigma_kms.cpp` | ✅ |
| E-07 | PCIe MSI-X HAL | `hal/sigma_pci.cpp` | ✅ |
| E-08 | Cgroup Enforcement | `kernel/core/orchestrator/sigma_cgroup.cpp` | ✅ |
| E-09 | Sovereign Package Registry | `userland/pkg/sigma_registry.cpp` | ✅ |

### ✅ Phase F — Type Migration & Hardening (Complete — Q2 2026)

| # | Feature | File | Status |
|---|---------|------|--------|
| F-01 | USB HCD Sovereign Types | `drivers/usb/sigma_usb_hcd.cpp` | ✅ |
| F-02 | Audit Header Fix | `include/sigma_audit.h` | ✅ |
| F-03 | Full PQC API Header | `include/sigma_pqc.h` | ✅ |
| F-04 | Error Code Expansion | `include/sigma_error_codes.h` | ✅ |
| F-05 | HW Profile Bitmask | `include/sigma_profiles.h` | ✅ |
| F-06 | HWTest Include Paths | `kernel/tests/sigma_hw_test.cpp` | ✅ |
| F-07 | TCP SYN Brace Bug | `net/tcp.c` | ✅ |

---

### 🔄 Phase G — Wireless & Protocol Stacks (In Progress — Q3 2026)

| # | Feature | Target File | Status | Priority |
|---|---------|-------------|--------|----------|
| G-01 | Wi-Fi IEEE 802.11 stack | `net/wifi/sigma_wifi.cpp` | ✅ | 🔴 High |
| G-02 | Bluetooth HCI layer | `net/bt/sigma_bt_hci.cpp` | ✅ | 🔴 High |
| G-03 | WPA3/SAE authentication | `net/wifi/sigma_wpa3.cpp` | 🔄 | 🟠 Med |
| G-04 | DNS resolver | `net/dns/sigma_dns.cpp` | 🔄 | 🟠 Med |
| G-05 | TLS 1.3 (Kyber-hybrid) | `net/tls/sigma_tls.cpp` | 🔄 | 🟠 Med |
| G-06 | DHCP client | `net/dhcp/sigma_dhcp.cpp` | 🔄 | 🟡 Low |

---

### 📋 Phase H — Recovery GUI & Compositor Integration (Q3 2026)

| # | Feature | Target File | Priority |
|---|---------|-------------|----------|
| H-01 | Rescuezilla-style Recovery GUI | `zenith_desktop/recovery/sigma_recovery_gui.cpp` | 🔴 High |
| H-02 | Compositor ↔ input event wiring | `zenith_desktop/compositor/` | 🔴 High |
| H-03 | Auto-tiling compositor integration | `zenith_desktop/wm/sigma_tiling_wm.cpp` | 🟠 Med |
| H-04 | Zenith SDK app framework | `zenith_desktop/sdk/` | 🟠 Med |
| H-05 | Theme engine hot-reload | `zenith_desktop/themes/` | 🟡 Low |

---

### 📋 Phase I — First ISO Release (Q4 2026)

| # | Milestone | Notes |
|---|-----------|-------|
| I-01 | Bootable ISO (GRUB2 + SigmaOS kernel) | x86_64 target |
| I-02 | QEMU boot verified | All profiles |
| I-03 | Hardware test suite clean | All mandatory tests pass |
| I-04 | sigma-pkg functional | Install/remove/upgrade .spkg |
| I-05 | Zenith desktop launches | Compositor + shell + settings |
| I-06 | v0.1.0 release tag | GitHub Releases + wiki |

---

### 🔮 Phase J+ — Future (2027+)

- **Rust kernel modules** — Rust FFI bridge for memory-safe driver extensions
- **RISC-V port** — Architecture abstraction for HAL + MMU
- **Live kernel patching** — Hot-swap kernel modules without reboot
- **Multi-node Lattice** — Distributed microkernel across cluster nodes
- **Snap/Flatpak compat shim** — Run existing Linux apps via compat layer
- **Indian Compliance (MeitY)** — DPDPA 2023 / BIS certification track

---

## Build Profiles

```bash
make PROFILE=standalone      # x86_64 desktop (default)
make PROFILE=browser-wasm    # WebAssembly for browser
make PROFILE=cloud-native    # Containerized cloud target
make PROFILE=vm-image        # QEMU/VirtualBox VM image
make PROFILE=container-docker # Docker base image
make PROFILE=iot-arm64       # ARM64 embedded (Raspberry Pi)
make PROFILE=serverless      # Serverless function runtime
make PROFILE=forensic        # CAINE-style read-only forensic
```

---

## Contributing to the Roadmap

Have a feature idea? Open an issue with the `feat` label and tag the relevant `subsystem:*` label. Phase G/H features are especially welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).

---

### 🏛️ Phase K — Governance & Community (Q3 2026)

| # | Milestone | File | Status |
|---|-----------|------|--------|
| K-01 | Steering Committee charter | `GOVERNANCE.md` | ✅ |
| K-02 | Security disclosure policy | `SECURITY.md` | ✅ |
| K-03 | RFC process definition | `GOVERNANCE.md` §3 | ✅ |
| K-04 | Working Groups (6 WGs formed) | `GOVERNANCE.md` §2.3 | ✅ |
| K-05 | Contributor recognition tiers | `GOVERNANCE.md` §5 | ✅ |
| K-06 | SigmaOS Foundation registration | External | 📋 Planned |
| K-07 | Developer portal + forums | `https://sigma.os/community` | 📋 Planned |
| K-08 | Hackathon #1 | Community event | 📋 Planned |

---

### 🏢 Phase L — Enterprise & Sovereignty (Q4 2026)

| # | Milestone | File | Status |
|---|-----------|------|--------|
| L-01 | ISO 27001 / NIST 800-53 compliance map | `docs/COMPLIANCE.md` | ✅ |
| L-02 | GDPR / India DPDPA 2023 alignment | `docs/COMPLIANCE.md` | ✅ |
| L-03 | LTS release policy (18-month cadence) | `docs/LTS_POLICY.md` | ✅ |
| L-04 | ABI stability guarantee for LTS | `docs/LTS_POLICY.md` §ABI | ✅ |
| L-05 | Enterprise telemetry dashboard | `userland/devtools/sigma_telemetry_ui.cpp` | 📋 Planned |
| L-06 | FIPS 140-3 PQC module validation | External | 📋 2027 |
| L-07 | ISO 27001 third-party audit | External | 📋 Q3 2027 |
| L-08 | v1.0-LTS "Sigma Prime" release | All subsystems | 📋 Q4 2026 |

---

### 🔮 Phase M — Future Innovations (2027+)

| # | Feature | File | Status |
|---|---------|------|--------|
| M-01 | AI-native filesystem (AIFS) | `kernel/fs/sigma_aifs.h` | ✅ Stubbed |
| M-02 | Self-healing kernel module | `kernel/core/sigma_self_healing.h` | ✅ Stubbed |
| M-03 | AI-assisted debugger | `userland/devtools/sigma_ai_debug.h` | ✅ Stubbed |
| M-04 | Unikernel build target | `userland/init/sigma_unikernel_target.h` | ✅ Stubbed |
| M-05 | RISC-V port | Architecture HAL abstraction | 📋 2027 |
| M-06 | Live kernel patching | Hot-swap module API | 📋 2027 |
| M-07 | Multi-node Lattice (distributed μkernel) | Cluster mesh networking | 📋 2028 |
| M-08 | Snap/Flatpak compat shim | sigma_sandbox_pkg compat layer | 📋 2027 |
| M-09 | Rust kernel module FFI bridge | Rust-C interop headers | 📋 2027 |
| M-10 | Quantum-safe PKI infrastructure | SigmaOS Trust Authority (STA) | 📋 2027 |

