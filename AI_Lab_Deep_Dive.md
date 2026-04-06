# 🤖 AI Lab Deep Dive

SigmaOS is **AI-native by design** — intelligence is embedded at every kernel layer, not installed as an external package.

---

## Architecture

```text
User Prompt
    │
    ▼
[IPC Dispatcher: sigma_ai_distribute.c]
    │
    ├──► Local LLM Socket (/var/ipc/sigma_llm.sock)       [Language tasks]
    ├──► Code Model Socket (/var/ipc/sigma_code.sock)      [Code generation]
    └──► Forensic AI Socket (/var/ipc/sigma_forensic.sock) [Threat analysis]
```

All routing happens at Ring-0 level via UNIX socket IPC. **No HTTP, no cloud, no API keys.**

---

## Local AI Models Supported

| Model | Type | Socket |
| :--- | :--- | :--- |
| `Sigma_QWen_local` | General LLM | `/var/ipc/sigma_llm.sock` |
| `Sigma_StarCoder_local` | Code generation | `/var/ipc/sigma_code.sock` |
| `Sigma_Forensic_Analyst` | Security analysis | `/var/ipc/sigma_forensic.sock` |

---

## OOP AI Dispatch Pattern

```c
CLASS_DECLARE(AIModel) {
    SigmaObject_t core;
    const char*   name;
    sigma_u32     priority_weight;
    VIRTUAL(void, dispatch, struct AIModel* self, const char* prompt);
};
```

Example usage from the kernel:

```c
AIModel_t llm = create_ai_model("Sigma_QWen_local", "/var/ipc/sigma_llm.sock", 100);
llm.dispatch(&llm, "Summarize memory allocation anomalies in last 10 seconds.");
```

---

## AI Features Across the OS

| Feature | Description | Location |
| :--- | :--- | :--- |
| **Multi-Model Distributor** | Route one prompt to N models in parallel | `sigma_ai_distribute.c` |
| **Autonomous Scheduler AI** | Predict resource needs before OOM | `scheduler_ai.c` |
| **AI VFS Organizer** | Classify files by content automatically | `SovereignSearch.c` |
| **AI Forensic Detector** | ML-based anomaly detection in memory | `SovereignForensicMatrix.c` |
| **NCERT AI Tutor** | Subject-aware contextual quiz generation | NCERT shards |
| **DSA AI Coach** | Real-time hints and complexity analysis | DSA shard |
| **LLM-Assisted Shell** | Predictive command suggestions in omni_shell | `omni_shell.c` |

---

## Sovereign AI Manifest

- **No cloud dependency**: All inference runs locally
- **No API keys**: No third-party AI services required
- **No data leakage**: Zero network calls during AI operations
- **Persona-aware**: AI adapts to developer, student, researcher, forensic analyst personas

---

## Future Roadmap

- Native GGUF model loading without llama.cpp
- Hardware GPU dispatch via direct OpenCL syscall simulation
- Gradient descent visualization rendered natively in the browser UI
- Multi-agent co-work bus with task allocation and result merging
