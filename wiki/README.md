# 🏛️ SigmaOS Master Documentation Wiki
## 🚀 The Ultimate Reference Hub for Sovereign, AI-Native, and Post-Quantum Computing

Welcome to the ground-truth technical wiki for SigmaOS. This comprehensive reference hub organizes the documentation hierarchy, clarifies our zero-dependency lattice layers, integrates our open-source Tier 1 integrations, and details the Phase G GUI Installer architecture.

---

## 🗂️ 1. Documentation Hierarchy & Map

To preserve absolute technical clarity, the SigmaOS documentation structure is partitioned into three logical pillars:

```
                  +----------------------------------------------+
                  |            SIGMAOS MASTER INDEX              |
                  |               (wiki/README.md)               |
                  +----------------------------------------------+
                                         |
         +-------------------------------+-------------------------------+
         v                                                               v
+----------------------------------+                           +----------------------------------+
|    TECHNICAL SPECIFICATIONS      |                           |   ROADMAPS & TIMELINE SCHEMAS    |
| - 10-Layer Lattice Architecture  |                           | - 36-Month Technical Roadmap     |
| - Tier 1 Integration (Wasmer...) |                           | - Phase G GUI Installer Sequence |
| - Security, Sandboxing, PQC PKI  |                           | - Distro Absorption Milestones   |
|   (wiki/README.md)               |                           |   (FUTURE-DEVELOPMENT-ROADMAP.md)|
+----------------------------------+                           +----------------------------------+
```

1.  **Technical Reference Core (`wiki/README.md`):** Governs structural layers, system designs, API patterns, and embedded open-source Tier 1 integrations.
2.  **Release & Progress Log (`wiki/CHANGELOG.md`):** Records continuous feature updates, security patches, and structural stabilization achievements.
3.  **Strategic Vision & Milestones (`FUTURE-DEVELOPMENT-ROADMAP.md`):** Governs the long-term evolution roadmap, competitive matrix benchmarks, and architectural rollout schedules.

---

## 🏛️ 2. The Sovereign 10-Layer Lattice Architecture

SigmaOS rejects POSIX-legacy bloat, employing a hierarchical lattice model where each layer operates with explicit privilege boundaries and zero dynamic upward dependencies.

```
+-----------------------------------------------------------------------------+
| Layer 10: Sovereign Nexus - Enterprise Suite (ERP, CRM, Productivity)       |
+-----------------------------------------------------------------------------+
| Layer 9: Ecosystem Abstraction (POSIX, Windows PE/S-WINE Translate Core)   |
+-----------------------------------------------------------------------------+
| Layer 8: Sovereign Claw AI Automation (Agent Gateway, Live Canvas)          |
+-----------------------------------------------------------------------------+
| Layer 7: Sovereign AI & Orchestration (Intent-to-Shard Dispatch)            |
+-----------------------------------------------------------------------------+
| Layer 6: Zenith UI & Morphic Shell (Wayland-Native, Direct Framebuffer)     |
+-----------------------------------------------------------------------------+
| Layer 5: Sovereign Package Ecosystem (sigma-pkg, CAS Store, SAT Solver)     |
+-----------------------------------------------------------------------------+
| Layer 4: Capability-Gated Security (PQC, TPM 2.0, Pledge/Unveil Sandbox)    |
+-----------------------------------------------------------------------------+
| Layer 3: Sovereign Virtual Filesystem (VFS, CoW Merkle, Shard Namespaces)   |
+-----------------------------------------------------------------------------+
| Layer 2: Genesis Kernel & Scheduling (S-MM Slab, SHS Hybrid Scheduler)      |
+-----------------------------------------------------------------------------+
| Layer 1: Universal Hardware Abstraction (HAL, Polymorphic Peripherals)      |
+-----------------------------------------------------------------------------+
```

### Lattice Layer Glossary

