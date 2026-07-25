# SigmaOS — AI, ML, Data Science & CS Algorithms

## Overview

SigmaOS is designed as an **AI-native OS** — not just an OS that runs AI applications, but one where AI is woven into the kernel, scheduler, security, and user experience.

---

## On-Device AI Stack

### sigma-ai Daemon
- **Location:** `sigmad/sigma_ai_daemon.py` + `kernel/core/sigma_local_llm.rs`
- **Protocol:** HTTP API on `localhost:11434` (Ollama-compatible)
- **Models supported:** Phi-3-mini, Gemma-2B, DeepSeek-Coder (GGUF format)

```bash
sigma-ai run "explain this error"
sigma-ai generate --model phi3 "write a Python hello world"
sigma-ai embed "text to embed" --model bert
```

### Inference Engine (`sigma_local_llm.rs`)
Built-in transformer inference without any Python or external ML library:
- **Attention:** scaled dot-product with RoPE positional encoding
- **Normalization:** RMS norm
- **Activation:** SiLU (Llama FFN style)
- **Quantization:** F32/F16 full precision + Q4_K/Q8_0 planned
- **Sampling:** temperature + top-p nucleus sampling
- **Tokenizer:** BPE-compatible with greedy longest-match

### Supported Models (Planned Full Integration)
| Model | Size | Use Case | Phase |
|-------|------|----------|-------|
| Phi-3-mini Q4_K | 2.3 GB | General / shell | M |
| Gemma-2B Q4_K | 1.5 GB | Fast inference | M |
| DeepSeek-Coder 1.3B | 0.8 GB | Code completion | M |
| Whisper-small | 0.5 GB | Voice/Bhashini | M |
| BERT-multilingual | 0.5 GB | Indic NLP | M |
| Llama-3 8B Q2_K | 3.5 GB | Advanced (16GB+) | N |

---

## AI in the Kernel

### Adaptive Scheduler (`kernel/sched/sigma_transformer_sched.rs`)
- Predicts whether a task is I/O-bound or CPU-bound using a tiny 2-layer transformer
- Features: IPC counter, cache miss rate, voluntary/involuntary context switches
- Adjusts MLFQ level pre-emptively → 15–30% lower tail latency

### AI Intrusion Detection (`kernel/core/`)
- Anomaly detection on syscall sequences
- Baseline: normal process behavior profile
- Alert: deviation > 3σ from baseline
- Zero false-positive guarantee via conservative threshold

### Neural Memory Prefetcher
- Trains on page access patterns
- Prefetches pages before they fault
- Implemented in `kernel/optimizations/`

---

## Machine Learning Algorithms (Built-In)

### Kernel Data Structures
All implemented without `std` or external crates:

| Algorithm | Location | Use |
|-----------|----------|-----|
| Buddy allocator | `sigma_pmm.rs` | Physical memory |
| Slab allocator | `sigma_mm.rs` | Kernel objects |
| MLFQ scheduler | `sigma_sched.rs` | Process scheduling |
| Red-black tree | `kernel/core/` | VMA management |
| B+ tree | `sigma_vfs_ext4.rs` | Filesystem index |
| Lock-free ring buffer | `sigma_ipc_pipe.rs`, `sigma_sound.rs` | IPC, audio |
| Skip list | `kernel/sched/` | CFS vruntime |
| Bloom filter | `userland/pkg/` | Package cache |

### ML Algorithms (Phase M, `modules/sdk/sigma_ml_kit/`)
- Linear regression + ridge
- K-means clustering
- Decision tree + random forest
- Gradient boosting
- PCA / SVD decomposition
- Naive Bayes
- SVM with RBF kernel
- LSTM/GRU inference
- Transformer forward pass (already in `sigma_local_llm.rs`)
- Bayesian inference
- Gaussian processes

---

## Data Science Tools

### Built-in (`kernel/core/sigma_stats_engine.rs`)
- Descriptive statistics: mean, variance, std, percentiles
- Hypothesis testing: t-test, chi-square, ANOVA
- Linear regression
- FFT (for signal processing)
- Matrix operations: multiply, inverse, determinant, SVD

### sigma-data CLI
```bash
sigma-data analyze /path/to/data.csv --stats
sigma-data plot histogram --col age
sigma-data train --model linear_regression --target price
sigma-data predict --model my_model.sigml --input features.json
```

---

## Computer Science Fundamentals

### Algorithms in Kernel
| Category | Algorithms |
|----------|-----------|
| Sorting | Timsort (process scheduling), Heapsort (priority queue) |
| Graph | BFS/DFS (package deps), Dijkstra (routing), Bellman-Ford |
| String | KMP (pattern matching in IDS), Rabin-Karp (virus scan) |
| Crypto | SHA-256, HMAC, PBKDF2, HKDF, Kyber, Dilithium |
| Compression | LZ77, Huffman, Zstd (planned) |
| Hashing | FNV-1a, xxHash, SipHash-2-4 (HashMap) |
| Consensus | Raft (distributed updates), CRDT (offline sync) |
| Trees | AVL, Red-Black, B+, Radix, Trie (DNS/routing) |

---

## AI Workflow Integration

### Shell Completion
```bash
# Type partial command, AI suggests completion
$ sigma-ai shell "list all files modified today"
→ find . -newermt $(date +%Y-%m-%d) -type f
```

### Error Explanation
```bash
$ myapp
Segmentation fault (core dumped)
$ sigma-explain last-error
→ Process accessed unmapped memory at 0x0000000000000000.
  Likely cause: NULL pointer dereference in line 42 of main.c.
  Suggestion: check return value of malloc() before use.
```

### Code Review
```bash
$ sigma-ai review --file my_driver.rs --focus safety
→ Line 47: unsafe block without bounds check on raw pointer.
  Suggestion: add assert!(offset < buf.len()) before write.
```

---

## Federated Learning (Phase M)

SigmaOS nodes can participate in federated model training:
- Local gradient computation on private data
- Differential privacy: add calibrated Gaussian noise (ε=1.0)
- Aggregation via Raft-based coordinator
- No raw data leaves the device

```bash
sigma-ai federate join --coordinator 192.168.1.100:9090
sigma-ai federate contribute --model sigma-intrusion-v1
```

---

## Explainable AI

Every AI decision in SigmaOS can be explained:
- Scheduler: "Task X moved to Q2 because CPU burst exceeded 8ms quantum"
- IDS: "Alert: process opened 500 files in 1s (baseline: 5/s)"
- Package recommender: "Suggested sigma-vim because 80% of developers also install it"

This is implemented via attention weight visualization in `sigma_local_llm.rs`.
