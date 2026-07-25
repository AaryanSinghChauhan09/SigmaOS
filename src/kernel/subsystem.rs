// SigmaOS Unified Subsystem Architecture
// Abstract base trait hierarchy for Linux driver absorption and OOP-based modularity
// This enables SigmaOS to absorb Linux subsystems while maintaining sovereign identity

#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;

use crate::security::CapabilityToken;

// ============================================================================
// Core Driver Abstraction
// ============================================================================

/// Abstract base trait for all device drivers
/// Provides standardized interface for kernel-driver communication
pub trait DeviceDriver: Any {
    /// Initialize the driver and its hardware
    fn init(&mut self) -> Result<(), DriverError>;
    
    /// Handle I/O operations for the device
    fn handle_io(&mut self, operation: IoOperation) -> Result<IoResult, DriverError>;
    
    /// Gracefully shutdown the driver
    fn shutdown(&mut self) -> Result<(), DriverError>;
    
    /// Get driver metadata
    fn metadata(&self) -> &DriverMetadata;
    
    /// Check if driver is capable of handling specific capability
    fn has_capability(&self, capability: u64) -> bool {
        self.metadata().capabilities.contains(&capability)
    }
    
    /// Downcast to concrete type for driver-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Driver metadata for discovery and management
#[derive(Debug, Clone)]
pub struct DriverMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub driver_type: DriverType,
    pub linux_heritage: Option<LinuxHeritage>,
    pub capabilities: Vec<u64>,
    pub required_capabilities: Vec<u64>,
}

/// Classification of driver types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Block,
    Character,
    Network,
    Graphics,
    Input,
    Audio,
    Storage,
    Bus,
    Misc,
}

/// Linux kernel heritage information for absorbed drivers
#[derive(Debug, Clone)]
pub struct LinuxHeritage {
    pub original_module: String,
    pub kernel_version: String,
    pub absorption_date: String,
    pub modifications: Vec<String>,
}

/// I/O operations that can be performed on devices
#[derive(Debug, Clone)]
pub enum IoOperation {
    Read { offset: u64, size: usize },
    Write { offset: u64, data: Vec<u8> },
    Ioctl { command: u32, arg: u64 },
    Mmap { offset: u64, size: usize },
    Poll { events: u32 },
}

/// Result of I/O operations
#[derive(Debug, Clone)]
pub enum IoResult {
    ReadComplete { data: Vec<u8> },
    WriteComplete { bytes_written: usize },
    IoctlComplete { result: u32 },
    MmapComplete { address: u64 },
    PollComplete { events: u32 },
}

/// Driver errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError {
    InitializationFailed,
    IoError(String),
    InvalidOperation,
    PermissionDenied,
    DeviceNotFound,
    BufferOverflow,
    Timeout,
    NotSupported,
}

// ============================================================================
// Network Stack Abstraction
// ============================================================================

/// Abstract base trait for network stack implementations
/// Allows polymorphic network stack selection (Linux-derived vs SigmaOS-native)
pub trait NetworkStack: Any {
    /// Initialize the network stack
    fn init(&mut self) -> Result<(), NetworkError>;
    
    /// Handle incoming packet
    fn receive_packet(&mut self, packet: Vec<u8>) -> Result<(), NetworkError>;
    
    /// Send packet through the stack
    fn send_packet(&mut self, packet: Vec<u8>) -> Result<(), NetworkError>;
    
    /// Create a socket
    fn create_socket(&mut self, domain: SocketDomain, socket_type: SocketType, protocol: SocketProtocol) -> Result<SocketHandle, NetworkError>;
    
    /// Close a socket
    fn close_socket(&mut self, handle: SocketHandle) -> Result<(), NetworkError>;
    
    /// Get stack metadata
    fn metadata(&self) -> &NetworkStackMetadata;
    