*   **Layer 1: Universal Hardware Abstraction (HAL)**
    *   Exposes the polymorphic `UnifiedPeripheral` interface, standardizing MMIO and Port I/O device registers seamlessly.
    *   Provides low-level interrupt routing, physical memory-mapped register locks, and multicore initialization.
*   **Layer 2: Genesis Kernel & Scheduling**
    *   Hosts the **SHS (Sovereign Hybrid Scheduler)**, combining real-time Fair-Queue EEVDF and priority-based preemptive scheduling.
    *   Manages the lock-free `S-MM` slab allocator and high-efficiency buddy system page-frame managers.
*   **Layer 3: Sovereign Virtual Filesystem (VFS)**
    *   Implements Copy-on-Write (CoW) Merkle tree systems where directories are cryptographically verified.
    *   Manages process namespace isolations and relative-path traversals protection.
*   **Layer 4: Capability-Gated Security**
    *   Enforces hardware-backed **TPM 2.0** remote attestation and local key escrow management.
    *   Implements post-quantum encryption (KIST FIPS 203/204 standard: `Kyber-1024` KEM and `Dilithium-5` signatures).
*   **Layer 5: Sovereign Package Ecosystem**
    *   Governs package resolution constraints through a zero-allocation backtracking **SAT Solver**.
    *   Tracks system states declaratively via SHA-256 content-addressed storage (CAS) hashes.
*   **Layer 6: Zenith UI & Morphic Shell**
    *   Renders visual desktop panels, window animations, and tile systems directly onto framebuffer layers via `vesa::VesaDriver`, bypassing Wayland/X11 entirely.
    *   Implements high-contrast filters and voice accessibility screen readers in the visual compositor thread.
*   **Layer 7: Sovereign AI & Orchestration**
    *   Operates the `AiOptimizer` to dynamically tune hardware power states, scaling governor profiles, and thread allocations predictive-wise.
*   **Layer 8: Sovereign Claw AI Automation**
    *   Coordinates the `SovereignMultiAgentPlanner`, running local agent tasks inside unprivileged sandbox instances.
*   **Layer 9: Ecosystem Abstraction (S99)**
    *   Emulates standard POSIX call interfaces, translates Windows PE binary targets on-the-fly, and maps compatible library boundaries.
*   **Layer 10: Sovereign Nexus - Enterprise Suite**
    *   Deploys high-fidelity, standard-library-free, zero-dependency data science, tax calculations, and office automation suites.

---

## 📦 3. Open Source Tier 1 Projects Integration Layer

To guarantee digital sovereignty, SigmaOS integrates zero-dependency, clean-room equivalent implementations of leading Tier 1 open-source projects, avoiding bulky external binary downloads.

### 1. WebAssembly Integration (Wasmer Parity)
*   **Execution Model:** Implemented in `src/compatibility/open_source_tier1.rs` and `wasm_sandbox.rs`.
*   **Mechanism:** Compiles and executes Wasm bytecode on-the-fly inside unprivileged, capability-gated sandbox containers. Restricts virtual memory boundaries, preventing host-register corruption and out-of-bounds heap escapes.

### 2. Networking Integration (smoltcp Parity)
*   **Execution Model:** Implemented in `src/compatibility/open_source_tier1.rs` and `net/tcpip_stack.rs`.
*   **Mechanism:** Implements an event-driven, zero-copy TCP/IP stack running entirely without heap allocations. Interfaces directly with device queues, passing ethernet frames through lock-free ring-buffer structures.

### 3. Cryptography Integration (libsodium Parity)
*   **Execution Model:** Implemented in `src/compatibility/open_source_tier1.rs`.
*   **Mechanism:** Translates classic libsodium API endpoints (e.g. `crypto_secretbox_easy`, `crypto_sign`) to our advanced post-quantum cryptographic primitives. Dilithium-5 and Kyber-1024 are mapped directly to safeguard data against quantum decryption threats.

