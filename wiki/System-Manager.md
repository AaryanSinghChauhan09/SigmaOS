# System Manager

## Overview

The System Manager provides comprehensive system management inspired by Omarchy's system tools. It includes system information tracking, resource monitoring, health status management, and system summary generation.

## Features

- **System Information**: Hostname, kernel version, OS version, architecture, uptime tracking
- **CPU Monitoring**: Usage percentage, temperature, core/thread counts
- **Memory Monitoring**: Total, used, available, cached memory with human-readable formatting
- **Disk Monitoring**: Per-mount-point usage tracking with filesystem type
- **Network Monitoring**: Interface statistics, IP/MAC addresses, traffic tracking
- **Health Status**: Automatic health assessment based on resource usage
- **System Summary**: Consolidated system overview

## Components

### SystemHealth

```rust
pub enum SystemHealth {
    Healthy,   // System operating normally
    Warning,   // Elevated resource usage
    Critical,  // Critical resource levels
}
```

Health is automatically calculated based on CPU and memory load averages.

### SystemInfo

System metadata including:
- Hostname
- Kernel version
- OS version
- Architecture
- Uptime (seconds and human-readable)

### CpuInfo

CPU metrics including:
- CPU model
- Core and thread counts
- Frequency (MHz)
- Usage percentage
- Temperature (Celsius)

### MemoryInfo

Memory statistics including:
- Total, available, used bytes
- Cached memory
- Swap total/used
- Human-readable formatting

### DiskInfo

Per-disk information including:
- Mount point and device
- Filesystem type
- Total, used, available bytes
- Usage percentage

### NetworkInfo

Network interface details including:
- Interface name
- IP and MAC addresses
- Sent/received bytes
- Link status

### SystemManager

Main management interface with:
- System information tracking
- Resource monitoring
- Health status updates
- Disk and network management
- Summary generation

## Usage

### Basic Usage

```rust
use sigmaos::system::SystemManager;

let mut manager = SystemManager::new();

// Get system information
let info = manager.get_system_info();

// Get CPU information
let cpu = manager.get_cpu_info();

// Get system health
let health = manager.get_health();

// Get system summary
let summary = manager.get_summary();
```

### Resource Monitoring

```rust
// Update CPU usage
manager.update_cpu_usage(75.0);

// Update memory usage
manager.update_memory_usage(4 * 1024 * 1024 * 1024);

// Update disk usage
manager.update_disk_usage("/", 250 * 1024 * 1024 * 1024);
```

### Disk and Network Management

```rust
// Add additional disk
let disk = DiskInfo::new("/home".to_string());
manager.add_disk(disk);

// Add network interface
let network = NetworkInfo::new("wlan0".to_string());
manager.add_network(network);

// Get all disks
let disks = manager.get_disks();

// Get all networks
let networks = manager.get_networks();
```

## AI Agent Maintenance Instructions

When maintaining the System Manager:

1. **Resource Accuracy**: Ensure CPU, memory, disk, and network metrics are accurately updated
2. **Health Calculation**: Maintain proper health assessment logic based on resource thresholds
3. **Human Formatting**: Keep human-readable formatting functions accurate and consistent
4. **Device Management**: Ensure proper addition and tracking of multiple disks and network interfaces
5. **Timestamp Tracking**: Maintain accurate uptime and boot time tracking

## Testing

Run the unit tests with:

```bash
cargo test --lib system::system_manager
```

## Future Enhancements

- Integration with actual system APIs (/proc, sysfs)
- Historical resource tracking and graphing
- Alert system for critical health states
- Process-specific resource usage
- Temperature sensor integration
- Fan speed monitoring
