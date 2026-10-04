# Multi-Agent Provider Orchestration Engine

SigmaOS features `OmarchyMultiAgentProvider` (`src/ai/omarchy_multi_agent_provider.rs`), inspired by Omarchy's `agents-cursor-opencode-copilot-muse` branch.

---

## 1. Architectural Highlights

* **Unified Agent Hub**: Standardizes communication across heterogeneous AI agents:
  * **GitHub Copilot**: Inline completions and PR workflows.
  * **Cursor Agent**: Context-aware refactoring and architecture planning.
  * **OpenCode**: Autonomous shell and code execution interpreter.
  * **Muse**: Creative, multimodal media and theme generation.
  * **Sovereign Local**: Local offline LLM fallback engine.
* **Kernel-Side Context Sharing**: Zero-copy ring-buffer IPC allows agents to pass project context without redundant network serialization.
* **Deterministic Fallback**: Automatically routes queries to the local offline engine if cloud providers are unreachable.

---

## 2. API & Usage

```rust
use crate::ai::omarchy_multi_agent_provider::OmarchyMultiAgentProvider;

let mut orchestrator = OmarchyMultiAgentProvider::new();
assert_eq!(orchestrator.agent_count(), 5);

// Dispatch task to Cursor Agent
let task_id = orchestrator.dispatch_prompt("cursor-agent", "Refactor scheduler").unwrap();
```
