# SigmaOS System Observability & Self-Healing

## Overview

SigmaOS incorporates a local observability stack designed for low-overhead telemetry gathering and autonomous self-healing. System metrics are exposed via native endpoints compatible with Prometheus and Grafana, while the anomaly detection daemon monitors kernel telemetry to trigger automated snapshot rollbacks on critical failures.

### Key Features

- **Low-Overhead Telemetry**: Minimal performance impact from monitoring
- **Prometheus Compatible**: Native Prometheus metrics exporter
- **Grafana Dashboards**: Pre-configured visualization dashboards
- **Anomaly Detection**: ML-based anomaly detection for system health
- **Self-Healing**: Automated recovery from failures
- **Snapshot Rollback**: Automatic rollback on critical failures
- **Distributed Tracing**: OpenTelemetry integration for distributed tracing

## Architecture

### Observability & Self-Healing Flow

```
 [System/Kernel Metrics] ──► [sigmad-monitor (Prometheus exporter)]
                                       │
                                       ▼
 [Autonomous Self-Healer] ◄────────────┤
         │                             ▼
         ▼                      [Grafana Dashboard]
 [Anomaly Detected?]
         │
         └──► Yes ──► Terminate Process / Rollback Snapshot
```

### Component Architecture

```
┌─────────────────────────────────────────┐
│         Observability Stack             │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Metrics  │ Tracing  │ Logging      │ │
│  │ Collector│ Collector│ Aggregator   │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Self-Healing Engine                │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Anomaly  │ Recovery │ Rollback     │ │
│  │ Detector │ Manager  │ Manager      │ │
│  └──────────┴──────────┴──────────────┘ │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│      Visualization Layer               │
│  ┌──────────┬──────────┬──────────────┐ │
│  │ Grafana  │ Alerts   │ Reports      │ │
│  │ Dashboards│ Engine   │ Generator    │ │
│  └──────────┴──────────┴──────────────┘ │
└─────────────────────────────────────────┘
```

## Configuration

### Observability Configuration

**File**: `/etc/sigma/observability.conf`

```toml
[observability]
enabled = true
export_interval = "15s"
prometheus_port = 9100
metrics_retention = "30d"
log_retention = "7d"

[metrics]
cpu = true
memory = true
disk = true
network = true
process = true
kernel = true

[tracing]
enabled = true
sample_rate = 0.1
exporter = "otlp"
endpoint = "http://localhost:4317"

[self_healing]
enabled = true
memory_critical_threshold_percent = 95
cpu_critical_threshold_percent = 90
auto_rollback_on_panic = true
max_rollback_attempts = 3
```

### Prometheus Configuration

**File**: `/etc/sigma/prometheus.yml`

```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'sigmaos'
    static_configs:
      - targets: ['localhost:9100']
  
  - job_name: 'node_exporter'
    static_configs:
      - targets: ['localhost:9101']

alerting:
  alertmanagers:
    - static_configs:
        - targets:
          - localhost:9093
```

## Technical Implementation

### Metrics Collector

