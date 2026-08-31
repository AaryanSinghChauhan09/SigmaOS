# 🗺️ SigmaOS Strategic Roadmap Overlay (2026 — 2029)

This roadmap documents our short-term, mid-term, and long-term milestones. It charts how SigmaOS expands from a capability-gated microkernel into the ultimate sovereign OS ecosystem, mapping dependencies across kernel, filesystem, security, and UX development.

---

## 📊 SigmaOS Feature Scorecard

| Feature | Dependency | Quarterly KPI | Status |
|----------------|----------------|----------------|----------------|
| Kernel Hybrid | Foundation for scheduler, FS, security | Boot stability ≥ 95% | 🔴 Pending |
| Transactional FS | Requires kernel stability | Rollback success ≥ 99% | 🟠 In progress |
| Adaptive Scheduler | Depends on kernel hybrid | Latency < 10ms | 🔴 Pending |
| Visual Sandbox GUI | Needs FS + kernel hooks | Policy adoption ≥ 80% | 🟠 Prototype |
| Unified Firewall Dashboard | Requires networking stack | Rule accuracy ≥ 99% | 🔴 Pending |
| Native Containers | Needs scheduler + FS | Launch time < 2s | 🔴 Pending |
| Zenith Overlays | Independent, but benefits from FS rollback | Stability ≥ 95% uptime | 🟠 Early design |
| Compliance Handbook | Parallel to all features | Coverage ≥ 90% | 🟢 Drafting |

---

## 📅 Roadmap Overlay (Dependencies)

- **Quarter 1–2 (Foundation)**
  - Kernel hybrid → Transactional FS → Compliance handbook baseline.

- **Quarter 3–4 (Expansion)**
  - Adaptive scheduler → Visual sandbox GUI → Unified firewall dashboard.

- **Quarter 5–6 (Differentiation)**
  - Native containers → Zenith overlays → Distributed FS overlay.

---

## 🌟 Strategic Flow

- **Kernel hybrid is the backbone** → unlocks scheduler + FS.
- **Transactional FS ensures resilience** → supports sandboxing + rollback.
- **Security overlays (sandbox + firewall)** → compliance differentiator.
- **UX overlays (Zenith)** → user adoption driver.
- **Docs run parallel** → ensure community + compliance visibility.

---

## 📈 Milestone Timeline Overview

```
┌───────────────────────────┬───────────────────────────┬───────────────────────────┐
│ Short-Term (1 - 6 Months) │ Mid-Term (6 - 18 Months)  │ Long-Term (18 - 36 Months)│
├───────────────────────────┼───────────────────────────┼───────────────────────────┤
│ Core Utilities Refinement │ Virtualization & Parity   │ Universal Mesh & AI-Hub   │
│ Indian Compliance Tools   │ Developer Toolkits        │ Self-Hosting OS Core      │
└───────────────────────────┴───────────────────────────┴───────────────────────────┘
```

---

## 🚀 1. Short-Term Overlay (1 — 6 Months): Core Systems & Localized Gaps

Focuses on immediate stability fixes, replacing GNU coreutils with safe Rust structures, and adding institutional compliance.

- **`sigma-core-utils` & `sigma-sh`**: Finalize all core utility commands (mkdir, touch, pwd, whoami) in Rust, fully replacing BusyBox.
- **Indian National Localizations**: Scale the `LocalizationManager` to support real-time localization of menus and text across all 22 scheduled languages of India.
- **Legal & Judicial Timelines**: Integrate `JudicialTimelinePlanner` with ePrisons databases to calculate default bail eligibility under Section 480 of the Bharatiya Nagarik Suraksha Sanhita (BNSS).
- **MSME Compliance**: Deploy the `MsmeComplianceEngine` within the sovereign office pipeline to automate delayed payment compound interest calculations under Section 16 of the MSMED Act.

---

## ⚙️ 2. Mid-Term Overlay (6 — 18 Months): Virtualization & Compatibility Parity

Focuses on bringing full AMD-V and Intel VT-x hardware-virtualization parity, and deploying on-device AI workspaces.

- **Intel VT-x and IOMMU Support**: Expand the hypervisor virtual managers to support Intel VT-x and Intel VT-d IOMMU page protection models, achieving parity with legacy hypervisors.
- **Open Computer (Anything-LLM) Workspaces**: Launch native Open Computer agent virtual desktop overlays (.qcow2 deltas) running with local on-device Gemma and Qwen models.
- **Accessibility & Custom Routines**: Launch the `AccessibilityOverlayManager` and `AutomationRoutineController` to enable custom macro actions and voice-controlled interface navigation.
- **Developer Toolkits**: Integrate cross-language converters (`DeveloperToolkitConverter`) capable of translating legacy Python/C++ logic into secure, zero-allocation memory-safe Rust code blocks.

---

## 🌐 3. Long-Term Overlay (18 — 36 Months): Universal Mesh & AI-Native Mastery

Focuses on establishing global peer-to-peer mesh connectivity and compiling SigmaOS entirely on top of itself.

- **Self-Hosting Microkernel**: Port and optimize the GCC/Clang/Rustc toolchains to run and compile the entire SigmaOS codebase natively on top of the SigmaOS microkernel, eliminating third-party compile dependencies.
- **IoT Mesh Orchestration**: Implement the `IotDeviceMeshOrchestrator` to coordinate zero-trust, peer-to-peer mesh communication across hundreds of local smart devices.
- **Forensic Readiness Trails**: Scale `ForensicReadinessAuditor` to maintain cryptographically hashed log streams for enterprise-wide chain-of-custody audits.
- **Cross-Device Continuity**: Seamlessly delegate active task capability tokens from Zenith Desktops to local edge nodes, phones, and wear devices.
