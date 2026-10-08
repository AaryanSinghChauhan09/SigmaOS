//! DRM (Direct Rendering Manager) and KMS (Kernel Mode Setting)
//! Modern graphics driver infrastructure inspired by Linux DRM subsystem
//! Reference: Linux drivers/gpu/drm/

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

/// DRM device types
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmDeviceType {
    Primary = 0,
    Control = 1,
    Render = 2,
}

/// Display connector types (from DRM)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmConnectorType {
    Unknown = 0,
    Vga = 1,
    DviI = 2,
    DviD = 3,
    DviA = 4,
    Composite = 5,
    SVideo = 6,
    Lvds = 7,
    Component = 8,
    Dp = 10,    // DisplayPort
    Hdmi = 11,  // HDMI Type A
    HdmiB = 12, // HDMI Type B
    Tv = 13,
    Edp = 14, // Embedded DisplayPort
    Virtual = 15,
    Dsi = 16, // DSI (Mobile)
    Dpi = 17, // DPI (Mobile)
    Writeback = 18,
    Spi = 19,
    UsbC = 20,
}

/// Connector status
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmConnectorStatus {
    Connected = 1,
    Disconnected = 2,
    Unknown = 3,
}

/// Display mode flags
#[derive(Debug, Clone, Copy)]
pub struct DrmModeFlags {
    pub interlace: bool,
    pub doublescan: bool,
    pub csync: bool,
    pub pvsync: bool, // Positive vsync
    pub nvsync: bool, // Negative vsync
    pub phsync: bool, // Positive hsync
    pub nhsync: bool, // Negative hsync
}

/// Display mode timing information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DrmModeInfo {
    pub clock: u32,       // Pixel clock in kHz
    pub hdisplay: u16,    // Horizontal display size
    pub hsync_start: u16, // Horizontal sync start
    pub hsync_end: u16,   // Horizontal sync end
    pub htotal: u16,      // Horizontal total size
    pub vdisplay: u16,    // Vertical display size
    pub vsync_start: u16, // Vertical sync start
    pub vsync_end: u16,   // Vertical sync end
    pub vtotal: u16,      // Vertical total size
    pub vrefresh: u16,    // Refresh rate in Hz
    pub flags: u32,       // Mode flags
    pub name: [u8; 32],   // Mode name
}

impl DrmModeInfo {
    /// Create standard 1920x1080@60Hz mode
    pub fn mode_1080p_60() -> Self {
        Self {
            clock: 148500,
            hdisplay: 1920,
            hsync_start: 2008,
            hsync_end: 2052,
            htotal: 2200,
            vdisplay: 1080,
            vsync_start: 1084,
            vsync_end: 1089,
            vtotal: 1125,
            vrefresh: 60,
            flags: 0x5, // +hsync +vsync
            name: *b"1920x1080\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        }
    }

    /// Create standard 1280x720@60Hz mode
    pub fn mode_720p_60() -> Self {
        Self {
            clock: 74250,
            hdisplay: 1280,
            hsync_start: 1390,
            hsync_end: 1430,
            htotal: 1650,
            vdisplay: 720,
            vsync_start: 725,
            vsync_end: 730,
            vtotal: 750,
            vrefresh: 60,
            flags: 0x5,
            name: *b"1280x720\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        }
    }
}

/// Framebuffer pixel format (FourCC codes)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmPixelFormat {
    Rgb565 = 0x36314752,   // 'RG16'
    Rgb888 = 0x34324752,   // 'RG24'
    Xrgb8888 = 0x34325258, // 'XR24'
    Argb8888 = 0x34325241, // 'AR24'
    Bgr565 = 0x36314742,   // 'BG16'
    Xbgr8888 = 0x34324258, // 'XB24'
    Abgr8888 = 0x34324241, // 'AB24'
}

impl DrmPixelFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            DrmPixelFormat::Rgb565 | DrmPixelFormat::Bgr565 => 2,
            DrmPixelFormat::Rgb888 => 3,
            DrmPixelFormat::Xrgb8888
            | DrmPixelFormat::Argb8888
            | DrmPixelFormat::Xbgr8888
            | DrmPixelFormat::Abgr8888 => 4,
        }
    }
}

/// DRM Framebuffer object
#[derive(Debug, Clone)]
pub struct DrmFramebuffer {
    pub fb_id: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32, // Bytes per scanline
    pub format: DrmPixelFormat,
    pub modifier: u64,   // Format modifier (tiling, compression)
    pub gem_handle: u32, // GEM buffer object handle
}

