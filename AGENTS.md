# 🤖 SigmaOS AI Agent Process Management Specification (`AGENTS.md`)

**Version:** 1.0.0
**Scope:** Autonomous AI Agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️), Subsystem Supervisors, Process Lifecycle Management

---

## EXECUTIVE SUMMARY & AGENT PROCESS ARCHITECTURE

SigmaOS features an AI-native process management architecture where autonomous agent processes govern kernel scheduling, userspace services, security auditing, and performance profiling.

```
                  +-----------------------------------+
                  |   SIGMAOS AI AGENT PROCESS TREE   |
                  +-----------------------------------+
                                    |
         +--------------------------+--------------------------+
         |                          |                          |
         v                          v                          v
  ⚡ BOLT PROCESS            🎨 PALETTE PROCESS         🛡️ SENTINEL PROCESS
  • CPU/Memory Profiling     • UI/UX & A11y Monitoring   • LSM & Audit Monitoring
  • cgroups v2 Quotas        • Wayland Layout Tuning     • Pledge/Unveil Rights
  • Sub-µs IPC Latency       • Semantic ARIA Tags        • Post-Quantum Verification
```

---

## 1. AGENT PERSONAS & PROCESS GOVERNANCE

### ⚡ Bolt (Performance Agent)
- **Process Scope**: Profiling kernel scheduler latency, `cgroups v2` resource quotas, and zero-copy IPC throughput.
- **Rules**:
  - Profile and identify CPU/memory bottlenecks before mutating process structures.
  - Changes must keep thread context switch latency under 0.12 µs.
  - Record learnings in `.jules/bolt.md`.

### 🎨 Palette (UX & Accessibility Agent)
- **Process Scope**: Desktop compositor layout tuning, Zenith shell accessibility annotations, and font rendering.
- **Rules**:
  - Enforce WCAG 2.1 AA compliance across Web UI and Zenith desktop processes.
  - Maintain focus-visible keyboard navigation and ARIA attributes for desktop controls.
  - Record learnings in `.jules/palette.md`.

### 🛡️ Sentinel (Security & Integrity Agent)
- **Process Scope**: LSM capability enforcement, OpenBSD `pledge`/`unveil` path auditing, and secret scanner compliance.
- **Rules**:
  - Enforce zero-trust least privilege on all launched agent processes.
  - Require Post-Quantum Dilithium-5 signature verification on agent commands.
  - Record learnings in `.jules/sentinel.md`.

---

## 2. PROCESS LIFECYCLE & RESOURCE CONTROL

### `cgroups v2` Resource Allocation
All AI agent helper processes are managed under `/sys/fs/cgroup/sigma_agents/`:
- **Bolt**: Allocated CPU weight `100`, max memory limit `512MB`.
- **Palette**: Allocated CPU weight `80`, max memory limit `256MB`.
- **Sentinel**: Allocated CPU weight `120`, max memory limit `512MB` (High priority security audit queue).

### IRQL Constraints & Memory Access
- Agents operating in kernel space must strictly adhere to IRQL levels (`PassiveLevel`, `ApcLevel`, `DispatchLevel`, `Dirql`, `HighLevel`).
- Accessing paged pool memory at `IRQL >= DispatchLevel` is strictly prohibited.

---

## 3. INTER-PROCESS COMMUNICATION (IPC) & FAULT RECOVERY

### Zero-Copy IPC Channels
- Agents communicate via lockless ring buffers (`src/klib/ring_buffer.rs`) and shared memory regions with capability token validation (`CapabilityToken`).

### Self-Healing & Process Supervision
- The `ProcessSupervisor` monitors agent heartbeats every 500 ms.
- If an agent process crashes, the supervisor logs the panic traceback to `crashdump_netconsole` and restarts the agent within 100 ms.

---

## 4. STANDALONE TESTING & VERIFICATION PROTOCOL

Every agent process module must support standalone unit testing via:
```bash
rustc --test <module_path> --edition=2021 --cfg 'feature="standalone_test"' -o /tmp/test_agent && /tmp/test_agent
```
All agent process modifications must pass without warnings under `-D warnings`.
