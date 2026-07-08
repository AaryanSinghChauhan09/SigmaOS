// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/camera/camera_device_base.rs — Base Device Trait for Camera Drivers
//
// Defines the OOP base class for all camera devices using Rust traits.
// This provides a common interface for camera operations with V4L2 compatibility.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Camera Error Codes ─────────────────────────────────────────────────

pub const CAMERA_OK: I32 = 0;
pub const CAMERA_ERR_NO_DEVICE: I32 = -1;
pub const CAMERA_ERR_INIT_FAILED: I32 = -2;
pub const CAMERA_ERR_OUT_OF_MEM: I32 = -3;
pub const CAMERA_ERR_NOT_SUPPORTED: I32 = -4;
pub const CAMERA_ERR_IO: I32 = -5;
pub const CAMERA_ERR_INVALID_PARAM: I32 = -6;

// ─── Camera Type ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraType {
    Webcam,
    USB,
    MIPI,
    ISP,
    Unknown,
}

// ─── Pixel Format ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PixelFormat {
    RGB332,
    RGB444,
    RGB555,
    RGB565,
    RGB565X,
    RGB555X,
    BGR24,
    RGB24,
    BGR32,
    RGB32,
    GREY,
    Y16,
    PAL8,
    YVU410,
    YVU420,
    YUYV,
    UYVY,
    YUV422P,
    YUV411P,
    Y41P,
    YUV444,
    YUV555,
    YUV565,
    YUV32,
    NV12,
    NV21,
    NV16,
    NV61,
    YUV410,
    YUV420,
    YUV422,
    MJPEG,
    JPEG,
}

// ─── Camera Format ─────────────────────────────────────

#[repr(C)]
pub struct CameraFormat {
    pub width: U32,
    pub height: U32,
    pub pixel_format: PixelFormat,
    pub field: Field,
    pub bytesperline: U32,
    pub sizeimage: U32,
    pub colorspace: Colorspace,
}

impl CameraFormat {
    pub const fn new() -> Self {
        CameraFormat {
            width: 640,
            height: 480,
            pixel_format: PixelFormat::YUYV,
            field: Field::None,
            bytesperline: 0,
            sizeimage: 0,
            colorspace: Colorspace::SRGB,
        }
    }
}

// ─── Field ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Field {
    None,
    Top,
    Bottom,
    Interlaced,
    SequentialTop,
    SequentialBottom,
    Alternate,
}

// ─── Colorspace ─────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Colorspace {
    Default,
    SMPTE170M,
    SMPTE240M,
    REC709,
    BT878,
    470SystemM,
    470SystemBG,
    JPEG,
    SRGB,
    OPRGB,
    BT2020,
}

// ─── Camera Capability ─────────────────────────────

#[repr(C)]
pub struct CameraCapability {
    pub driver: [U8; 32],
    pub card: [U8; 32],
    pub bus_info: [U8; 32],
    pub version: U32,
    pub capabilities: U32,
    pub device_caps: U32,
}

impl CameraCapability {
    pub const fn new() -> Self {
        CameraCapability {
            driver: [0; 32],
            card: [0; 32],
            bus_info: [0; 32],
            version: 0,
            capabilities: 0,
            device_caps: 0,
        }
    }
}

// ─── Camera Buffer ─────────────────────────────────

#[repr(C)]
pub struct CameraBuffer {
    pub index: U32,
    pub type_: BufferType,
    pub bytesused: U32,
    pub flags: U32,
    pub field: Field,
    pub timestamp: U64,
    pub sequence: U32,
    pub memory: MemoryType,
    pub offset: U32,
    pub length: U32,
    pub userptr: U64,
    pub planes: [U32; 8],
}

impl CameraBuffer {
    pub const fn new() -> Self {
        CameraBuffer {
            index: 0,
            type_: BufferType::VideoCapture,
            bytesused: 0,
            flags: 0,
            field: Field::None,
            timestamp: 0,
            sequence: 0,
            memory: MemoryType::Mmap,
            offset: 0,
            length: 0,
            userptr: 0,
            planes: [0; 8],
        }
    }
}

// ─── Buffer Type ─────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BufferType {
    VideoCapture,
    VideoOutput,
    VideoOverlay,
    VbiCapture,
    VbiOutput,
    SlicedVbiCapture,
    SlicedVbiOutput,
    VideoCaptureMplane,
    VideoOutputMplane,
}

