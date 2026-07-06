# sigma-ai — On-Device AI Daemon Specification

**Status:** Draft · Target: v0.3 (local inference) / v1.0 (full integration)
**Owner:** intelligence/ai team
**Canonical source:** `userland/sigma-ai/`, `suites/S034-AI/`

---

## Overview

sigma-ai is the on-device AI inference daemon for SigmaOS. It loads quantised GGUF models locally, exposes a REST + IPC inference API, and powers sigma-copilot (shell completion, doc search, code assist). No data leaves the device — ever.

## Goals

- 100 % on-device: zero cloud API calls, zero telemetry

- First token latency < 500 ms on x86_64 with AVX-512 (8B Q4 model)

- Capability-gated access: processes must hold `"ai"` pledge token

- Streaming token output via Server-Sent Events (REST) and sigma-bus stream IPC

- Model packages distributed as `.sigpkg` format (same pipeline as regular packages)

- Audit log: every inference request logged with requesting PID and token count

---

## Architecture

```
┌──────────────────────────────────────────────────┐
│  Clients                                         │
│  sigma-copilot  │  REST (127.0.0.1:7734)         │
│  sigma-sh       │  IPC sigma-bus://ai/infer      │
└────────────────┬─────────────────────────────────┘
                 │
┌────────────────▼─────────────────────────────────┐
│  sigma-aid daemon (pledge: stdio rpath ai tpm2)  │
│  ┌──────────────┐  ┌──────────────────────────┐  │
│  │ Model Loader │  │ Inference Engine         │  │
│  │ GGUF parser  │  │ AVX-512 / NEON backends  │  │
│  │ mmap weights │  │ llama.cpp-inspired KV    │  │
│  └──────────────┘  └──────────────────────────┘  │
│  ┌──────────────┐  ┌──────────────────────────┐  │
│  │ Rate Limiter │  │ Audit Logger             │  │
│  │ token bucket │  │ /var/log/sigma-ai.audit  │  │
│  └──────────────┘  └──────────────────────────┘  │
└──────────────────────────────────────────────────┘
                 │ mmap
┌────────────────▼─────────────────────────────────┐
│  Model Store: /opt/sigma-ai/models/<name>.gguf   │
└──────────────────────────────────────────────────┘
```

---

## GGUF Model Loading

- Parse GGUF header: magic, version, metadata KV pairs, tensor descriptors

- `mmap()` entire weight file — no heap copy; OS page cache handles eviction

- Support quantisation types: Q4_0, Q4_K_M, Q8_0, F16, F32

- Multi-model: up to 4 models loaded simultaneously (per-process mmap regions)

- Model package format: standard `.sigpkg` with `payload/opt/sigma-ai/models/<name>.gguf`

---

## Inference API

### REST (HTTP/1.1 on 127.0.0.1:7734)

```
POST /v1/infer
Content-Type: application/json
{
  "model": "sigma-7b-q4",
  "prompt": "...",
  "max_tokens": 256,
  "temperature": 0.7,
  "stream": true
}

Response (stream=true): text/event-stream
data: {"token": "Hello", "done": false}
data: {"token": " world", "done": false}
data: {"token": "", "done": true, "total_tokens": 12}
```

```
GET  /v1/models          → JSON array of loaded model metadata
POST /v1/embed           → return embedding vector (float32 array)
GET  /v1/health          → {"status":"ok","model_count":2}
```

### IPC (sigma-bus)

`sigma-bus://ai/infer` — same JSON request body; response delivered as bus message stream.

---

## Compute Backends

| Backend | Condition | Ops/s target |
|---------|-----------|-------------|
| AVX-512 | x86_64 + AVX-512 flag | 25 tok/s (7B Q4) |
| AVX2 | x86_64 fallback | 12 tok/s |
| NEON | ARM64 | 18 tok/s (NEON + dotprod) |
| WASM SIMD | Browser profile | 4 tok/s |
| Scalar | All others | 2 tok/s |

Backend selected at daemon startup via CPUID / AT_HWCAP.

---

## Rate Limiting

- Per-PID token bucket: 10 000 tokens/minute burst, 1 000 tokens/minute sustained

- System-wide cap: 50 000 tokens/minute total (configurable in `/etc/sigma-ai.conf`)

- Exceeded: HTTP 429 / IPC `SIGMA_ERR_RATE_LIMITED`

- Admin processes with `"ai_unlimited"` pledge bypass rate limiting

---

## Capability Gate

Callers need `"ai"` in sigma_pledge. vaultd provides capability tokens. If capability absent: connection rejected with audit entry `"DENIED"`.

---

## Audit Log

Format (JSON Lines, append-only, fsync each entry):
```json
{"ts":1700000000,"pid":512,"comm":"sigma-copilot","model":"sigma-7b-q4",
 "prompt_tokens":128,"completion_tokens":64,"latency_ms":312,"result":"ok"}
```

---

## sigma-copilot Integration

sigma-copilot is a thin shard (`suites/S034-AI/copilot/`) that wraps sigma-aid:

- Shell completion: sends last 512 tokens of shell history as prompt; returns completion suggestion

- Doc lookup: embeds query, cosine-searches local docs embedding index

- Code assist: context-window injection of open file + cursor position

---

## Model sigpkg Format

```
sigma-7b-q4-0.1.0-any.sigpkg
├── META/manifest.toml     (type="model", requires_capability="ai")
├── META/checksums.b3
├── META/signature.dil5
└── payload/
    └── opt/sigma-ai/models/sigma-7b-q4.gguf
```

---

## Implementation Plan

- [ ] 1. GGUF header parser (`src/gguf.c`)

- [ ] 2. mmap weight loader (`src/model_loader.c`)

- [ ] 3. Tensor compute engine — scalar baseline (`src/compute_scalar.c`)

- [ ] 4. AVX-512 backend (`src/compute_avx512.c`)

- [ ] 5. NEON backend (`src/compute_neon.c`)

- [ ] 6. KV cache manager (`src/kv_cache.c`)

- [ ] 7. Tokenizer (BPE, loaded from model metadata)

- [ ] 8. REST HTTP server (127.0.0.1:7734) with SSE streaming

- [ ] 9. sigma-bus IPC endpoint

- [ ] 10. Rate limiter (token bucket per PID)

- [ ] 11. sigma_pledge self-restriction for sigma-aid

- [ ] 12. Audit logger

- [ ] 13. sigma-copilot shard (shell + docs + code assist)

- [ ] 14. Model sigpkg installer hook

- [ ] 15. Tests: GGUF load, tokenizer round-trip, rate limit, audit log

---

## Status

| Feature | State |
|---------|-------|
| GGUF loader | ⬜ Not started |
| Scalar inference | ⬜ Not started |
| AVX-512 backend | ⬜ Not started |
| REST API | ⬜ Not started |
| sigma-copilot | ⬜ Not started |
| Rate limiting | ⬜ Not started |
| Audit log | ⬜ Not started |
