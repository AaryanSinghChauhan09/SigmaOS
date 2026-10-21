# SigmaAlert

System Notification and Alerting Engine for SigmaOS. Inspired by modern distributed alerting systems (Prometheus Alertmanager, syslog priority levels, desktop notification daemons), SigmaAlert provides centralized alert ingestion, severity classification, notification routing, acknowledgment, and lifecycle tracking.

## Overview

In mission-critical and cloud deployments, kernel events and service failures must be promptly logged, categorized, and presented to users or automated controllers. SigmaAlert serves as the central notification clearinghouse for all SigmaOS components, decoupling event producers (VFS, memory manager, firewall, health monitors) from notification consumers (system consoles, desktop notification hubs, audit logs).

## Features

- **Standard Severity Taxonomy**: Info, Notice, Warning, Error, Critical, and Emergency levels
- **Domain-Specific Categorization**: System, Storage, Security, Network, Hardware, and Service categories
- **Multi-Channel Dispatch**: Configurable channel delivery to Console, SystemLog, DesktopBus, and NetworkTrap
- **Alert Lifecycle Management**: Active, Acknowledged, Silenced, and Resolved state transitions with audit timestamps
- **Targeted Queries & Filtering**: Fast querying by category, minimum severity, or active status
- **Global Kernel Singleton**: Zero-allocation access patterns with safe concurrency suitable for kernel and daemon integration

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                     SigmaAlert Engine                           │
├───────────────────────────────┬─────────────────────────────────┤
│        Ingestion & Registry   │       Routing & Subscriptions   │
├───────────────────────────────┼─────────────────────────────────┤
│  - Alert ID generator         │  - SystemLog channel            │
│  - Severity & Category tags   │  - Console broadcast            │
│  - Status lifecycle tracking  │  - DesktopBus notifications     │
│  - Acknowledgment records     │  - NetworkTrap SNMP/webhooks    │
└───────────────────────────────┴─────────────────────────────────┘
```

## Data Structures

### AlertSeverity & AlertCategory

```rust
pub enum AlertSeverity {
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Emergency,
}

pub enum AlertCategory {
    System,
    Storage,
    Security,
    Network,
    Hardware,
    Service,
}
```

### AlertStatus & AlertChannel

```rust
pub enum AlertStatus {
    Active,
    Acknowledged,
    Silenced,
    Resolved,
}

pub enum AlertChannel {
    SystemLog,
    Console,
    DesktopBus,
    NetworkTrap,
}
```

### Alert

```rust
pub struct Alert {
    pub id: u64,
    pub title: String,
    pub message: String,
    pub source: String,
    pub severity: AlertSeverity,
    pub category: AlertCategory,
    pub status: AlertStatus,
    pub timestamp: u64,
    pub acknowledged_by: Option<String>,
    pub resolved_timestamp: Option<u64>,
}
```

## Public API & Manager

### `SigmaAlert`

```rust
pub struct SigmaAlert { ... }

impl SigmaAlert {
    pub fn new() -> Self;
    pub fn start(&mut self);
    pub fn stop(&mut self);
    pub fn is_running(&self) -> bool;

    pub fn subscribe(&mut self, category: AlertCategory, channel: AlertChannel);
    pub fn raise_alert(&mut self, title: &str, message: &str, source: &str, severity: AlertSeverity, category: AlertCategory, timestamp: u64) -> u64;
    pub fn acknowledge(&mut self, alert_id: u64, user: &str) -> bool;
    pub fn silence(&mut self, alert_id: u64) -> bool;
    pub fn resolve(&mut self, alert_id: u64, timestamp: u64) -> bool;

    pub fn get_alert(&self, alert_id: u64) -> Option<&Alert>;
    pub fn list_active(&self) -> Vec<&Alert>;
    pub fn list_by_min_severity(&self, min_sev: AlertSeverity) -> Vec<&Alert>;
    pub fn list_by_category(&self, cat: AlertCategory) -> Vec<&Alert>;
    pub fn active_count(&self) -> usize;
    pub fn total_count(&self) -> usize;
}
```

### Global Singleton & Convenience Functions

```rust
pub fn init_sigmaalert();
pub fn get_sigmaalert() -> Option<&'static SigmaAlert>;
pub fn get_sigmaalert_mut() -> Option<&'static mut SigmaAlert>;
pub fn send_alert(title: &str, message: &str, source: &str, severity: AlertSeverity, category: AlertCategory) -> u64;
pub fn resolve_alert(alert_id: u64) -> bool;
pub fn active_alert_count() -> usize;
```

## Usage Examples

```rust
use sigmaos::system::{
    init_sigmaalert, send_alert, resolve_alert, active_alert_count,
    AlertSeverity, AlertCategory,
};

init_sigmaalert();

// Raise an alert
let id = send_alert(
    "Disk Space Low",
    "/var partition usage reached 92%",
    "sigmastorage",
    AlertSeverity::Warning,
    AlertCategory::Storage,
);

assert_eq!(active_alert_count(), 1);

// Resolve alert when freed
resolve_alert(id);
assert_eq!(active_alert_count(), 0);
```

## Testing

SigmaAlert includes unit test coverage:
- `test_alert_lifecycle`: Alert creation, acknowledgment tracking, and resolution
- `test_alert_filtering`: Filtering alerts by minimum severity and category
- `test_global_singleton`: Singleton accessor and convenience functions