// ─── Memory Type ─────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryType {
    Mmap,
    UserPtr,
    DmaBuf,
    Overlay,
}

// ─── Camera Device Trait ─────────────────────────────

/// Trait for camera device operations
pub trait CameraDevice {
    /// Initialize the camera device
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get device name
    fn get_device_name(&self) -> &'static str;
    
    /// Get camera type
    fn get_camera_type(&self) -> CameraType;
    
    /// Get capabilities
    fn get_capabilities(&self, caps: *mut CameraCapability) -> I32;
    
    /// Query supported formats
    fn query_formats(&self, index: U32, format: *mut PixelFormat) -> I32;
    
    /// Get current format
    fn get_format(&self, format: *mut CameraFormat) -> I32;
    
    /// Set format
    fn set_format(&mut self, format: *const CameraFormat) -> I32;
    
    /// Try format
    fn try_format(&mut self, format: *mut CameraFormat) -> I32;
    
    /// Request buffers
    fn request_buffers(&mut self, count: U32, memory: MemoryType) -> I32;
    
    /// Query buffer
    fn query_buffer(&self, buffer: *mut CameraBuffer) -> I32;
    
    /// Queue buffer
    fn queue_buffer(&mut self, buffer: *mut CameraBuffer) -> I32;
    
    /// Dequeue buffer
    fn dequeue_buffer(&mut self, buffer: *mut CameraBuffer) -> I32;
    
    /// Stream on
    fn stream_on(&mut self, type_: BufferType) -> I32;
    
    /// Stream off
    fn stream_off(&mut self, type_: BufferType) -> I32;
    
    /// Get control
    fn get_control(&self, id: U32, value: *mut I32) -> I32;
    
    /// Set control
    fn set_control(&mut self, id: U32, value: I32) -> I32;
    
    /// Reset the device
    fn reset(&mut self) -> I32;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> I32;
}

// ─── Camera Control IDs ─────────────────────────────

pub const V4L2_CID_BRIGHTNESS: U32 = 0x00980900;
pub const V4L2_CID_CONTRAST: U32 = 0x00980901;
pub const V4L2_CID_SATURATION: U32 = 0x00980902;
pub const V4L2_CID_HUE: U32 = 0x00980903;
pub const V4L2_CID_AUTO_WHITE_BALANCE: U32 = 0x0098090C;
pub const V4L2_CID_DO_WHITE_BALANCE: U32 = 0x0098090D;
pub const V4L2_CID_RED_BALANCE: U32 = 0x0098090E;
pub const V4L2_CID_BLUE_BALANCE: U32 = 0x0098090F;
pub const V4L2_CID_GAMMA: U32 = 0x00980910;
pub const V4L2_CID_EXPOSURE: U32 = 0x00980911;
pub const V4L2_CID_AUTOGAIN: U32 = 0x00980913;
pub const V4L2_CID_GAIN: U32 = 0x00980914;
pub const V4L2_CID_HFLIP: U32 = 0x00980918;
pub const V4L2_CID_VFLIP: U32 = 0x00980919;
pub const V4L2_CID_POWER_LINE_FREQUENCY: U32 = 0x0098091A;
pub const V4L2_CID_HUE_AUTO: U32 = 0x0098091B;
pub const V4L2_CID_WHITE_BALANCE_TEMPERATURE: U32 = 0x0098091C;
pub const V4L2_CID_SHARPNESS: U32 = 0x0098091D;
pub const V4L2_CID_BACKLIGHT_COMPENSATION: U32 = 0x0098091E;
pub const V4L2_CID_EXPOSURE_ABSOLUTE: U32 = 0x00980926;
pub const V4L2_CID_EXPOSURE_AUTO_PRIORITY: U32 = 0x00980927;
pub const V4L2_CID_FOCUS_ABSOLUTE: U32 = 0x00980A0A;
pub const V4L2_CID_FOCUS_RELATIVE: U32 = 0x00980A0B;
pub const V4L2_CID_FOCUS_AUTO: U32 = 0x00980A0C;
pub const V4L2_CID_ZOOM_ABSOLUTE: U32 = 0x00980A14;
pub const V4L2_CID_ZOOM_RELATIVE: U32 = 0x00980A15;
pub const V4L2_CID_ZOOM_CONTINUOUS: U32 = 0x00980A16;
