// SPDX-License-Identifier: MIT
// SigmaOS Peripheral Archive Grid (PeripheralArchiveGrid)
// Provides simulated grid layouts for legacy hardware components with absolute zero overhead

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridSlotType {
    FloppyDisk,
    TapeDrive,
    CrtDisplay,
    DotMatrixPrinter,
}

#[derive(Debug, Clone)]
pub struct PeripheralArchiveGrid {
    pub slot_type: GridSlotType,
    pub sector_capacity: u32,
    pub tape_reel_feet: u32,
}

impl PeripheralArchiveGrid {
    pub fn new(slot: GridSlotType) -> Self {
        match slot {
            GridSlotType::FloppyDisk => PeripheralArchiveGrid {
                slot_type: slot,
                sector_capacity: 2880,
                tape_reel_feet: 0,
            },
            GridSlotType::TapeDrive => PeripheralArchiveGrid {
                slot_type: slot,
                sector_capacity: 0,
                tape_reel_feet: 1200,
            },
            GridSlotType::CrtDisplay => PeripheralArchiveGrid {
                slot_type: slot,
                sector_capacity: 64000,
                tape_reel_feet: 0,
            },
            GridSlotType::DotMatrixPrinter => PeripheralArchiveGrid {
                slot_type: slot,
                sector_capacity: 0,
                tape_reel_feet: 0,
            },
        }
    }

    pub fn query_capacity(&self) -> u32 {
        self.sector_capacity + self.tape_reel_feet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_grid_floppy() {
        let grid = PeripheralArchiveGrid::new(GridSlotType::FloppyDisk);
        assert_eq!(grid.query_capacity(), 2880);
    }

    #[test]
    fn test_archive_grid_tape() {
        let grid = PeripheralArchiveGrid::new(GridSlotType::TapeDrive);
        assert_eq!(grid.query_capacity(), 1200);
    }
}
