//! # UEFI Graphics Output Protocol (GOP) & Framebuffer Handoff
//!
//! Production-grade UEFI GOP discovery, framebuffer mapping, and kernel handoff engine.
//! Inspired by Linux `drivers/firmware/efi/libstub/gop.c`, FreeBSD `sys/dev/efi/efifb.c`,
//! Redox OS `bootloader/src/uefi/output.rs`, and SerenityOS `Prekernel/UEFIPrekernel.cpp`.
//!
//! Handles EFI_GRAPHICS_OUTPUT_PROTOCOL pixel format negotiation, mode selection,
//! and linear framebuffer handoff into the sovereign kernel boot info record.

#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

// ============================================================================
// 1. UEFI GOP PIXEL FORMATS (from UEFI Spec 2.10 §12.9)
// ============================================================================

/// UEFI EFI_GRAPHICS_PIXEL_FORMAT enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EfiPixelFormat {
    PixelRedGreenBlueReserved8BitPerColor = 0, // BGR-X
    PixelBlueGreenRedReserved8BitPerColor = 1, // RGB-X (most common)
    PixelBitMask = 2,                           // Custom bitmask
    PixelBltOnly = 3,                           // No direct framebuffer access
    PixelFormatMax = 4,
}

/// EFI_PIXEL_BITMASK — used when PixelFormat = PixelBitMask
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EfiPixelBitmask {
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub reserved_mask: u32,
}

impl EfiPixelBitmask {
    /// Standard XRGB8888 bitmask (matches most real hardware)
    pub const fn xrgb8888() -> Self {
        Self {
            red_mask: 0x00FF0000,
            green_mask: 0x0000FF00,
            blue_mask: 0x000000FF,
            reserved_mask: 0xFF000000,
        }
    }
}

// ============================================================================
// 2. EFI_GRAPHICS_OUTPUT_MODE_INFORMATION
// ============================================================================

/// Mode information structure returned by QueryMode
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EfiGraphicsOutputModeInfo {
    pub version: u32,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: EfiPixelFormat,
    pub pixel_information: EfiPixelBitmask,
    pub pixels_per_scan_line: u32,
}

impl EfiGraphicsOutputModeInfo {
    pub fn new(width: u32, height: u32, format: EfiPixelFormat) -> Self {
        Self {
            version: 0,
            horizontal_resolution: width,
            vertical_resolution: height,
            pixel_format: format,
            pixel_information: EfiPixelBitmask::xrgb8888(),
            pixels_per_scan_line: width,
        }
    }

    pub fn framebuffer_size_bytes(&self) -> usize {
        (self.pixels_per_scan_line * self.vertical_resolution * 4) as usize
    }

    pub fn bits_per_pixel(&self) -> u8 {
        match self.pixel_format {
            EfiPixelFormat::PixelRedGreenBlueReserved8BitPerColor
            | EfiPixelFormat::PixelBlueGreenRedReserved8BitPerColor
            | EfiPixelFormat::PixelBitMask => 32,
            EfiPixelFormat::PixelBltOnly => 0,
            EfiPixelFormat::PixelFormatMax => 0,
        }
    }
}

// ============================================================================
// 3. SOVEREIGN GOP MODE REGISTRY
// ============================================================================

/// A single discovered display mode
#[derive(Debug, Clone)]
pub struct GopDisplayMode {
    pub mode_number: u32,
    pub info: EfiGraphicsOutputModeInfo,
}

/// GOP Mode Selection Result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GopModeSelectResult {
    Selected,
    Unavailable,
    NoPreferred,
}

/// Sovereign GOP Mode Registry — scans and ranks available display modes
pub struct SovereignGopRegistry {
    pub modes: Vec<GopDisplayMode>,
    pub selected_mode: Option<u32>,
}

impl SovereignGopRegistry {
    pub fn new() -> Self {
        Self {
            modes: Vec::new(),
            selected_mode: None,
        }
    }

    /// Register a mode discovered from UEFI QueryMode
    pub fn register_mode(&mut self, mode_number: u32, info: EfiGraphicsOutputModeInfo) {
        self.modes.push(GopDisplayMode { mode_number, info });
    }

    /// Select best display mode: prefer native 1920×1080, 2560×1440, 3840×2160
    /// Fallback: highest resolution available (inspired by Linux gop.c selection logic)
    pub fn select_best_mode(&mut self) -> GopModeSelectResult {
        let preferred = [
            (3840, 2160),
            (2560, 1440),
            (2560, 1080),
            (1920, 1080),
            (1280, 800),
            (1024, 768),
        ];

        for (pw, ph) in &preferred {
            if let Some(mode) = self.modes.iter().find(|m| {
                m.info.horizontal_resolution == *pw && m.info.vertical_resolution == *ph
            }) {
                self.selected_mode = Some(mode.mode_number);
                return GopModeSelectResult::Selected;
            }
        }

        // Fallback: pick highest resolution
        if let Some(best) = self.modes.iter().max_by_key(|m| {
            m.info.horizontal_resolution as u64 * m.info.vertical_resolution as u64
        }) {
            self.selected_mode = Some(best.mode_number);
            return GopModeSelectResult::Selected;
        }

        GopModeSelectResult::NoPreferred
    }

