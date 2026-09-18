# System Monitoring and Observability

SigmaOS implements comprehensive system monitoring and observability with real-time metrics, distributed tracing, log aggregation, and alerting for production-grade system management.

## Overview

System monitoring provides:
- Real-time CPU, memory, disk, and network metrics
- Distributed tracing with OpenTelemetry
- Log aggregation with structured logging
- Alerting and notification systems
- Performance profiling and debugging
- Container and process monitoring
- Hardware health monitoring

## Architecture

### Monitoring Stack
```
Application → Metrics → Prometheus → Grafana
                ↓
            Tracing → Jaeger → UI
                ↓
            Logging → Loki → UI
                ↓
            Alerts → Alertmanager → Notifications
```

### Metrics Collection
- **CPU**: Usage, load average, context switches, interrupts
- **Memory**: Usage, swap, buffers, cache, slab
- **Disk**: I/O operations, throughput, latency, space
- **Network**: Traffic, connections, errors, latency
- **Process**: CPU, memory, file descriptors, threads
- **Container**: Resource usage, limits, requests

## Implementation

### Metrics Collector
```rust
// src/monitoring/metrics.rs
pub struct MetricsCollector {
    pub cpu_collector: CpuCollector,
    pub memory_collector: MemoryCollector,
    pub disk_collector: DiskCollector,
    pub network_collector: NetworkCollector,
    pub process_collector: ProcessCollector,
}

impl MetricsCollector {
    pub fn new() -> Self {
        MetricsCollector {
            cpu_collector: CpuCollector::new(),
            memory_collector: MemoryCollector::new(),
            disk_collector: DiskCollector::new(),
            network_collector: NetworkCollector::new(),
            process_collector: ProcessCollector::new(),
        }
    }

    pub fn collect_all(&self) -> SystemMetrics {
        SystemMetrics {
            cpu: self.cpu_collector.collect(),
            memory: self.memory_collector.collect(),
            disk: self.disk_collector.collect(),
            network: self.network_collector.collect(),
            processes: self.process_collector.collect(),
            timestamp: SystemTime::now(),
        }
    }
}
```

### CPU Collector
```rust
// src/monitoring/cpu.rs
pub struct CpuCollector {
    pub prev_idle: u64,
    pub prev_total: u64,
}

impl CpuCollector {
    pub fn new() -> Self {
        CpuCollector {
            prev_idle: 0,
            prev_total: 0,
        }
    }

    pub fn collect(&mut self) -> CpuMetrics {
        let idle = self.read_cpu_idle();
        let total = self.read_cpu_total();
        
        let idle_delta = idle.saturating_sub(self.prev_idle);
        let total_delta = total.saturating_sub(self.prev_total);
        
        let usage = if total_delta > 0 {
            100.0 - (idle_delta as f64 / total_delta as f64 * 100.0)
        } else {
            0.0
        };
        
        self.prev_idle = idle;
        self.prev_total = total;
        
        CpuMetrics {
            usage_percent: usage,
            load_average: self.read_load_average(),
            context_switches: self.read_context_switches(),
            interrupts: self.read_interrupts(),
        }
    }
}
```

### Memory Collector
```rust
// src/monitoring/memory.rs
pub struct MemoryCollector;

impl MemoryCollector {
    pub fn new() -> Self {
        MemoryCollector
    }

    pub fn collect(&self) -> MemoryMetrics {
        MemoryMetrics {
            total: self.read_total_memory(),
            used: self.read_used_memory(),
            free: self.read_free_memory(),
            buffers: self.read_buffers(),
            cached: self.read_cached(),
            swap_total: self.read_swap_total(),
            swap_used: self.read_swap_used(),
            slab: self.read_slab(),
        }
    }
}
```

### Distributed Tracing
```rust
// src/monitoring/tracing.rs
pub struct TracingEngine {
    pub span_exporter: SpanExporter,
    pub trace_id_generator: TraceIdGenerator,
}

impl TracingEngine {
    pub fn new() -> Self {
        TracingEngine {
            span_exporter: SpanExporter::new(),
            trace_id_generator: TraceIdGenerator::new(),
        }
    }

    pub fn start_span(&self, name: &str) -> Span {
        let trace_id = self.trace_id_generator.generate();
        let span_id = self.generate_span_id();
        
        Span {
            trace_id,
            span_id,
            parent_id: None,
            name: name.to_string(),
            start_time: SystemTime::now(),
            end_time: None,
            attributes: BTreeMap::new(),
            events: Vec::new(),
        }
    }

    pub fn end_span(&self, span: &mut Span) {
        span.end_time = Some(SystemTime::now());
        self.span_exporter.export(span);
    }
}
```

### Log Aggregation
```rust
// src/monitoring/logging.rs
pub struct LogAggregator {
    pub log_buffer: RingBuffer<LogEntry>,
    pub log_index: BTreeMap<String, Vec<usize>>,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: SystemTime,
    pub level: LogLevel,
    pub component: String,
    pub message: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogAggregator {
    pub fn new(capacity: usize) -> Self {
        LogAggregator {
            log_buffer: RingBuffer::new(capacity),
            log_index: BTreeMap::new(),
        }
    }

    pub fn log(&mut self, entry: LogEntry) {
        let index = self.log_buffer.push(entry.clone());
        
        self.log_index
            .entry(entry.component.clone())
            .or_insert_with(Vec::new)
            .push(index);
    }

    pub fn query(&self, component: &str, level: LogLevel) -> Vec<LogEntry> {
        let indices = self.log_index.get(component).unwrap_or(&vec![]);
        
        indices
            .iter()
            .filter_map(|&i| self.log_buffer.get(i))
            .filter(|entry| entry.level >= level)
            .cloned()
            .collect()
    }
}
```

