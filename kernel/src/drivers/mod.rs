pub mod vga;

/// Core trait representing a hardware driver in SigmaOS.
/// This OOP-based approach ensures a consistent interface across all devices.
pub trait Driver {
    /// Initialize the hardware device.
    fn init(&mut self) -> Result<(), &'static str>;

    /// Check the status of the device.
    fn status(&self) -> DriverStatus;
    
    /// Get the name of the driver for logging/registration.
    fn name(&self) -> &'static str;
}

#[derive(Debug, PartialEq, Eq)]
pub enum DriverStatus {
    Uninitialized,
    Ready,
    Error(&'static str),
}

/// A centralized registry to keep track of loaded drivers.
pub struct DriverRegistry {
    // In a real implementation with dynamic allocation, this would be a Vec<Box<dyn Driver>>.
    // For our no_std bare-metal environment, we'll keep it simple for now.
    pub vga: vga::VgaDriver,
}

impl DriverRegistry {
    pub fn new() -> Self {
        Self {
            vga: vga::VgaDriver::new(),
        }
    }

    pub fn init_all(&mut self) {
        // Initialize VGA Driver
        if let Err(e) = self.vga.init() {
            // If VGA fails, we can't really print an error to VGA.
            // In the future, this would log to a serial port.
            panic!("VGA Initialization failed: {}", e);
        }
    }
}
