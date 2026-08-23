# S-AI: SigmaOS AI Subsystem

## Architecture

```
User Request
  ↓
Orchestrator (decompose + route)
  ↓
Specialist Agents (Code, System, Security, Analysis, Debug)
  ↓
LLM Router (select best local model)
  ↓
Local LLM: llama.cpp | Ollama | LM Studio | vLLM
```

## Sigma Copilot

```bash
sigma-ai "how do I set up WireGuard?"      # Q&A
sigma-ai code "write a Rust HTTP server"   # Code gen
sigma-ai debug --log /var/log/sigma.log    # Debug
sigma-ai analyze --system                  # System analysis
sigma-ai explain src/kernel/scheduler.rs   # Code explain
sigma-ai summarize docs/security.md        # Summarize
```

**Access**: `Super+A` hotkey, system tray button, or `sigma-ai` CLI.

## Agents

| Agent | Specialization |
|-------|---------------|
| PlannerAgent | Multi-step decomposition |
| CodeAgent | Code generation/review |
| SystemAgent | OS configuration |
| SecurityAgent | Threat analysis |
| DebugAgent | Error diagnosis |
| SearchAgent | Information retrieval |
| SummaryAgent | Text summarization |

## LLM Router Model Selection

| Task | Model |
|------|-------|
| Code generation | Codestral-22B / Deepseek-Coder |
| System analysis | Llama-3.1-70B |
| Quick Q&A | Phi-3-mini (3.8B) |
| Summarization | Mistral-7B |

## AI-Native OS Features

### Neural Power Manager

```
Workload history → LSTM predictor → CPU frequency decision → cpufreq update
(every 100ms)       (5min lookahead)   (P-state 0–max)
```

### Predictive Prefetcher

```
Session file access history → Markov chain → Prefetch list → readahead()
```

### AI Crash Analyzer

Auto-analyzes core dumps on crash, identifies root cause, generates fix suggestions.

### Intelligent Terminal

Context-aware command completion, error explanation, natural language to command.

## Model Management

```bash
sigma-ai models list
sigma-ai models download llama3.1:8b
sigma-ai models set-default llama3.1:8b
sigma-ai models benchmark --task code-gen
```

## Privacy

- All inference **100% local** by default
- Zero telemetry/data collection
- AI memory isolated from user data
- Optional cloud API (user-configured, opt-in)