```rust
// userland/system_api/observability/src/metrics_collector.rs
use prometheus::{Counter, Gauge, Histogram, Registry};

pub struct MetricsCollector {
    registry: Registry,
    cpu_usage: Gauge,
    memory_usage: Gauge,
    disk_usage: Gauge,
    network_bytes: Counter,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let registry = Registry::new();
        
        let cpu_usage = Gauge::new("sigmaos_cpu_usage_percent", "CPU usage percentage").unwrap();
        let memory_usage = Gauge::new("sigmaos_memory_usage_percent", "Memory usage percentage").unwrap();
        let disk_usage = Gauge::new("sigmaos_disk_usage_percent", "Disk usage percentage").unwrap();
        let network_bytes = Counter::new("sigmaos_network_bytes_total", "Total network bytes").unwrap();
        
        registry.register(Box::new(cpu_usage.clone())).unwrap();
        registry.register(Box::new(memory_usage.clone())).unwrap();
        registry.register(Box::new(disk_usage.clone())).unwrap();
        registry.register(Box::new(network_bytes.clone())).unwrap();
        
        Self {
            registry,
            cpu_usage,
            memory_usage,
            disk_usage,
            network_bytes,
        }
    }
    
    pub fn collect_metrics(&self) -> Result<(), MetricsError> {
        // Collect CPU usage
        let cpu = self.get_cpu_usage()?;
        self.cpu_usage.set(cpu);
        
        // Collect memory usage
        let memory = self.get_memory_usage()?;
        self.memory_usage.set(memory);
        
        // Collect disk usage
        let disk = self.get_disk_usage()?;
        self.disk_usage.set(disk);
        
        Ok(())
    }
    
    fn get_cpu_usage(&self) -> Result<f64, MetricsError> {
        // Read from /proc/stat
        let stat = std::fs::read_to_string("/proc/stat")?;
        let parts: Vec<&str> = stat.split_whitespace().collect();
        
        let user: u64 = parts[1].parse()?;
        let nice: u64 = parts[2].parse()?;
        let system: u64 = parts[3].parse()?;
        let idle: u64 = parts[4].parse()?;
        
        let total = user + nice + system + idle;
        let usage = ((total - idle) as f64 / total as f64) * 100.0;
        
        Ok(usage)
    }
    
    fn get_memory_usage(&self) -> Result<f64, MetricsError> {
        // Read from /proc/meminfo
        let meminfo = std::fs::read_to_string("/proc/meminfo")?;
        
        let total = self.parse_meminfo_line(&meminfo, "MemTotal:")?;
        let free = self.parse_meminfo_line(&meminfo, "MemFree:")?;
        let available = self.parse_meminfo_line(&meminfo, "MemAvailable:")?;
        
        let used = total - available;
        let usage = (used as f64 / total as f64) * 100.0;
        
        Ok(usage)
    }
}
```

### Memory Statistics

```rust
// kernel/mm/sigma_vmm.rs
pub fn get_memory_utilization() -> MemoryStats {
    let total_pages = get_total_physical_pages();
    let free_pages = get_free_physical_pages();
    MemoryStats {
        total: total_pages * PAGE_SIZE,
        free: free_pages * PAGE_SIZE,
        used: (total_pages - free_pages) * PAGE_SIZE,
    }
}

pub struct MemoryStats {
    pub total: u64,
    pub free: u64,
    pub used: u64,
}
```

### Anomaly Detection

```rust
// userland/system_api/observability/src/anomaly_detector.rs
pub struct AnomalyDetector {
    model: AnomalyModel,
    threshold: f64,
}

impl AnomalyDetector {
    pub fn new(threshold: f64) -> Self {
        AnomalyDetector {
            model: AnomalyModel::new(),
            threshold,
        }
    }
    
    pub fn detect_anomaly(&self, metrics: &SystemMetrics) -> Result<AnomalyResult, DetectorError> {
        let score = self.model.compute_anomaly_score(metrics)?;
        
        if score > self.threshold {
            Ok(AnomalyResult {
                is_anomaly: true,
                score,
                severity: self.compute_severity(score),
                suggested_action: self.suggest_action(metrics),
            })
        } else {
            Ok(AnomalyResult {
                is_anomaly: false,
                score,
                severity: Severity::None,
                suggested_action: Action::None,
            })
        }
    }
    
    fn compute_severity(&self, score: f64) -> Severity {
        if score > 0.9 {
            Severity::Critical
        } else if score > 0.7 {
            Severity::High
        } else if score > 0.5 {
            Severity::Medium
        } else {
            Severity::Low
        }
    }
    
    fn suggest_action(&self, metrics: &SystemMetrics) -> Action {
        if metrics.memory_usage > 95.0 {
            Action::TerminateProcess
        } else if metrics.cpu_usage > 90.0 {
            Action::ReduceLoad
        } else if metrics.disk_usage > 95.0 {
            Action::CleanupDisk
        } else {
            Action::Monitor
        }
    }
}
```

### Self-Healing Engine

