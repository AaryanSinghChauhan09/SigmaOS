# Automation Workflows: Orchestrator Pipeline

Learn how to configure multi-step execution graphs in SigmaOS without human intervention.

---

## 🔗 Action-Trigger Model

Workflows are modeled as Directed Acyclic Graphs (DAGs) in `sigma_logic.rs`.

```
[System Trigger / Metric Alert]
              │
              ▼
    [Filter & Pre-Condition]
              │
              ▼
   ┌──────────┴──────────┐
   ▼                     ▼
[Action A: Sandbox]    [Action B: Telemetry]
   │                     │
   └──────────┬──────────┘
              ▼
     [Final Consolidation]
```

## ⚙️ Example: Dev Environment Setup
To setup a new development workspace, the agent automatically executes:
1. `sigpkg install gcc git rust`
2. `git clone <repository>`
3. Set workspace environment variables
4. Launch `sigma-edit` bound to the workspace directory.
