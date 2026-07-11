//! SigmaOS Common Types
//! Shared type definitions used across SigmaOS drivers and subsystems
//! Reduces code duplication and ensures type consistency

#![no_std]

/// SigmaOS unsigned 8-bit integer
pub type SigmaU8 = u8;

/// SigmaOS unsigned 16-bit integer
pub type SigmaU16 = u16;

/// SigmaOS unsigned 32-bit integer
pub type SigmaU32 = u32;

/// SigmaOS unsigned 64-bit integer
pub type SigmaU64 = u64;

/// SigmaOS signed 32-bit integer
pub type SigmaI32 = i32;

/// SigmaOS signed 64-bit integer
pub type SigmaI64 = i64;

/// SigmaOS 32-bit floating point
pub type SigmaF32 = f32;

/// SigmaOS 64-bit floating point
pub type SigmaF64 = f64;

/// SigmaOS boolean
pub type SigmaBool = bool;

/// SigmaOS usize
pub type SigmaUsize = usize;

/// SigmaOS isize
pub type SigmaIsize = isize;

// ─── Common Error Types ─────────────────────────────────────────────────────

/// Common result type for SigmaOS operations
pub type SigmaResult<T> = Result<T, SigmaError>;

/// Common error type for SigmaOS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigmaError {
    Ok = 0,
    Invalid = -1,
    NotFound = -2,
    PermissionDenied = -3,
    AlreadyExists = -4,
    NoMemory = -5,
    IoError = -6,
    Timeout = -7,
    NotSupported = -8,
    Busy = -9,
}

// ─── Common Traits ───────────────────────────────────────────────────────

/// Trait for devices that can be initialized
pub trait DeviceInit {
    /// Initialize the device
    fn init(&mut self) -> SigmaResult<()>;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
}

/// Trait for devices that can be enabled/disabled
pub trait DeviceControl {
    /// Enable the device
    fn enable(&mut self) -> SigmaResult<()>;
    
    /// Disable the device
    fn disable(&mut self) -> SigmaResult<()>;
    
    /// Reset the device
    fn reset(&mut self) -> SigmaResult<()>;
    
    /// Check if device is enabled
    fn is_enabled(&self) -> bool;
}

/// Trait for devices that can provide status information
pub trait DeviceStatus {
    /// Get device status
    fn get_status(&self) -> DeviceStatusInfo;
    
    /// Get device name
    fn get_name(&self) -> &'static str;
}

/// Device status information
#[derive(Debug, Clone, Copy)]
pub struct DeviceStatusInfo {
    pub initialized: bool,
    pub enabled: bool,
    pub error_code: SigmaError,
}

impl DeviceStatusInfo {
    pub const fn new() -> Self {
        Self {
            initialized: false,
            enabled: false,
            error_code: SigmaError::Ok,
        }
    }
}

/// Trait for devices that can be configured
pub trait DeviceConfig {
    /// Apply configuration
    fn configure(&mut self, config: &[u8]) -> SigmaResult<()>;
    
    /// Get current configuration
    fn get_config(&self, buffer: &mut [u8]) -> SigmaResult<usize>;
}

/// Trait for resources that can be allocated and freed
pub trait Resource {
    /// Allocate the resource
    fn allocate(&mut self) -> SigmaResult<()>;
    
    /// Free the resource
    fn free(&mut self) -> SigmaResult<()>;
    
    /// Check if resource is allocated
    fn is_allocated(&self) -> bool;
}

/// Trait for objects that can be serialized
pub trait Serializable {
    /// Serialize to buffer
    fn serialize(&self, buffer: &mut [u8]) -> SigmaResult<usize>;
    
    /// Get serialized size
    fn serialized_size(&self) -> usize;
}

/// Trait for objects that can be deserialized
pub trait Deserializable {
    /// Deserialize from buffer
    fn deserialize(&mut self, buffer: &[u8]) -> SigmaResult<usize>;
}

/// Trait for objects with unique identifiers
pub trait Identifiable {
    /// Get unique identifier
    fn get_id(&self) -> u64;
    
    /// Set unique identifier
    fn set_id(&mut self, id: u64);
}

/// Trait for objects that can be cloned
pub trait Cloneable {
    /// Clone the object
    fn clone(&self) -> Self;
}