### Alerting System
```rust
// src/monitoring/alerting.rs
pub struct AlertManager {
    pub alert_rules: Vec<AlertRule>,
    pub notification_channels: Vec<NotificationChannel>,
    pub alert_history: RingBuffer<Alert>,
}

#[derive(Debug, Clone)]
pub struct AlertRule {
    pub name: String,
    pub condition: AlertCondition,
    pub severity: AlertSeverity,
    pub cooldown: Duration,
}

#[derive(Debug, Clone)]
pub enum AlertCondition {
    CpuUsageAbove(f64),
    MemoryUsageAbove(f64),
    DiskUsageAbove(f64),
    NetworkErrorRateAbove(f64),
    ProcessCrashed(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

impl AlertManager {
    pub fn new() -> Self {
        AlertManager {
            alert_rules: Vec::new(),
            notification_channels: Vec::new(),
            alert_history: RingBuffer::new(1000),
        }
    }

    pub fn evaluate(&mut self, metrics: &SystemMetrics) -> Vec<Alert> {
        let mut alerts = Vec::new();
        
        for rule in &self.alert_rules {
            if self.check_rule(rule, metrics) {
                let alert = Alert {
                    rule_name: rule.name.clone(),
                    severity: rule.severity,
                    timestamp: SystemTime::now(),
                    metrics: metrics.clone(),
                };
                
                alerts.push(alert.clone());
                self.alert_history.push(alert);
            }
        }
        
        alerts
    }

    fn check_rule(&self, rule: &AlertRule, metrics: &SystemMetrics) -> bool {
        match &rule.condition {
            AlertCondition::CpuUsageAbove(threshold) => metrics.cpu.usage_percent > *threshold,
            AlertCondition::MemoryUsageAbove(threshold) => {
                let usage = metrics.memory.used as f64 / metrics.memory.total as f64 * 100.0;
                usage > *threshold
            }
            AlertCondition::DiskUsageAbove(threshold) => {
                // Check disk usage
                false
            }
            AlertCondition::NetworkErrorRateAbove(threshold) => {
                // Check network error rate
                false
            }
            AlertCondition::ProcessCrashed(process_name) => {
                // Check if process crashed
                false
            }
        }
    }
}
```

## Configuration

### Monitoring Configuration
```toml
# /etc/sigmaos/monitoring.toml
[metrics]
# Metrics collection settings
enabled = true
interval_seconds = 15
retention_days = 30

[tracing]
# Distributed tracing settings
enabled = true
sample_rate = 0.1
exporter = "jaeger"

[logging]
# Logging settings
level = "info"
format = "json"
retention_days = 90

[alerting]
# Alerting settings
enabled = true
cooldown_minutes = 5

[alerting.rules.cpu]
name = "high_cpu_usage"
condition = "cpu_usage_above"
threshold = 90.0
severity = "warning"

[alerting.rules.memory]
name = "high_memory_usage"
condition = "memory_usage_above"
threshold = 85.0
severity = "critical"
```

### Runtime Control
```bash
# Start monitoring
sigmon start

# View metrics
sigmon metrics

# View logs
sigmon logs --component kernel

# View traces
sigmon traces --trace-id 123456

# View alerts
sigmon alerts

# Set log level
sigmon set-log-level debug

# Enable tracing
sigmon enable-tracing
```

## Performance Optimization

### Metrics Collection
Optimize metrics collection for performance:
```bash
# Increase collection interval
sigmon set-interval 30

# Disable expensive metrics
sigmon disable-metrics detailed_process

# Enable sampling
sigmon set-sample-rate 0.05
```

### Log Aggregation
Optimize log aggregation:
```bash
# Set log level to reduce volume
sigmon set-log-level warn

# Enable log compression
sigmon enable-compression

# Set retention period
sigmon set-retention 30
```

### Tracing
Optimize tracing:
```bash
# Adjust sample rate
sigmon set-sample-rate 0.01

# Disable tracing for production
sigmon disable-tracing

# Enable head-based sampling
sigmon enable-head-sampling
```

## Troubleshooting

### High CPU Usage
If monitoring daemon has high CPU usage:
1. Check collection interval: `sigmon get-interval`
2. Increase interval: `sigmon set-interval 30`
3. Disable expensive metrics: `sigmon disable-metrics detailed`
4. Enable sampling: `sigmon set-sample-rate 0.05`

### High Memory Usage
If monitoring daemon has high memory usage:
1. Check retention period: `sigmon get-retention`
2. Reduce retention: `sigmon set-retention 7`
3. Check buffer size: `sigmon get-buffer-size`
4. Reduce buffer size: `sigmon set-buffer-size 1000`

### Missing Metrics
If metrics are missing:
1. Check collector status: `sigmon status`
2. Restart collector: `sigmon restart`
3. Check permissions
4. Check kernel metrics availability

### Alert Flood
If alerts are flooding:
1. Check cooldown: `sigmon get-cooldown`
2. Increase cooldown: `sigmon set-cooldown 10`
3. Adjust thresholds: `sigmon set-threshold cpu 95`
4. Disable non-critical alerts: `sigmon disable-alert high_cpu_usage`

---

**[System Administration](Category-System-Administration)** | **[Monitoring](Category-Monitoring)** | **[Observability](Category-Observability)**
