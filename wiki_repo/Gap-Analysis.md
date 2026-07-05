# SigmaOS Gap Analysis — Round 32

Updated after Rounds 1–32. Compares SigmaOS against Tier 1 (Linux distros), Tier 2 (microkernels/research OSes), Tier 3 (cloud-native OSes), and India-specific requirements.

---

## Status Legend

| Symbol | Meaning |
|---|---|
| ✅ | Implemented and committed |
| 🔧 | Header/stub present — full implementation pending |
| ☐ | Not yet started |
| 🔴 | Critical — blocks production/real hardware boot |
| 🟠 | High — needed for v1.0 |
| 🟡 | Medium — quality/completeness |
| 🟢 | Low — polish/stretch goal |

---

## 🔴 CRITICAL GAPS — Block Real Hardware Boot

These five gaps mean SigmaOS **cannot run on real hardware today**. All other work is irrelevant until these are resolved.

| # | Gap | Status | Notes |
|---|---|---|---|
| 1 | Kernel scheduler implementation | ☐ | `kernel/core/sigma_sched.cpp` — MLFQ+MCS bodies missing |
| 2 | Memory manager implementation | ☐ | `kernel/core/sigma_mm.cpp` — physical/virtual MM missing |
| 3 | Syscall dispatch implementation | ☐ | `kernel/core/sigma_syscall_dispatch.cpp` missing |
| 4 | IRQ / interrupt controller | ☐ | `kernel/core/sigma_irq.cpp` — APIC/GIC missing |
| 5 | GPU / framebuffer driver | ☐ | Zenith compositor cannot run without DRM/KMS or VESA fallback |
| 6 | WiFi + Ethernet SDF drivers | ☐ | No network = no packages, no updates, no sigma-commnet |
| 7 | CryptFS real key derivation | 🔧 | `derive_key()` still returns 32 zero bytes — all encryption is fake |
| 8 | UEFI bootloader binary | ☐ | `sigma-boot.efi` does not exist yet — cannot boot without GRUB |
| 9 | Working ISO build pipeline | ☐ | `make iso` does not produce a bootable image |

---

## 🟠 HIGH PRIORITY — Needed for v1.0

### Package Management

| Gap | Status | Notes |
|---|---|---|
| Package repository server | ☐ | No `sigma-repo-server` — nowhere to host packages |
| Bootstrap package set (50 pkgs) | ☐ | bash, coreutils, curl, git, Python, GCC, Go minimum |
| `.deb` / `.rpm` / `.apk` compat | ☐ | Format resolver in sigma-pkg not implemented |
| Binary delta updates | 🔧 | `sigma_delta.h` exists — no implementation |
| India CDN mirror infrastructure | ☐ | NIC/DigitalIndia-hosted mirrors for zero foreign dependency |
| Universal package format (sigma-snapd) | ✅ | Implemented in `userland/snapd/sigma_snapd.rs` (Ubuntu Snap-inspired) |
| Multiple version support (sigma-modularity) | ✅ | Implemented in `userland/modularity/sigma_modularity.rs` (Fedora Modularity-inspired) |

### Display & Login

| Gap | Status | Notes |
|---|---|---|
| Display manager (`sigma-dm`) | ☐ | DRM/KMS-based — no X11, no Wayland |
| DID-based login screen | ☐ | Scan QR → DID auth → session. No username/password |
| sigma-pam replacement | ☐ | PAM for DID-based pluggable auth |
| Zenith WM startup | ☐ | Compositor exists as header — not integrated with DRM |

### Networking

| Gap | Status | Notes |
|---|---|---|
| TCP state machine | ☐ | `net/tcp/sigma_tcp.cpp` — full RFC 793 state machine |
| IPv6 support | ☐ | ICMPv6, SLAAC, DHCPv6 all missing |
| UDP socket layer | ☐ | UDP needed for DNS, NTP, DHCP, many apps |
| sigma-bus capability passing | ☐ | IPC header complete — cap token passing not implemented |
| sigma-busctl introspection tool | ☐ | D-Bus-compat introspection |

### QEMU CI Integration

| Gap | Status | Notes |
|---|---|---|
| Automated QEMU boot test in CI | ☐ | `test_boot_sequence.sh` exists — not wired to GitHub Actions |
| Hardware CI farm | ☐ | All tests run in software emulation only |

---

## 🟡 MEDIUM PRIORITY — Quality & Completeness

