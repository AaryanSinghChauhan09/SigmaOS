# Roadmap

SigmaOS development follows a phased approach. Each phase ships a complete, usable feature set before the next begins.

---

## Current Status: Phase G (Active)

---

## Phase Timeline

### ✅ Phase E — Core Subsystems (Q2 2026)
- SovereignVMM: 4-level page tables, CoW, demand paging
- Zero-Trust Engine: PAM/ACL, capability rings, runtime threat scoring
- RFC-793 TCP/IP stack from scratch
- Ext4/JBD2 full read/write filesystem
- NVMe 1.4 + xHCI USB 3.x drivers
- ACPI power management

### ✅ Phase F — Type Hardening & Security (Q2 2026)
- Post-quantum crypto: Kyber-1024 KEM + Dilithium-5 signatures
- sigma-pkg: Reproducible + verifiable `.spkg` packages
- Immutable CRC32C-checksummed audit log
- CoW and TCP state machine fuzzing test harnesses

### 🔄 Phase G — Wi-Fi, Audio & Web Desktop (Q3 2026 — Active)
- [x] mac80211-style 802.11 state machine (`sigma_80211.cpp`)
- [x] HDA codec enumeration + ALSA mixer API (`sigma_hda.cpp`)
- [x] Go daemons: `sigmad-process`, `sigmad-ai`, `sigmad-sync`
- [x] Web shell launcher + App Store (`userland/web-shell/`)
- [x] `navigator.sigmaos` extension API (`inject.js`)
- [x] 3 demo apps: SigmaNotes, SigmaTerm, SigmaPaint
- [x] Capability grant UI (`settings/caps.html`)
- [x] `pkg.ensure` — Alpine `apk` in user namespace
- [ ] Intel i915 modesetting: full scanout on real hardware
- [ ] Wi-Fi PHY driver for Intel AX200/AX210

### 📋 Phase H — Recovery & Forensics (Q3 2026)
- Recovery GUI shell (boot into recovery mode)
- Ext4 read-only forensic mounting (`sigma_recovery_mount_ro`)
- JBD2 journal replay tool
- Snapshot rollback UI
- Bootloader repair automation

### 📋 Phase I — First ISO Release (Q4 2026)
- Polished first-boot experience
- App store with 10+ community apps
- `rclone`-based workspace autosync wizard
- Public ISO download on GitHub Releases
- `docs.sigmaos.dev` hosted on gh-pages

### 📋 Phase J — v0.2 Features (2027)
- `navigator.sigmaos.usb` — Arduino/3D printer access
- `navigator.sigmaos.window` — native frameless OS windows from web apps
- Magic Theme engine — recolor any site to match OS palette
- Bluetooth HCI driver
- Full AMDGPU KMS acceleration

### 📋 Phase K — Mobile & RTOS (2027)
- ARM64 port (Raspberry Pi 4/5, Apple Silicon via UTM)
- `release/mobile` profile: touch UI compositor
- `release/rtos` profile: deterministic scheduling for robotics
- Power-optimized build for embedded SoCs

---

## Branch → Phase Mapping

| Branch | Phase |
|--------|-------|
| `main` | Active development |
| `release/standalone` | Phase I ISO |
| `release/cloud` | Phase J cloud-native |
| `release/mobile` | Phase K ARM64 |
| `release/rtos` | Phase K real-time |
| `release/browser` | Phase J WASM runtime |
| `release/dual-boot` | Phase I dual-boot installer |
| `performance-optimized` | Ongoing — AVX-512, hugepages |
| `prepare-sigmaos-launch` | Phase I launch assets |

---

## Metrics Goals

| Metric | v0.1 Target | v1.0 Target |
|--------|-------------|-------------|
| Boot time (QEMU) | < 5s | < 2s |
| ISO size | < 200MB | < 150MB |
| Kernel LOC | ~15k | ~25k |
| Daemon LOC | ~2.2k Go | ~4k Go |
| Test coverage | VMM + TCP | All subsystems |
