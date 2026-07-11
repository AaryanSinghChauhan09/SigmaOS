pub mod vga;
pub mod bus;
pub mod nic;
pub mod traits;
#[path = "../../drivers/framebuffer.rs"]
pub mod framebuffer;

// Re-export the improved OOP traits
pub use traits::*;

// Legacy compatibility types
#[derive(Debug, PartialEq, Eq)]
pub enum DeviceClass {
    Network,
    Block,
    Character,
    Display,
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BusType {
    PCI,
    USB,
    Platform,
    None,
}

/// Legacy driver trait for backward compatibility
/// New code should use the traits from traits.rs
pub trait Driver {
    /// Initialize the hardware device.
    fn init(&mut self) -> Result<(), &'static str>;

    /// Check the status of the device.
    fn status(&self) -> DriverStatus;
    
    /// Get the name of the driver for logging/registration.
    fn name(&self) -> &'static str;

    /// Get the class of the device this driver controls.
    fn class(&self) -> DeviceClass {
        DeviceClass::Unknown
    }

    /// Get the bus type this driver is attached to.
    fn bus_type(&self) -> BusType {
        BusType::None
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DriverStatus {
    Uninitialized,
    Ready,
    Error(&'static str),
}

/// A centralized registry to keep track of loaded drivers.
/// Zero-allocation design with fixed-capacity storage
pub struct DriverRegistry {
    pub vga: vga::VgaDriver,
    // In a dynamic system, we would have arrays/vectors of generic drivers here.
}

impl DriverRegistry {
    pub fn new() -> Self {
        Self {
            vga: vga::VgaDriver::new(),
        }
    }

    /// The sovereign registration function that adds drivers to the central registry.
    /// In a fully dynamic OS, this would take a `Box<dyn Driver>`.
    pub fn sigma_register_driver(&mut self, _driver_name: &str) {
        // Placeholder for dynamic registration logic.
    }

    pub fn init_all(&mut self) {
        if let Err(e) = self.vga.init() {
            panic!("VGA Initialization failed: {}", e);
        }
    }
}
