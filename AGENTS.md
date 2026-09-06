# 🤖 SigmaOS AI Agent Governance Specification (`AGENTS.md`)

**Version:** 1.1.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Process Management, Memory Management Policies

---

## EXECUTIVE SUMMARY & AGENT ARCHITECTURE

SigmaOS features an AI-native process and memory management architecture where autonomous agent processes govern kernel scheduling, userspace services, security auditing, and memory optimization.

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT GOVERNANCE     |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • CPU/Memory Profiling     • UI/UX & A11y Monitoring   • LSM & Audit Monitoring
  • cgroups v2 Quotas        • Wayland Layout Tuning     • Pledge/Unveil Rights
  • Sub-µs Memory Access     • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 1. AGENT PERSONAS & GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Process & Memory Scope**: CPU scheduling latency, `cgroups v2` quotas, pre-allocated ring buffers (`src/klib/ring_buffer.rs`), zero-allocation hot paths.
- **Rules**:
  - Profile memory footprints and context-switch overhead before mutating system state.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Process & Memory Scope**: Desktop compositor layout, font cache allocation, accessibility state trees.
- **Rules**:
  - Enforce WCAG 2.1 AA compliance and smooth 60+ FPS compositor rendering.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Process & Memory Scope**: LSM capability auditing, OpenBSD `pledge`/`unveil` path verification, secure memory zeroization.
- **Rules**:
  - Require Post-Quantum Dilithium-5 signatures on elevated commands.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. PROCESS MANAGEMENT & RESOURCE CONTROL

### `cgroups v2` Resource Allocation
All AI agent processes are managed under `/sys/fs/cgroup/sigma_agents/`:
- **Bolt**: CPU weight `100`, max memory limit `512MB`.
- **Palette**: CPU weight `80`, max memory limit `256MB`.
- **Sentinel**: CPU weight `120`, max memory limit `512MB`.

---

## 3. MEMORY MANAGEMENT POLICIES

### Allocators & Pool Memory Rules
- **Hot-Path Allocations**: Critical agent loops use `SlabAllocator` or static `LookasideList` buffers in `src/klib/`.
- **IRQL Paging Restrictions**: Paged pool memory (`PagedPool`) must NOT be accessed at `IRQL >= DispatchLevel`.
- **Memory Scrubbing**: Sensitive agent memory buffers are zeroized upon deallocation.

---

## 4. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