### 4. Database Integration (SQLite Parity)
*   **Execution Model:** Implemented in `src/compatibility/open_source_tier1.rs` and `storage/sql_engine.rs`.
*   **Mechanism:** Hosts a zero-allocation, serverless SQL engine that parses declarative SQL grammar and translates operations to pre-compiled topological traversals over filesystem Merkle-trees.

---

## 🖥️ 4. Phase G: Sovereign GUI Installer Wizard

The GUI Installer Wizard (implemented in `web_ui/index.html` and styled in `web_ui/styles/style.css`) replaces fragile legacy CLI installer scripts with an interactive, accessible, and high-fidelity installation environment.

```
+---------------------------------------------------------------------------------+
|                         GUI INSTALLER WIZARD FLOW                               |
+---------------------------------------------------------------------------------+
| [Step 1: Welcome] --> [Step 2: Disk Select] --> [Step 3: Configuration Settings]|
|                           (Lattice devices)       (Locale, updates, channels)   |
+---------------------------------------------------------------------------------+
                                                                 |
                                                                 v
| [Step 5: Completion] <-- [Step 4: Live Install Progress] <-----+
|  (Kyber machine keys)     (Simulated log terminal, SovereignFS formatting)
+---------------------------------------------------------------------------------+
```

### Step-by-Step Installation Stages

1.  **Welcome & Licensing Panel:** Outlines the core features of SigmaOS and requires explicit acceptance of the Digital Autonomy Agreement.
2.  **Target Storage Selection:** Polls active silicon storage buses, displaying available block devices (e.g., primary SSD, virtual block drives) in an interactive choice group.
3.  **System Parameters & Release Channels:** Configures standard locale options, IST timezones, hostnames, and locks the node to one of three release streams (LTS, Rolling, or Experimental).
4.  **Automatic Extraction Terminal:** Triggers dynamic, multi-threaded formatting and extraction. Simulates live log feedback, covering partition mappings, filesystem formatting (`SovereignFS`), and Kyber keypair creation.
5.  **Success & Finalization Screen:** Displays a structured installation summary, registers the public pq-key, and instructs the user on media removal and final rebooting.

---

## 🛡️ 5. Clean-Room Linux Distribution Compatibility Layers

SigmaOS implements pure, clean-room compatibility layers to easily ingest and run packages from alternative operating system families:

### A. Arch Linux Package & Database Emulation
*   **Pacman DB Parser:** Parsers legacy Pacman database descriptors inside `/var/lib/pacman/local/`, translating them into our native content-addressed formats.
*   **AUR PKGBUILD Compiler:** Compiles declarative Arch build recipes into S-PKG package layers inside unprivileged build sandboxes.

### B. Debian Package Priority Support
*   **Priority Hierarchy:** Supports standard Debian manifest priority flags (`Optional`, `Standard`, `Important`, `Required`, `Essential`).
*   **Protection Rules:** Essential-level packages (e.g. init system, core system libraries) are locked, preventing manual or automated removal to guarantee kernel boot integrity.

### C. Fedora/Red Hat Package Engine
*   **DNF Resolver:** Tracks RPM metadata states and resolves complex multi-tier dependencies utilizing our zero-allocation SAT solver.
*   **Mock Chroot Sandbox:** Creates isolated namespaces with mapped bind paths (`/dev`, `/proc`), ensuring clean and reproducible builds of source RPMs.

---

## 🧭 6. Outdated Information Cleared & Updated

*   **Deprecated POSIX Assumptions:** All mentions of standard POSIX execution dependencies and bulky glibc runtime bindings are fully cleared. The system runs strictly on standard-library-free capability interfaces.
*   **Unified Encryption:** Standard elliptic curve and RSA cryptography mentions are retired, replaced exclusively by post-quantum `Kyber-1024` and `Dilithium-5` definitions.
*   **Consolidated State:** Fragmented, mutable configuration folders like `/etc` are replaced by a single, declarative, and structured system configuration manifest.
