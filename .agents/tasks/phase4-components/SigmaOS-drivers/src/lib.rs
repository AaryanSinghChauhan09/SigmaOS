//! SigmaOS hardware driver management.
//!
//! Detects hardware, recommends proprietary or open-source drivers, and
//! handles installation and switching.

#![warn(missing_docs)]

/// A detected hardware device.
#[derive(Debug, Clone)]
pub struct Device {
    /// PCI or USB id.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Currently active driver.
    pub active_driver: Option<String>,
}

/// A driver available for a device.
#[derive(Debug, Clone)]
pub struct Driver {
    /// Package name of the driver.
    pub package: String,
    /// Whether the driver is open source.
    pub open_source: bool,
    /// Whether the driver is currently recommended.
    pub recommended: bool,
}

/// Trait for the driver manager backend.
pub trait DriverManager {
    /// Detect all hardware needing driver attention.
    fn detect(&self) -> Result<Vec<Device>, DriverError>;
    /// List drivers available for a device.
    fn drivers(&self, device: &Device) -> Result<Vec<Driver>, DriverError>;
    /// Install a driver by package name.
    fn install(&mut self, package: &str) -> Result<(), DriverError>;
}

/// Errors returned by the driver manager.
#[derive(Debug)]
pub enum DriverError {
    /// The device was not found.
    DeviceNotFound,
    /// The driver package is not available.
    PackageNotFound(String),
    /// Installation failed.
    InstallFailed(String),
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::DeviceNotFound => write!(f, "device not found"),
            DriverError::PackageNotFound(pkg) => write!(f, "package not found: {pkg}"),
            DriverError::InstallFailed(msg) => write!(f, "install failed: {msg}"),
        }
    }
}

impl std::error::Error for DriverError {}