    pub fn selected_info(&self) -> Option<&EfiGraphicsOutputModeInfo> {
        self.selected_mode.and_then(|mn| {
            self.modes.iter().find(|m| m.mode_number == mn).map(|m| &m.info)
        })
    }
}

// ============================================================================
// 4. KERNEL FRAMEBUFFER HANDOFF RECORD
//    Inspired by Linux struct screen_info (include/uapi/linux/screen_info.h)
//    and Multiboot2 framebuffer tag
// ============================================================================

/// Screen orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramebufferRotation {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

/// Sovereign Kernel Framebuffer Handoff — passed from bootloader to kernel
#[derive(Debug)]
pub struct SovereignFramebufferHandoff {
    pub base_addr: u64,          // Physical base address of linear framebuffer
    pub size_bytes: u64,         // Total framebuffer size in bytes
    pub width: u32,              // Horizontal pixels
    pub height: u32,             // Vertical pixels
    pub stride_bytes: u32,       // Bytes per scanline (may be > width × bpp/8)
    pub bits_per_pixel: u8,      // Usually 32
    pub red_shift: u8,
    pub red_size: u8,
    pub green_shift: u8,
    pub green_size: u8,
    pub blue_shift: u8,
    pub blue_size: u8,
    pub pixel_format: EfiPixelFormat,
    pub rotation: FramebufferRotation,
    pub fb_ready: AtomicBool,
    pub frames_presented: AtomicU64,
}

impl Clone for SovereignFramebufferHandoff {
    fn clone(&self) -> Self {
        Self {
            base_addr: self.base_addr,
            size_bytes: self.size_bytes,
            width: self.width,
            height: self.height,
            stride_bytes: self.stride_bytes,
            bits_per_pixel: self.bits_per_pixel,
            red_shift: self.red_shift,
            red_size: self.red_size,
            green_shift: self.green_shift,
            green_size: self.green_size,
            blue_shift: self.blue_shift,
            blue_size: self.blue_size,
            pixel_format: self.pixel_format,
            rotation: self.rotation,
            fb_ready: AtomicBool::new(self.fb_ready.load(Ordering::SeqCst)),
            frames_presented: AtomicU64::new(self.frames_presented.load(Ordering::SeqCst)),
        }
    }
}

impl SovereignFramebufferHandoff {
    /// Build standard XRGB8888 handoff record
    pub fn new_xrgb8888(base_addr: u64, width: u32, height: u32) -> Self {
        let stride = width * 4;
        let size = (stride * height) as u64;
        Self {
            base_addr,
            size_bytes: size,
            width,
            height,
            stride_bytes: stride,
            bits_per_pixel: 32,
            red_shift: 16,
            red_size: 8,
            green_shift: 8,
            green_size: 8,
            blue_shift: 0,
            blue_size: 8,
            pixel_format: EfiPixelFormat::PixelBlueGreenRedReserved8BitPerColor,
            rotation: FramebufferRotation::Rotate0,
            fb_ready: AtomicBool::new(true),
            frames_presented: AtomicU64::new(0),
        }
    }

    /// Build from GOP mode info (called after SetMode in bootloader)
    pub fn from_gop_mode(base_addr: u64, info: &EfiGraphicsOutputModeInfo) -> Self {
        let stride = info.pixels_per_scan_line * 4;
        let size = (stride * info.vertical_resolution) as u64;
        let (rs, gs, bs) = match info.pixel_format {
            EfiPixelFormat::PixelRedGreenBlueReserved8BitPerColor => (0u8, 8u8, 16u8),
            _ => (16u8, 8u8, 0u8), // BGR-X default
        };
        Self {
            base_addr,
            size_bytes: size,
            width: info.horizontal_resolution,
            height: info.vertical_resolution,
            stride_bytes: stride,
            bits_per_pixel: info.bits_per_pixel(),
            red_shift: rs,
            red_size: 8,
            green_shift: gs,
            green_size: 8,
            blue_shift: bs,
            blue_size: 8,
            pixel_format: info.pixel_format,
            rotation: FramebufferRotation::Rotate0,
            fb_ready: AtomicBool::new(true),
            frames_presented: AtomicU64::new(0),
        }
    }

    pub fn pixel_offset(&self, x: u32, y: u32) -> usize {
        (y * self.stride_bytes + x * (self.bits_per_pixel as u32 / 8)) as usize
    }

    /// Encode an RGB colour into a pixel word for this framebuffer
    pub fn encode_color(&self, r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << self.red_shift)
            | ((g as u32) << self.green_shift)
            | ((b as u32) << self.blue_shift)
    }
}

// ============================================================================
// 5. UEFI MEMORY MAP (EFI_MEMORY_DESCRIPTOR)
//    Inspired by Linux arch/x86/platform/efi/efi.c
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EfiMemoryType {
    ReservedMemoryType = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootServicesCode = 3,
    BootServicesData = 4,
    RuntimeServicesCode = 5,
    RuntimeServicesData = 6,
    ConventionalMemory = 7,   // Usable RAM
    UnusableMemory = 8,
    AcpiReclaimMemory = 9,    // Reclaimable after ACPI tables parsed
    AcpiMemoryNvs = 10,
    MemoryMappedIo = 11,
    MemoryMappedIoPortSpace = 12,
    PalCode = 13,
    PersistentMemory = 14,
}