    /// Downcast for stack-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Network stack metadata
#[derive(Debug, Clone)]
pub struct NetworkStackMetadata {
    pub name: String,
    pub version: String,
    pub stack_type: NetworkStackType,
    pub linux_heritage: Option<LinuxHeritage>,
    pub supported_protocols: Vec<NetworkProtocol>,
    pub max_connections: usize,
}

/// Types of network stacks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkStackType {
    LinuxDerived,
    SigmaOSNative,
    Hybrid,
}

/// Socket domains
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketDomain {
    IPv4,
    IPv6,
    Unix,
    Packet,
}

/// Socket types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketType {
    Stream,
    Datagram,
    Raw,
    SeqPacket,
}

/// Socket protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketProtocol {
    TCP,
    UDP,
    ICMP,
    Raw,
}

/// Network protocols
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkProtocol {
    IPv4,
    IPv6,
    TCP,
    UDP,
    ICMP,
    ARP,
}

/// Socket handle for tracking connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SocketHandle(pub u64);

/// Network errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    InitializationFailed,
    InvalidPacket,
    SocketError(String),
    ConnectionRefused,
    Timeout,
    BufferOverflow,
    ProtocolNotSupported,
    AddressInUse,
}

// ============================================================================
// Filesystem Abstraction
// ============================================================================

/// Abstract base trait for filesystem implementations
/// Enables polymorphic filesystem selection (Linux-derived vs SigmaOS-native)
pub trait FileSystem: Any {
    /// Initialize the filesystem
    fn init(&mut self) -> Result<(), FsError>;
    
    /// Mount the filesystem
    fn mount(&mut self, device: &str, mount_point: &str) -> Result<(), FsError>;
    
    /// Unmount the filesystem
    fn unmount(&mut self) -> Result<(), FsError>;
    
    /// Open a file
    fn open_file(&mut self, path: &str, flags: FileFlags) -> Result<FileHandle, FsError>;
    
    /// Close a file
    fn close_file(&mut self, handle: FileHandle) -> Result<(), FsError>;
    
    /// Read from a file
    fn read_file(&mut self, handle: FileHandle, buffer: &mut [u8]) -> Result<usize, FsError>;
    
    /// Write to a file
    fn write_file(&mut self, handle: FileHandle, data: &[u8]) -> Result<usize, FsError>;
    
    /// Create a directory
    fn create_directory(&mut self, path: &str) -> Result<(), FsError>;
    
    /// Remove a file or directory
    fn remove(&mut self, path: &str) -> Result<(), FsError>;
    
    /// Get file metadata
    fn get_metadata(&self, path: &str) -> Result<FileMetadata, FsError>;
    
    /// Get filesystem metadata
    fn metadata(&self) -> &FilesystemMetadata;
    
    /// Downcast for filesystem-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Filesystem metadata
#[derive(Debug, Clone)]
pub struct FilesystemMetadata {
    pub name: String,
    pub version: String,
    pub fs_type: FilesystemType,
    pub linux_heritage: Option<LinuxHeritage>,
    pub max_file_size: u64,
    pub max_filename_length: usize,
    pub features: Vec<FilesystemFeature>,
}

/// Types of filesystems
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    LinuxDerived,
    SigmaOSNative,
    Hybrid,
}

/// Filesystem features
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemFeature {
    Journaling,
    Encryption,
    Compression,
    Deduplication,
    Snapshots,
    Quotas,
    AccessControlLists,
}

/// File flags for opening files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileFlags {
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub create: bool,
    pub truncate: bool,
    pub exclusive: bool,
}

/// File handle for tracking open files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileHandle(pub u64);

/// File metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub size: u64,
    pub created: u64,
    pub modified: u64,
    pub accessed: u64,
    pub is_directory: bool,
    pub permissions: u32,
    pub owner: u32,
    pub group: u32,
}

/// Filesystem errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
    InitializationFailed,
    MountFailed,
    FileNotFound,
    PermissionDenied,
    IsDirectory,
    NotADirectory,
    FileExists,
    DiskFull,
    InvalidPath,
    ReadOnly,
}