### Kernel / Architecture

| Gap | Status | Notes |
|---|---|---|
| Rust migration Phase 1 | ☐ | sigma-net, sigma-fs, SDF in Rust — planned, not started |
| sigma-dna HW profiler implementation | 🔧 | Header exists — CPUID/DMI/PCI reader not written |
| ARM64 native build | 🔧 | Stubs in `arch/arm64/` — no working cross-compile toolchain |
| RISC-V native build | 🔧 | Stubs present — not buildable |
| Formal verification | ☐ | `sigma_contracts.h` exists — no Frama-C proofs |
| SDF userspace driver ABI | 🔧 | Framework header complete — no actual driver binary produced |
| Immutable base system (sigma-ostree) | ✅ | Implemented in `userland/ostree/sigma_ostree.rs` (Fedora Silverblue/RHEL Image Mode-inspired) |

### Security

| Gap | Status | Notes |
|---|---|---|
| ML-KEM (FIPS 203) full impl | 🔧 | Kyber header present — NIST final standard bindings missing |
| ML-DSA (FIPS 204) full impl | 🔧 | Dilithium header present — NIST final standard bindings missing |
| SLH-DSA (FIPS 205) | ☐ | Hash-based signature for code signing — not started |
| sigma-pentest module | ☐ | IT Act-compliant ethical hacking tools in sigma-jail |
| TEMPEST compliance profile | ☐ | For air-gapped government/defence use |
| Module signing enforcement | 🔧 | `sigma_module_sign.h` — no kernel enforcement yet |

### India Stack

| Gap | Status | Notes |
|---|---|---|
| ABDM OAuth2 + FHIR client | ☐ | sigma-health references ABDM — no actual API client |
| GST IRN generation (IRP API) | ☐ | sigma-accounts has structs — no IRN call to NIC portal |
| e-Way Bill API client | ☐ | Transport > ₹50,000 mandatory — not implemented |
| HSN/SAC offline database | ☐ | 25,000+ codes needed for GST invoicing |
| ONDC Protocol 1.1 full client | ☐ | Buyer+seller node referenced, not implemented |
| UPI autopay / mandate | ☐ | Recurring payments via NACH/UPI Autopay |
| CBDC (e₹) wallet | ☐ | RBI retail CBDC API — not started |

### AI / ML

| Gap | Status | Notes |
|---|---|---|
| Local LLM backend | ☐ | sigma-heal/sigma-lex reference "sigma-ai analyzes" — no LLM |
| Indian LLM model integration | ☐ | Sarvam-1, OpenHathi, Krutrim GGUF models |
| sigma-bhashini offline models | 🔧 | API client exists — offline model files not bundled |
| Federated learning coordinator | ☐ | sigma_fedlearn.h client exists — no server coordinator |

---

## 🟢 LOW PRIORITY — Polish & Stretch

### Developer Experience

| Gap | Status | Notes |
|---|---|---|
| Auto-generated API docs | ☐ | Doxygen/Hawkmoth from all .h files |
| `sigma_error.h` standard | ☐ | Consistent `sigma_err_t` return type across all APIs |
| Man pages (50 more tools) | ☐ | Round 20 added 2 — need 50+ for all CLI tools |
| sigma-observatory dashboard | ☐ | Native Prometheus+Grafana equivalent in Zenith |
| D-Bus compatibility bridge | ☐ | Needed for running existing Linux apps |
| sigma-bus TLA+/Alloy model | ☐ | Formal IPC protocol specification |
| Web console (sigma-cockpit) | ✅ | Implemented in `userland/cockpit/sigma_cockpit.rs` (Fedora Cockpit-inspired) |
| Unified configuration tool (sigma-yast) | ✅ | Implemented in `userland/yast/sigma_yast.rs` (openSUSE YaST-inspired) |

### Multilingual & Accessibility

| Gap | Status | Notes |
|---|---|---|
| Indian IME (input method) | ☐ | No Inscript/phonetic keyboard for any Indian language |
| sigma-l10n catalogues | ☐ | `sigma_locale.h` exists — translation strings not written |
| Braille display support | ☐ | AT-SPI2 screen reader exists — no Braille output |
| Switch access (motor impairment) | ☐ | Single-switch scanning interface for motor-disabled users |

---

## Tier 1 Gap Analysis — vs Linux Distributions

