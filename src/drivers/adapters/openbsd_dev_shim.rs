// OpenBSD Autoconf Device Matcher & wscons Input Shim for SigmaOS (`src/drivers/adapters/openbsd_dev_shim.rs`)

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct OpenBsdDeviceMatch {
    pub dev_name: String,
    pub match_score: i32,
}

pub struct SovereignOpenBsdDevShimEngine {
    pub registered_devices: Vec<OpenBsdDeviceMatch>,
}

impl SovereignOpenBsdDevShimEngine {
    pub fn new() -> Self {
        Self {
            registered_devices: Vec::new(),
        }
    }

    pub fn config_found(&mut self, name: &str, score: i32) {
        self.registered_devices.push(OpenBsdDeviceMatch {
            dev_name: name.to_string(),
            match_score: score,
        });
    }

    pub fn wscons_input_event(&self, event_type: u16, code: u16, value: i32) -> (u16, u16, i32) {
        (event_type, code, value)
    }
}

impl Default for SovereignOpenBsdDevShimEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openbsd_dev_shim() {
        let mut shim = SovereignOpenBsdDevShimEngine::new();
        shim.config_found("wsmouse0", 100);
        assert_eq!(shim.registered_devices.len(), 1);

        let evt = shim.wscons_input_event(1, 272, 1);
        assert_eq!(evt, (1, 272, 1));
    }
}
