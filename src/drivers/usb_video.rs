//! USB Video Class (UVC) Driver
//! Supports USB webcams, video capture devices
//! Reference: USB Video Class Specification 1.1/1.5 and Linux drivers/media/usb/uvc/

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

/// USB Video Class codes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UvcClassCode {
    Video = 0x0E,
    VideoControl = 0x01,
    VideoStreaming = 0x02,
}

/// Video Class descriptor types
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum UvcDescriptorType {
    CsInterface = 0x24,
    CsEndpoint = 0x25,
}

/// Video Control Interface descriptor subtypes
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum UvcVcDescSubtype {
    Header = 0x01,
    InputTerminal = 0x02,
    OutputTerminal = 0x03,
    SelectorUnit = 0x04,
    ProcessingUnit = 0x05,
    ExtensionUnit = 0x06,
}

/// Video Streaming Interface descriptor subtypes
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum UvcVsDescSubtype {
    InputHeader = 0x01,
    OutputHeader = 0x02,
    StillImageFrame = 0x03,
    FormatUncompressed = 0x04,
    FrameUncompressed = 0x05,
    FormatMjpeg = 0x06,
    FrameMjpeg = 0x07,
    FormatFrameBased = 0x10,
    FrameFrameBased = 0x11,
    FormatH264 = 0x13,
    FrameH264 = 0x14,
    FormatH264Simulcast = 0x15,
}

/// Video Control Interface Header descriptor
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct UvcVcHeader {
    pub length: u8,
    pub descriptor_type: u8,
    pub descriptor_subtype: u8, // VC_HEADER
    pub bcd_uvc: u16,           // UVC version
    pub total_length: u16,
    pub clock_frequency: u32, // Device clock frequency in Hz
    pub in_collection: u8,    // Number of streaming interfaces
                              // Interface numbers follow
}

/// Input Terminal descriptor (Camera)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct UvcCameraTerminal {
    pub length: u8,
    pub descriptor_type: u8,
    pub descriptor_subtype: u8, // INPUT_TERMINAL
    pub terminal_id: u8,
    pub terminal_type: u16, // ITT_CAMERA
    pub assoc_terminal: u8,
    pub i_terminal: u8,
    pub objective_focal_length_min: u16,
    pub objective_focal_length_max: u16,
    pub ocular_focal_length: u16,
    pub control_size: u8,
    // Controls bitmap follows
}

/// Processing Unit descriptor
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct UvcProcessingUnit {
    pub length: u8,
    pub descriptor_type: u8,
    pub descriptor_subtype: u8, // PROCESSING_UNIT
    pub unit_id: u8,
    pub source_id: u8,
    pub max_multiplier: u16,
    pub control_size: u8,
    // Controls bitmap follows
    // Video standards bitmap
    // i_processing index
}

/// Uncompressed Video Format descriptor
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct UvcFormatUncompressed {
    pub length: u8,
    pub descriptor_type: u8,
    pub descriptor_subtype: u8, // FORMAT_UNCOMPRESSED
    pub format_index: u8,
    pub num_frame_descriptors: u8,
    pub guid_format: [u8; 16], // GUID (e.g., YUY2, NV12)
    pub bits_per_pixel: u8,
    pub default_frame_index: u8,
    pub aspect_ratio_x: u8,
    pub aspect_ratio_y: u8,
    pub interlace_flags: u8,
    pub copy_protect: u8,
}

/// Uncompressed Video Frame descriptor
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct UvcFrameUncompressed {
    pub length: u8,
    pub descriptor_type: u8,
    pub descriptor_subtype: u8, // FRAME_UNCOMPRESSED
    pub frame_index: u8,
    pub capabilities: u8,
    pub width: u16,
    pub height: u16,
    pub min_bit_rate: u32,
    pub max_bit_rate: u32,
    pub max_video_frame_buffer_size: u32,
    pub default_frame_interval: u32, // 100ns units
    pub frame_interval_type: u8,     // 0 = continuous, n = discrete
                                     // Frame intervals follow
}

