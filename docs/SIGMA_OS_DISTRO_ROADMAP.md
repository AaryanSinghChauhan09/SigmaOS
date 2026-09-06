# SigmaOS Linux & BSD Distro Development Roadmap & Issue Matrix

This roadmap details prioritized development initiatives inspired by leading Linux & BSD distribution paradigms (Arch, Fedora, FreeBSD, OpenBSD, NixOS, Void Linux, Clear Linux, Haiku).

---

## 🛡️ 1. Security & Sandboxing

### 1.1 Per-Tab Capability Model (Pledge / Unveil / Capsicum)
- **Status:** Implemented in `src/distro/wiki_ideas_implementation.rs` & `src/security/input_validation.rs`.
- **Target:** Enforce least privilege process boundaries at launch time across Linux (seccomp/bpf), macOS (app sandbox), and BSDs (Capsicum/pledge/unveil).
- **Metric:** 100% of renderer helper processes launched in sandboxed capability mode.

### 1.2 SBOM Generation, Sigstore Cosign Signing & CVE Scanning
- **Status:** Implemented in `.github/workflows/sbom-cosign-cve-scan-ci.yml` & `src/sigpkg/verifier.rs`.
- **Target:** Automated SPDX/CycloneDX SBOM generation, Post-Quantum Dilithium-5 / Cosign signing, and vulnerability tracking via `SecurityAdvisoryTracker`.
- **Metric:** Zero unmitigated critical CVEs in production release artifacts.

---

## 🚀 2. Release Engineering & Atomic Updates

### 2.1 Atomic Updates & One-Click Rollback (OSTree / Snapper Parity)
- **Status:** Implemented in `src/compatibility/fedora.rs` (`FedoraOfflineUpdateEngine`) & `src/distro/wiki_ideas_implementation.rs` (Btrfs/Snapper recovery).
- **Target:** Transactional offline system updates with instant subvolume rollback.
- **Metric:** 100% update success rate with 0 bricked system states.

### 2.2 Reproducible Build Verification Pipeline
- **Status:** Implemented in `src/sigpkg/sovereign_sigpkg.rs` (`ReproducibleBuildContext`) & `src/arch_kernel_inspirations.rs`.
- **Target:** Diffoscope-style bit-for-bit build verification normalized via `SOURCE_DATE_EPOCH`.
- **Metric:** > 99% bit-for-bit binary reproducibility across release targets.

---

## ⚡ 3. Process Control & System Supervision

### 3.1 Void Linux Runit Supervisor & Automated Health Checks
- **Status:** Implemented in `src/distro/void_runit.rs`.
- **Target:** 3-stage process supervision with automated failure threshold detection and recovery.
- **Metric:** < 100ms service restart latency on failure.

### 3.2 Linux Cgroups v2 & FreeBSD rctl Quota Enforcement
- **Status:** Implemented in `src/memory/resource_allocator.rs` & `src/unimplemented_tools.rs`.
- **Target:** Strict per-process memory, CPU, and IO bandwidth limits.
- **Metric:** Zero system-wide out-of-memory (OOM) lockups under high render load.

---

## 🖥️ 4. Native Desktop & JS Reduction

### 4.1 Native WASM Desktop UI & Accessibility Engine
- **Status:** Implemented in `zenith_desktop/src/lib.rs` & `src/desktop/web_wasm_bridge.rs`.
- **Target:** Direct Rust/WASM event routing for keyboard focus, ARIA attributes, and DOM manipulation without JavaScript runtime overhead.
- **Metric:** 0ms JavaScript event loop blockage during desktop navigation.