| Feature Area | Ubuntu 24.04 | Fedora 41 | Debian 12 | SigmaOS |
|---|---|---|---|---|
| PQ cryptography | ❌ | ❌ | ❌ | 🔧 (header complete) |
| Atomic A/B updates | ❌ | ❌ | ❌ | ✅ |
| DID identity | ❌ | ❌ | ❌ | 🔧 (no login UI yet) |
| India compliance | ❌ | ❌ | ❌ | 🔧 (no API clients) |
| Self-heal | ❌ | ❌ | ❌ | 🔧 (header complete) |
| Live kernel patch | Paid | ❌ | ❌ | 🔧 (header complete) |
| ABI-stable drivers | ❌ | ❌ | ❌ | 🔧 (framework only) |
| Memory safety (Rust) | ❌ | ❌ | ❌ | ☐ (Phase 1 planned) |
| Real-time scheduler | Optional | Optional | Optional | ✅ |
| Immutable root | ❌ | ❌ | ❌ | ✅ |
| Reproducible builds | Partial | Partial | ✅ | ✅ |
| Universal package format | Snap (Ubuntu) | ❌ | ❌ | ✅ (sigma-snapd) |
| Multiple version support | ❌ | Modularity (Fedora) | ❌ | ✅ (sigma-modularity) |
| Web console | ❌ | Cockpit (Fedora) | ❌ | ✅ (sigma-cockpit) |
| Unified config tool | ❌ | ❌ | ❌ | ✅ (sigma-yast) |

---

## Tier 2 Gap Analysis — vs Microkernels / Research OSes

| Feature | seL4 | MINIX 3 | Genode | Haiku | SigmaOS |
|---|---|---|---|---|---|
| Capability security | ✅ | ❌ | ✅ | ❌ | ✅ |
| Reincarnation server | ❌ | ✅ | ❌ | ❌ | ✅ |
| Formal verification | ✅ | ❌ | ❌ | ❌ | ☐ |
| Real-time scheduler | ✅ | ❌ | ✅ | ❌ | ✅ |
| Userspace drivers | ✅ | ✅ | ✅ | ✅ | 🔧 |
| India compliance | ❌ | ❌ | ❌ | ❌ | 🔧 |
| PQ cryptography | ❌ | ❌ | ❌ | ❌ | 🔧 |
| DID identity | ❌ | ❌ | ❌ | ❌ | 🔧 |

---

## Tier 3 Gap Analysis — vs Cloud-Native OSes

| Feature | Talos | Bottlerocket | Flatcar | NixOS | SigmaOS |
|---|---|---|---|---|---|
| Immutable root | ✅ | ✅ | ✅ | ❌ | ✅ |
| Atomic A/B | ✅ | ✅ | ✅ | ✅ | ✅ |
| Reproducible builds | ❌ | Partial | ❌ | ✅ | ✅ |
| PQ cryptography | ❌ | ❌ | ❌ | ❌ | 🔧 |
| India compliance | ❌ | ❌ | ❌ | ❌ | 🔧 |
| gRPC management API | ✅ | ✅ | ❌ | ❌ | 🔧 |
| Self-heal | ❌ | ❌ | ❌ | ❌ | 🔧 |
| Live kernel patch | ❌ | ✅ | ❌ | ❌ | 🔧 |

---

## 🚨 CRITICAL BLOCKING GAPS

The following gaps currently prevent SigmaOS from functioning like established Linux distros (Ubuntu, Fedora, Arch, Debian). These must be addressed before SigmaOS can be considered a functional operating system.

### 🧩 Core System Gaps (BLOCKING - Phase 1)
| Gap | Priority | Impact |
|---|---|---|
| No stable kernel or hardware driver layer | 🔴 CRITICAL | Cannot boot on real hardware |
| No tested installer and bootloader integration | 🔴 CRITICAL | Cannot install system |
| No service manager (systemd, OpenRC, etc.) defined | 🔴 CRITICAL | Cannot manage system services |
| No modular kernel builds (desktop, server, mobile) | 🔴 CRITICAL | Cannot optimize for different use cases |
| No update mechanism (rolling + LTS) | 🔴 CRITICAL | Cannot keep system updated |

### ⚙️ Package Ecosystem Gaps (BLOCKING - Phase 2)
| Gap | Priority | Impact |
|---|---|---|
| sigpkg is proposed but not fully implemented | 🔴 CRITICAL | Cannot install software |
| No central repo of software packages | 🔴 CRITICAL | No software distribution |
| Missing package signing, updates, rollback | 🔴 CRITICAL | Cannot trust or manage packages |
| No GUI shell (GNOME/KDE/XFCE equivalent) | 🔴 CRITICAL | No desktop environment |
| Display server integration missing (Wayland/X.Org) | 🔴 CRITICAL | No graphical interface |