/// DRM CRTC (Cathode Ray Tube Controller - display controller)
pub struct DrmCrtc {
    pub crtc_id: u32,
    pub pipe: u32, // Hardware pipe index
    pub enabled: bool,
    pub mode: Option<DrmModeInfo>,
    pub fb_id: Option<u32>, // Current framebuffer
    pub x: i32,             // Panning offset X
    pub y: i32,             // Panning offset Y
}

impl DrmCrtc {
    pub fn new(crtc_id: u32, pipe: u32) -> Self {
        Self {
            crtc_id,
            pipe,
            enabled: false,
            mode: None,
            fb_id: None,
            x: 0,
            y: 0,
        }
    }

    /// Set display mode
    pub fn set_mode(&mut self, mode: DrmModeInfo, fb_id: u32) -> Result<(), DrmError> {
        self.mode = Some(mode);
        self.fb_id = Some(fb_id);
        self.enabled = true;
        Ok(())
    }

    /// Disable CRTC
    pub fn disable(&mut self) {
        self.enabled = false;
        self.mode = None;
        self.fb_id = None;
    }
}

/// DRM Encoder (converts CRTC output to connector signal)
pub struct DrmEncoder {
    pub encoder_id: u32,
    pub encoder_type: u32,
    pub possible_crtcs: u32,  // Bitmask of compatible CRTCs
    pub possible_clones: u32, // Bitmask of clone-able encoders
}

/// DRM Connector (physical display output)
pub struct DrmConnector {
    pub connector_id: u32,
    pub connector_type: DrmConnectorType,
    pub connector_type_id: u32,
    pub status: DrmConnectorStatus,
    pub modes: Vec<DrmModeInfo>,
    pub encoder_id: Option<u32>,
    pub dpms: DrmDpmsMode,
}

impl DrmConnector {
    pub fn new(connector_id: u32, connector_type: DrmConnectorType) -> Self {
        Self {
            connector_id,
            connector_type,
            connector_type_id: 0,
            status: DrmConnectorStatus::Unknown,
            modes: Vec::new(),
            encoder_id: None,
            dpms: DrmDpmsMode::Off,
        }
    }

    /// Detect connector and read EDID
    pub fn detect(&mut self) -> Result<DrmConnectorStatus, DrmError> {
        // Create DDC I2C bus for this connector
        let ddc = DdcI2cBus::new(self.connector_id);

        // Attempt to read EDID
        match ddc.read_edid() {
            Ok(edid) => {
                self.status = DrmConnectorStatus::Connected;

                // Extract modes from EDID
                let edid_modes = edid.extract_modes();
                self.modes.extend(edid_modes);

                // Add fallback standard modes if EDID modes are limited
                if self.modes.is_empty() {
                    self.modes.push(DrmModeInfo::mode_1080p_60());
                    self.modes.push(DrmModeInfo::mode_720p_60());
                }

                Ok(self.status)
            }
            Err(DrmError::InvalidEdid) => {
                // EDID read failed but connector may still be connected
                self.status = DrmConnectorStatus::Unknown;

                // Add standard modes as fallback
                self.modes.push(DrmModeInfo::mode_1080p_60());
                self.modes.push(DrmModeInfo::mode_720p_60());

                Ok(self.status)
            }
            Err(e) => Err(e),
        }
    }
}

/// DPMS (Display Power Management Signaling) modes
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmDpmsMode {
    On = 0,
    Standby = 1,
    Suspend = 2,
    Off = 3,
}

/// GEM (Graphics Execution Manager) buffer object
pub struct DrmGemObject {
    pub handle: u32,
    pub size: u64,
    pub phys_addr: u64, // Physical address (or GPU address)
    pub flags: u32,
    pub refcount: AtomicU32,
}

impl DrmGemObject {
    pub fn new(handle: u32, size: u64) -> Self {
        Self {
            handle,
            size,
            phys_addr: 0,
            flags: 0,
            refcount: AtomicU32::new(1),
        }
    }
}

/// DRM atomic commit (for mode setting)
pub struct DrmAtomicCommit {
    pub flags: u32,
    pub crtc_updates: Vec<(u32, Option<DrmModeInfo>)>,
    pub fb_updates: Vec<(u32, u32)>,
    pub connector_updates: Vec<(u32, u32)>,
}

