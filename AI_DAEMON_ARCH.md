# 🤖 SigmaOS: AI-Native Next-Gen Orchestration Architecture

This document specifies the internal orchestration and security architecture of the next-generation AI daemon in SigmaOS.

---

## 🏗️ 1. Multi-Device Model Scheduling

SigmaOS prioritizes on-device deep learning execution. The AI scheduler (`LocalLlmOrchestrator`) manages allocations:

```
                  +---------------------------+
                  |  LocalLlmOrchestrator     |
                  +---------------------------+
                                |
            +------------------+------------------+
            |                  |                  |
            v                  v                  v
      +------------+     +------------+     +------------+
      | TPU Memory |     | GPU Memory |     | CPU Fallbk |
      +------------+     +------------+     +------------+
```

1. **TPU Allocation:** Attempts mapping of local LLM models (e.g., Llama, Phi, Mistral) onto hardware-accelerated TPU slots.
2. **GPU Fallback:** If TPU memory limit is reached, schedules the model onto VRAM with strict bounds checking.
3. **CPU Fallback:** Employs standard virtual memory demand paging with Transparent Huge Pages (THP) for efficient CPU instruction execution.

---

## 🔒 2. OpenShell Sandboxing & Privacy Guardrails

To run safe, always-on AI assistants completely immune to prompt injection:
- **`PrivacyRouter`:** Scans input prompt bytes, redacting confidential markers (Credit Cards, Aadhaar) before they are dispatched to local model weights.
- **`DefaultDenyNetworkPolicy`:** Default-denies all outbound Internet access from agent-spawned sub-processes, allowing connection only to whitelisted API endpoints.
- **`OpenShellAgentSandbox`:** Filters output commands against shell-escaping injection sequences (such as `sudo`, `chmod`, `rm -rf`).
