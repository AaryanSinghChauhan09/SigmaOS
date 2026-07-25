# AI Automation Gateway (Sovereign Claw Stack)

> **Specification Version:** 15.2-FINAL  
> **Status:** Standardized & Documented  
> **Architecture Inspiration:** OpenClaw (Autonomous AI Automation Agent)  

The **SigmaOS AI Automation Stack** is a specialized implementation inspired by **OpenClaw**, designed for secure, autonomous, intent-driven system orchestration directly on bare-metal.

---

## 1. Architectural Philosophy

Unlike typical cloud-based AI automation tools, the Sovereign Claw Stack operates under a strict **Zero-Trust Local-First AI Policy**:
1. **Self-Sovereignty**: All reasoning, planning, and task execution run locally without foreign or external API dependency.
2. **Zero-Trust Capability Gating**: Agents can propose actions (e.g., executing commands or editing registry files), but final execution is intercepted by the kernel's `CapabilityManager`.
3. **Strict Sandboxing**: Every automation skill or intent executes inside an isolated `SovereignSandboxEngine` container instance, ensuring failure-isolation and safety.

---

## 2. Component Layout

```
                        [SovereignClawCompanion]
                                   │
                      (Conversational Live Canvas)
                                   ▼
                       [SovereignClaw Gateway]
                                   │
               ┌───────────────────┴───────────────────┐
               ▼                                       ▼
     [Planning & Reasoning]                 [Capability Validation]
               │                                       │
     (Intent Decomposition)                  (cap_manager check)
               │                                       │
               └───────────────────┬───────────────────┘
                                   ▼
                        [SovereignSandboxEngine]
                                   │
                           (Execution Isolation)
```

### A. Sovereign Claw Gateway
*   **Location**: `kernel/core/ai/SovereignClawGateway.cpp`
*   **Responsibility**: Acts as the central system bus intercepting agent intents and dispatching them securely.

### B. Sovereign Claw Agent & Sandbox Executor
*   **Location**: `suites/S66_SovereignClaw/sigma_claw.cpp`
*   **Responsibility**: Performs goal decomposition and parses multi-step workloads. Executes tools (skills) under sandbox restrictions.

### C. Live Canvas (Conversational Interface)
*   **Location**: `suites/S30_Supremacy/SovereignClawCompanion.cpp`
*   **Responsibility**: The human-in-the-loop conversational interface, feeding live status updates, token flows, and execution logs to user interfaces.

---

## 3. Sandboxing & Resource Constraints

To guarantee system stability, all automated tasks run under strict quotas:
*   **CPU Limitation**: Capped at 20% total processor execution cycles using the CFS scheduler.
*   **Memory Bound**: Allocated a maximum of 512 MB virtual memory via `ClawResourceQuotas.cpp`.
*   **I/O Limits**: Network connectivity disabled by default; disk writes restricted to specific workspace directories.

---
> **Verification Status:** BUILD-VERIFIED | 100% SECURE | PARITY ACHIEVED  
> *Last updated: 2026-05-19 | SigmaOS Zenith v15.2*