/// Main DRM device structure
pub struct DrmDevice {
    pub dev_type: DrmDeviceType,
    pub vendor_id: u16,
    pub device_id: u16,
    pub crtcs: Vec<DrmCrtc>,
    pub encoders: Vec<DrmEncoder>,
    pub connectors: Vec<DrmConnector>,
    pub framebuffers: BTreeMap<u32, DrmFramebuffer>,
    pub gem_objects: BTreeMap<u32, DrmGemObject>,
    next_id: AtomicU32,
}

impl DrmDevice {
    pub fn new(dev_type: DrmDeviceType, vendor_id: u16, device_id: u16) -> Self {
        Self {
            dev_type,
            vendor_id,
            device_id,
            crtcs: Vec::new(),
            encoders: Vec::new(),
            connectors: Vec::new(),
            framebuffers: BTreeMap::new(),
            gem_objects: BTreeMap::new(),
            next_id: AtomicU32::new(1),
        }
    }

    /// Allocate new object ID
    fn alloc_id(&self) -> u32 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Initialize device and enumerate outputs
    pub fn init(&mut self) -> Result<(), DrmError> {
        // Create CRTCs (typically 2-4 per GPU)
        for pipe in 0..2 {
            let crtc = DrmCrtc::new(self.alloc_id(), pipe);
            self.crtcs.push(crtc);
        }

        // Create connectors
        let connector = DrmConnector::new(self.alloc_id(), DrmConnectorType::Hdmi);
        self.connectors.push(connector);

        // Detect connected displays
        for connector in &mut self.connectors {
            connector.detect()?;
        }

        Ok(())
    }

    /// Create GEM buffer object
    pub fn gem_create(&mut self, size: u64) -> Result<u32, DrmError> {
        let handle = self.alloc_id();
        let gem = DrmGemObject::new(handle, size);
        self.gem_objects.insert(handle, gem);
        Ok(handle)
    }

    /// Create framebuffer
    pub fn add_framebuffer(
        &mut self,
        width: u32,
        height: u32,
        format: DrmPixelFormat,
        gem_handle: u32,
    ) -> Result<u32, DrmError> {
        if !self.gem_objects.contains_key(&gem_handle) {
            return Err(DrmError::InvalidGemHandle);
        }

        let fb_id = self.alloc_id();
        let pitch = width * format.bytes_per_pixel();

        let fb = DrmFramebuffer {
            fb_id,
            width,
            height,
            pitch,
            format,
            modifier: 0,
            gem_handle,
        };

        self.framebuffers.insert(fb_id, fb);
        Ok(fb_id)
    }

    /// Set CRTC mode (mode setting)
    pub fn set_crtc(
        &mut self,
        crtc_id: u32,
        fb_id: u32,
        mode: DrmModeInfo,
    ) -> Result<(), DrmError> {
        let crtc = self
            .crtcs
            .iter_mut()
            .find(|c| c.crtc_id == crtc_id)
            .ok_or(DrmError::InvalidCrtc)?;

        if !self.framebuffers.contains_key(&fb_id) {
            return Err(DrmError::InvalidFramebuffer);
        }

        crtc.set_mode(mode, fb_id)?;

        // In real implementation: program hardware registers
        Ok(())
    }

    /// Page flip (atomic buffer swap)
    pub fn page_flip(&mut self, crtc_id: u32, fb_id: u32) -> Result<(), DrmError> {
        let crtc = self
            .crtcs
            .iter_mut()
            .find(|c| c.crtc_id == crtc_id)
            .ok_or(DrmError::InvalidCrtc)?;

        if !self.framebuffers.contains_key(&fb_id) {
            return Err(DrmError::InvalidFramebuffer);
        }

        crtc.fb_id = Some(fb_id);

        // In real implementation: wait for vblank and flip
        Ok(())
    }

    /// Get connector by ID
    pub fn get_connector(&self, connector_id: u32) -> Option<&DrmConnector> {
        self.connectors
            .iter()
            .find(|c| c.connector_id == connector_id)
    }
}

/// EDID (Extended Display Identification Data) structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdidHeader {
    pub magic: [u8; 8],     // "00 FF FF FF FF FF FF 00"
    pub manufacturer: [u8; 2], // Manufacturer ID (PNP ID)
    pub product_code: [u8; 2], // Product code
    pub serial_number: u32,    // Serial number
    pub manufacture_week: u8, // Week of manufacture
    pub manufacture_year: u8, // Year of manufacture (year - 1990)
    pub edid_version: u8,     // EDID version
    pub edid_revision: u8,    // EDID revision
}