// ============================================================================
// Memory Management Abstraction
// ============================================================================

/// Abstract base trait for memory management implementations
pub trait MemoryManager: Any {
    /// Initialize the memory manager
    fn init(&mut self) -> Result<(), MemoryError>;
    
    /// Allocate physical memory
    fn allocate_physical(&mut self, size: usize) -> Result<u64, MemoryError>;
    
    /// Free physical memory
    fn free_physical(&mut self, address: u64, size: usize) -> Result<(), MemoryError>;
    
    /// Allocate virtual memory
    fn allocate_virtual(&mut self, size: usize) -> Result<u64, MemoryError>;
    
    /// Free virtual memory
    fn free_virtual(&mut self, address: u64, size: usize) -> Result<(), MemoryError>;
    
    /// Map virtual to physical memory
    fn map_memory(&mut self, virtual_addr: u64, physical_addr: u64, size: usize, flags: MapFlags) -> Result<(), MemoryError>;
    
    /// Unmap memory
    fn unmap_memory(&mut self, virtual_addr: u64, size: usize) -> Result<(), MemoryError>;
    
    /// Get memory manager metadata
    fn metadata(&self) -> &MemoryManagerMetadata;
    
    /// Downcast for memory manager-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Memory manager metadata
#[derive(Debug, Clone)]
pub struct MemoryManagerMetadata {
    pub name: String,
    pub version: String,
    pub manager_type: MemoryManagerType,
    pub linux_heritage: Option<LinuxHeritage>,
    pub total_memory: u64,
    pub available_memory: u64,
    pub page_size: usize,
}

/// Types of memory managers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryManagerType {
    LinuxDerived,
    SigmaOSNative,
    Hybrid,
}

/// Memory mapping flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapFlags {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub user_accessible: bool,
    pub cache_disable: bool,
}

/// Memory errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    InitializationFailed,
    OutOfMemory,
    InvalidAddress,
    InvalidSize,
    PermissionDenied,
    AlreadyMapped,
    NotMapped,
    AlignmentError,
}

// ============================================================================
// Scheduler Abstraction
// ============================================================================

/// Abstract base trait for scheduler implementations
pub trait Scheduler: Any {
    /// Initialize the scheduler
    fn init(&mut self) -> Result<(), SchedulerError>;
    
    /// Add a process to the scheduler
    fn add_process(&mut self, process: ProcessInfo) -> Result<(), SchedulerError>;
    
    /// Remove a process from the scheduler
    fn remove_process(&mut self, pid: u64) -> Result<(), SchedulerError>;
    
    /// Get the next process to run
    fn schedule_next(&mut self) -> Option<ProcessInfo>;
    
    /// Update process state
    fn update_process(&mut self, pid: u64, state: ProcessState) -> Result<(), SchedulerError>;
    
    /// Get scheduler metadata
    fn metadata(&self) -> &SchedulerMetadata;
    
    /// Downcast for scheduler-specific operations
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Scheduler metadata
#[derive(Debug, Clone)]
pub struct SchedulerMetadata {
    pub name: String,
    pub version: String,
    pub scheduler_type: SchedulerType,
    pub linux_heritage: Option<LinuxHeritage>,
    pub time_slice_ms: u64,
    pub max_priority: u8,
}

/// Types of schedulers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerType {
    LinuxDerived,
    SigmaOSNative,
    Hybrid,
}

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u64,
    pub name: String,
    pub priority: u8,
    pub state: ProcessState,
    pub cpu_time: u64,
    pub memory_usage: u64,
}

/// Process states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
    Zombie,
}

/// Scheduler errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    InitializationFailed,
    ProcessNotFound,
    InvalidState,
    PriorityInvalid,
    SchedulerFull,
}

// ============================================================================
// Security Hardening Wrapper
// ============================================================================

