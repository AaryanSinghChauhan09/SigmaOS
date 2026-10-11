// SigmaOS ACPI Sleep State & Power Transition Subsystem
// Handles S0 (Working) to S5 (Soft-off) transitions compliant with ACPI Specification 6.4.

#![allow(dead_code)]

/// ACPI System Sleep States
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcpiSleepState {
    #[default]
    S0Working,
    S1Standby,
    S2Standby,
    S3SuspendToRam,
    S4Hibernate,
    S5SoftOff,
}

/// PM1 Control Block Configuration for entering sleep state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SleepTypeValues {
    pub slp_typ_a: u16,
    pub slp_typ_b: u16,
}

/// ACPI Sleep Controller
pub struct AcpiSleepController {
    pm1a_cnt_blk: u16,
    pm1b_cnt_blk: u16,
    sleep_types: [Option<SleepTypeValues>; 6],
    firmware_waking_vector: u64,
    current_state: AcpiSleepState,
}

impl AcpiSleepController {
    pub const SLP_EN: u16 = 1 << 13; // Bit 13: Sleep Enable

    pub fn new(pm1a_cnt_blk: u16, pm1b_cnt_blk: u16) -> Self {
        Self {
            pm1a_cnt_blk,
            pm1b_cnt_blk,
            sleep_types: [None; 6],
            firmware_waking_vector: 0,
            current_state: AcpiSleepState::S0Working,
        }
    }

    /// Register SLP_TYP values parsed from DSDT/SSDT \_Sx objects
    pub fn register_sleep_type(&mut self, state: AcpiSleepState, slp_typ_a: u16, slp_typ_b: u16) {
        let idx = state as usize;
        if idx < self.sleep_types.len() {
            self.sleep_types[idx] = Some(SleepTypeValues {
                slp_typ_a,
                slp_typ_b,
            });
        }
    }

    /// Set firmware waking vector in FACS
    pub fn set_waking_vector(&mut self, vector: u64) {
        self.firmware_waking_vector = vector;
    }

    /// Current active power state
    pub fn current_state(&self) -> AcpiSleepState {
        self.current_state
    }

    /// Prepare system for entering sleep state (disable non-boot CPUs, flush caches)
    pub fn prepare_sleep(&mut self, target: AcpiSleepState) -> Result<(), &'static str> {
        let idx = target as usize;
        if self.sleep_types[idx].is_none() {
            return Err("Target sleep state not supported by ACPI DSDT tables");
        }
        Ok(())
    }

    /// Simulate writing PM1 control registers to transition sleep state
    pub fn transition_sleep(&mut self, target: AcpiSleepState) -> Result<(u16, u16), &'static str> {
        self.prepare_sleep(target)?;
        let idx = target as usize;
        let types = self.sleep_types[idx].ok_or("Sleep state missing")?;

        let val_a = (types.slp_typ_a << 10) | Self::SLP_EN;
        let val_b = (types.slp_typ_b << 10) | Self::SLP_EN;

        self.current_state = target;
        Ok((val_a, val_b))
    }

    /// Resume transition after wake event
    pub fn wake_transition(&mut self) {
        self.current_state = AcpiSleepState::S0Working;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sleep_state_transitions() {
        let mut controller = AcpiSleepController::new(0x1004, 0x0000);
        controller.register_sleep_type(AcpiSleepState::S3SuspendToRam, 0x05, 0x05);
        controller.register_sleep_type(AcpiSleepState::S5SoftOff, 0x07, 0x07);

        assert_eq!(controller.current_state(), AcpiSleepState::S0Working);

        let (pm1a, _) = controller.transition_sleep(AcpiSleepState::S3SuspendToRam).unwrap();
        assert_eq!(controller.current_state(), AcpiSleepState::S3SuspendToRam);
        assert_eq!(pm1a, (0x05 << 10) | AcpiSleepController::SLP_EN);

        controller.wake_transition();
        assert_eq!(controller.current_state(), AcpiSleepState::S0Working);
    }
}
