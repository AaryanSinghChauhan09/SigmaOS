# 🤖 Autonomous Agents: The Intelligence Layer

SigmaOS replaces traditional background daemons and systemd services with **Autonomous Agents**. These agents are self-healing, goal-oriented shards that orchestrate the system based on the **Context Manager**.

---

## 🏛️ Agent Hierarchy

The agent ecosystem is modularized into specialized tiers:

1. **Governance Agents**: Enforce security policies and resource quotas.
2. **Maintenance Agents**: Perform self-healing, log rotation, and cache purging.
3. **Observation Agents**: Monitor silicon health and network entropy.
4. **Interface Agents**: Suggest workflows and optimize the Zenith UI.
5. **Bridge Agents**: Manage legacy compatibility (e.g., Linux translation).

---

## 🏗️ Design Patterns

### 1. Goal-Based Execution
Agents are not just scripts; they are given "Intents" (e.g., "Minimize latency for gaming"). The agent then orchestrates kernel parameters, resource quotas, and background shards to achieve the goal.

### 2. Event-Driven Communication
All agents communicate via the **Sovereign Event Bus**. This ensures loose coupling—an agent can be swapped or updated without affecting the rest of the lattice.

### 3. Self-Healing Watchdogs
Each agent is monitored by a **Watchdog Shard**. If an agent crashes or consumes excessive resources, the watchdog restarts it and rolls back its state to the last known good configuration.

---

## 🛠️ Developer SDK

Developers can add new agents by subclassing `AgentBase`. 

```cpp
class MyCustomAgent : public AgentBase {
public:
    void onIntent(const Intent& goal) override {
        // Logic to achieve the goal
    }
};
```

All agents must adhere to the **Capability-Gated Security** model and report status to the `SovereignMonitor`.

---
*Autonomous agents: The brain of the Sovereign Lattice.*