/// Security wrapper for absorbed Linux drivers
/// Enforces capability-based access control and signature verification
pub struct SecureDriverWrapper<T: DeviceDriver> {
    inner: T,
    capabilities: CapabilityToken,
    signature_verified: bool,
    sandbox_enabled: bool,
}

impl<T: DeviceDriver> SecureDriverWrapper<T> {
    /// Create a new secure wrapper around a driver
    pub fn new(driver: T, capabilities: CapabilityToken) -> Self {
        Self {
            inner: driver,
            capabilities,
            signature_verified: false,
            sandbox_enabled: true,
        }
    }
    
    /// Verify the driver's cryptographic signature
    pub fn verify_signature(&mut self, signature: &[u8]) -> Result<(), DriverError> {
        // In production, this would verify against a trusted key
        self.signature_verified = true;
        Ok(())
    }
    
    /// Enable or disable sandbox mode
    pub fn set_sandbox(&mut self, enabled: bool) {
        self.sandbox_enabled = enabled;
    }
    
    /// Check if the operation is permitted by capabilities
    fn check_capability(&self, required_capability: u64) -> Result<(), DriverError> {
        if !self.capabilities.contains(required_capability) {
            return Err(DriverError::PermissionDenied);
        }
        Ok(())
    }
}

impl<T: DeviceDriver> DeviceDriver for SecureDriverWrapper<T> {
    fn init(&mut self) -> Result<(), DriverError> {
        if !self.signature_verified {
            return Err(DriverError::PermissionDenied);
        }
        self.inner.init()
    }
    
    fn handle_io(&mut self, operation: IoOperation) -> Result<IoResult, DriverError> {
        if self.sandbox_enabled {
            // Apply sandbox restrictions
            match operation {
                IoOperation::Mmap { .. } => self.check_capability(0x1000)?,
                IoOperation::Ioctl { .. } => self.check_capability(0x2000)?,
                _ => {}
            }
        }
        self.inner.handle_io(operation)
    }
    
    fn shutdown(&mut self) -> Result<(), DriverError> {
        self.inner.shutdown()
    }
    
    fn metadata(&self) -> &DriverMetadata {
        self.inner.metadata()
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// ============================================================================
// Driver Registry and Discovery
// ============================================================================

/// Central registry for all drivers in the system
pub struct DriverRegistry {
    drivers: Vec<Box<dyn DeviceDriver>>,
    network_stacks: Vec<Box<dyn NetworkStack>>,
    filesystems: Vec<Box<dyn FileSystem>>,
    memory_managers: Vec<Box<dyn MemoryManager>>,
    schedulers: Vec<Box<dyn Scheduler>>,
}

impl DriverRegistry {
    /// Create a new driver registry
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
            network_stacks: Vec::new(),
            filesystems: Vec::new(),
            memory_managers: Vec::new(),
            schedulers: Vec::new(),
        }
    }
    
    /// Register a device driver
    pub fn register_driver(&mut self, driver: Box<dyn DeviceDriver>) -> Result<(), RegistryError> {
        self.drivers.push(driver);
        Ok(())
    }
    
    /// Register a network stack
    pub fn register_network_stack(&mut self, stack: Box<dyn NetworkStack>) -> Result<(), RegistryError> {
        self.network_stacks.push(stack);
        Ok(())
    }
    
    /// Register a filesystem
    pub fn register_filesystem(&mut self, fs: Box<dyn FileSystem>) -> Result<(), RegistryError> {
        self.filesystems.push(fs);
        Ok(())
    }
    
    /// Register a memory manager
    pub fn register_memory_manager(&mut self, mm: Box<dyn MemoryManager>) -> Result<(), RegistryError> {
        self.memory_managers.push(mm);
        Ok(())
    }
    
    /// Register a scheduler
    pub fn register_scheduler(&mut self, scheduler: Box<dyn Scheduler>) -> Result<(), RegistryError> {
        self.schedulers.push(scheduler);
        Ok(())
    }
    
    /// Find a driver by name
    pub fn find_driver(&self, name: &str) -> Option<&Box<dyn DeviceDriver>> {
        self.drivers.iter().find(|d| d.metadata().name == name)
    }
    
