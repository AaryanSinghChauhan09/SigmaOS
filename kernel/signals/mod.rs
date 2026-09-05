// SPDX-License-Identifier: MIT
/// Signal Management Module
/// Handles POSIX signal delivery, handling, and masking

pub mod delivery;

pub use delivery::{CpuContext, SignalFrame, SignalDeliveryEngine};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signals_module_loads() {
        // Module loads successfully
        assert!(true);
    }
}