/// EDID detailed timing descriptor
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdidTimingDescriptor {
    pub pixel_clock: u16,     // Pixel clock in 10kHz units
    pub h_active: u8,         // Horizontal active pixels
    pub h_blank: u8,          // Horizontal blanking
    pub h_active_hi: u8,      // High bits of h_active and h_blank
    pub h_sync_off: u8,       // Horizontal sync offset
    pub h_sync_width: u8,     // Horizontal sync pulse width
    pub h_sync_hi: u8,        // High bits of sync offset and width
    pub v_active: u8,         // Vertical active lines
    pub v_blank: u8,          // Vertical blanking
    pub v_active_hi: u8,      // High bits of v_active and v_blank
    pub v_sync_off: u8,       // Vertical sync offset
    pub v_sync_width: u8,     // Vertical sync pulse width
    pub v_sync_hi: u8,        // High bits of sync offset and width
    pub flags: u8,            // Misc flags
}

/// Full EDID structure (128 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, PartialEq)]
pub struct Edid {
    pub header: EdidHeader,
    pub established_timings: [u8; 3],
    pub standard_timings: [[u8; 2]; 8],
    pub detailed_timings: [EdidTimingDescriptor; 4],
    pub extension_flag: u8,
    pub checksum: u8,
}

impl Edid {
    /// Parse EDID from raw data
    pub fn from_bytes(data: &[u8; 128]) -> Result<Self, DrmError> {
        // Verify magic header
        if &data[0..8] != b"\x00\xFF\xFF\xFF\xFF\xFF\xFF\x00" {
            return Err(DrmError::InvalidEdid);
        }

        // Verify checksum
        let sum: u8 = data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        if sum != 0 {
            return Err(DrmError::InvalidEdidChecksum);
        }

        unsafe {
            let edid_ptr = data.as_ptr() as *const Edid;
            Ok((*edid_ptr).clone())
        }
    }

    /// Extract display modes from EDID
    pub fn extract_modes(&self) -> Vec<DrmModeInfo> {
        let mut modes = Vec::new();

        // Extract from detailed timing descriptors
        for timing in &self.detailed_timings {
            if timing.pixel_clock != 0 {
                let h_active = u16::from(timing.h_active) |
                    ((u16::from(timing.h_active_hi) & 0xF0) << 4);
                let h_blank = u16::from(timing.h_blank) |
                    ((u16::from(timing.h_active_hi) & 0x0F) << 8);
                let v_active = u16::from(timing.v_active) |
                    ((u16::from(timing.v_active_hi) & 0xF0) << 4);
                let v_blank = u16::from(timing.v_blank) |
                    ((u16::from(timing.v_active_hi) & 0x0F) << 8);

                let clock_khz = u32::from(timing.pixel_clock) * 10;
                let htotal = h_active + h_blank;
                let vtotal = v_active + v_blank;
                let vrefresh = ((clock_khz * 1000) / u32::from(htotal * vtotal)) as u16;

                modes.push(DrmModeInfo {
                    clock: clock_khz,
                    hdisplay: h_active as u16,
                    hsync_start: h_active as u16,
                    hsync_end: htotal as u16,
                    htotal: htotal as u16,
                    vdisplay: v_active as u16,
                    vsync_start: v_active as u16,
                    vsync_end: vtotal as u16,
                    vtotal: vtotal as u16,
                    vrefresh,
                    flags: 0x5,
                    name: [0; 32],
                });
            }
        }

        modes
    }
}

/// I2C/DDC interface for EDID reading
pub struct DdcI2cBus {
    pub bus_id: u32,
}

impl DdcI2cBus {
    pub fn new(bus_id: u32) -> Self {
        Self { bus_id }
    }

