# SigmaHealth

System Health Monitoring Engine for SigmaOS. Inspired by hardware monitoring daemons, system diagnostics, and proactive reliability monitors, SigmaHealth provides continuous probe checks, degradation detection, subsystem scoring, and diagnostic reporting across CPU, Memory, Storage, Network, Kernel, and Security domains.

## Overview

High availability requires continuous evaluation of system health before failures occur. SigmaHealth implements a multi-probe health model that regularly samples CPU load, memory pressure, storage utilization, network gateway availability, kernel oops counters, and security integrity modules. Probes aggregate into per-subsystem states and a unified 0-100 system health score.

## Features

- **Multi-Subsystem Health Probes**: Automated probes monitoring CPU, Memory, Storage, Network, Kernel, and Security subsystems
- **Tiered Health Status**: Clear four-state health classification (`Healthy`, `Degraded`, `Critical`, `Unknown`)
- **Normalized Health Score (0 - 100)**: Quantifiable metric representing the overall operating condition of the system
- **Extensible Probe Architecture**: Register custom hardware or application-level probes
- **Structured Diagnostic Reporting**: Instant generation of complete system health snapshots for logs, support reports, and dashboards
- **Global Kernel Singleton**: Centralized engine accessible throughout kernel and userspace daemons

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     SigmaHealth Monitor                         │
├───────────────────────────────┬─────────────────────────────────┤
│        Probes Registry        │       Diagnostics & Reporting   │
├───────────────────────────────┼─────────────────────────────────┤
│  - cpu_load (CPU)             │  - Subsystem State Map          │
│  - memory_pressure (Memory)   │  - Overall Health Score (0-100) │
│  - root_disk_space (Storage)  │  - Active Issues Counter        │
│  - network_gateway (Network)  │  - Timestamped Health Reports   │
│  - kernel_oops (Kernel)       │                                 │
│  - tpm_security (Security)    │                                 │
└───────────────────────────────┴─────────────────────────────────┘
```

## Data Structures

### Subsystem & HealthState

```rust
pub enum Subsystem {
    Cpu,
    Memory,
    Storage,
    Network,
    Kernel,
    Security,
}

pub enum HealthState {
    Healthy,
    Degraded,
    Critical,
    Unknown,
}
```

### HealthProbe

```rust
pub struct HealthProbe {
    pub name: String,
    pub subsystem: Subsystem,
    pub state: HealthState,
    pub message: String,
    pub score: u8, // 0 - 100
    pub last_check_timestamp: u64,
}
```

Key methods:
- `new(name: &str, subsystem: Subsystem) -> Self`
- `update(&mut self, state: HealthState, score: u8, message: &str, timestamp: u64)`

### SystemHealthReport

```rust
pub struct SystemHealthReport {
    pub overall_state: HealthState,
    pub overall_score: u8,
    pub subsystem_states: BTreeMap<Subsystem, HealthState>,
    pub active_issues_count: usize,
    pub timestamp: u64,
}
```

## Public API & Manager

### `SigmaHealth`

```rust
pub struct SigmaHealth { ... }

impl SigmaHealth {
    pub fn new() -> Self;
    pub fn start(&mut self);
    pub fn stop(&mut self);
    pub fn is_running(&self) -> bool;

    pub fn register_probe(&mut self, probe: HealthProbe);
    pub fn update_probe(&mut self, name: &str, state: HealthState, score: u8, message: &str, timestamp: u64) -> bool;
    pub fn overall_state(&self) -> HealthState;
    pub fn overall_score(&self) -> u8;
    pub fn subsystem_state(&self, sub: Subsystem) -> HealthState;
    pub fn generate_report(&self, timestamp: u64) -> SystemHealthReport;
    pub fn probe_count(&self) -> usize;
}
```

### Global Singleton & Convenience Functions

```rust
pub fn init_sigmahealth();
pub fn get_sigmahealth() -> Option<&'static SigmaHealth>;
pub fn get_sigmahealth_mut() -> Option<&'static mut SigmaHealth>;
pub fn system_health_state() -> HealthState;
pub fn system_health_score() -> u8;
pub fn generate_health_report(timestamp: u64) -> Option<SystemHealthReport>;
```

## Usage Examples

```rust
use sigmaos::system::{
    init_sigmahealth, system_health_state, system_health_score,
    generate_health_report, HealthState,
};

init_sigmahealth();

// Check overall health
assert_eq!(system_health_state(), HealthState::Healthy);
assert_eq!(system_health_score(), 100);

// Generate diagnostic snapshot
let report = generate_health_report(1000).unwrap();
assert_eq!(report.active_issues_count, 0);
```

## Testing

SigmaHealth includes unit test coverage:
- `test_health_monitor_lifecycle`: Default probe setup, state degradation, critical escalation, and report generation
- `test_global_singleton`: Singleton accessor and convenience functions
