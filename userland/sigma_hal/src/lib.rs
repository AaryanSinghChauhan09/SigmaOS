pub mod device;

pub use device::{DeviceManager, Uevent, DeviceNode};

/// SigmaHAL: Hardware Abstraction Layer
/// Displaces `udev` to manage hardware events and device nodes dynamically.
pub struct SigmaHal {
    pub manager: DeviceManager,
}

impl Default for SigmaHal {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaHal {
    pub fn new() -> Self {
        Self {
            manager: DeviceManager::new(),
        }
    }
}
