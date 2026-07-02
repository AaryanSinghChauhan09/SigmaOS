# OpenTelemetry Collector Integration

## Overview

SigmaOS runs the [OpenTelemetry Collector](https://github.com/open-telemetry/opentelemetry-collector) (Apache-2.0) as a **sigma-bus shard** (`sigma-otel-collector`). It collects distributed traces from sigma-bus IPC calls and exports metrics from sigma-monitor to Jaeger (traces) and Prometheus (metrics).

---

## Architecture

```
sigma-bus IPC calls
        │  OTLP/gRPC spans (opentelemetry-rust SDK)
        ▼
  sigma-otel-collector (OTel Collector shard)
        │  pipeline: receiver → processor → exporter
        ├──► Jaeger exporter (traces)  → Jaeger UI
        └──► Prometheus exporter (metrics) → Grafana
```

---

## OTel Collector Config (`/etc/sigma/otel-config.yaml`)

```yaml
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: "127.0.0.1:4317"
      http:
        endpoint: "127.0.0.1:4318"

processors:
  batch:
    timeout: 5s
    send_batch_size: 512
  memory_limiter:
    check_interval: 1s
    limit_mib: 64

exporters:
  jaeger:
    endpoint: "jaeger:14250"
    tls:
      insecure: true
  prometheus:
    endpoint: "0.0.0.0:8888"
  logging:
    verbosity: normal

service:
  pipelines:
    traces:
      receivers:  [otlp]
      processors: [memory_limiter, batch]
      exporters:  [jaeger, logging]
    metrics:
      receivers:  [otlp]
      processors: [memory_limiter, batch]
      exporters:  [prometheus]
```

---

## Instrumenting sigma-bus IPC Calls

Every sigma-bus message automatically emits a span via the `opentelemetry` Rust SDK:

```rust
// kernel/sigma_bus/src/ipc.rs (instrumented)

use opentelemetry::{global, trace::{Tracer, TraceContextExt}};
use opentelemetry_otlp::WithExportConfig;

pub fn send_message(dest: ShardId, msg: BusMessage) -> Result<(), BusError> {
    let tracer = global::tracer("sigma-bus");
    let span = tracer
        .span_builder(format!("bus.send/{}", dest))
        .with_kind(opentelemetry::trace::SpanKind::Producer)
        .start(&tracer);
    let cx = opentelemetry::Context::current_with_span(span);

    // Inject trace context into message header
    let propagator = opentelemetry::global::text_map_propagator();
    let mut carrier = BusMessageCarrier(&mut msg.headers);
    propagator.inject_context(&cx, &mut carrier);

    // Actual send
    let result = do_send(dest, msg);
    cx.span().set_status(match &result {
        Ok(_)  => opentelemetry::trace::Status::Ok,
        Err(e) => opentelemetry::trace::Status::error(e.to_string()),
    });
    result
}
```

---

## sigma-monitor Metrics Export

```rust
// userland/sigma_monitor/src/metrics.rs

use opentelemetry::metrics::{Counter, Histogram, MeterProvider};

pub struct SigmaMetrics {
    pub ipc_calls:   Counter<u64>,
    pub ipc_latency: Histogram<f64>,
    pub shard_count: opentelemetry::metrics::ObservableGauge<u64>,
}

impl SigmaMetrics {
    pub fn new(meter: opentelemetry::metrics::Meter) -> Self {
        Self {
            ipc_calls:   meter.u64_counter("sigma.bus.ipc_calls").init(),
            ipc_latency: meter.f64_histogram("sigma.bus.ipc_latency_ms").init(),
            shard_count: meter.u64_observable_gauge("sigma.shards.active").init(),
        }
    }
}
```

---

## Grafana Dashboard (YAML Snippet)

```yaml
# grafana/dashboards/sigmaos-overview.yaml
apiVersion: 1
providers:
  - name: SigmaOS
    orgId: 1
    type: file
    options:
      path: /var/lib/grafana/dashboards

# Dashboard panels (abbreviated):
panels:
  - title: "IPC Calls/sec"
    type: graph
    datasource: Prometheus
    targets:
      - expr: rate(sigma_bus_ipc_calls_total[1m])
  - title: "IPC Latency p99 (ms)"
    type: graph
    targets:
      - expr: histogram_quantile(0.99, sigma_bus_ipc_latency_ms_bucket)
  - title: "Active Shards"
    type: stat
    targets:
      - expr: sigma_shards_active
```

---

## Exit Criteria

- Traces from sigma-bus IPC calls are visible in Jaeger UI at `http://localhost:16686`.
- `sigma_bus_ipc_calls_total` metric is scraped by Prometheus at `:8888/metrics`.
- sigma-otel-collector shard restarts automatically on crash (sigma-bus supervisor policy).
