// SigmaOS Legacy Firmware Bridge Grid (FirmwareBridgeGrid)
// Deploys unified boot grid parameters to support modern/ancient hardware booting seamlessly

pub struct BIOSBridgeGrid {
    pub interrupt_13h_supported: bool,
    pub legacy_mbr_offset: u32,
}

pub struct UEFIBridgeGrid {
    pub gop_fb_width: u32,
    pub gop_fb_height: u32,
}

pub struct CorebootBridgeGrid {
    pub lb_table_addr: u64,
}

pub enum FirmwareBridgeGrid {
    Bios(BIOSBridgeGrid),
    Uefi(UEFIBridgeGrid),
    Coreboot(CorebootBridgeGrid),
}

impl FirmwareBridgeGrid {
    pub fn is_graphic_output_ready(&self) -> bool {
        match self {
            FirmwareBridgeGrid::Bios(_) => false, // legacy BIOS uses VGA text/Vesa modes bar
            FirmwareBridgeGrid::Uefi(grid) => grid.gop_fb_width > 0 && grid.gop_fb_height > 0,
            FirmwareBridgeGrid::Coreboot(_) => true,
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_grid_uefi() {
        let uefi_grid = UEFIBridgeGrid {
            gop_fb_width: 1920,
            gop_fb_height: 1080,
        };
        let bridge = FirmwareBridgeGrid::Uefi(uefi_grid);
        assert!(bridge.is_graphic_output_ready());
    }

    #[test]
    fn test_bridge_grid_bios() {
        let bios_grid = BIOSBridgeGrid {
            interrupt_13h_supported: true,
            legacy_mbr_offset: 0x7C00,
        };
        let bridge = FirmwareBridgeGrid::Bios(bios_grid);
        assert!(!bridge.is_graphic_output_ready());
    }
}
