# SigmaOS Master Execution Roadmap

> 📖 **Practical Linux & BSD Distro Roadmap:** See [`docs/SIGMA_OS_DISTRO_ROADMAP.md`](docs/SIGMA_OS_DISTRO_ROADMAP.md) for the detailed 11-pillar strategic blueprint combining Linux hardware support with BSD security discipline and service design.

```
+-----------------------------------------------------------------------------------+
|                        SIGMAOS DESKTOP EDITION ROADMAP                            |
+-----------------------------------------------------------------------------------+
  Phase 0: Engineering Contract & Toolchain Baseline               [COMPLETE]
  Phase 1: Build & Test Automation Baseline                         [COMPLETE]
  Phase 2: SigmaOS Desktop Preview (Zenith Compositor)             [IN PROGRESS]
  Phase 3: Production-Worthy Native `sigpkg` System                [PLANNED]
  Phase 4: Declarative System State & Atomic A/B Updates           [PLANNED]
  Phase 5: User-Understandable Capability Security                 [PLANNED]
  Phase 6: Hardware Validation & Reference Device Support          [PLANNED]
  Phase 7: Zenith Desktop Polish & Design System                   [PLANNED]
  Phase 8: Developer SDK, Package Recipes & Ecosystem              [PLANNED]
+-----------------------------------------------------------------------------------+
```

---

## 📅 Staged Rollout Timeline & Persona Expansion

### 6 Months (v1.0 – Core Essentials)
- **Lightweight Text Editor**: Quick edits out-of-the-box (`sigma-edit`).
- **Universal Compression Utility**: Universal archive support (`.tar.gz`, `.tar.xz`, `.zip`, `.zst`, `.7z`).
- **Network Diagnostics Engine**: Integrated utilities (`ping`, `curl`, `traceroute`, `netstat`).
- **System Monitoring Dashboard**: Integrated resource view (`sigma-top` / Zenith HUD).
- **Backup Snapshot Tool**: Btrfs/ZFS O(1) CAS generation checkpoints and rollback baseline (`sigpkg rollback`).

---

### 12 Months (v1.2 – Persona Expansion)
- **Developer Persona**:
  - File Conversion Utility (`sigma-convert` for code/media format conversions).
  - Lightweight IDE overlay with LSP language server integration.
- **Compliance Persona**:
  - Universal Package Fetcher (`sigpkg fetch` for drivers and apps).
  - Compliance Checklist Generator (automated CIS, ISO 27001, SOC2 reports).

---

### 18 Months (v1.5 – Differentiation Layer)
- **Student Persona**:
  - Productivity Micro-Tools (Pomodoro timer, checklist manager, quick notes).
  - Flashcard/quiz overlay for interactive study.
- **Gaming Persona**:
  - GPU scheduler micro-tool for performance tuning.
  - Network latency monitor for network optimization and bufferbloat reduction.

---

## 📊 Timeline Dashboard

| Milestone | Key Tools & Capabilities | Target Persona | Impact & Strategy |
|:---|:---|:---|:---|
| **6 Months (v1.0)** | Text editor, compression, network diagnostics, monitoring, snapshot rollback | Core OS Baseline | **Completeness Baseline** |
| **12 Months (v1.2)** | File converter, IDE overlay, package fetcher, compliance tools | Developer + Compliance | **Adoption Boost** |
| **18 Months (v1.5)** | Productivity tools, flashcard/quiz overlay, GPU scheduler, latency monitor | Student + Gaming | **Market Differentiation** |

---

## Detailed Phase Status
1. **Phase 0 & 1 (Baseline)**: Consolidated Rust std desktop development target with 100% test pass rates across native Rust runner (`./run_sigma_tests.sh`) and pytest suites.
2. **Phase 2 (Desktop Preview)**: Zenith compositor prototype with keyboard-driven Wayland tiling, WASM UI bridge, and integrated control center.
3. **Phase 3 (Universal Package Engine)**: Multi-distro format adapter supporting 60+ Linux/BSD package extensions with GPG verification and CoW snapshot rollbacks.
4. **Phase 4 - 8 (Next Steps)**: Hardware matrix qualification, declarative profile activation, Zorin Exec Guard integration, and community recipe SDK.
