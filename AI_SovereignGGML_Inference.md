# Sovereign GGML Inference Engine: Local LLM Runtime Specification

> **Status**: 🔄 Active | **Component**: `SigmaGGML` | **Phase**: Phase 1 — Foundation AI

---

## 1. Executive Summary

The `SigmaGGML` inference engine provides fully local, offline-capable LLM execution inside SigmaOS without requiring any cloud API access. Inspired by the architecture of `llama.cpp` and `ggml`, it is implemented natively in Rust with a hardware-abstraction layer that automatically selects the best available compute backend: CPU SIMD, Vulkan compute shaders, CUDA, or Apple Metal.

All models are loaded in quantized formats (INT4, INT8, or FP16) to minimize memory footprint and maximize throughput on consumer hardware. The inference engine exposes a high-level Rust API consumed by the AI shell, NL-CLI, and semantic search subsystems.

---

## 2. Architecture

### 2.1 Core Components

```
┌─────────────────────────────────────────────────────────────────┐
│                      SIGMA GGML RUNTIME                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   MODEL LAYER                             │  │
│  │  ┌──────────────┐  ┌──────────────┐  ┌────────────────┐  │  │
│  │  │ Gemma-2B-INT4│  │ Whisper-Small│  │   MiniLM-L6   │  │  │
│  │  │ (1.2GB, Chat)│  │ (244MB, ASR) │  │ (22MB, Embed) │  │  │
│  │  └──────┬───────┘  └──────┬───────┘  └────────┬───────┘  │  │
│  └─────────┼─────────────────┼───────────────────┼───────────┘  │
│            └─────────────────┼───────────────────┘              │
│                    ┌─────────▼──────────┐                       │
│                    │   SigmaGGML Core   │                       │
│                    │ (Model loader, KV  │                       │
│                    │  cache, tokenizer) │                       │
│                    └─────────┬──────────┘                       │
│                              │                                  │
│  ┌───────────────────────────▼───────────────────────────────┐  │
│  │                  BACKEND ABSTRACTION                      │  │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐  │  │
│  │  │ CPU/SIMD │  │  Vulkan  │  │   CUDA   │  │  Metal  │  │  │
│  │  │ AVX2/512 │  │ Compute  │  │ (NVIDIA) │  │ (Apple) │  │  │
│  │  └──────────┘  └──────────┘  └──────────┘  └─────────┘  │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### 2.2 Rust API

```rust
// kernel/ai/inference/mod.rs
// SPDX-License-Identifier: MIT

pub struct SigmaInference {
    model: SigmaGGML,
    backend: InferBackend,
    kv_cache: KVCache,
}

pub enum Quantization {
    INT4,   // Lowest memory, ~10% quality loss
    INT8,   // Balanced — recommended default
    FP16,   // Highest quality, requires GPU
}

pub enum InferBackend {
    CpuSimd,    // AVX2/AVX-512 on x86, NEON on ARM
    Vulkan,     // Cross-platform GPU — recommended for iGPU
    Cuda,       // NVIDIA discrete GPU
    Metal,      // Apple Silicon
}

impl SigmaInference {
    pub fn load_model(path: &str, quant: Quantization) -> Result<Self> {
        let model   = SigmaGGML::load(path, quant)?;
        let backend = InferBackend::detect_best_available()?;
        let kv_cache = KVCache::new(model.context_length(), &backend);
        Ok(Self { model, backend, kv_cache })
    }

    /// Text generation — streamed token by token
    pub fn generate(&mut self, prompt: &str, max_tokens: usize) -> impl Iterator<Item=String> {
        self.model.generate_tokens(prompt, max_tokens, &self.backend, &mut self.kv_cache)
            .map(|tok| self.model.decode_token(tok))
    }

    /// Embedding — returns 384-dim vector for semantic search
    pub fn embed(&self, text: &str) -> Vec<f32> {
        self.model.embed(text, &self.backend)
    }
}
```

---

## 3. Model Zoo

| Model | Size (INT4) | Quantization | Primary Use Case | Backend |
|:------|:------------|:-------------|:-----------------|:--------|
| Gemma-2B | 1.2 GB | INT4 | NL-CLI, Chat | CPU/GPU |
| Whisper-Small | 244 MB | FP16 | Voice commands / ASR | CPU |
| MiniLM-L6 | 22 MB | INT8 | Semantic search embeddings | CPU |
| TinyBERT | 17 MB | INT8 | Anomaly classification | CPU |
| SchedulerNet | 256 KB | INT8 | EEVDF kernel autotuning | CPU |

---

## 4. Performance Targets

| Hardware Tier | Backend | Expected Tokens/s |
|:-------------|:--------|:-----------------|
| Budget (Celeron, 4GB RAM) | CPU AVX2 | ~3 tok/s |
| Mainstream (Core i5, 8GB) | CPU AVX2 | ~12 tok/s |
| Gaming (RTX 3060, 12GB VRAM) | CUDA | ~90 tok/s |
| Laptop iGPU (Intel Arc) | Vulkan | ~25 tok/s |
| Apple M2 | Metal | ~60 tok/s |

> [!NOTE]
> On hardware not meeting 4GB RAM, only the TinyBERT and SchedulerNet models are loaded. NL-CLI falls back to pattern-matching mode.

---

## 5. Privacy Guarantees
- All inference runs **entirely locally** — no telemetry, no model calls to cloud endpoints.
- Models are stored in `/sigma/store/ai-models/` with content-addressed SHA-256 verification to prevent tampered model injection.
- User prompts and query history are held only in RAM for the duration of the session; they are not persisted to disk unless explicitly requested.

---

## 6. References & Standards
- GGML Tensor Library — Georgi Gerganov (ggml.ai)
- Gemma Model Architecture (Google DeepMind)
- INT4/INT8 quantization for LLM inference (GPTQ, AWQ)
- Vulkan Compute API specifications