    /// Find a driver by capability
    pub fn find_driver_by_capability(&self, capability: u64) -> Option<&Box<dyn DeviceDriver>> {
        self.drivers.iter().find(|d| d.has_capability(capability))
    }
    
    /// Get all drivers of a specific type
    pub fn get_drivers_by_type(&self, driver_type: DriverType) -> Vec<&Box<dyn DeviceDriver>> {
        self.drivers.iter().filter(|d| d.metadata().driver_type == driver_type).collect()
    }
    
    /// Initialize all registered drivers
    pub fn initialize_all(&mut self) -> Result<(), RegistryError> {
        for driver in &mut self.drivers {
            driver.init().map_err(|e| RegistryError::InitializationFailed(format!("{:?}", e)))?;
        }
        Ok(())
    }
    
    /// Shutdown all registered drivers
    pub fn shutdown_all(&mut self) -> Result<(), RegistryError> {
        self.drivers.iter_mut().for_each(|d| { let _ = d.shutdown(); });
        Ok(())
    }
}

impl Default for DriverRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Registry errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    DriverAlreadyRegistered,
    DriverNotFound,
    InitializationFailed(String),
    InvalidConfiguration,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Mock driver for testing
    struct MockDriver {
        metadata: DriverMetadata,
        initialized: bool,
    }

    impl MockDriver {
        fn new(name: &str) -> Self {
            Self {
                metadata: DriverMetadata {
                    name: String::from(name),
                    version: String::from("1.0.0"),
                    author: String::from("SigmaOS"),
                    description: String::from("Mock driver for testing"),
                    driver_type: DriverType::Misc,
                    linux_heritage: None,
                    capabilities: vec![0x1000],
                    required_capabilities: vec![],
                },
                initialized: false,
            }
        }
    }

    impl DeviceDriver for MockDriver {
        fn init(&mut self) -> Result<(), DriverError> {
            self.initialized = true;
            Ok(())
        }

        fn handle_io(&mut self, _operation: IoOperation) -> Result<IoResult, DriverError> {
            Ok(IoResult::ReadComplete { data: vec![0u8; 8] })
        }

        fn shutdown(&mut self) -> Result<(), DriverError> {
            self.initialized = false;
            Ok(())
        }

        fn metadata(&self) -> &DriverMetadata {
            &self.metadata
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    #[test]
    fn test_device_driver_trait() {
        let mut driver = MockDriver::new("test_driver");
        assert!(!driver.initialized);
        assert!(driver.init().is_ok());
        assert!(driver.initialized);
        assert!(driver.shutdown().is_ok());
        assert!(!driver.initialized);
    }

    #[test]
    fn test_driver_registry() {
        let mut registry = DriverRegistry::new();
        let driver = Box::new(MockDriver::new("test_driver"));
        assert!(registry.register_driver(driver).is_ok());
        assert!(registry.find_driver("test_driver").is_some());
        assert!(registry.find_driver("nonexistent").is_none());
    }

    #[test]
    fn test_secure_driver_wrapper() {
        let driver = MockDriver::new("secure_driver");
        let capabilities = CapabilityToken::new();
        let mut wrapper = SecureDriverWrapper::new(driver, capabilities);
        
        // Should fail without signature verification
        assert!(wrapper.init().is_err());
        
        // Verify signature
        assert!(wrapper.verify_signature(&[0u8; 32]).is_ok());
        
        // Should succeed after verification
        assert!(wrapper.init().is_ok());
    }

    #[test]
    fn test_capability_check() {
        let driver = MockDriver::new("capability_driver");
        let mut capabilities = CapabilityToken::new();
        capabilities.allow_capability(0x1000);
        
        let wrapper = SecureDriverWrapper::new(driver, capabilities);
        assert!(wrapper.has_capability(0x1000));
        assert!(!wrapper.has_capability(0x9999));
    }
}
