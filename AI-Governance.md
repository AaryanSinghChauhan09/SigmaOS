# AI Governance: Autonomous Agent Quotas

SigmaOS introduces the **AI-Native Governance Layer**, a completely unique orchestrator that decisively crushes the static resource management models of **Clear Linux**, **Gentoo**, and **NixOS**.

## The Autonomous Edge
Unlike traditional distros that rely on static package configs and upstream hypervisors, SigmaOS dynamically allocates compute resources (CPU, GPU, TensorCores) based on real-time AI workloads via Autonomous Agent Quotas. 

### 🧭 Command Grammar
SigmaOS executes tasks via short, sovereign commands. This grammar is optimized for AI-native execution without external dependencies.

| Command | Purpose | Example |
|---------|---------|---------|
| `agent.start` | Boot an autonomous agent | `agent.start` |
| `agent.quota` | Assign resource quotas | `agent.quota set=GPU:80%` |
| `agent.task` | Execute a kernel build or task | `agent.task run=compile_kernel` |
| `agent.sync` | Trigger Emergency Lattice Sync | `agent.sync` |
| `agent.container` | Deploy sovereign immutable containers | `agent.container deploy=nginx` |
| `agent.gaming` | Engage Vulkan/Proton gaming stack | `agent.gaming engage` |

### 🏗️ Agent Class Hierarchy (OOP)
Implemented in `/agents/` using strict C++ OOP principles:
- **`AgentBase`**: Abstract base class defining `start()`, `stop()`, and `executeTask()`.
- **`GPUAgent`**: Extends `AgentBase` for high-performance GPU tasks.
- **`NetAgent`**: Extends `AgentBase` for zero-trust network orchestration.
- **`QuotaManager`**: Singleton handling resource allocation policies.

### 🛡️ Quota Policies
- **Isolation**: Each agent manages its own quota to prevent resource leaking.
- **Dynamic Scaling**: Quotas adapt in real-time to system load.
- **Sovereign Enforcement**: Enforced at the kernel level via `SovereignAgentQuotasExtended.cpp`.

### ⚙️ Orchestration Logic
The `/agents/orchestration/CommandInterpreter.cpp` parses sovereign commands and maps them directly to system calls, bypassing the overhead of traditional shell interpreters.

1. **Parser**: Tokenizes the short command.
2. **Validator**: Checks against `GovernanceRules` for compliance.
3. **Executor**: Invokes the corresponding agent routine.
4. **Recovery**: Fallback to `/recovery/` hooks if execution fails.

This architecture permanently differentiates SigmaOS as the sole AI-native Sovereign OS.
