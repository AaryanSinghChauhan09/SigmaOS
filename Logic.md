# System Logic & File Relationships (SigmaOS Zenith)

> **Specification Version:** 15.2-FINAL  
> **Status:** Standardized & Verified  
> **Target Audience:** Core Kernel Engineers & Automators  

This document defines the structural relationships, interaction pipelines, and dependencies connecting all primary files and directories in the SigmaOS Zenith codebase.

---

## 1. Directory Topology & Logic Layer Map

The SigmaOS codebase is structured as a **Four-Tier Decoupled Sovereign Lattice**. Each directory represents a distinct logical ring-boundary:

```
┌─────────────────────────────────────────────────────────┐
│                      Tier 1: User UI                    │
│   (zenith_desktop/, suites/S30_Supremacy/ companion)    │
└────────────────────────────┬────────────────────────────┘
                             │ (User Events / Intent)
                             ▼
┌─────────────────────────────────────────────────────────┐
│                   Tier 2: System Suites                 │
│      (suites/S66_SovereignClaw, suites/S99_LinuxNative) │
└────────────────────────────┬────────────────────────────┘
                             │ (Kernel Syscalls / APIs)
                             ▼
┌─────────────────────────────────────────────────────────┐
│                 Tier 3: Core Kernel & HAL               │
│        (kernel/core/ai, kernel/core/security, drivers/) │
└────────────────────────────┬────────────────────────────┘
                             │ (Zero-Trust Validation)
                             ▼
┌─────────────────────────────────────────────────────────┐
│              Tier 4: Shared Shards & Utils              │
│               (include/, kernel/shards/, tools/)        │
└─────────────────────────────────────────────────────────┘
```

---

## 2. Exhaustive File & Subsystem Dependency Registry

Below is the logical mapping of each critical file and its relationships across the OS architecture:

### A. AI Automation Subsystem (`SovereignClaw`)
1. **`include/ai/sigma_claw.h`**
   * *Logic*: Declares the public C-bridge APIs for the AI companion and agent daemon.
   * *Relationships*: Included by `SovereignClawCompanion.cpp` (UI interaction) and implemented in `sigma_claw.cpp` (daemon).
2. **`suites/S66_SovereignClaw/sigma_claw.cpp`**
   * *Logic*: Houses the core OpenClaw-inspired execution agent. Decomposes goal intents into specific commands and runs them in strict sandbox containers.
   * *Relationships*: Uses `sigma_cap_manager.h` for validation and `security/sigma_sandbox.h` for secure container initialization.
3. **`kernel/core/ai/SovereignClawGateway.cpp`**
   * *Logic*: Intercepts incoming workflow requests at the kernel boundary and matches them against user authorization profiles.
   * *Relationships*: Bridges execution between the high-level workspace interfaces and low-level resource management.
4. **`suites/S30_Supremacy/SovereignClawCompanion.cpp`**
   * *Logic*: Provides conversational UI event handling and the status visualization canvas.
   * *Relationships*: Calls `claw_route_message` and `claw_render_canvas` declared in `sigma_claw.h`.

### B. Security & Sandbox Boundary
5. **`include/sigma_cap_manager.h`**
   * *Logic*: Declares the `CapabilityManager` class and global `cap_manager` instance, managing authorization tokens like `SIGMA_CAP_EXEC_SKILL` and `SIGMA_CAP_VFS_READ`.
   * *Relationships*: Included by both `sigma_claw.cpp` (AI security checks) and `sigma_posix.cpp` (POSIX security checks).
6. **`include/security/sigma_sandbox.h`**
   * *Logic*: Public declaration for sandbox container control (creation, execution, and demolition).
   * *Relationships*: Implemented in `kernel/core/security/SovereignSandbox.cpp` and used by `sigma_claw.cpp` to run tasks safely.

### C. System Emulation & Binary Compatibility
7. **`suites/S99_LinuxNative/sigma_posix.cpp`**
   * *Logic*: Emulates POSIX standard behaviors by intercepting Linux syscalls (e.g. read, write, exit) and mapping them directly to native sovereign operations.
   * *Relationships*: Uses `sigma_cap_manager.h` to gate raw filesystem operations using capability tokens.

### D. Compliance & Professional Calculations
8. **`tools/profession_calculators.cpp`**
   * *Logic*: Contains Indian compliance, tax (GST, Income Tax), corporate quorum, gratuity, and RERA interest calculators.
   * *Relationships*: Exposes zero-dependency C-linkage entry points (like `c_calculate_gst`) to allow seamless shell and GUI integrations.

### E. Orchestration & Build Automation
9. **`orchestrator/main.cpp`**
   * *Logic*: Standard C++ command line entry dispatcher (`s-cli`) parsing user requests and routing them to target kernel operations.
   * *Relationships*: Interfaces with `SovereignClaw` to dispatch skills.
10. **`tools/sync_all_branches.js`**
    * *Logic*: Release engineering manager script. Merges core updates across the 12 branches (`release/*`, `performance-optimized`, `gh-pages`) to ensure repo-wide parity.
    * *Relationships*: Orchestrates Git branches at the repository root level.

---
> **Verification Status:** BUILD-VERIFIED | PARITY ACHIEVED  
> *Last updated: 2026-05-19 | SigmaOS Zenith Release*