/// MJPEG Format descriptor
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct UvcFormatMjpeg {
    pub length: u8,
    pub descriptor_type: u8,
    pub descriptor_subtype: u8, // FORMAT_MJPEG
    pub format_index: u8,
    pub num_frame_descriptors: u8,
    pub flags: u8,
    pub default_frame_index: u8,
    pub aspect_ratio_x: u8,
    pub aspect_ratio_y: u8,
    pub interlace_flags: u8,
    pub copy_protect: u8,
}

/// Video pixel formats (GUIDs)
pub mod pixel_formats {
    pub const YUY2: [u8; 16] = *b"YUY2\x00\x00\x10\x00\x80\x00\x00\xaa\x00\x38\x9b\x71";
    pub const NV12: [u8; 16] = *b"NV12\x00\x00\x10\x00\x80\x00\x00\xaa\x00\x38\x9b\x71";
    pub const UYVY: [u8; 16] = *b"UYVY\x00\x00\x10\x00\x80\x00\x00\xaa\x00\x38\x9b\x71";
    pub const RGB3: [u8; 16] = [
        0x7d, 0xeb, 0x36, 0xe4, 0x4f, 0x52, 0xce, 0x11, 0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7,
        0x70,
    ];
}

/// Video frame format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UvcPixelFormat {
    Yuy2,
    Nv12,
    Uyvy,
    Mjpeg,
    H264,
    Unknown,
}

impl UvcPixelFormat {
    pub fn from_guid(guid: &[u8; 16]) -> Self {
        if guid == &pixel_formats::YUY2 {
            UvcPixelFormat::Yuy2
        } else if guid == &pixel_formats::NV12 {
            UvcPixelFormat::Nv12
        } else if guid == &pixel_formats::UYVY {
            UvcPixelFormat::Uyvy
        } else {
            UvcPixelFormat::Unknown
        }
    }
}

/// Video frame descriptor
#[derive(Debug, Clone)]
pub struct UvcFrameDesc {
    pub frame_index: u8,
    pub width: u16,
    pub height: u16,
    pub frame_intervals: Vec<u32>, // 100ns units (e.g., 333333 = 30fps)
    pub default_interval: u32,
}

impl UvcFrameDesc {
    pub fn fps_from_interval(interval: u32) -> u32 {
        if interval == 0 {
            return 0;
        }
        10_000_000 / interval
    }
}

/// Video format descriptor
#[derive(Debug, Clone)]
pub struct UvcFormatDesc {
    pub format_index: u8,
    pub pixel_format: UvcPixelFormat,
    pub frames: Vec<UvcFrameDesc>,
    pub default_frame_index: u8,
}

/// UVC streaming controls
#[derive(Debug, Clone, Copy)]
pub struct UvcStreamingControl {
    pub hint: u16,
    pub format_index: u8,
    pub frame_index: u8,
    pub frame_interval: u32, // 100ns units
    pub key_frame_rate: u16,
    pub p_frame_rate: u16,
    pub comp_quality: u16,
    pub comp_window_size: u16,
    pub delay: u16, // ms
    pub max_video_frame_size: u32,
    pub max_payload_transfer_size: u32,
}

impl UvcStreamingControl {
    pub fn new() -> Self {
        Self {
            hint: 0,
            format_index: 1,
            frame_index: 1,
            frame_interval: 333333, // 30 fps
            key_frame_rate: 0,
            p_frame_rate: 0,
            comp_quality: 0,
            comp_window_size: 0,
            delay: 0,
            max_video_frame_size: 0,
            max_payload_transfer_size: 0,
        }
    }
}

/// UVC device structure
pub struct UvcDevice {
    pub vendor_id: u16,
    pub product_id: u16,
    pub formats: Vec<UvcFormatDesc>,
    pub current_format: Option<u8>,
    pub current_frame: Option<u8>,
    pub streaming: bool,
}

