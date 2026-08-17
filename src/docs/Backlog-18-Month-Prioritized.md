# SigmaOS: 18-Month Prioritized Backlog

This document defines the prioritized development roadmap, issues, task dependencies, and approximate effort estimations (Story Points) for SigmaOS over an 18-month timeframe.

---

## 📊 Backlog Roadmap Overview

```
 [ Phase 1: Months 1-6 ] ----> [ Phase 2: Months 6-12 ] ----> [ Phase 3: Months 12-18 ]
  - Shard Stabilization         - Graphic Composition        - Regional Dominance
  - PQC Networking              - Transactional VFS          - Sovereign AI Agents
  - WDM Driver Base             - Syscall Translations       - Long-Term Support (LTS)
```

---

## 🚀 Prioritized Backlog

### Phase 1: Months 1-6 (Core Shard Stabilization & WDM Foundations)

#### 1. Implement WDM Framebuffer GPU Driver Base (ID: SIG-101)
* **Goal:** Create a basic, hardware-agnostic VESA/UEFI Framebuffer driver within the newly designed WDM framework, making it the foundation for our GUI compositors.
* **Dependencies:** WDM Driver Base, physical memory paging.
* **Effort:** **13 Story Points (High)**

#### 2. Fully Integrate TLS 1.3 0-RTT PSK inside Secure Net Shard (ID: SIG-102)
* **Goal:** Connect TLS session ticket generation directly to standard sockets, validating zero-knowledge session resumption over live network interfaces.
* **Dependencies:** TCP/UDP networking, ALPN protocol parser.
* **Effort:** **8 Story Points (Medium)**

#### 3. Software Bill of Materials (SBOM) Code Signing (ID: SIG-103)
* **Goal:** Integrate automated CycloneDX SBOM generation into the build Makefile, verifying signatures using post-quantum Dilithium-5 algorithms at boot time.
* **Dependencies:** `CryptoVerifier`, S-BOOT firmware scanner.
* **Effort:** **5 Story Points (Medium)**

---

### Phase 2: Months 6-12 (Graphic Composition & Syscall Compatibility Layers)

#### 4. Mach-style Zero-Copy IPC Window Compositor (ID: SIG-201)
* **Goal:** Implement the primary frame swap chain in the Zenith Window Compositor, transferring frame buffers from apps to the compositor via Mach-style zero-copy IPC portals.
* **Dependencies:** WDM Framebuffer, Sovereign IPC Bus.
* **Effort:** **21 Story Points (Very High)**

#### 5. Expand L-Trans Syscall Translation Layer (ID: SIG-202)
* **Goal:** Map the first 50 core Linux syscalls (memory management, basic I/O, file descriptor access) to native capability-gated microkernel IPC transactions.
* **Dependencies:** Syscall translation hooks, Virtual Memory Manager.
* **Effort:** **13 Story Points (High)**

#### 6. Transactional Metadata Journaling for SigmaFS (ID: SIG-203)
* **Goal:** Build a robust, power-failure safe transactional journal for the content-addressed filesystem (`SigmaFs`), preventing file corruption upon sudden power loss.
* **Dependencies:** VFS layer, Ext4 block storage support.
* **Effort:** **8 Story Points (Medium)**

---

### Phase 3: Months 12-18 (Regional Dominance & AI-Native Workstations)

#### 7. India-First Localization & GST Service Stack (ID: SIG-301)
* **Goal:** Integrate local compliance toolkits directly into system services (including UPI transaction endpoints, GST invoice validation, and multi-lingual UI translation across 22 official languages).
* **Dependencies:** Zenith Desktop UI, Secure Networking.
* **Effort:** **13 Story Points (High)**

#### 8. Sandboxed Local LLM Execution Pipeline (ID: SIG-302)
* **Goal:** Run local AI models (such as LLaMA or Phi-3) inside rootless, seccomp-restricted, unprivileged user namespaces with zero-copy shared memory model access.
* **Dependencies:** S-AI Orchestrator, Container runtime.
* **Effort:** **13 Story Points (High)**

#### 9. Production-Ready Long-Term Support (LTS) Launch (ID: SIG-303)
* **Goal:** Compile reproducible, enterprise-ready SigmaOS releases, publishing stable repository images and automated security-patch backport tools.
* **Dependencies:** All previous milestones.
* **Effort:** **8 Story Points (Medium)**
