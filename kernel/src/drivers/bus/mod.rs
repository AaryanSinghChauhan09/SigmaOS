/// Scaffold for Bus subsystem (e.g., PCI, USB)
/// In the Linux equivalent, this handles device discovery and bus enumeration.

use super::{Driver, BusType};

pub mod pci;

pub trait BusController {
    /// Initialize the bus controller.
    fn init(&mut self) -> Result<(), &'static str>;

    /// Scan the bus for attached devices.
    fn scan(&self);
    
    /// Get the type of this bus.
    fn get_type(&self) -> BusType;
}

// Future implementation: PCIBus, USBHostController