### 🎨 Desktop & User Experience Gaps (BLOCKING - Phase 2)
| Gap | Priority | Impact |
|---|---|---|
| Accessibility tools absent (screen readers, multilingual UI) | 🟠 HIGH | Not accessible to all users |
| No theme store or customization hub | 🟡 MEDIUM | Poor user experience |
| No input method integration for Indic scripts | 🟠 HIGH | Cannot type in Indian languages |

### 🔒 Security & Privacy Gaps (HIGH PRIORITY - Phase 4)
| Gap | Priority | Impact |
|---|---|---|
| No QubesOS-style sandboxing/isolation | 🟠 HIGH | Security vulnerability |
| No Suricata/Snort integration | 🟠 HIGH | No network threat detection |
| AI transparency logging not implemented | 🟠 HIGH | Cannot audit AI decisions |
| No KeePassXC integration | 🟡 MEDIUM | No password management |

### 🧠 AI & Automation Gaps (HIGH PRIORITY - Phase 4)
| Gap | Priority | Impact |
|---|---|---|
| SigmaAI Agent still conceptual, not embedded | 🟠 HIGH | No natural language control |
| No n8n/Airflow-style workflow engine | 🟡 MEDIUM | No automation capabilities |
| Natural language → CLI translator not functional | 🟠 HIGH | No voice/text control |

### 📘 Education & Professional Tools Gaps (MEDIUM PRIORITY - Phase 3)
| Gap | Priority | Impact |
|---|---|---|
| CBSE Tools (GeoGebra, Scilab, OpenBoard) not bundled | 🟡 MEDIUM | Not suitable for education |
| Professional Modules (ERPNext, Koha, QGIS, GNUCash) missing | 🟡 MEDIUM | Not suitable for professional use |
| Indic language packs not integrated | 🟠 HIGH | Not accessible in Indian languages |

---

## New Gaps Identified — Rounds 29–35

The following gaps were discovered while implementing self-heal, commnet, continuous auth, federated learning, the XR/DataSov platform, Linux distro-inspired components, open source project-inspired components, and the new categorized component structure:

