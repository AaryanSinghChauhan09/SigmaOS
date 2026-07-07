# AI Agent & Automation Features

SigmaOS integrates AI natively to assist developers, operators, and end-users.

## Kernel Predictive Scheduler (`sigma_ai_sched.rs`)
An adaptive kernel subsystem that tracks historical metrics for every executing process (execution duration, peak memory footprint).
* **Pre-warming:** Predicts the memory required for incoming tasks to aggressively pre-allocate buffers.
* **Scheduling Hint:** Anticipates execution time to inform CFS and EDF scheduling queues dynamically.

## Natural Language Translator (`sigma_ai_agent.rs`)
A userland stub tool designed to convert natural language queries (e.g., "how much free memory do I have?") into executable shell commands (e.g., `free -h`). Future integration includes a fully offline llama.cpp LLM backend communicating via IPC.

## Semantic Error Explainer (`sigma_error_explain.rs`)
Intercepts common system error codes (like `EPERM` or `ENOENT`) and produces human-readable diagnostic messages. Includes actionable fix hints, effectively replacing generic opaque system failure codes with contextual AI-driven guidance.
