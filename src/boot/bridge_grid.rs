#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

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

#[cfg(test)]
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
