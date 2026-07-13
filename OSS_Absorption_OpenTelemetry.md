# OSS Absorption: OpenTelemetry — Unified Observability

> **Status**: 📋 Planned | **Source Project**: OpenTelemetry (CNCF) | **Target Shard**: `SigmaOS Sovereign Observability`

---

## 1. Executive Summary

OpenTelemetry is the industry standard for collecting observability signals — traces, metrics, and logs — from applications and infrastructure. It provides vendor-neutral SDKs, a collector agent, and a wire protocol (OTLP).

SigmaOS absorbs OpenTelemetry's **unified signal model** (traces + metrics + logs in one pipeline) and **OTLP protocol** into the `sigma-telemetry` shard, making all kernel events, IPC calls, and application activity automatically observable without code changes.

---

## 2. Key Features to Absorb

### 2.1 Kernel-Level Auto-Instrumentation

Every shard IPC call, syscall, and filesystem operation automatically emits OTLP spans. Applications gain full observability without adding instrumentation code.

```bash
$ sigma telemetry query --trace "sigma-pkg install helix"
Σ [TELEMETRY] Auto-traces for sigma-pkg install helix:
  ┌─ [0ms]     IPC: sigpkg → S-NET (resolve mirrors)       5ms
  ├─ [5ms]     HTTP: GET pkg.sigma.dev/helix-1.0.spkg     340ms
  ├─ [345ms]   IPC: sigpkg → S-SEC (verify Dilithium5)      3ms
  └─ [348ms]   IPC: sigpkg → S-FS (extract + register)     22ms
  Total: 370ms
```

### 2.2 Unified Dashboard (`sigma telemetry dashboard`)

```bash
$ sigma telemetry dashboard
Σ [DASHBOARD] SigmaOS Observability (live)
  CPU:      42% (sched p99: 1.2ms)
  Memory:   3.1GB / 8GB (pfra: 0 evictions/s)
  Network:  1.2GB/s in, 340MB/s out
  Top spans: sigma-gateway (45ms p99), S-FS write (12ms p99)
  Errors:   0 in last 5m
```

### 2.3 OTLP Export for External Tooling

All signals are exportable over OTLP to external Grafana/Prometheus/Jaeger instances for teams that already have monitoring infrastructure.

```toml
# /etc/sigma/telemetry.toml
[export]
protocol = "otlp"
endpoint = "https://monitoring.example.com:4317"
tls = true
```

---

## 3. References & Standards

- OpenTelemetry — `opentelemetry.io` (Apache-2.0)
