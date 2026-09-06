# 🤖 SigmaOS AI Agent Governance Specification (`AGENTS.md`)

**Version:** 1.2.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Process, Memory, & Loader Management

---

## EXECUTIVE SUMMARY & AGENT ARCHITECTURE

SigmaOS features an AI-native process, memory, and module loader architecture where autonomous agent processes govern kernel scheduling, memory pools, security auditing, and dynamic module loading.

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT GOVERNANCE     |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • Boot Speed Profiling     • Bootloader UI Styling     • Module Signature Audit
  • Module Load Optimization • Boot Splash Graphics      • Secure Boot Key Check
  • Sub-µs Memory Access     • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Scope**: CPU scheduling, `cgroups v2`, boot time profiling (`src/tools/bootloader.rs`), initramfs decompression speed, zero-allocation hot paths.
- **Rules**:
  - Profile kernel module loading times (`src/kernel/module_loader.rs`) and eliminate boot delay bottlenecks.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Scope**: Desktop compositor layout, boot menu styling, console progress indicators, accessibility state trees.
- **Rules**:
  - Maintain WCAG 2.1 AA compliant boot and desktop interfaces.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Scope**: LSM auditing, OpenBSD `pledge`/`unveil`, Post-Quantum Dilithium-5 kernel module signature verification, Secure Boot validation.
- **Rules**:
  - Enforce Dilithium-5 digital signature checks prior to kernel module loading.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. PROCESS & LOADER MANAGEMENT POLICIES

### Module Loader Rules (`src/kernel/module_loader.rs`)
- **Signature Verification**: Every kernel module must be signed with Dilithium-5 keys before symbol relocation.
- **A/B Boot Rollback**: Failed module or stage-2 boot attempts trigger automatic fallback via `Firmitas` A/B slot mechanics.

---

## 3. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