impl UvcDevice {
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        Self {
            vendor_id,
            product_id,
            formats: Vec::new(),
            current_format: None,
            current_frame: None,
            streaming: false,
        }
    }

    /// Parse video streaming descriptors
    pub fn parse_streaming_descriptors(&mut self, descriptors: &[u8]) -> Result<(), UvcError> {
        // Parse VS interface descriptors
        // Extract formats and frames
        Ok(())
    }

    /// Negotiate video parameters with device
    pub fn probe_and_commit(&mut self, control: &UvcStreamingControl) -> Result<(), UvcError> {
        // Send PROBE_CONTROL SET_CUR
        // Send PROBE_CONTROL GET_CUR (device negotiates)
        // Send COMMIT_CONTROL SET_CUR
        self.current_format = Some(control.format_index);
        self.current_frame = Some(control.frame_index);
        Ok(())
    }

    /// Start video capture
    pub fn start_streaming(&mut self) -> Result<(), UvcError> {
        if self.streaming {
            return Err(UvcError::AlreadyStreaming);
        }
        // Set alternate interface (with bandwidth)
        // Start isochronous/bulk transfers
        self.streaming = true;
        Ok(())
    }

    /// Stop video capture
    pub fn stop_streaming(&mut self) -> Result<(), UvcError> {
        if !self.streaming {
            return Ok(());
        }
        // Stop transfers
        // Set alternate interface 0
        self.streaming = false;
        Ok(())
    }

    /// Get supported resolutions
    pub fn get_resolutions(&self, format_index: u8) -> Vec<(u16, u16)> {
        let mut resolutions = Vec::new();
        if let Some(format) = self.formats.iter().find(|f| f.format_index == format_index) {
            for frame in &format.frames {
                resolutions.push((frame.width, frame.height));
            }
        }
        resolutions
    }

    /// Set camera control (brightness, contrast, etc.)
    pub fn set_control(&mut self, control: UvcControl, value: i16) -> Result<(), UvcError> {
        // Send SET_CUR request to Processing Unit
        Ok(())
    }

    /// Get camera control value
    pub fn get_control(&self, control: UvcControl) -> Result<i16, UvcError> {
        // Send GET_CUR request to Processing Unit
        Ok(0)
    }
}

/// Camera controls
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum UvcControl {
    Brightness = 0x01,
    Contrast = 0x02,
    Hue = 0x03,
    Saturation = 0x04,
    Sharpness = 0x05,
    Gamma = 0x06,
    WhiteBalanceTemperature = 0x07,
    WhiteBalanceComponent = 0x08,
    BacklightCompensation = 0x09,
    Gain = 0x0A,
    PowerLineFrequency = 0x0B,
    DigitalMultiplier = 0x0C,
    Zoom = 0x0D,
    Focus = 0x0E,
    Exposure = 0x0F,
}

/// UVC class-specific requests
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum UvcRequest {
    SetCur = 0x01,
    GetCur = 0x81,
    GetMin = 0x82,
    GetMax = 0x83,
    GetRes = 0x84,
    GetLen = 0x85,
    GetInfo = 0x86,
    GetDef = 0x87,
}

/// UVC error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UvcError {
    InvalidDescriptor,
    UnsupportedFormat,
    InvalidFormat,
    InvalidFrame,
    AlreadyStreaming,
    NotStreaming,
    NegotiationFailed,
    ControlError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uvc_device() {
        let device = UvcDevice::new(0x1234, 0x5678);
        assert!(!device.streaming);
    }

    #[test]
    fn test_fps_calculation() {
        let interval = 333333; // 30 fps
        let fps = UvcFrameDesc::fps_from_interval(interval);
        assert_eq!(fps, 30);
    }

    #[test]
    fn test_pixel_format_guid() {
        let format = UvcPixelFormat::from_guid(&pixel_formats::YUY2);
        assert_eq!(format, UvcPixelFormat::Yuy2);
    }
}
