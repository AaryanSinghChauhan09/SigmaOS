# Σ SIGMAOS: AUTONOMOUS AGENTS DEEP DIVE (🤝🔮🦾)

[![Domain](https://img.shields.io/badge/Domain-AGENTS-00d2ff?style=for-the-badge)]()

**SIGMA_AGENTS** disrupts the cloud-based multi-agent paradigm (e.g., Claude Co-Work, Perplexity, OpenClaw) by enforcing local, silicon-parity computation and Inter-Process Communication (IPC).

## 🤝 CO-WORK IPC (`coworkshard`)

Traditional co-work spaces relay your data through remote servers. SigmaOS initiates a **Multi-Agent Collaboration Bus** inside the local kernel memory using `SovereignCowork.c`. Messages are passed directly via C-pointers (`SigmaAgentIPC`), simulating multi-agent synchronicity with **Zero external web-socket connections**.

## 🔮 COMPUTE ORACLE (`oracleshard`)

Instead of hitting external graph APIs for retrieval-augmented generation (RAG), the Compute Oracle relies on **Local VFS RAM-Disk scans**. Using `sigma_silicon_search` in `SovereignComputeOracle.c`, knowledge indexing is purely bound to your local hardware. You have absolute privacy.

## 🦾 MACRO CLAW (`clawshard`)

Computer use models often rely on heavy automation wrappers (Electron). The **Macro Claw** strips this down to hardware pointer mutation. `sigma_teleport_mouse` in `SovereignMacroClaw.c` manually edits the `(x,y)` coordinate memory of the active cursor state, allowing for precise, low-level OS automation.

---
<<<<<<< HEAD

## Architecture

```text
Mission Context (User Input)
    └─► Co-Work IPC Bus (ipc.c)
            ├─► Agent Alpha: LLM Analysis     (/var/ipc/sigma_llm.sock)
            ├─► Agent Beta:  Code Generation  (/var/ipc/sigma_code.sock)
            ├─► Agent Gamma: Forensic Scan    (/var/ipc/sigma_forensic.sock)
            └─► Orchestrator: Merge + Report  (sigma_ai_distribute.c)
```

---

## Agent Types

| Agent | Specialty | Socket |
| ------- | ----------- | -------- |
| **Sigma LLM** | General reasoning, summarization | `/var/ipc/sigma_llm.sock` |
| **Sigma Coder** | Code generation, debugging | `/var/ipc/sigma_code.sock` |
| **Sigma Forensic** | Threat analysis, anomaly detection | `/var/ipc/sigma_forensic.sock` |
| **Sigma Tutor** | Educational scaffolding (NCERT/DSA) | In-process (ncert_shard.c) |
| **Sigma Researcher** | Literature synthesis, citation analysis | In-process (cs_research_shard.c) |

---

## The Orchestrator Pattern

The `sigma_ai_distribute.c` orchestrator dispatches one mission to all agents simultaneously:

```c
// All agents receive the same prompt
const char* mission = "Analyze kernel memory for unauthorized write hooks.";

local_llm.dispatch(&local_llm,   mission);   // Reasoning track
code_model.dispatch(&code_model,  mission);  // Code fix track
forensic.dispatch(&forensic,      mission);  // Threat track
```

Responses are collected over IPC and merged by the orchestrator into a unified report.

---

## Agent Priority System

Each `AIModel_t` instance carries a `priority_weight`:

| Priority | Meaning |
| ---------- | --------- |
| 100 | Critical path — runs first, blocks others |
| 80–99 | Standard — runs concurrently |
| < 80 | Background — runs when CPU idle |

```c
AIModel_t llm     = create_ai_model("Sigma_QWen",    sock1, 100); // Critical
AIModel_t coder   = create_ai_model("Sigma_Coder",   sock2, 85);  // Standard
AIModel_t forensic = create_ai_model("Sigma_Forensic",sock3, 95); // High
```

---

## Aether Orchestrator (`SovereignAetherOrchestrator.c`)

A higher-level orchestration kernel that manages multi-shard AI pipelines:

- Task queue allocation across agents
- Resource quota enforcement per agent
- Graceful degradation if an agent fails
- Log aggregation and unified report assembly

---

## Roadmap

- [ ] Dynamic agent spawning based on task complexity detection
- [ ] Agent specialization via persona-aware fine-tuning
- [ ] Cross-agent memory sharing via shared VFS namespace
- [ ] Streaming token merging for real-time unified responses
=======
**Σ SIGMAOS: LOCAL AGENTS. ABSOLUTE PRIVACY. SOVEREIGN AUTOMATION.**

>>>>>>> 83e117acaff1ccc62b67a2adfc253454bcf701ae
