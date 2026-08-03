// SigmaOS Ancient Peripheral Simulation Shard (PeripheralSim)
// Encapsulates software-based simulation of obsolete devices (Floppy drives, Tape drives, CRT graphics screens)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimType {
    FloppyDisk,
    TapeDrive,
    CrtGraphics,
}

pub struct PeripheralSim {
    pub sim_type: SimType,
    pub is_enabled: bool,
    pub simulated_status_register: u8,
}

impl PeripheralSim {
    pub fn new(sim_type: SimType) -> Self {
        PeripheralSim {
            sim_type,
            is_enabled: false,
            simulated_status_register: 0x00,
        }
    }

    pub fn enable_simulation(&mut self) {
        self.is_enabled = true;
        match self.sim_type {
            SimType::FloppyDisk => {
                self.simulated_status_register = 0x80; // Floppy disk data register ready
            }
            SimType::TapeDrive => {
                self.simulated_status_register = 0x40; // Tape drive rewound and online
            }
            SimType::CrtGraphics => {
                self.simulated_status_register = 0x08; // CRT screen vertical retrace active (vblank)
            }
        }
    }

    pub fn disable_simulation(&mut self) {
        self.is_enabled = false;
        self.simulated_status_register = 0x00;
    }

    pub fn simulated_inb(&self, port_offset: u32) -> Result<u8, ()> {
        if !self.is_enabled {
            return Err(());
        }
        if port_offset == 0 {
            Ok(self.simulated_status_register)
        } else {
            Ok(0x00)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floppy_simulation() {
        let mut sim = PeripheralSim::new(SimType::FloppyDisk);
        assert!(!sim.is_enabled);
        assert!(sim.simulated_inb(0).is_err());

        sim.enable_simulation();
        assert!(sim.is_enabled);
        assert_eq!(sim.simulated_inb(0).unwrap(), 0x80);

        sim.disable_simulation();
        assert!(!sim.is_enabled);
    }

    #[test]
    fn test_crt_simulation() {
        let mut sim = PeripheralSim::new(SimType::CrtGraphics);
        sim.enable_simulation();
        assert_eq!(sim.simulated_inb(0).unwrap(), 0x08);
    }
}
