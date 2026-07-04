# SigmaOS Roadmap

## Phase 0 (0-3 months)
- Finalize architecture RFC (microkernel vs hybrid, async syscalls)
- Set kernel coding standards (Rust, unsafe usage rules)
- Implement minimal reproducible build pipeline & cross-toolchain
- Build boot-to-userspace demo:
  - Boot kernel
  - Spawn userspace process
  - Demonstrate simple async syscall & IPC
- Create driver SDK prototypes (WASM + Rust host ABI)
- Configure CI with cross-compilation, unit tests, nightly benchmarks
- Begin outreach to 3 hardware vendors for driver partnerships

## Phase 1 (3-9 months)
- Kernel v0: memory management, basic process model, IPC, async syscall interface
- Basic userspace: shell, minimal filesystem, package manager skeleton
- Linux-compat prototype (container runtime or syscall shim)

## Phase 2 (9-18 months)
- Stable driver model (WASM host)
- NIC and block drivers (bare minimum)
- Scheduler tuning
- Basic security features (secure boot)
- Mature developer tooling

## Phase 3 (18-36 months)
- Full userspace stack
- Production-grade filesystems
- GPU/graphics stack
- NVMe performance optimizations
- Enterprise-grade CI
- Fuzzing everywhere
- Formal verification for critical modules

```
Phase F  ████████████████████  100% ✅  (KMS, cgroup, pkg registry)
Phase G  ████████████████████  100% ✅  (kernel boot — COMPLETE)
Phase H  ████████████████░░░░  50% 🔄  (India Stack — ACTIVE)
  - sigma-health (ABDM FHIR): ✅
  - sigma-accounts (GST IRN): ✅
  - sigma-pay (UPI/NPCI): ✅
  - sigma-aadhaar (QR Auth): ✅
Phase I  ████████████████████  100% ✅  (Desktop & Drivers — COMPLETE)
  - Zenith Desktop Compositor: ✅
  - Auto-tiling Window Manager: ✅
  - Application Launcher: ✅
  - System Tray: ✅
  - Accessibility Features: ✅
  - i915 GPU Driver: ✅
  - iwlwifi Wi-Fi Driver: ✅
  - AMD amdgpu Driver: ✅
  - HDA Audio Driver: ✅
  - sigma-ai Daemon: ✅
  - 10 Bundled Applications: ✅
Stage 0 ████████████████████  100% ✅  (Bootable Foundation — COMPLETE)
  - Kernel Scheduler (MLFQ/CFS/EDF): ✅
  - Memory Manager (Buddy/Slab/ASLR): ✅
  - Interrupt Controller (APIC/PIC): ✅
  - Virtual Memory (4-level page tables): ✅
  - Syscall Gate (30 syscalls): ✅
  - UEFI Bootloader (sigma-boot.zig): ✅
  - Bootable ISO (build-iso.sh): ✅
Phase 5B ████████████████████  100% ✅  (Desktop Dominance — COMPLETE)
  - Window Manager (auto-tiling): ✅
  - Application Launcher (fuzzy search): ✅
  - System Tray (time/battery/network): ✅
  - Accessibility (WCAG AAA): ✅
  - sigma-edit (text editor): ✅
  - sigma-files (file manager): ✅
  - sigma-terminal (terminal): ✅
  - sigma-browser (web stub): ✅
  - sigma-mail (email client): ✅
  - sigma-calc (calculator): ✅
  - sigma-calendar (calendar): ✅
  - sigma-notes (note app): ✅
  - sigma-clock (system clock): ✅
  - sigma-settings (settings panel): ✅
Education ████████████████████  100% ✅  (CBSE & Professional — COMPLETE)
  - Virtual Lab (physics/chemistry/biology): ✅
  - Data Visualization (graphing/plotting): ✅
  - Symbolic Math Engine (algebra/calculus): ✅
  - Adaptive Practice (CBSE syllabus): ✅
  - Math Proof Assistant (step-by-step solver): ✅
  - AI Exam Paper Generator (NCERT aligned): ✅
  - Coding Playground (Python/C++/Java): ✅
  - Curriculum Projects (IT practicals): ✅
  - Multilingual Support (Hindi/Gujarati/Tamil/Bengali): ✅
  - Exam Prep (UPSC/SSC/GATE/NET): ✅
Security ████████████████████  100% ✅  (IT Training — COMPLETE)
  - Cybersecurity Sandbox (malware/firewalls): ✅
  - Audit Trail Visualizer (logs/monitoring): ✅
  - Security Policy Advisor (AI best practices): ✅
  - Networking Simulator (TCP/IP/routing): ✅
Law & Governance ████████████████████  100% ✅  (Legal Professionals — COMPLETE)
  - Labour Code Explorer (Labour Law/OSH/Social Security): ✅
  - Case Law Database (Indian judgments): ✅
  - Legal Drafting Assistant (petitions/contracts): ✅
  - Policy Simulation (workplace law testing): ✅
Professional Tools ████████████████████  100% ✅  (Indian Sectors — COMPLETE)
  - Healthcare (medical data analysis): ✅
  - Engineering (CAD/circuit simulators): ✅
  - Finance (GST/TDS/TCS compliance): ✅
  - Agriculture (crop yield/soil health): ✅
  - Multilingual Office Suite (9 Indian languages): ✅
```

## The Critical Path

Everything depends on `kernel-exp` shipping Phase 0:

1. `kernel-exp` → bootable kernel
2. `drivers-dev` → GPU + Wi-Fi drivers
3. `fs-dev` → VFS + SigmaFS
4. All `release/*` profiles become functional

## Quick Links

- [CURRENT_PROBLEMS_MANIFEST.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CURRENT_PROBLEMS_MANIFEST.md)
- [FEATURE_MATRIX.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/FEATURE_MATRIX.md)
- [CONTRIBUTOR_ROADMAP.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/CONTRIBUTOR_ROADMAP.md)
- [GitHub Issues](https://github.com/AaryanSinghChauhan09/SigmaOS/issues)

## Phase 4 (36+ months)
- Certifications
- Vendor partnerships
- Mainstream device driver coverage
- Migration tools
- Scale community and support offerings
