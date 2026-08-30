# SigmaOS Pre-defined Function Dependency Reduction Plan

## Overview

This document outlines strategies for reducing SigmaOS's dependency on pre-defined functions and implementing custom alternatives to improve self-sufficiency and control over system behavior.

## Table of Contents

1.  [Current Dependencies](#current-dependencies)
2.  [Custom Logging System](#custom-logging-system)
3.  [Custom String Operations](#custom-string-operations)
4.  [Custom Memory Operations](#custom-memory-operations)
5.  [Custom I/O Operations](#custom-io-operations)
6.  [Custom Time Functions](#custom-time-functions)
7.  [Custom Math Functions](#custom-math-functions)
8.  [Implementation Strategy](#implementation-strategy)

## Current Dependencies

### Pre-defined Functions in Use

Based on code analysis, SigmaOS currently depends on these pre-defined functions:

*   **println!** - Standard output for debugging
*   **format!** - String formatting
*   **vec!** - Vector creation
*   **String::from()** - String conversions
*   **HashMap::new()** - HashMap creation
*   **Thread::spawn()** - Thread spawning (where applicable)
*   **File operations** - File I/O via std::fs

### Dependency Reduction Goals

1.  Replace `println!` with custom logging system
2.  Implement custom string formatting
3.  Replace standard collections with klib implementations
4.  Create custom memory allocation wrappers
5.  Implement custom I/O primitives
6.  Build custom time management functions

## Custom Logging System

### SigmaOS Logger Implementation

```rust
// Custom logging system to replace println!
pub struct SigmaLogger {
    pub log_level: LogLevel,
    pub output_targets: Vec<LogTarget>,
    pub buffer: RingBuffer<LogEntry>,
}

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

pub enum LogTarget {
    SerialPort,
    Framebuffer,
    MemoryBuffer,
    Network,
}

pub struct LogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub module: &'static str,
    pub message: SigmaString,
}

impl SigmaLogger {
    pub fn new() -> Self {
        Self {
            log_level: LogLevel::Info,
            output_targets: vec![LogTarget::SerialPort],
            buffer: RingBuffer::new(1024),
        }
    }
    
    pub fn log(&mut self, level: LogLevel, module: &'static str, message: &str) {
        if self.should_log(level) {
            let entry = LogEntry {
                timestamp: self.get_timestamp(),
                level,
                module,
                message: SigmaString::from(message),
            };
            
            self.buffer.push(entry.clone());
            self.write_to_targets(&entry);
        }
    }
    
    pub fn info(&mut self, module: &'static str, message: &str) {
        self.log(LogLevel::Info, module, message);
    }
    
    pub fn error(&mut self, module: &'static str, message: &str) {
        self.log(LogLevel::Error, module, message);
    }
    
    pub fn debug(&mut self, module: &'static str, message: &str) {
        self.log(LogLevel::Debug, module, message);
    }
    
    fn should_log(&self, level: LogLevel) -> bool {
        level as u8 >= self.log_level as u8
    }
    
    fn get_timestamp(&self) -> u64 {
        // Custom timestamp implementation
        unsafe {
            let timestamp: u64;
            asm!("rdtsc" : "={eax}"(timestamp));
            timestamp
        }
    }
    
    fn write_to_targets(&self, entry: &LogEntry) {
        for target in &self.output_targets {
            match target {
                LogTarget::SerialPort => self.write_to_serial(entry),
                LogTarget::Framebuffer => self.write_to_framebuffer(entry),
                LogTarget::MemoryBuffer => {} // Already buffered
                LogTarget::Network => self.write_to_network(entry),
            }
        }
    }
    
    fn write_to_serial(&self, entry: &LogEntry) {
        // Write to serial port
        let message = self.format_entry(entry);
        for byte in message.as_bytes() {
            unsafe {
                let serial_port = 0x3F8 as *mut u8;
                core::ptr::write_volatile(serial_port, *byte);
            }
        }
    }
    
    fn format_entry(&self, entry: &LogEntry) -> SigmaString {
        // Custom string formatting
        let mut result = SigmaString::new();
        result.append_str("[");
        result.append_u64(entry.timestamp);
        result.append_str("] ");
        result.append_str(match entry.level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRIT",
        });
        result.append_str(" ");
        result.append_str(entry.module);
        result.append_str(": ");
        result.append_str(&entry.message);
        result
    }
}

// Global logger instance
static mut LOGGER: SigmaLogger = SigmaLogger::new();

pub fn sigma_log(level: LogLevel, module: &'static str, message: &str) {
    unsafe {
        LOGGER.log(level, module, message);
    }
}

// Convenience macros
#[macro_export]
macro_rules! sigma_info {
    ($module:expr, $msg:expr) => {
        sigma_log(LogLevel::Info, $module, $msg);
    };
}

#[macro_export]
macro_rules! sigma_error {
    ($module:expr, $msg:expr) => {
        sigma_log(LogLevel::Error, $module, $msg);
    };
}

#[macro_export]
macro_rules! sigma_debug {
    ($module:expr, $msg:expr) => {
        sigma_log(LogLevel::Debug, $module, $msg);
    };
}
```

## Custom String Operations

### SigmaString Implementation

```rust
// Custom string operations to replace std::string::String
pub struct SigmaString {
    pub data: Vec<u8>,
    pub len: usize,
}

impl SigmaString {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }
    
    pub fn from(s: &str) -> Self {
        let mut result = Self::new();
        result.append_str(s);
        result
    }
    
    pub fn append_str(&mut self, s: &str) {
        for byte in s.as_bytes() {
            self.data.push(*byte);
        }
        self.len += s.len();
    }
    
    pub fn append_u64(&mut self, value: u64) {
        let mut buffer = [0u8; 20];
        let mut i = 0;
        let mut n = value;
        
        if n == 0 {
            self.data.push(b'0');
            self.len += 1;
            return;
        }
        
        while n > 0 {
            buffer[i] = (n % 10) as u8 + b'0';
            n /= 10;
            i += 1;
        }
        
        // Reverse the buffer
        for j in 0..i {
            self.data.push(buffer[i - 1 - j]);
        }
        self.len += i;
    }
    
    pub fn append_i64(&mut self, value: i64) {
        if value < 0 {
            self.data.push(b'-');
            self.len += 1;
            self.append_u64((-value) as u64);
        } else {
            self.append_u64(value as u64);
        }
    }
    
    pub fn as_str(&self) -> &str {
        unsafe {
            core::str::from_utf8_unchecked(&self.data[..self.len])
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    pub fn split_whitespace(&self) -> Vec<SigmaString> {
        let mut result = Vec::new();
        let mut current = SigmaString::new();
        let mut in_word = false;
        
        for &byte in &self.data[..self.len] {
            if byte == b' ' || byte == b'\t' || byte == b'\n' {
                if in_word {
                    result.push(current);
                    current = SigmaString::new();
                    in_word = false;
                }
            } else {
                current.data.push(byte);
                current.len += 1;
                in_word = true;
            }
        }
        
        if in_word {
            result.push(current);
        }
        
        result
    }
    
    pub fn contains(&self, pattern: &str) -> bool {
        let pattern_bytes = pattern.as_bytes();
        if pattern_bytes.is_empty() {
            return true;
        }
        
        for i in 0..=(self.len() - pattern_bytes.len()) {
            let mut found = true;
            for j in 0..pattern_bytes.len() {
                if self.data[i + j] != pattern_bytes[j] {
                    found = false;
                    break;
                }
            }
            if found {
                return true;
            }
        }
        
        false
    }
}
```

## Custom Memory Operations

### Custom Memory Allocator Wrapper

```rust
// Custom memory operations to reduce std dependency
pub struct SigmaAllocator {
    pub buddy_allocator: BuddyAllocator,
    pub pool_manager: KernelPoolManager,
}

impl SigmaAllocator {
    pub fn new() -> Self {
        Self {
            buddy_allocator: BuddyAllocator::new(),
            pool_manager: KernelPoolManager::new(),
        }
    }
    
    pub fn allocate(&mut self, size: usize, tag: &[u8; 4]) -> Result<*mut u8, &'static str> {
        // Try buddy allocator first
        if let Some(block) = self.buddy_allocator.allocate(size) {
            Ok(block.addr.as_ptr() as *mut u8)
        } else {
            // Fall back to pool allocation
            let pool_block = self.pool_manager.allocate_pool(PoolType::NonPaged, size, tag)?;
            Ok(pool_block.addr as *mut u8)
        }
    }
    
    pub fn deallocate(&mut self, ptr: *mut u8, size: usize) {
        let block = MemoryBlock {
            addr: unsafe { NonNull::new(ptr).unwrap() },
            size,
        };
        self.buddy_allocator.deallocate(block);
    }
    
    pub fn allocate_zeroed(&mut self, size: usize, tag: &[u8; 4]) -> Result<*mut u8, &'static str> {
        let ptr = self.allocate(size, tag)?;
        unsafe {
            core::ptr::write_bytes(ptr, 0, size);
        }
        Ok(ptr)
    }
}

// Global allocator instance
static mut ALLOCATOR: SigmaAllocator = SigmaAllocator::new();

pub fn sigma_alloc(size: usize, tag: &[u8; 4]) -> Result<*mut u8, &'static str> {
    unsafe {
        ALLOCATOR.allocate(size, tag)
    }
}

pub fn sigma_dealloc(ptr: *mut u8, size: usize) {
    unsafe {
        ALLOCATOR.deallocate(ptr, size);
    }
}
```

## Custom I/O Operations

### Custom I/O Primitives

```rust
// Custom I/O operations to replace std::io
pub struct SigmaIo {
    pub serial_port: Option<SerialPort>,
    pub framebuffer: Option<Framebuffer>,
}

pub struct SerialPort {
    pub base_address: u16,
    pub baud_rate: u32,
}

pub struct Framebuffer {
    pub base_address: usize,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
}

impl SigmaIo {
    pub fn new() -> Self {
        Self {
            serial_port: Some(SerialPort {
                base_address: 0x3F8,
                baud_rate: 115200,
            }),
            framebuffer: None,
        }
    }
    
    pub fn init_serial(&mut self) {
        if let Some(ref mut serial) = self.serial_port {
            serial.init();
        }
    }
    
    pub fn write_byte(&mut self, byte: u8) {
        if let Some(ref serial) = self.serial_port {
            serial.write_byte(byte);
        }
    }
    
    pub fn write_str(&mut self, s: &str) {
        for byte in s.as_bytes() {
            self.write_byte(*byte);
        }
    }
    
    pub fn read_byte(&mut self) -> Option<u8> {
        if let Some(ref serial) = self.serial_port {
            serial.read_byte()
        } else {
            None
        }
    }
    
    pub fn write_line(&mut self, s: &str) {
        self.write_str(s);
        self.write_byte(b'\n');
    }
}

impl SerialPort {
    pub fn init(&mut self) {
        // Initialize serial port
        unsafe {
            let base = self.base_address as *mut u8;
            
            // Disable interrupts
            core::ptr::write_volatile(base.offset(1), 0x00u8);
            
            // Enable DLAB (set baud rate divisor)
            core::ptr::write_volatile(base.offset(3), 0x80u8);
            
            // Set divisor to 115200 baud
            let divisor = 115200 / self.baud_rate;
            core::ptr::write_volatile(base.offset(0), (divisor & 0xFF) as u8);
            core::ptr::write_volatile(base.offset(1), ((divisor >> 8) & 0xFF) as u8);
            
            // 8 bits, no parity, 1 stop bit
            core::ptr::write_volatile(base.offset(3), 0x03u8);
            
            // Enable FIFO, clear them, with 14-byte threshold
            core::ptr::write_volatile(base.offset(2), 0xC7u8);
            core::ptr::write_volatile(base.offset(2), 0xE7u8);
            
            // Enable interrupts (receive data available)
            core::ptr::write_volatile(base.offset(1), 0x01u8);
        }
    }
    
    pub fn write_byte(&self, byte: u8) {
        unsafe {
            let base = self.base_address as *mut u8;
            
            // Wait for transmit buffer to be empty
            while core::ptr::read_volatile(base.offset(5)) & 0x20 == 0 {
                core::hint::spin_loop();
            }
            
            core::ptr::write_volatile(base, byte);
        }
    }
    
    pub fn read_byte(&self) -> Option<u8> {
        unsafe {
            let base = self.base_address as *mut u8;
            
            // Check if data is available
            if core::ptr::read_volatile(base.offset(5)) & 0x01 == 0 {
                return None;
            }
            
            Some(core::ptr::read_volatile(base))
        }
    }
}
```

## Custom Time Functions

### Custom Time Management

```rust
// Custom time functions to replace std::time
pub struct SigmaTime {
    pub boot_time: u64,
    pub tick_rate_hz: u32,
}

impl SigmaTime {
    pub fn new() -> Self {
        Self {
            boot_time: Self::read_tsc(),
            tick_rate_hz: 1000,
        }
    }
    
    pub fn get_uptime_ms(&self) -> u64 {
        let current_tsc = Self::read_tsc();
        let elapsed_tsc = current_tsc - self.boot_time;
        // Convert TSC to milliseconds (assuming 2.4 GHz CPU)
        elapsed_tsc / 2400000
    }
    
    pub fn get_uptime_us(&self) -> u64 {
        let current_tsc = Self::read_tsc();
        let elapsed_tsc = current_tsc - self.boot_time;
        elapsed_tsc / 2400
    }
    
    pub fn get_uptime_ns(&self) -> u64 {
        let current_tsc = Self::read_tsc();
        let elapsed_tsc = current_tsc - self.boot_time;
        elapsed_tsc * 417  // Approximate nanosecond conversion
    }
    
    pub fn sleep_ms(&self, ms: u64) {
        let target_tsc = Self::read_tsc() + (ms * 2400000);
        while Self::read_tsc() < target_tsc {
            core::hint::spin_loop();
        }
    }
    
    pub fn sleep_us(&self, us: u64) {
        let target_tsc = Self::read_tsc() + (us * 2400);
        while Self::read_tsc() < target_tsc {
            core::hint::spin_loop();
        }
    }
    
    unsafe fn read_tsc() -> u64 {
        let tsc: u64;
        asm!("rdtsc" : "={eax}"(tsc));
        tsc
    }
}

// Global time instance
static mut TIME_MANAGER: SigmaTime = SigmaTime::new();

pub fn sigma_sleep_ms(ms: u64) {
    unsafe {
        TIME_MANAGER.sleep_ms(ms);
    }
}

pub fn sigma_sleep_us(us: u64) {
    unsafe {
        TIME_MANAGER.sleep_us(us);
    }
}

pub fn sigma_get_uptime_ms() -> u64 {
    unsafe {
        TIME_MANAGER.get_uptime_ms()
    }
}
```

## Custom Math Functions

### Custom Math Operations

```rust
// Custom math functions to reduce std dependency
pub struct SigmaMath;

impl SigmaMath {
    pub fn min(a: u64, b: u64) -> u64 {
        if a < b { a } else { b }
    }
    
    pub fn max(a: u64, b: u64) -> u64 {
        if a > b { a } else { b }
    }
    
    pub fn clamp(value: u64, min: u64, max: u64) -> u64 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
    
    pub fn align_up(value: usize, alignment: usize) -> usize {
        (value + alignment - 1) & !(alignment - 1)
    }
    
    pub fn align_down(value: usize, alignment: usize) -> usize {
        value & !(alignment - 1)
    }
    
    pub fn is_power_of_two(n: usize) -> bool {
        n > 0 && (n & (n - 1)) == 0
    }
    
    pub fn next_power_of_two(n: usize) -> usize {
        if n == 0 {
            return 1;
        }
        
        let mut result = 1usize;
        while result < n {
            result <<= 1;
        }
        
        result
    }
    
    pub fn trailing_zeros(n: usize) -> u32 {
        if n == 0 {
            return 0;
        }
        
        let mut count = 0;
        while (n & 1) == 0 {
            count += 1;
            n >>= 1;
        }
        
        count
    }
    
    pub fn leading_zeros(n: usize) -> u32 {
        if n == 0 {
            return 0;
        }
        
        let mut count = 0;
        let mut shifted = n;
        
        while shifted != 0 {
            shifted >>= 1;
            count += 1;
        }
        
        (core::mem::size_of::<usize>() * 8) as u32 - count
    }
    
    pub fn div_ceil(a: usize, b: usize) -> usize {
        (a + b - 1) / b
    }
    
    pub fn abs_diff(a: i64, b: i64) -> u64 {
        if a > b {
            (a - b) as u64
        } else {
            (b - a) as u64
        }
    }
}
```

## Implementation Strategy

### Phase 1: Logging System Replacement

1.  **Identify all println! usage**: Replace with sigma\_log macros
2.  **Implement SigmaLogger**: Create custom logging infrastructure
3.  **Update modules**: Replace logging calls systematically
4.  **Test thoroughly**: Ensure logging functionality is preserved

### Phase 2: String Operations

1.  **Replace String::from()**: Use SigmaString::from()
2.  **Implement custom formatting**: Build SigmaString formatting methods
3.  **Update string operations**: Systematically replace std::string usage
4.  **Validate correctness**: Ensure string operations work correctly

### Phase 3: Memory Operations

1.  **Replace Vec::new()**: Use klib::Vec::new()
2.  **Implement custom allocators**: Use SigmaAllocator for memory management
3.  **Update memory operations**: Replace std::alloc usage
4.  **Memory safety testing**: Ensure memory operations are safe

### Phase 4: I/O Operations

1.  **Replace std::io operations**: Use SigmaIo for I/O
2.  **Implement custom I/O primitives**: Build serial port, framebuffer drivers
3.  **Update I/O calls**: Replace std::io usage systematically
4.  **I/O testing**: Ensure I/O operations work correctly

### Phase 5: Time Functions

1.  **Replace std::time usage**: Use SigmaTime for time operations
2.  **Implement custom time functions**: Build TSC-based time management
3.  **Update time calls**: Replace std::time usage
4.  **Time accuracy testing**: Ensure time operations are accurate

## Testing and Validation

### Function Replacement Testing

```rust
#[cfg(test)]
mod function_reduction_tests {
    use super::*;
    
    #[test]
    fn test_custom_logging() {
        let mut logger = SigmaLogger::new();
        logger.info("test_module", "Test message");
        // Verify logging works
    }
    
    #[test]
    fn test_custom_string() {
        let mut s = SigmaString::new();
        s.append_str("Hello, ");
        s.append_str("World!");
        assert_eq!(s.as_str(), "Hello, World!");
    }
    
    #[test]
    fn test_custom_math() {
        assert_eq!(SigmaMath::min(10, 20), 10);
        assert_eq!(SigmaMath::max(10, 20), 20);
        assert_eq!(SigmaMath::clamp(15, 10, 20), 15);
        assert_eq!(SigmaMath::clamp(5, 10, 20), 10);
        assert_eq!(SigmaMath::clamp(25, 10, 20), 20);
    }
    
    #[test]
    fn test_custom_time() {
        let time = SigmaTime::new();
        let uptime1 = time.get_uptime_ms();
        time.sleep_ms(10);
        let uptime2 = time.get_uptime_ms();
        assert!(uptime2 >= uptime1 + 10);
    }
}
```

## Migration Guide

### Replacing println! Statements

```rust
// Before
println!("Loading driver: {}", driver_name);

// After
sigma_info!("driver_manager", &format_args!("Loading driver: {}", driver_name));
```

### Replacing String Operations

```rust
// Before
let message = String::from("Hello");
let formatted = format!("Value: {}", value);

// After
let message = SigmaString::from("Hello");
let mut formatted = SigmaString::new();
formatted.append_str("Value: ");
formatted.append_u64(value);
```

### Replacing Time Operations

```rust
// Before
let now = std::time::Instant::now();
std::thread::sleep(Duration::from_millis(100));

// After
let now = sigma_get_uptime_ms();
sigma_sleep_ms(100);
```

## Benefits

1.  **Reduced Dependency**: Less reliance on std library
2.  **Better Control**: Fine-grained control over system behavior
3.  **Performance**: Optimized for specific use cases
4.  **Security**: Reduced attack surface
5.  **Portability**: Easier to port to different platforms

## Resources

*   [SigmaOS Architecture](ARCHITECTURE)
*   [Kernel Customization Guide](KERNEL_CUSTOMIZATION_GUIDE)
*   [Std Reduction Plan](STD_REDUCTION_PLAN)
*   [Zero Dependency Architecture](ZERO_DEPENDENCY_ARCHITECTURE)

## Contributing

When implementing function reduction:

1.  Provide clear migration paths
2.  Include comprehensive testing
3.  Document performance characteristics
4.  Ensure compatibility with existing code
5.  Update relevant documentation

## License

Copyright © 2026 SigmaOS Project. Licensed under MIT License.
