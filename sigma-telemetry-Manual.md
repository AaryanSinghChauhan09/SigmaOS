# sigma-telemetry Manual

## NAME

`sigma-telemetry` — SigmaOS syscall latency histogram and OpenTelemetry exporter

## SYNOPSIS

```
sigma-telemetry <stats|export|power|input-latency|help> [options]
```

## DESCRIPTION

`sigma-telemetry` reads per-syscall latency histograms from `/proc/sigma/syscall_stats` (a SigmaOS kernel extension) and exports them as OpenTelemetry OTLP/HTTP metrics. It also measures RAPL power attribution per shard and end-to-end input latency. Source: `userland/tools/sigma_syscall_telemetry.nim`.

## COMMANDS

### `stats [--top <n>]`

Show a latency histogram table for all tracked syscalls, sorted by p99 latency descending.

```bash
sigma-telemetry stats           # all syscalls
sigma-telemetry stats --top 10  # top 10 by p99
```

Output columns: `Syscall | Calls | p50 | p95 | p99 | mean (µs)`

Colour coding:
- Green: p99 < 100 µs (fast)
- Yellow: p99 100–1000 µs (moderate)
- Red: p99 > 1000 µs (slow — investigate)

### `export [endpoint]`

Push metrics to an OpenTelemetry collector (OTLP/HTTP):

```bash
sigma-telemetry export                          # default: localhost:4318
sigma-telemetry export http://otel-collector:4318
```

Also saves a local copy to `~/.cache/sigma/otel_metrics.json`.

The output format is OTLP-compatible `resourceMetrics` JSON with histogram data points, bucket counts, and `sigma.syscall.latency.<name>` metric names.

### `power [pid]`

Measure per-process power consumption using Intel RAPL counters (requires `/sys/class/powercap/intel-rapl`):

```bash
sigma-telemetry power          # total package power (2s window)
sigma-telemetry power 1234     # attribute to PID 1234
```

Returns power in Watts. Falls back gracefully on non-Intel hardware.

### `input-latency`

Measure end-to-end input latency (key press → frame rendered to screen):

```bash
sigma-telemetry input-latency
```

Reports p50, p95, p99 latency in milliseconds and flags whether the p95 target of 16ms (60 FPS) is met.

## HISTOGRAM FORMAT

Buckets (µs): `1, 5, 10, 25, 50, 100, 250, 500, 1000, 5000, 10000`

The 12-bucket histogram captures the full latency distribution from single-digit µs (fast syscalls like `getpid`) to multi-millisecond (slow syscalls like `fork`, `execve`).

## OPENTELEMETRY INTEGRATION

When pushing to a collector, the resource attributes include:

| Attribute | Value |
|-----------|-------|
| `service.name` | `sigmaos-kernel` |
| `host.name` | system hostname |

Each metric is named `sigma.syscall.latency.<syscall_name>` as a histogram metric with `unit: us`.

### Example Grafana query
```
histogram_quantile(0.99, rate(sigma_syscall_latency_read_bucket[5m]))
```

## EXAMPLES

```bash
# Show syscall stats with top-10 hotspots
sigma-telemetry stats --top 10

# Export to local Prometheus/OTel stack
sigma-telemetry export http://localhost:4318

# Check power use during a build
sigma-telemetry power $(pgrep cargo)

# Verify input latency before releasing a desktop update
sigma-telemetry input-latency
```

## VERSION

sigma-telemetry 1.0.0 (Nim, stdlib only)

## SEE ALSO

`sigma-trace(1)`, `sigma-monitor(1)`, `sigma_diagnostics(1)`, `sigma bench(1)`
