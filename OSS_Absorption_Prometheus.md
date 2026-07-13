# OSS Absorption: Prometheus — Time-Series Metrics

> **Status**: 📋 Planned | **Source Project**: Prometheus | **Target Shard**: `SigmaOS Telemetry & Observability`

---

## 1. Executive Summary

Prometheus is the leading open-source systems monitoring and alerting toolkit. It records real-time metrics in a time-series database (TSDB) and allows high-dimensional querying via PromQL.

SigmaOS absorbs the **Prometheus Metric Exposition Format** and **Time-Series Storage** natively into its kernel observability layer, ensuring zero-overhead metric collection for all running processes.

---

## 2. Key Features Absorbed

### 2.1 Native Metrics Endpoint

In traditional Linux, installing node_exporter is required to get system metrics. In SigmaOS, the kernel natively exposes a `/sigma/metrics` endpoint over IPC or HTTP that perfectly matches the Prometheus exposition format.

```bash
$ curl http://localhost:9090/sigma/metrics
# HELP sigma_cpu_seconds_total Total CPU time consumed
# TYPE sigma_cpu_seconds_total counter
sigma_cpu_seconds_total{cpu="0",mode="user"} 452.1
sigma_cpu_seconds_total{cpu="0",mode="kernel"} 12.5
# HELP sigma_memory_bytes Memory usage
sigma_memory_bytes{type="active"} 4096000
```

### 2.2 Built-in Alerting

Rather than requiring an external Alertmanager daemon, the `sigma-ai-daemon` directly subscribes to internal kernel metrics. Using embedded PromQL-like rules, it triggers remediation actions locally.

```toml
# /etc/sigma/alerts.toml
[[alert]]
name = "HighMemoryUsage"
condition = "sigma_memory_bytes{type='active'} > 85%"
duration = "5m"
action = "reboot_unessential_services"
```

---

## 3. References & Standards

- Prometheus — `prometheus.io` (Apache-2.0)