| Gap | Discovered While | Priority |
|---|---|---|
| No local LLM — sigma-heal AI analysis is a stub | sigma-heal implementation | 🟠 |
| No ZK-SNARK library — sigma-datasov ZK proofs unimplemented | sigma_datasov.h | 🟡 |
| No WebXR/OpenXR runtime binary — sigma-xr has no runnable code | sigma_xr.h | 🟡 |
| No federated learning coordinator server | sigma_fedlearn.h | 🟡 |
| No IoT sensor protocol stack (MQTT/Modbus/OPC-UA) | sigma_digital_twin.h | 🟡 |
| No Indian IME for sigma-gamelearn text input | sigma_gamelearn.h | 🟠 |
| No biometric hardware driver (fingerprint/iris) | sigma_continuous_auth.h | 🟠 |
| sigma-commnet needs iptables/nftables NAT — no implementation | sigma_commnet.h | 🟠 |
| Snapshot & rollback system (sigma-snapper) — not implemented | Linux distro research | 🟡 |
| Build service infrastructure (sigma-obs) — not implemented | Linux distro research | 🟡 |
| Application catalog (sigma-appstreams) — not implemented | Linux distro research | 🟡 |
| GUI package manager (sigma-pamac) — not implemented | Linux distro research | 🟡 |
| Rust-based desktop (sigma-cosmic) — not implemented | Linux distro research | 🟡 |
| User-friendly desktop (sigma-pantheon) — not implemented | Linux distro research | 🟡 |
| System utilities (sigma-mint-tools) — not implemented | Linux distro research | 🟡 |
| Guided installer (sigma-archinstall) — not implemented | Linux distro research | 🟡 |
| Fast package manager (sigma-pacman) — not implemented | Linux distro research | 🟡 |
| ISO build system (sigma-live-build) — not implemented | Linux distro research | 🟡 |
| Advanced storage (sigma-stratis) — not implemented | Linux distro research | 🟡 |
| Mandatory access control (sigma-selinux) — not implemented | Linux distro research | 🟡 |
| Telemetry and analytics (sigma-insights) — not implemented | Linux distro research | 🟡 |
| Configuration database (sigma-debconf) — not implemented | Linux distro research | 🟡 |
| Rolling release model (sigma-rolling) — not implemented | Linux distro research | 🟡 |
| Digital painting suite (sigma-paint) — not implemented | Open source research | 🟡 |
| Vector graphics editor (sigma-vector) — not implemented | Open source research | 🟡 |
| 3D creation suite (sigma-3d) — not implemented | Open source research | 🟡 |
| Photography workflow (sigma-photo) — not implemented | Open source research | 🟡 |
| Live streaming studio (sigma-stream) — not implemented | Open source research | 🟡 |
| Audio production suite (sigma-audio) — not implemented | Open source research | 🟡 |
| Office suite (sigma-office) — not implemented | Open source research | 🟡 |
| Collaborative office (sigma-collab) — not implemented | Open source research | 🟡 |
| Privacy-focused notes (sigma-notes) — not implemented | Open source research | 🟡 |
| Reference manager (sigma-ref) — not implemented | Open source research | 🟡 |
| Email client (sigma-mail) — not implemented | Open source research | 🟡 |
| Self-hosted cloud (sigma-cloud) — not implemented | Open source research | 🟡 |
| Privacy browser (sigma-privacy) — not implemented | Open source research | 🟡 |
| Modern web browser (sigma-web) — not implemented | Open source research | 🟡 |
| Secure VPN (sigma-vpn) — not implemented | Open source research | 🟡 |
| Machine learning framework (sigma-ml) — not implemented | Open source research | 🟡 |
| Mathematics software (sigma-math) — not implemented | Open source research | 🟡 |
| Interactive whiteboard (sigma-edu) — not implemented | Open source research | 🟡 |
| Library management (sigma-library) — not implemented | Open source research | 🟡 |
| Business management (sigma-erp) — not implemented | Open source research | 🟡 |
| Geographic information system (sigma-gis) — not implemented | Open source research | 🟡 |
| Personal finance (sigma-finance) — not implemented | Open source research | 🟡 |
| Rolling release model (sigma-rolling) — not implemented | Core system research | 🟡 |
| Long-term support release (sigma-lts) — not implemented | Core system research | 🟡 |
| System init choices (sigma-init) — not implemented | Core system research | 🟡 |
| Kernel customization (sigma-kernel-custom) — not implemented | Core system research | 🟡 |
| Multiple desktop environments (sigma-desktops) — not implemented | Desktop UX research | 🟡 |
| Display server options (sigma-display) — not implemented | Desktop UX research | 🟡 |
| Accessibility tools (sigma-a11y) — not implemented | Desktop UX research | 🟡 |
| Multilingual UI (sigma-i18n) — not implemented | Desktop UX research | 🟡 |
| Theme & extension store (sigma-theme-store) — not implemented | Desktop UX research | 🟡 |
| Home theater media center (sigma-kodi) — not implemented | Creative tools research | 🟡 |
| Digital audio workstation (sigma-ardour) — not implemented | Creative tools research | 🟡 |
| RAW photo editor (sigma-rawtherapee) — not implemented | Creative tools research | 🟡 |
| Version control integration (sigma-git) — not implemented | Development tools research | 🟡 |
| Kubernetes orchestration (sigma-k8s) — not implemented | Development tools research | 🟡 |
| IT automation framework (sigma-automation) — not implemented | Development tools research | 🟡 |
| Interactive notebooks (sigma-jupyter) — not implemented | Development tools research | 🟡 |
| Statistical computing (sigma-rstudio) — not implemented | Development tools research | 🟡 |
| Password manager (sigma-password) — not implemented | Security tools research | 🟡 |
| Antivirus protection (sigma-av) — not implemented | Security tools research | 🟡 |
| Intrusion detection (sigma-ids) — not implemented | Security tools research | 🟡 |
| Audit trail tools (sigma-audit) — not implemented | Security tools research | 🟡 |
| Healthcare records (sigma-health) — not implemented | Indian professional research | 🟡 |
| Engineering CAD (sigma-cad) — not implemented | Indian professional research | 🟡 |
| Regional language integration (sigma-boss) — not implemented | Indian professional research | 🟡 |
| System snapshot manager (sigma-snapshot) — not implemented | Distro absorption research | 🟡 |
| Community package repository (sigma-aur) — not implemented | Distro absorption research | 🟡 |
| System management suite (sigma-mint-tools) — not implemented | Distro absorption research | 🟡 |
| Lightweight C library (sigma-musl) — not implemented | Distro absorption research | 🟡 |
| Lightweight init system (sigma-runit) — not implemented | Distro absorption research | 🟡 |
| Declarative configuration (sigma-declarative) — not implemented | Distro absorption research | 🟡 |
| System configuration tool (sigma-yast2) — not implemented | Distro absorption research | 🟡 |
| Mandatory access control (sigma-selinux) — not implemented | Distro absorption research | 🟡 |
| Modern desktop environment (sigma-cosmic) — not implemented | Distro absorption research | 🟡 |
| User-friendly desktop (sigma-pantheon) — not implemented | Distro absorption research | 🟡 |
| Unified settings hub (sigma-budgie) — not implemented | Distro absorption research | 🟡 |
| Fast package manager (sigma-xbps) — not implemented | Distro absorption research | 🟡 |
| Source-based package manager (sigma-portage) — not implemented | Distro absorption research | 🟡 |