/// EFI Memory Descriptor (one entry in the EFI memory map)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EfiMemoryDescriptor {
    pub memory_type: u32,
    pub physical_start: u64,
    pub virtual_start: u64,
    pub number_of_pages: u64,  // 4KB pages
    pub attribute: u64,
}

impl EfiMemoryDescriptor {
    pub fn size_bytes(&self) -> u64 {
        self.number_of_pages * 4096
    }

    pub fn is_usable_ram(&self) -> bool {
        self.memory_type == EfiMemoryType::ConventionalMemory as u32
            || self.memory_type == EfiMemoryType::BootServicesCode as u32
            || self.memory_type == EfiMemoryType::BootServicesData as u32
    }
}

/// Sovereign UEFI Memory Map
#[derive(Debug, Clone, Default)]
pub struct SovereignUefiMemoryMap {
    pub descriptors: Vec<EfiMemoryDescriptor>,
    pub total_usable_bytes: u64,
    pub total_reserved_bytes: u64,
}

impl SovereignUefiMemoryMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_descriptor(&mut self, desc: EfiMemoryDescriptor) {
        let size = desc.size_bytes();
        if desc.is_usable_ram() {
            self.total_usable_bytes += size;
        } else {
            self.total_reserved_bytes += size;
        }
        self.descriptors.push(desc);
    }

    pub fn usable_regions(&self) -> impl Iterator<Item = &EfiMemoryDescriptor> {
        self.descriptors.iter().filter(|d| d.is_usable_ram())
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(any(test, feature = "standalone_test"))]
mod tests {
    use super::*;

    #[test]
    fn test_gop_mode_selection_prefers_fhd() {
        let mut registry = SovereignGopRegistry::new();
        registry.register_mode(0, EfiGraphicsOutputModeInfo::new(640, 480, EfiPixelFormat::PixelBlueGreenRedReserved8BitPerColor));
        registry.register_mode(1, EfiGraphicsOutputModeInfo::new(1920, 1080, EfiPixelFormat::PixelBlueGreenRedReserved8BitPerColor));
        registry.register_mode(2, EfiGraphicsOutputModeInfo::new(1280, 720, EfiPixelFormat::PixelBlueGreenRedReserved8BitPerColor));

        let result = registry.select_best_mode();
        assert_eq!(result, GopModeSelectResult::Selected);
        assert_eq!(registry.selected_mode, Some(1));
    }

    #[test]
    fn test_gop_mode_fallback_to_highest() {
        let mut registry = SovereignGopRegistry::new();
        registry.register_mode(0, EfiGraphicsOutputModeInfo::new(640, 480, EfiPixelFormat::PixelBlueGreenRedReserved8BitPerColor));
        registry.register_mode(1, EfiGraphicsOutputModeInfo::new(1366, 768, EfiPixelFormat::PixelBlueGreenRedReserved8BitPerColor));

        let result = registry.select_best_mode();
        assert_eq!(result, GopModeSelectResult::Selected);
        assert_eq!(registry.selected_mode, Some(1));
    }

    #[test]
    fn test_framebuffer_handoff_pixel_encoding() {
        let fb = SovereignFramebufferHandoff::new_xrgb8888(0xE000_0000, 1920, 1080);
        assert_eq!(fb.width, 1920);
        assert_eq!(fb.height, 1080);
        assert_eq!(fb.stride_bytes, 1920 * 4);
        assert_eq!(fb.size_bytes, 1920 * 1080 * 4);

        // Pure red: R=255, G=0, B=0 → should encode to 0x00FF0000 for XRGB8888
        let red_pixel = fb.encode_color(255, 0, 0);
        assert_eq!(red_pixel, 0x00FF_0000);

        // Pixel at (10, 5) → offset = 5*stride + 10*4
        let offset = fb.pixel_offset(10, 5);
        assert_eq!(offset, 5 * 1920 * 4 + 10 * 4);
    }

    #[test]
    fn test_uefi_memory_map_accounting() {
        let mut mmap = SovereignUefiMemoryMap::new();
        mmap.add_descriptor(EfiMemoryDescriptor {
            memory_type: EfiMemoryType::ConventionalMemory as u32,
            physical_start: 0x100000,
            virtual_start: 0,
            number_of_pages: 512,  // 512 × 4KB = 2MB
            attribute: 0xF,
        });
        mmap.add_descriptor(EfiMemoryDescriptor {
            memory_type: EfiMemoryType::MemoryMappedIo as u32,
            physical_start: 0xFEC00000,
            virtual_start: 0,
            number_of_pages: 1,
            attribute: 0,
        });

        assert_eq!(mmap.total_usable_bytes, 512 * 4096);
        assert_eq!(mmap.total_reserved_bytes, 4096);
        assert_eq!(mmap.usable_regions().count(), 1);
    }
}
