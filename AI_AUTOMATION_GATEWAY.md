# 🤖 Sovereign AI Automation Gateway

## Overview
SigmaOS integrates an advanced, local-first **Artificial Intelligence Automation System**, heavily inspired by the OpenClaw architecture ("The lobster way 🦞"). This system provides a zero-overhead, highly-secure capability for executing AI agents, routing multi-channel inputs, and managing automation workflows directly at the kernel level.

## Core Components
### 1. SovereignClawGateway
Located at `kernel/core/ai/SovereignClawGateway.cpp`, this component acts as the control plane.
- **Multi-channel Inbox**: Receives incoming events/messages.
- **Tool Execution**: Spawns securely sandboxed AI tasks.
- **Live Canvas Support**: Communicates with the userland Universal UI for visual feedback.

### 2. SovereignSandboxEngine
To guarantee **safety & security**, the AI Gateway strictly executes its tools within the `SovereignSandboxEngine`. 
- Utilizing Seccomp-BFP, amnesic memory profiles, and strict capability matrices.
- AI Agents are entirely confined based on policy (`STRICT`, `NON_MAIN`, `OPEN`).

### 3. SovereignWorkflowEngine
Located at `kernel/core/automation/SovereignWorkflowEngine.cpp`, this engine allows the OS to react to AI decisions and vice versa using deterministic IF/THEN automation rules with no STL footprint.

## Security Guarantees
- **Zero-Trust**: AI tools run in unprivileged containers. They cannot modify system state unless a specific workflow rule triggers a kernel action.
- **Local-First Privacy**: All processing runs locally, ensuring that SigmaOS preserves absolute sovereignty over data.

## Integration
All AI components are declaratively linked in `SHARDS.manifest` and automatically compiled alongside the core kernel.

---
*Maintained by the SigmaOS Core Architecture Team.*
