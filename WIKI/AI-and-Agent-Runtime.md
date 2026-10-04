# AI and Agent Runtime

SigmaOS ships a first-class AI subsystem built entirely in Rust. It provides on-device language model inference, autonomous OS-level agents, crash analysis, predictive resource management, and a plugin ABI for third-party AI extensions — all without Python or C dependencies.

---

## Architecture Overview

```
 ┌───────────────────────────────────────────────────────┐
 │                  User / Application Layer              │
 │   SigmaShell AI  │  GUI Copilot  │  Third-Party Apps  │
 └────────────────────────┬──────────────────────────────┘
                          │ Agent API
 ┌────────────────────────▼──────────────────────────────┐
 │               AI Agent Runtime (src/ai/)               │
 │  AgentRuntime │ TaskScheduler │ PluginRegistry         │
 │  CrashAnalyzer │ ResourcePredictor │ NlpProcessor      │
 └────────────────────────┬──────────────────────────────┘
                          │ Inference API
 ┌────────────────────────▼──────────────────────────────┐
 │            Inference Engine (src/ml/)                  │
 │  GGML bridge (Nim/Zig) │ Tensor ops │ Quantization     │
 └───────────────────────────────────────────────────────┘
```

---

## Components

### 1. Agent Runtime (`src/ai/agent_runtime.rs`)
- Multi-agent task executor with priority queuing
- Agent lifecycle: `Idle → Running → Suspended → Done`
- Crash dump analysis: automatic root-cause inference
- `ProcessId`-based correlation to running processes

### 2. NLP Processor (`src/ai/nlp_processor.rs`)
- Tokenization, stemming, entity recognition
- Intent classification for shell commands
- On-device inference (no cloud calls)

### 3. Resource Predictor (`src/ai/resource_predictor.rs`)
- Time-series forecasting of CPU/RAM/IO demand
- Feeds into the smart optimizer (`src/performance/smart_optimizer.rs`)
- Proactive NUMA page migration based on predictions

### 4. Crash Analyzer (`src/ai/crash_analyzer.rs`)
- Parses kernel oops, segfaults, panic traces
- Correlates with known CVEs and bug patterns
- Suggests fixes or mitigations automatically

### 5. Plugin Registry (`src/ai/plugin_registry.rs`)
- Stable ABI for third-party AI plugins (Rust dylib)
- Hot-reload without rebooting the agent runtime
- Sandboxed execution via SigmaOS container primitives

### 6. ML / Inference Engine (`src/ml/`)
- Quantized model loader (GGUF/GGML format)
- SIMD-accelerated matrix multiplication (AVX2/AVX-512)
- Batch inference with dynamic batching
- Written in Rust + Zig for critical inner loops

---

## Supported Models

| Model Size | Format | Use Case |
|-----------|--------|---------|
| 1–3B params | GGUF Q4 | Shell assistant, crash analysis |
| 7B params | GGUF Q4/Q8 | Code completion, doc generation |
| 13B+ params | GGUF Q2 | On-device reasoning (high-RAM systems) |

---

## AI-Driven OS Features

| Feature | Description |
|---------|-------------|
| **Smart Scheduler** | AI predicts next-to-run processes; preloads into cache |
| **Predictive I/O** | Prefetches files before app requests them |
| **Auto-Tune** | Adjusts kernel parameters based on workload profile |
| **Anomaly Detection** | Detects unusual CPU/network spikes, alerts user |
| **Natural Language Shell** | `sigma> "increase browser priority"` → `renice -5 $(pgrep firefox)` |

---

## Comparison vs Linux / Omarchy / Mint

| Feature | Linux | Omarchy | Mint | **SigmaOS** |
|---------|-------|---------|------|-------------|
| On-device AI | ❌ | ❌ | ❌ | ✅ |
| AI crash analysis | ❌ | ❌ | ❌ | ✅ |
| NL shell | ❌ | ❌ | ❌ | ✅ |
| Predictive I/O | ❌ | ❌ | ❌ | ✅ |
| AI plugin ABI | ❌ | ❌ | ❌ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/ai/agent_runtime.rs` | Core agent executor |
| `src/ai/nlp_processor.rs` | NLP tokenizer and intent classifier |
| `src/ai/resource_predictor.rs` | Workload forecasting |
| `src/ai/crash_analyzer.rs` | Automated crash triage |
| `src/ai/plugin_registry.rs` | Third-party AI plugin loader |
| `src/ml/` | Inference engine and model loader |
| `src/nlp/` | Low-level NLP primitives |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/ai/`, `src/ml/`, `src/nlp/`
> - Update supported model table when new GGUF versions are validated
> - Add new AI features as they are implemented in the source
> - Keep the comparison table current — check Omarchy AI roadmap on GitHub
