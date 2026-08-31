# SigmaOS Kernel Customization Guide

## Overview

This guide provides comprehensive instructions for customizing the SigmaOS kernel to meet specific requirements, from embedded systems to high-performance computing environments.

## Table of Contents

1. [Kernel Configuration](#kernel-configuration)
2. [Build Profiles](#build-profiles)
3. [Memory Management Customization](#memory-management-customization)
4. [Scheduler Tuning](#scheduler-tuning)
5. [Device Driver Integration](#device-driver-integration)
6. [Security Hardening](#security-hardening)
7. [Performance Optimization](#performance-optimization)

## Kernel Configuration

### Configuration File Structure

SigmaOS uses a declarative configuration system located in `sigma-core.toml`:

```toml
[profile]
name = "custom"
target = "x86_64-unknown-none"

[memory]
# Memory pool configuration
paged_pool_size = "256MB"
non_paged_pool_size = "64MB"
enable_thp = true

[scheduler]
# Scheduler configuration
algorithm = "mlfq+cfs+edf"
time_slice_ms = 4
priority_levels = 8

[security]
# Security hardening options
enable_kaslr = true
enable_cfi = true
enable_stack_canaries = true
pqc_enabled = true
```

### Build-Time Configuration

Kernel features can be enabled/disabled via feature flags:

```bash
# Minimal kernel for embedded systems
cargo build --no-default-features --features "minimal"

# Full-featured desktop kernel
cargo build --features "desktop,audio,graphics,networking"

# Real-time kernel
cargo build --features "rtos,preempt_rt"
```

## Build Profiles

### Available Profiles

1. **Standalone**: Full desktop environment
2. **RTOS**: Hard real-time capabilities
3. **Cloud**: Headless cloud deployment
4. **Browser**: WASM bundle for web deployment
5. **Embedded**: Minimal footprint for embedded systems

### Custom Profile Creation

Create a new profile in `sigma-build/profiles/`:

```toml
# sigma-build/profiles/custom.toml
[build]
target = "aarch64-unknown-none"
optimization_level = "3"
lto = true

[features]
include = ["memory_management", "scheduler", "basic_io"]
exclude = ["graphics", "audio"]

[memory]
heap_size = "32MB"
stack_size = "8MB"
```

Build with custom profile:
```bash
make PROFILE=custom all
```

## Memory Management Customization

### Buddy Allocator Tuning

The buddy allocator can be tuned for specific workloads:

```rust
// In src/kernel/memory.rs
pub struct BuddyAllocatorConfig {
    pub min_order: usize,      // Minimum allocation order
    pub max_order: usize,      // Maximum allocation order
    pub split_threshold: usize, // Threshold for block splitting
    pub merge_coalesce: bool,  // Enable automatic coalescing
}

impl BuddyAllocatorConfig {
    pub fn for_embedded() -> Self {
        Self {
            min_order: 0,
            max_order: 8,  // Smaller max for embedded
            split_threshold: 2,
            merge_coalesce: true,
        }
    }
    
    pub fn for_server() -> Self {
        Self {
            min_order: 0,
            max_order: 12, // Larger max for servers
            split_threshold: 4,
            merge_coalesce: true,
        }
    }
}
```

### Pool Tag Configuration

Customize Windows NT-style pool tags for debugging:

```rust
// Custom pool tags for driver identification
const DRIVER_TAGS: &[&[u8; 4]] = &[
    b"Net ",  // Network drivers
    b"Vid ",  // Video drivers
    b"Stor",  // Storage drivers
    b"Aud ",  // Audio drivers
    b"Cust",  // Custom drivers
];
```

## Scheduler Tuning

### Multi-Level Feedback Queue (MLFQ) Configuration

```rust
pub struct MlfqConfig {
    pub queue_count: usize,        // Number of priority queues
    pub boost_interval_ms: u64,    // Priority boost interval
    pub time_slice_base_ms: u64,   // Base time slice
    pub aging_factor: f32,         // Aging factor for process priority
}

impl MlfqConfig {
    pub fn for_interactive() -> Self {
        Self {
            queue_count: 8,
            boost_interval_ms: 1000,
            time_slice_base_ms: 4,
            aging_factor: 0.1,
        }
    }
    
    pub fn for_batch() -> Self {
        Self {
            queue_count: 4,
            boost_interval_ms: 5000,
            time_slice_base_ms: 16,
            aging_factor: 0.05,
        }
    }
}
```

### Real-Time Scheduler Configuration

For hard real-time requirements:

```rust
pub struct RtSchedulerConfig {
    pub deadline_monotonic: bool,  // Use deadline monotonic scheduling
    pub priority_inheritance: bool, // Enable priority inheritance
    pub preemption_threshold: u8,  // Preemption threshold
    pub latency_target_us: u64,    // Target latency in microseconds
}
```

## Device Driver Integration

### Custom Driver Development

1. **Implement the Device trait**:

```rust
use sigmaos::device::{Device, DeviceClass, DeviceError};

pub struct MyCustomDevice {
    id: DeviceID,
    name: [u8; 64],
    device_class: DeviceClass,
    // Custom device-specific fields
}

impl Device for MyCustomDevice {
    fn id(&self) -> DeviceID {
        self.id
    }
    
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    
    fn device_class(&self) -> DeviceClass {
        self.device_class
    }
    
    fn initialize(&mut self) -> Result<(), DeviceError> {
        // Custom initialization logic
        Ok(())
    }
    
    fn shutdown(&mut self) -> Result<(), DeviceError> {
        // Custom shutdown logic
        Ok(())
    }
}
```

2. **Register with Device Manager**:

```rust
use sigmaos::device::{DeviceManager, SimpleDeviceManager};

let mut device_manager = SimpleDeviceManager::new();
let custom_device = Box::new(MyCustomDevice::new(/* params */));
device_manager.register_device(custom_device)?;
```

### OOP-Based Driver Framework

SigmaOS uses an OOP-based driver framework with traits:

```rust
// Available base traits
pub trait DeviceDriver {
    fn device_id(&self) -> DeviceID;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError>;
    fn write(&mut self, data: &[u8]) -> Result<usize, DeviceError>;
    fn ioctl(&mut self, request: u32, arg: usize) -> Result<(), DeviceError>;
}

pub trait NetworkDriver {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), NetworkError>;
    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, NetworkError>;
}

pub trait StorageDriver {
    fn read_block(&mut self, lba: u64, buffer: &mut [u8]) -> Result<(), StorageError>;
    fn write_block(&mut self, lba: u64, data: &[u8]) -> Result<(), StorageError>;
}
```

## Security Hardening

### Capability-Based Security Configuration

```rust
use sigmaos::security::{CapabilityToken, SecurityEnforcer};

// Define custom capability tokens
pub struct CustomCapabilities {
    pub process_id: u32,
    pub allowed_operations: u64,  // Bitmask of allowed operations
    pub resource_limits: ResourceLimits,
}

impl CustomCapabilities {
    pub fn new_restricted(pid: u32) -> Self {
        Self {
            process_id: pid,
            allowed_operations: 0x01,  // Only basic operations
            resource_limits: ResourceLimits::minimal(),
        }
    }
    
    pub fn new_privileged(pid: u32) -> Self {
        Self {
            process_id: pid,
            allowed_operations: 0xFFFFFFFF,  // All operations
            resource_limits: ResourceLimits::maximum(),
        }
    }
}
```

### Post-Quantum Cryptography Configuration

```rust
pub struct PqcConfig {
    pub kem_algorithm: KemAlgorithm,     // Key encapsulation mechanism
    pub signature_algorithm: SigAlgorithm, // Digital signature algorithm
    pub key_rotation_interval_hours: u32,
}

pub enum KemAlgorithm {
    Kyber512,
    Kyber768,
    Kyber1024,  // Highest security level
}

pub enum SigAlgorithm {
    Dilithium2,
    Dilithium3,
    Dilithium5,  // Highest security level
}
```

## Performance Optimization

### Zero-Copy Networking

Enable zero-copy for high-performance networking:

```rust
use sigmaos::network::ZeroCopyNetwork;

let mut zero_copy_net = ZeroCopyNetwork::new();
zero_copy_net.enable_dma(true);
zero_copy_net.set_packet_pool_size(4096);
```

### CPU Affinity and NUMA

For multi-socket systems:

```rust
pub struct NumaConfig {
    pub node_count: usize,
    pub preferred_node: usize,
    pub interleave: bool,
}

impl NumaConfig {
    pub fn for_numa_aware() -> Self {
        Self {
            node_count: 4,
            preferred_node: 0,
            interleave: true,
        }
    }
}
```

### I/O Optimization

Configure I/O subsystems for specific workloads:

```rust
pub struct IoConfig {
    pub async_io: bool,
    pub io_uring_enabled: bool,
    pub max_io_depth: usize,
    pub io_priority_levels: u8,
}
```

## Testing and Validation

### Kernel Testing Framework

```rust
#[cfg(test)]
mod custom_kernel_tests {
    use super::*;
    
    #[test]
    fn test_custom_memory_allocation() {
        let config = BuddyAllocatorConfig::for_embedded();
        let allocator = BuddyAllocator::with_config(config);
        // Test custom allocation patterns
    }
    
    #[test]
    fn test_custom_scheduler_behavior() {
        let config = MlfqConfig::for_interactive();
        let scheduler = MlfqScheduler::with_config(config);
        // Test scheduling behavior
    }
}
```

## Build and Deployment

### Custom Build Commands

```bash
# Build with custom configuration
make PROFILE=custom FEATURES="custom_feature1,custom_feature2" all

# Run QEMU with custom kernel
qemu-system-x86_64 \
    -cdrom build/sigmaos-custom.iso \
    -m 4G \
    -smp 4 \
    -kernel build/kernel-custom.bin \
    -append "custom_param=value"

# Generate deployment image
make PROFILE=custom deploy-image
```

## Troubleshooting

### Common Issues

1. **Memory Allocation Failures**
   - Check pool sizes in configuration
   - Verify memory map for target hardware
   - Enable memory debugging in kernel config

2. **Scheduler Performance Issues**
   - Profile scheduler performance
   - Adjust time slice and priority levels
   - Consider workload-specific scheduler configuration

3. **Driver Initialization Failures**
   - Check device detection and registration
   - Verify interrupt handling configuration
   - Test driver in isolation before integration

## Additional Resources

- [SigmaOS Architecture Guide](ARCHITECTURE.md)
- [Device Driver Development Guide](DEVICE_DRIVER_GUIDE.md)
- [Security Hardening Guide](SECURITY_HARDENING_GUIDE.md)
- [Performance Tuning Guide](PERFORMANCE_TUNING_GUIDE.md)
- [API Reference](API_REFERENCE.md)

## Contributing

When contributing kernel customizations:

1. Document configuration options thoroughly
2. Provide example configurations for common use cases
3. Include performance benchmarks
4. Add test cases for custom functionality
5. Update relevant documentation

## License

Copyright © 2026 SigmaOS Project. Licensed under MIT License.