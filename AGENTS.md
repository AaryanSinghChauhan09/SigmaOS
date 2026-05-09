# SigmaOS Autonomous Agents

SigmaOS replaces traditional background daemons and systemd services with **Autonomous Agents**. These agents operate with self-healing capabilities, orchestrating system quotas, enforcing governance policies, and dynamically adjusting the system based on the Context Manager.

## Core Agents Directory (`/agents/`)
The agent hierarchy is modularised to prevent tight coupling:

### 1. Quota Agent (`/agents/quota/`)
- **Role**: Monitors hardware telemetry (CPU, RAM, GPU, Disk I/O).
- **Mechanism**: Dynamically allocates resources based on the active Profession Profile.
- **Example**: If the `cashier` profile is active, background compile jobs are starved of CPU in favor of the point-of-sale UI.

### 2. Policy Agent (`/agents/policy/`)
- **Role**: Enforces governance, compliance, and zero-trust sandbox rules.
- **Mechanism**: Reads from `sandbox_policy.json` and ensures no unauthorized IPC happens between shards.
- **Example**: Blocks network access to the `doctor` profile's EHR module unless explicitly verified by the QKD (Quantum Key Distribution) module.

### 3. Orchestration Agent (`/agents/orchestration/`)
- **Role**: The central command interpreter (`CommandInterpreter.cpp`).
- **Mechanism**: Replaces standard shell scripts with polymorphic commands.
- **Example**: Running `agent.task run` will use the Context Manager to look up the active profession and execute the contextual task (e.g., generating a tax report for the `accountant` profile).

## Agent Extensibility
Developers can add new agents by subclassing `AgentBase`. All agents must communicate exclusively via the `SovereignEventBus` or through the `ContextManager` to avoid hardcoded dependency coupling.
