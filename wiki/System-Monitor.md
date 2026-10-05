# System Monitor

## Overview

The System Monitor provides comprehensive system monitoring inspired by Linux Mint's system monitor and Omarchy's system utilities. It supports CPU, memory, disk, and network monitoring with real-time statistics.

## Features

- **CPU Monitoring**: Per-core usage and frequency tracking
- **Memory Monitoring**: RAM and swap usage with percentage calculation
- **Disk Monitoring**: Partition usage with mount points and file systems
- **Network Monitoring**: Interface status and traffic statistics
- **Real-time Updates**: Simulated real-time value updates
- **Statistics**: Comprehensive system statistics aggregation
- **Default Configuration**: Pre-configured CPU cores, memory, partitions, and network interfaces

## Components

### CpuCore

Represents a CPU core with:
- Core ID
- Usage percentage
- Frequency (MHz)

### RamMemoryInfo

Represents memory information with:
- Total memory (MB)
- Used memory (MB)
- Free memory (MB)
- Available memory (MB)
- Total swap (MB)
- Used swap (MB)

### DiskPartition

Represents a disk partition with:
- Device path
- Mount point
- File system type
- Total size (GB)
- Used size (GB)
- Free size (GB)

### NetworkInterface

Represents a network interface with:
- Interface name
- Bytes sent
- Bytes received
- Packets sent
- Packets received
- Up status

### SystemMonitor

Main management interface with:
- CPU core management
- Memory information tracking
- Partition management
- Network interface management
- Total disk usage calculation
- Total network traffic calculation
- Real-time updates
- Statistics aggregation

## Usage

### Basic Usage

```rust
use sigmaos::system::SystemMonitor;

let mut monitor = SystemMonitor::new();

// Get CPU cores
let cores = monitor.get_cpu_cores();
for core in cores {
    println!("Core {}: {}% @ {} MHz", core.id, core.usage_percent, core.frequency_mhz);
}
```

### CPU Monitoring

```rust
// Get CPU cores
let cores = monitor.get_cpu_cores();

// Get average CPU usage
let cpu_usage = monitor.get_cpu_usage();
println!("CPU Usage: {}%", cpu_usage);
```

### Memory Monitoring

```rust
// Get memory information
let memory = monitor.get_memory();
println!("Total: {} MB", memory.total_mb);
println!("Used: {} MB", memory.used_mb);
println!("Free: {} MB", memory.free_mb);
println!("Usage: {}%", memory.usage_percent());
```

### Disk Monitoring

```rust
// Add a partition
let mut partition = DiskPartition::new(
    "/dev/sda2".to_string(),
    "/home".to_string(),
    "ext4".to_string(),
);
partition.total_gb = 200;
partition.used_gb = 100;
partition.free_gb = 100;
monitor.add_partition(partition);

// List all partitions
let partitions = monitor.list_partitions();
for part in partitions {
    println!("{}: {} ({}% used)", part.mount_point, part.device, part.usage_percent());
}

// Get total disk usage
let (total, used) = monitor.get_total_disk_usage();
println!("Disk: {} GB / {} GB", used, total);
```

### Network Monitoring

```rust
// Add a network interface
let mut iface = NetworkInterface::new("eth1".to_string());
iface.set_up(true);
monitor.add_network_interface(iface);

// List all network interfaces
let interfaces = monitor.list_network_interfaces();
for iface in interfaces {
    println!("{}: {} (sent: {} bytes, recv: {} bytes)",
        iface.name,
        if iface.is_up { "UP" } else { "DOWN" },
        iface.bytes_sent,
        iface.bytes_recv
    );
}

// Get total network traffic
let (sent, recv) = monitor.get_total_network_traffic();
println!("Network: sent {} bytes, recv {} bytes", sent, recv);
```

### Real-time Updates

```rust
// Update simulated values
monitor.update();

// Get updated statistics
let stats = monitor.get_statistics();
println!("CPU: {}%", stats.cpu_usage);
println!("Memory: {}%", stats.memory_usage);
println!("Disk: {}%", stats.disk_usage);
```

### Statistics

```rust
let stats = monitor.get_statistics();
println!("CPU Cores: {}", stats.cpu_cores);
println!("CPU Usage: {}%", stats.cpu_usage);
println!("Memory Usage: {}%", stats.memory_usage);
println!("Disk Partitions: {}", stats.disk_partitions);
println!("Disk Usage: {}%", stats.disk_usage);
println!("Network Interfaces: {}", stats.network_interfaces);
println!("Network Up: {}", stats.network_up);
```

## Default Configuration

The System Monitor includes pre-configured hardware:

**CPU:**
- 4 cores at 2.4 GHz

**Memory:**
- 8 GB total (4 GB used, 4 GB free)

**Partitions:**
- /dev/sda1 mounted at / (ext4, 100 GB total, 50 GB used)

**Network Interfaces:**
- eth0 (UP)
- wlan0 (UP)

## AI Agent Maintenance Instructions

When maintaining the System Monitor:

1. **Value Ranges**: Ensure usage percentages are between 0-100
2. **Memory Consistency**: Ensure used + free ≤ total
3. **Disk Consistency**: Ensure used + free ≤ total
4. **Interface Validation**: Validate interface names before adding
5. **Update Frequency**: Update values at appropriate intervals
6. **Measurement Units**: Maintain consistent units (MB for memory, GB for disk)

## Testing

Run the unit tests with:

```bash
cargo test --lib system::system_monitor
```

## Future Enhancements

- Integration with actual system monitoring (procfs, sysfs)
- Real-time CPU frequency scaling
- Temperature monitoring
- Process monitoring
- I/O statistics
- Network speed calculation
- Historical data storage
- Graph visualization
- Alert thresholds
- Custom update intervals
- Per-core thermal throttling detection
