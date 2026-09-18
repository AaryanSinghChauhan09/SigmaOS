# SigmaOS Naming Conventions and Rules

SigmaOS follows strict naming conventions and coding rules to maintain consistency, readability, and maintainability across the codebase. These conventions are inspired by Rust best practices, Linux kernel style, and BSD coding standards.

## Overview

Naming conventions ensure:
- Consistent code style across the project
- Easy-to-read and maintainable code
- Clear separation between different types of identifiers
- Alignment with Rust and system programming best practices

## Rust Naming Conventions

### Types
**UpperCamelCase** for all types:
- Structs: `PciDevice`, `MemoryManager`, `GpuBuffer`
- Enums: `PciMode`, `CompressionType`, `QueuePriority`
- Traits: `DriverInterface`, `MemoryAllocator`, `Filesystem`
- Type Aliases: `PhysicalAddress`, `VirtualAddress`, `PageSize`

```rust
// Good
pub struct PciDevice {
    pub vendor_id: u16,
    pub device_id: u32,
}

pub enum PciMode {
    Hardware,
    Simulated,
}

pub trait MemoryAllocator {
    fn allocate(&mut self, size: usize) -> Result<*mut u8, AllocError>;
}
```

### Functions and Methods
**snake_case** for all functions and methods:
- Functions: `allocate_memory`, `initialize_device`, `read_register`
- Methods: `new()`, `read_data()`, `write_buffer()`
- Associated functions: `from_bytes()`, `default()`

```rust
// Good
impl PciDevice {
    pub fn new(vendor_id: u16, device_id: u32) -> Self {
        PciDevice { vendor_id, device_id }
    }

    pub fn read_register(&self, offset: u16) -> u32 {
        // Implementation
    }
}
```

### Variables
**snake_case** for all variables:
- Local variables: `buffer_size`, `page_offset`, `device_count`
- Parameters: `size`, `offset`, `flags`
- Static variables: `MAX_DEVICES`, `PAGE_SIZE`

```rust
// Good
fn process_data(data: &[u8], offset: usize) {
    let buffer_size = data.len();
    let page_offset = offset % PAGE_SIZE;
}
```

### Constants
**SCREAMING_SNAKE_CASE** for constants:
- Global constants: `MAX_DEVICES`, `PAGE_SIZE`, `QUEUE_CAPACITY`
- Module-level constants: `DEFAULT_TIMEOUT`, `MAX_RETRIES`

```rust
// Good
pub const MAX_DEVICES: usize = 256;
pub const PAGE_SIZE: usize = 4096;
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
```

### Modules
**snake_case** for module names:
- Modules: `pci`, `memory`, `networking`
- Files: `pci.rs`, `memory.rs`, `networking.rs`

```rust
// Good
mod pci {
    pub struct Device;
}

mod memory {
    pub struct Manager;
}
```

### Enum Variants
**UpperCamelCase** for enum variants:
- Simple variants: `Hardware`, `Simulated`, `Enabled`, `Disabled`
- Tuple variants: `Error(String)`, `Result(u32, u32)`
- Struct variants: `Connected { id: u32, name: String }`

```rust
// Good
pub enum PciMode {
    Hardware,
    Simulated,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub enum ConnectionState {
    Connected { id: u32, name: String },
    Disconnected,
}
```

### Type Parameters
**UpperCamelCase** for type parameters:
- Single-letter: `T`, `E`, `K`, `V`
- Descriptive: `Item`, `Error`, `Key`, `Value`

```rust
// Good
pub struct Container<T> {
    items: Vec<T>,
}

pub struct Map<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}
```

### Lifetimes
**Lowercase with leading apostrophe** for lifetimes:
- Short: `'a`, `'b`, `'c`
- Descriptive: `'data`, `'context`, `'buffer`

```rust
// Good
pub struct Buffer<'a> {
    data: &'a [u8],
}

pub fn process<'a>(data: &'a [u8]) -> &'a [u8] {
    data
}
```

## C-Style Bindings

### C Types
When using C types from FFI, maintain their original casing for compatibility:
- `c_int`, `c_uint`, `c_char`, `c_void`
- `c_long`, `c_ulong`, `c_longlong`, `c_ulonglong`
- `size_t`, `ssize_t`, `off_t`, `pid_t`

```rust
// Good
use std::os::raw::{c_int, c_uint, c_char, c_void};

extern "C" {
    fn c_function(arg: c_int) -> c_uint;
}
```