    /// Read EDID via I2C/DDC (typically from address 0x50)
    pub fn read_edid(&self) -> Result<Edid, DrmError> {
        // In real implementation: perform I2C transaction to read 128 bytes from 0x50
        // For now, return simulated EDID
        let mut edid_data = [0u8; 128];

        // Write valid EDID header
        edid_data[0..8].copy_from_slice(b"\x00\xFF\xFF\xFF\xFF\xFF\xFF\x00");

        // Write manufacturer ID (example: "DEL" for Dell)
        edid_data[8] = 0x10; // 'D'
        edid_data[9] = 0xAC; // 'E' + 'L' packed

        // Write EDID version 1.3
        edid_data[18] = 1;
        edid_data[19] = 3;

        // Add a 1920x1080@60Hz detailed timing
        let timing = EdidTimingDescriptor {
            pixel_clock: 14850, // 148.5 MHz in 10kHz units
            h_active: 208,
            h_blank: 48,
            h_active_hi: 0,
            h_sync_off: 32,
            h_sync_width: 5,
            h_sync_hi: 0,
            v_active: 81,
            v_blank: 3,
            v_active_hi: 0,
            v_sync_off: 6,
            v_sync_width: 5,
            v_sync_hi: 0,
            flags: 0x18,
        };

        unsafe {
            let timing_ptr = &timing as *const EdidTimingDescriptor as *const u8;
            let timing_bytes = core::slice::from_raw_parts(timing_ptr, core::mem::size_of::<EdidTimingDescriptor>());
            edid_data[54..54 + timing_bytes.len()].copy_from_slice(timing_bytes);
        }

        // Calculate and write checksum
        let sum: u8 = edid_data[0..127].iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        edid_data[127] = sum.wrapping_neg();

        Edid::from_bytes(&edid_data)
    }
}

/// DRM error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrmError {
    InvalidCrtc,
    InvalidEncoder,
    InvalidConnector,
    InvalidFramebuffer,
    InvalidGemHandle,
    InvalidMode,
    NoMemory,
    Busy,
    NotSupported,
    InvalidEdid,
    InvalidEdidChecksum,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drm_device_init() {
        let mut dev = DrmDevice::new(DrmDeviceType::Primary, 0x8086, 0x1916);
        assert!(dev.init().is_ok());
        assert!(dev.crtcs.len() > 0);
    }

    #[test]
    fn test_gem_create() {
        let mut dev = DrmDevice::new(DrmDeviceType::Primary, 0x8086, 0x1916);
        let handle = dev.gem_create(4096).unwrap();
        assert!(dev.gem_objects.contains_key(&handle));
    }

    #[test]
    fn test_mode_timing() {
        let mode = DrmModeInfo::mode_1080p_60();
        assert_eq!(mode.hdisplay, 1920);
        assert_eq!(mode.vdisplay, 1080);
        assert_eq!(mode.vrefresh, 60);
    }

    #[test]
    fn test_pixel_format_bpp() {
        assert_eq!(DrmPixelFormat::Rgb565.bytes_per_pixel(), 2);
        assert_eq!(DrmPixelFormat::Xrgb8888.bytes_per_pixel(), 4);
    }

    #[test]
    fn test_edid_parsing() {
        let mut edid_data = [0u8; 128];
        edid_data[0..8].copy_from_slice(b"\x00\xFF\xFF\xFF\xFF\xFF\xFF\x00");
        edid_data[18] = 1;
        edid_data[19] = 3;

        let sum: u8 = edid_data[0..127].iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        edid_data[127] = sum.wrapping_neg();

        let edid = Edid::from_bytes(&edid_data).unwrap();
        assert_eq!(edid.header.edid_version, 1);
        assert_eq!(edid.header.edid_revision, 3);
    }

    #[test]
    fn test_edid_invalid_magic() {
        let mut edid_data = [0u8; 128];
        edid_data[0..8].copy_from_slice(b"\x00\x00\x00\x00\x00\x00\x00\x00");

        let result = Edid::from_bytes(&edid_data);
        assert_eq!(result, Err(DrmError::InvalidEdid));
    }

    #[test]
    fn test_edid_invalid_checksum() {
        let mut edid_data = [0u8; 128];
        edid_data[0..8].copy_from_slice(b"\x00\xFF\xFF\xFF\xFF\xFF\xFF\x00");
        edid_data[127] = 0xFF; // Invalid checksum

        let result = Edid::from_bytes(&edid_data);
        assert_eq!(result, Err(DrmError::InvalidEdidChecksum));
    }

    #[test]
    fn test_ddc_read_edid() {
        let ddc = DdcI2cBus::new(0);
        let edid = ddc.read_edid().unwrap();
        assert_eq!(edid.header.edid_version, 1);
        assert_eq!(edid.header.edid_revision, 3);
    }

    #[test]
    fn test_edid_extract_modes() {
        let ddc = DdcI2cBus::new(0);
        let edid = ddc.read_edid().unwrap();
        let modes = edid.extract_modes();
        assert!(modes.len() > 0);
    }
}