```rust
// userland/system_api/observability/src/self_healing.rs
pub struct SelfHealingEngine {
    anomaly_detector: AnomalyDetector,
    recovery_manager: RecoveryManager,
    rollback_manager: RollbackManager,
}

impl SelfHealingEngine {
    pub fn new() -> Self {
        SelfHealingEngine {
            anomaly_detector: AnomalyDetector::new(0.8),
            recovery_manager: RecoveryManager::new(),
            rollback_manager: RollbackManager::new(),
        }
    }
    
    pub async fn monitor_and_heal(&mut self) -> Result<(), HealingError> {
        loop {
            // Collect metrics
            let metrics = self.collect_metrics()?;
            
            // Detect anomalies
            let anomaly_result = self.anomaly_detector.detect_anomaly(&metrics)?;
            
            if anomaly_result.is_anomaly {
                match anomaly_result.suggested_action {
                    Action::TerminateProcess => {
                        self.recovery_manager.terminate_high_memory_process()?;
                    }
                    Action::ReduceLoad => {
                        self.recovery_manager.reduce_system_load()?;
                    }
                    Action::CleanupDisk => {
                        self.recovery_manager.cleanup_disk()?;
                    }
                    Action::Rollback => {
                        self.rollback_manager.rollback_to_snapshot()?;
                    }
                    Action::None => {}
                }
            }
            
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        }
    }
}
```

## Grafana Dashboards

### Pre-configured Dashboards

**System Overview Dashboard**:
- CPU usage (current, average, peak)
- Memory usage (current, average, peak)
- Disk usage (current, average, peak)
- Network traffic (inbound, outbound)
- Process count
- System load average

**Self-Healing Dashboard**:
- Anomaly detection events
- Recovery actions taken
- Rollback events
- System health score
- Alert status

**Application Dashboard**:
- Application-specific metrics
- Request latency
- Error rates
- Throughput
- Resource usage per application

### Dashboard Configuration

**File**: `/etc/sigma/grafana/dashboards/system-overview.json`

```json
{
  "dashboard": {
    "title": "SigmaOS System Overview",
    "panels": [
      {
        "title": "CPU Usage",
        "targets": [
          {
            "expr": "sigmaos_cpu_usage_percent"
          }
        ]
      },
      {
        "title": "Memory Usage",
        "targets": [
          {
            "expr": "sigmaos_memory_usage_percent"
          }
        ]
      }
    ]
  }
}
```

## Distributed Tracing

### OpenTelemetry Integration

```rust
// userland/system_api/observability/src/tracing.rs
use opentelemetry::trace::{Tracer, TracerProvider};
use opentelemetry::global;

pub fn init_tracing() -> Result<(), TracingError> {
    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint("http://localhost:4317")
        .build()?;
    
    let provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(exporter)
        .build()?;
    
    global::set_tracer_provider(provider);
    
    Ok(())
}

pub fn trace_function<F, R>(name: &str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let tracer = global::tracer("sigmaos");
    let span = tracer.start(name);
    let _guard = span.enter();
    f()
}
```

## Best Practices

### Development

1. **Low Overhead**: Minimize performance impact
2. **Structured Logging**: Use structured logging formats
3. **Metric Naming**: Follow Prometheus naming conventions
4. **Sampling**: Use appropriate sampling rates for tracing

### Configuration

1. **Retention Policies**: Set appropriate retention periods
2. **Thresholds**: Configure appropriate thresholds
3. **Alerting**: Set up appropriate alert rules
4. **Dashboards**: Customize dashboards for specific needs

### Security

1. **Access Control**: Restrict access to metrics
2. **Encryption**: Encrypt metrics in transit
3. **Authentication**: Use authentication for endpoints
4. **Audit Logging**: Log access to observability data

## Roadmap & Milestones

### Phase 1 (Months 0-3)
- Prometheus metrics exporter daemon
- CPU/memory utilization metrics
- Basic Grafana dashboards
- Alert configuration

### Phase 2 (Months 3-6)
- Integrated Grafana dashboard service
- Advanced metrics collection
- Distributed tracing with OpenTelemetry
- Custom alert rules

### Phase 3 (Months 6-9)
- Self-healing watchdog daemon
- Service restart on failure
- Anomaly detection engine
- Automated recovery actions

### Phase 4 (Months 9-12)
- Automated ZFS/Btrfs snapshot rollback
- ML-based anomaly detection
- Predictive failure detection
- Advanced self-healing strategies

## References

- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
- [OpenTelemetry Documentation](https://opentelemetry.io/docs/)
- [Observability Best Practices](https://sre.google/workbook/alerting-on-slos/)
- [Self-Healing Systems](https://www.usenix.org/conference/atc16/technical-sessions/presentation/ananthalakshmi)