---

## Priority Queue — Recommended Next Rounds

### Round 37 — Distro Absorption Round 1 (NEW)
1. sigma-snapshot — System snapshot manager (openSUSE Snapper-inspired)
2. sigma-aur — Community package repository (Arch AUR-inspired)
3. sigma-mint-tools — System management suite (Linux Mint-inspired)
4. sigma-musl — Lightweight C library (Alpine-inspired)
5. sigma-runit — Lightweight init system (Void Linux-inspired)

### Round 36 — Categorized Components (NEW)
1. sigpkg — Universal package manager (Core System)
2. sigma-code — Code editor suite (Development & IT)
3. sigma-containers — Containerization (Development & IT)
4. sigma-i18n — Multilingual UI (Desktop & UX)
5. sigma-a11y — Accessibility tools (Desktop & UX)

### Round 35 — Open Source Project Components (NEW)
1. sigma-paint — Digital painting suite (Krita/GIMP-inspired)
2. sigma-office — Office suite (LibreOffice-inspired)
3. sigma-mail — Email client (Thunderbird-inspired)
4. sigma-cloud — Self-hosted cloud (Nextcloud-inspired)
5. sigma-web — Modern web browser (Firefox/Chromium-inspired)

### Round 34 — Linux Distro Components (NEW)
1. sigma-snapper — Snapshot & rollback system (openSUSE Snapper-inspired)
2. sigma-obs — Build service infrastructure (openSUSE OBS-inspired)
3. sigma-appstreams — Application catalog (RHEL AppStreams-inspired)
4. sigma-pamac — GUI package manager (Manjaro PAMAC-inspired)
5. sigma-cosmic — Rust-based desktop (Pop!_OS COSMIC-inspired)

### Round 33 — Make It Boot
1. VESA/GOP framebuffer driver (get pixels on screen)
2. Minimal scheduler implementation (`sigma_sched.cpp` — round-robin first)
3. QEMU boot test in CI (assert boots to shell)
4. `make iso` pipeline producing a bootable ISO

### Round 35 — Make It Connect  
1. TCP/UDP socket layer implementation
2. Basic WiFi SDF driver (iwlwifi or cfg80211 userspace)
3. sigma-pkg talking to a real repo server
4. sigma-bus IPC running end-to-end

### Round 36 — Make It Secure
1. Real Argon2id CryptFS key derivation (fix Issue #44)
2. TPM2 seal/unseal for disk key
3. DID login screen replacing username/password
4. sigma-trustd Dilithium3 certificate chain end-to-end

### Round 37 — Make It Indian
1. ABDM FHIR client (sigma-health goes live)
2. GST IRN API client (sigma-accounts e-invoice goes live)
3. IndiaStack UPI autopay working
4. Bhashini offline model bundle (22-language ASR/TTS)
5. Indian IME (Inscript + phonetic for Devanagari)

### Round 38 — Make It Smart
1. Local LLM integration (sigma-ai with llama.cpp backend)
2. sigma-heal AI analysis using local model
3. sigma-lex Gazette parser using local NLP
4. Federated learning coordinator server

---

*See also: [Future Development Ideas](Future-Development-Ideas) · [Improvements Overview](Improvements-Overview) · [Feature Roadmap](Feature-Roadmap) · [Architecture Overview](Architecture-Overview)*
