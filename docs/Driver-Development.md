# Driver Development Guide

This guide covers developing drivers for SigmaOS, including the driver model, supported hardware, and best practices.

## Table of Contents

- [Driver Model](#driver-model)
- [Driver Types](#driver-types)
- [Development Environment](#development-environment)
- [Writing a Driver](#writing-a-driver)
- [Driver API](#driver-api)
- [Testing Drivers](#testing-drivers)
- [Submitting Drivers](#submitting-drivers)

## Driver Model

SigmaOS uses an object-oriented driver model with capability-based security.

### DeviceDriver Trait

All drivers implement the DeviceDriver trait:

```rust
pub trait DeviceDriver {
    fn name(&self) -> &'static str;
    fn init(&mut self) -> Result<(), DriverError>;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DriverError>;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, DriverError>;
    fn ioctl(&mut self, cmd: u32, arg: usize) -> Result<(), DriverError>;
    fn cleanup(&mut self);
}
```

### Capability-Based Access

Drivers require capabilities for hardware access:

- **I/O Port Access**: `CAP_IO_PORT`
- **Memory-Mapped I/O**: `CAP_MMIO`
- **DMA Operations**: `CAP_DMA`
- **Interrupt Handling**: `CAP_IRQ`

### Driver Lifecycle

1. **Probe**: Detect hardware presence
2. **Initialize**: Set up hardware and driver state
3. **Register**: Register with kernel driver manager
4. **Operate**: Handle I/O requests
5. **Cleanup**: Release resources on unload

## Driver Types

### Block Drivers

Block drivers handle storage devices (NVMe, SATA, USB storage):

```rust
pub struct BlockDriver {
    device_id: DeviceId,
    block_size: u32,
    num_blocks: u64,
    // Implementation details
}

impl BlockDriver {
    pub fn read_block(&mut self, block: u64, buffer: &mut [u8]) -> Result<(), DriverError>;
    pub fn write_block(&mut self, block: u64, buffer: &[u8]) -> Result<(), DriverError>;
}
```

### Network Drivers

Network drivers handle network interfaces (Ethernet, Wi-Fi):

```rust
pub struct NetworkDriver {
    device_id: DeviceId,
    mac_address: [u8; 6],
    mtu: u16,
    // Implementation details
}

impl NetworkDriver {
    pub fn send_packet(&mut self, packet: &[u8]) -> Result<(), DriverError>;
    pub fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, DriverError>;
}
```

### GPU Drivers

GPU drivers handle graphics hardware (NVIDIA, AMD, Intel):

```rust
pub struct GpuDriver {
    device_id: DeviceId,
    framebuffer: Framebuffer,
    // Implementation details
}

impl GpuDriver {
    pub fn set_mode(&mut self, width: u32, height: u32) -> Result<(), DriverError>;
    pub fn blit(&mut self, src: &[u8], dst: &mut [u8]) -> Result<(), DriverError>;
}
```

### Input Drivers

Input drivers handle input devices (keyboard, mouse, touchscreen):

```rust
pub struct InputDriver {
    device_id: DeviceId,
    device_type: InputType,
    // Implementation details
}

impl InputDriver {
    pub fn read_event(&mut self) -> Result<InputEvent, DriverError>;
}
```

## Development Environment

### Prerequisites

- Rust toolchain (stable)
- Zig compiler (for low-level components)
- QEMU (for testing)
- Hardware documentation for target device

### Setting Up

```bash
# Clone SigmaOS repository
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS

# Install Rust
rustup install stable
rustup component add clippy rustfmt

# Install Zig
# See https://ziglang.org/download/

# Build kernel with driver support
make build
```

### Driver Development Directory

```
SigmaOS/
├── drivers/
│   ├── block/
│   │   ├── nvme/
│   │   ├── ahci/
│   │   └── usb/
│   ├── network/
│   │   ├── ethernet/
│   │   └── wifi/
│   ├── gpu/
│   │   ├── nvidia/
│   │   ├── amd/
│   │   └── intel/
│   └── input/
│       ├── keyboard/
│       ├── mouse/
│       └── touchscreen/
```

## Writing a Driver

### Example: Simple Block Driver

```rust
use sigmaos::driver::{DeviceDriver, DriverError, DeviceId};

pub struct SimpleBlockDriver {
    device_id: DeviceId,
    block_size: u32,
    num_blocks: u64,
}

impl SimpleBlockDriver {
    pub fn new(device_id: DeviceId) -> Self {
        Self {
            device_id,
            block_size: 512,
            num_blocks: 1024 * 1024, // 1GB
        }
    }
}

impl DeviceDriver for SimpleBlockDriver {
    fn name(&self) -> &'static str {
        "simple_block"
    }

    fn init(&mut self) -> Result<(), DriverError> {
        // Initialize hardware
        println!("Initializing simple block driver");
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DriverError> {
        // Read from device
        Ok(buffer.len())
    }

    fn write(&mut self, buffer: &[u8]) -> Result<usize, DriverError> {
        // Write to device
        Ok(buffer.len())
    }

    fn ioctl(&mut self, _cmd: u32, _arg: usize) -> Result<(), DriverError> {
        Ok(())
    }

    fn cleanup(&mut self) {
        println!("Cleaning up simple block driver");
    }
}
```

### Registering the Driver

```rust
use sigmaos::driver::DriverManager;

fn register_driver() {
    let driver = SimpleBlockDriver::new(DeviceId::new(0x1234, 0x5678));
    DriverManager::register(driver).expect("Failed to register driver");
}
```

### Driver Entry Point

```rust
#[no_mangle]
pub extern "C" fn driver_init() -> *mut dyn DeviceDriver {
    Box::into_raw(Box::new(SimpleBlockDriver::new(DeviceId::new(0x1234, 0x5678))))
}
```

## Driver API

### Memory Management

Drivers use custom allocators for memory management:

```rust
use sigmaos::memory::DriverAllocator;

// Allocate memory
let buffer = DriverAllocator::allocate(4096)?;

// Free memory
DriverAllocator::free(buffer);
```

### Interrupt Handling

Drivers register interrupt handlers:

```rust
use sigmaos::irq::InterruptHandler;

struct MyInterruptHandler;

impl InterruptHandler for MyInterruptHandler {
    fn handle(&self) {
        // Handle interrupt
    }
}

// Register handler
sigmaos::irq::register(IRQ_NUMBER, MyInterruptHandler)?;
```

### DMA Operations

Drivers use DMA for high-performance I/O:

```rust
use sigmaos::dma::DmaBuffer;

// Allocate DMA buffer
let dma_buffer = DmaBuffer::allocate(4096)?;

// Perform DMA transfer
dma_buffer.transfer(src, dst, length)?;
```

### Capability Requests

Drivers request capabilities during initialization:

```rust
use sigmaos::capability::{Capability, CapabilityRights};

// Request I/O port capability
let io_cap = Capability::request(CapabilityRights::IO_PORT, 0x3F8, 8)?;

// Request MMIO capability
let mmio_cap = Capability::request(CapabilityRights::MMIO, 0xFE000000, 0x1000)?;
```

## Testing Drivers

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_init() {
        let mut driver = SimpleBlockDriver::new(DeviceId::new(0x1234, 0x5678));
        assert!(driver.init().is_ok());
    }

    #[test]
    fn test_read_write() {
        let mut driver = SimpleBlockDriver::new(DeviceId::new(0x1234, 0x5678));
        driver.init().unwrap();

        let mut buffer = vec![0u8; 512];
        let written = driver.write(&buffer).unwrap();
        assert_eq!(written, 512);

        let read = driver.read(&mut buffer).unwrap();
        assert_eq!(read, 512);
    }
}
```

### Integration Testing

Test drivers in QEMU:

```bash
# Run SigmaOS with driver in QEMU
make qemu DRIVER=my_driver

# Run driver tests
make test-driver DRIVER=my_driver
```

### Hardware Testing

Test on real hardware:

1. Build driver into kernel image
2. Flash to test hardware
3. Boot and test driver functionality
4. Collect logs and performance data

## Best Practices

### Error Handling

Use Result types for error handling:

```rust
pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DriverError> {
    if buffer.len() < self.block_size as usize {
        return Err(DriverError::InvalidArgument);
    }
    // Implementation
    Ok(self.block_size as usize)
}
```

### Resource Cleanup

Ensure proper resource cleanup:

```rust
impl Drop for SimpleBlockDriver {
    fn drop(&mut self) {
        self.cleanup();
    }
}
```

### Capability Safety

Always check capabilities before operations:

```rust
fn perform_io(&self, address: u64) -> Result<(), DriverError> {
    if !self.capability.check(CapabilityRights::MMIO, address) {
        return Err(DriverError::AccessDenied);
    }
    // Perform I/O
    Ok(())
}
```

### Performance

Use zero-copy operations where possible:

```rust
// Use shared memory instead of copying
use sigmaos::ipc::SharedMemory;

let shm = SharedMemory::create(size)?;
let ptr = shm.map()?;
// Direct access to shared memory
```

## Submitting Drivers

### Driver Submission Checklist

- [ ] Driver follows DeviceDriver trait
- [ ] Proper error handling throughout
- [ ] Capability-based access control
- [ ] Comprehensive unit tests
- [ ] Integration tests in QEMU
- [ ] Documentation (README, API docs)
- [ ] Hardware compatibility notes
- [ ] Performance benchmarks
- [ ] License compliance

### Submission Process

1. **Fork** the SigmaOS repository
2. **Create** driver in appropriate directory
3. **Implement** driver following guidelines
4. **Test** thoroughly in QEMU and on hardware
5. **Document** driver and API
6. **Submit** pull request with description

### Pull Request Template

```markdown
## Driver Description
Brief description of the driver and hardware it supports.

## Hardware Support
- Device IDs supported
- Hardware features implemented
- Known limitations

## Testing
- Unit tests: [x] Passed
- QEMU tests: [x] Passed
- Hardware tests: [ ] Passed (specify hardware)

## Performance
- Benchmark results
- Comparison with alternatives

## Documentation
- API documentation: [x] Complete
- User documentation: [x] Complete
- Developer documentation: [x] Complete
```

## Supported Hardware

### Currently Supported

- **NVMe**: Basic NVMe controller support
- **AHCI**: SATA controller support
- **USB xHCI**: USB 3.0 controller support
- **Ethernet**: Common NICs (Intel, Realtek)
- **GPU**: Basic framebuffer support

### Hardware Wishlist

- **NVMe**: Advanced features (namespace management)
- **AHCI**: RAID support
- **USB**: Full USB stack
- **Wi-Fi**: 802.11ac/ax support
- **GPU**: Full 3D acceleration (NVIDIA, AMD, Intel)
- **Audio**: HD Audio support
- **Bluetooth**: Bluetooth controller support

## Further Reading

- [Kernel Internals](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Kernel-Internals)
- [Architecture](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Architecture)
- [Contributing](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Contributing)
- [Security Policy](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Security-Policy)

---

*Last Updated: 2026-07-14*
