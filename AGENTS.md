# 🤖 SigmaOS AI Agent Governance Specification (`AGENTS.md`)

**Version:** 1.4.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Process, Memory, Loader, Desktop, & Paging Management

---

## EXECUTIVE SUMMARY & AGENT ARCHITECTURE

SigmaOS features an AI-native architecture where autonomous agent processes govern kernel scheduling, memory pools, dynamic module loading, desktop environments, and virtual memory paging.

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT GOVERNANCE     |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • Page Walk Profiling      • Memory Map Visualization  • Page Table W^X Audit
  • TLB Shootdown Tuning     • Desktop UI Styling        • NX Bit Enforcement
  • Sub-µs Memory Access     • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Scope**: CPU scheduling, `cgroups v2`, boot speed profiling, Zenith compositor render frame-rate profiling, page translation walk profiling (`src/kernel/paging.rs`), TLB shootdown tuning.
- **Rules**:
  - Minimize TLB miss penalties and optimize 2MB/1GB huge page frame distribution.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Scope**: Desktop compositor layout, Control Center themes, visual memory map diagnostic views, WCAG 2.1 AA focus visible outlines, ARIA annotations.
- **Rules**:
  - Enforce WCAG 2.1 AA compliance across all desktop controls and visual diagnostic tools.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Scope**: LSM auditing, OpenBSD `pledge`/`unveil`, Post-Quantum Dilithium-5 signatures, page table W^X (Write XOR Execute) audit, No-Execute (`NX`) bit enforcement (`src/kernel/vmm_paging.rs`).
- **Rules**:
  - Enforce strict page table W^X invariants across userland and kernel address spaces.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. PAGING & VIRTUAL MEMORY POLICIES (`docs/AI_AGENTS_PAGING_MANAGEMENT.md`)

- **IRQL Paging Restrictions**: Paged pool memory (`PagedPool`) must NOT be accessed at `IRQL >= DispatchLevel`.
- **Page Fault Recovery**: `HardwareException::PageFault` handling extracts CR2 fault addresses and enforces page permissions.

---

## 3. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
