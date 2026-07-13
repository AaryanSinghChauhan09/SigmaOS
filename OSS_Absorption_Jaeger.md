# OSS Absorption: Jaeger — Distributed Tracing

> **Status**: 📋 Planned | **Source Project**: Jaeger (CNCF) | **Target Shard**: `SigmaOS Sovereign Trace Shard`

---

## 1. Executive Summary

Jaeger is an open-source, end-to-end distributed tracing system used for monitoring and troubleshooting microservice-based architectures. It implements the OpenTelemetry trace data model with span collection, storage, and visualization.

SigmaOS absorbs Jaeger's **trace context propagation**, **span collection pipeline**, and **adaptive sampling** into `sigma-trace`, giving every IPC message and syscall an automatic causal trace without external tooling.

---

## 2. Key Features Absorbed

### 2.1 Automatic Trace Context Propagation

Every IPC message in SigmaOS carries a 128-bit trace ID and a 64-bit span ID in its capability token header. When shard A calls shard B, the trace context is automatically forwarded, creating a complete call graph.

```bash
$ sigma trace list --last 5m
Σ [TRACE] Recent traces:
  TRACE ID                          ROOT SPAN           DURATION  SPANS
  a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6  sigma-pkg install   1.2s      14
  f1e2d3c4b5a6f7e8d9c0b1a2f3e4d5c6  sigma-gateway req   45ms       6
```

### 2.2 Span Visualization

```bash
$ sigma trace show a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6
Σ [TRACE] Trace a1b2..c5d6 — sigma-pkg install rust
  ├─ [0-200ms]  sigpkg::resolve_deps        — 3 deps found
  ├─ [200-400ms] sigpkg::download            — 2.1MB fetched
  ├─ [400-900ms] sigpkg::verify_signature    — Dilithium5 OK
  ├─ [900-1100ms] sigpkg::extract            — /sigma/store/...
  └─ [1100-1200ms] sigpkg::register          — DB updated
```

### 2.3 Adaptive Sampling

To avoid overwhelming storage with traces from high-throughput services, `sigma-trace` uses adaptive sampling: it traces 100% of slow or errored requests but samples only 1% of fast, successful requests.

---

## 3. References & Standards

- Jaeger — `jaegertracing.io` (Apache-2.0)
- OpenTelemetry — `opentelemetry.io`