### C Enums and Constants
Maintain original SCREAMING_SNAKE_CASE for C compatibility:
- `EVFILT_READ`, `EVFILT_WRITE`, `EVFILT_AIO`
- `O_RDONLY`, `O_WRONLY`, `O_RDWR`
- `MAP_SHARED`, `MAP_PRIVATE`, `MAP_ANONYMOUS`

```rust
// Good
pub const EVFILT_READ: i16 = -1;
pub const EVFILT_WRITE: i16 = -2;
pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1;
```

## Special Cases

### Acronyms
Treat acronyms as regular words:
- **CPU**: `CpuContext`, `cpu_count`, `MAX_CPUS`
- **GPU**: `GpuDevice`, `gpu_id`, `MAX_GPUS`
- **PCI**: `PciDevice`, `pci_id`, `MAX_PCIS`
- **USB**: `UsbDevice`, `usb_id`, `MAX_USBS`
- **IO**: `IoPort`, `io_base`, `MAX_IOS`
- **ID**: `device_id`, `user_id`, `max_id`

```rust
// Good
pub struct CpuContext {
    pub cpu_id: u32,
    pub core_count: u32,
}

pub struct GpuDevice {
    pub gpu_id: u32,
    pub memory_size: u64,
}

pub struct PciDevice {
    pub pci_id: u32,
    pub vendor_id: u16,
}
```

### Hardware Registers
**SCREAMING_SNAKE_CASE** for hardware register names:
- `PCI_COMMAND_REGISTER`, `PCI_STATUS_REGISTER`
- `MMIO_BASE_ADDRESS`, `MMIO_SIZE`
- `GPIO_CONTROL_REGISTER`, `GPIO_DATA_REGISTER`

```rust
// Good
pub const PCI_COMMAND_REGISTER: u16 = 0x04;
pub const PCI_STATUS_REGISTER: u16 = 0x06;
pub const MMIO_BASE_ADDRESS: u64 = 0xFEC00000;
```

## Error Handling

### Error Types
**UpperCamelCase** for error types:
- `AllocError`, `IoError`, `NetworkError`
- End with `Error` suffix

```rust
// Good
pub enum AllocError {
    OutOfMemory,
    InvalidSize,
}

pub enum IoError {
    ReadFailed,
    WriteFailed,
}
```

### Error Variants
**UpperCamelCase** for error variants:
- `OutOfMemory`, `InvalidSize`, `ReadFailed`
- Descriptive and clear

```rust
// Good
pub enum AllocError {
    OutOfMemory { requested: usize, available: usize },
    InvalidSize { size: usize },
    AlignmentError { alignment: usize },
}
```

## Testing

### Test Functions
**snake_case** with `test_` prefix:
- `test_allocate_memory()`, `test_read_register()`
- `test_pci_device_init()`, `test_gpu_buffer_validation()`

```rust
// Good
#[test]
fn test_allocate_memory() {
    let allocator = MemoryAllocator::new();
    let ptr = allocator.allocate(1024).unwrap();
    assert!(!ptr.is_null());
}

#[test]
fn test_pci_device_init() {
    let device = PciDevice::new(0x10EC, 0x8168);
    assert_eq!(device.vendor_id, 0x10EC);
}
```

## Common Mistakes to Avoid

### 1. Mixed Case in Identifiers
```rust
// Bad
let MyVariable = 42;
let my_variable = 42;
let MY_VARIABLE = 42; // Only for constants
```

### 2. Abbreviations in Names
```rust
// Bad
let dev = device;
let addr = address;
let cfg = config;

// Good
let device = get_device();
let address = get_address();
let config = get_config();
```

### 3. Hungarian Notation
```rust
// Bad
let iCount = 0;
let pDevice = &device;
let bEnabled = true;

// Good
let count = 0;
let device = &device;
let enabled = true;
```

## Tools and Automation

### Automatic Formatting
Use `rustfmt` for automatic code formatting:
```bash
cargo fmt
```

### Linting
Use `clippy` for additional linting:
```bash
cargo clippy
```

## Enforcement

### Pre-Commit Hooks
Require naming convention compliance before commits:
- Run `cargo fmt --check`
- Run `cargo clippy`
- Run `cargo check`

### CI/CD
Enforce naming conventions in CI:
- Fail builds on naming convention violations
- Automated fixes for simple violations
- Manual review required for complex cases

---

**[Development Guidelines](Category-Development)** | **[Style Guide](Style-Guide)** | **[Contributing](Contributing)**
