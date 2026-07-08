// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/camera/uvc.rs — USB Video Class (UVC) Camera Driver
//
// Implements the USB Video Class (UVC) driver for webcams.
// Supports UVC 1.1 specification.
// Based on Linux kernel uvc driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::camera_device_base::{CameraDevice, CameraType, PixelFormat, CameraFormat, CameraCapability, CameraBuffer, BufferType, MemoryType, Field, Colorspace, CAMERA_OK, CAMERA_ERR_NO_DEVICE, CAMERA_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── UVC Vendor IDs ─────────────────────────────────────

pub const LOGITECH_VENDOR_ID: U16 = 0x046D;
pub const MICROSOFT_VENDOR_ID: U16 = 0x045E;
pub const REALTEK_VENDOR_ID: U16 = 0x0BDA;
pub const SUNPLUS_VENDOR_ID: U16 = 0x04FC;
pub const CHICONY_VENDOR_ID: U16 = 0x04F2;

// ─── UVC Interface Descriptors ─────────────────────

pub const UVC_VC_INTERFACE: U8 = 0x01;
pub const UVC_VS_INTERFACE: U8 = 0x02;
pub const UVC_VIDEO_INTERFACE: U8 = 0x03;

// ─── UVC Video Control Interface ─────────────────

pub const UVC_VC_HEADER: U8 = 0x01;
pub const UVC_VC_PROCESSING_UNIT: U8 = 0x02;
pub const UVC_VC_EXTENSION_UNIT: U8 = 0x04;

// ─── UVC Video Streaming Interface ─────────────

pub const UVC_VS_INPUT_HEADER: U8 = 0x01;
pub const UVC_VS_OUTPUT_HEADER: U8 = 0x02;
pub const UVC_VS_STILL_IMAGE_FRAME: U8 = 0x03;
pub const UVC_VS_FORMAT_UNCOMPRESSED: U8 = 0x04;
pub const UVC_VS_FRAME_UNCOMPRESSED: U8 = 0x05;
pub const UVC_VS_FORMAT_MJPEG: U8 = 0x06;
pub const UVC_VS_FRAME_MJPEG: U8 = 0x07;
pub const UVC_VS_FORMAT_MPEG2TS: U8 = 0x0A;
pub const UVC_VS_FRAME_MPEG2TS: U8 = 0x0B;
pub const UVC_VS_FORMAT_DV: U8 = 0x0C;
pub const UVC_VS_FRAME_DV: U8 = 0x0D;
pub const UVC_VS_COLORFORMAT: U8 = 0x0D;
pub const UVC_VS_FORMAT_FRAME_BASED: U8 = 0x10;
pub const UVC_VS_FRAME_FRAME_BASED: U8 = 0x11;
pub const UVC_VS_FORMAT_STREAM_BASED: U8 = 0x12;

// ─── UVC Control Requests ─────────────────────

pub const UVC_SET_CUR: U8 = 0x01;
pub const UVC_GET_CUR: U8 = 0x81;
pub const UVC_GET_MIN: U8 = 0x82;
pub const UVC_GET_MAX: U8 = 0x83;
pub const UVC_GET_RES: U8 = 0x84;
pub const UVC_GET_LEN: U8 = 0x85;
pub const UVC_GET_INFO: U8 = 0x86;
pub const UVC_GET_DEF: U8 = 0x87;

// ─── UVC Video Controls ─────────────────────

pub const UVC_VC_CONTROL_UNDEFINED: U8 = 0x00;
pub const UVC_VC_VIDEO_POWER_MODE_CONTROL: U8 = 0x01;
pub const UVC_VC_REQUEST_ERROR_CODE_CONTROL: U8 = 0x02;

// ─── UVC Processing Unit Controls ─────────────

pub const UVC_PU_CONTROL_UNDEFINED: U8 = 0x00;
pub const UVC_PU_BACKLIGHT_COMPENSATION_CONTROL: U8 = 0x01;
pub const UVC_PU_BRIGHTNESS_CONTROL: U8 = 0x02;
pub const UVC_PU_CONTRAST_CONTROL: U8 = 0x03;
pub const UVC_PU_GAIN_CONTROL: U8 = 0x04;
pub const UVC_PU_POWER_LINE_FREQUENCY_CONTROL: U8 = 0x05;
pub const UVC_PU_HUE_CONTROL: U8 = 0x06;
pub const UVC_PU_SATURATION_CONTROL: U8 = 0x07;
pub const UVC_PU_SHARPNESS_CONTROL: U8 = 0x08;
pub const UVC_PU_GAMMA_CONTROL: U8 = 0x09;
pub const UVC_PU_WHITE_BALANCE_TEMPERATURE_CONTROL: U8 = 0x0A;
pub const UVC_PU_WHITE_BALANCE_TEMPERATURE_AUTO_CONTROL: U8 = 0x0B;
pub const UVC_PU_WHITE_BALANCE_COMPONENT_CONTROL: U8 = 0x0C;
pub const UVC_PU_WHITE_BALANCE_COMPONENT_AUTO_CONTROL: U8 = 0x0D;
pub const UVC_PU_DIGITAL_MULTIPLIER_CONTROL: U8 = 0x0E;
pub const UVC_PU_DIGITAL_MULTIPLIER_LIMIT_CONTROL: U8 = 0x0F;
pub const UVC_PU_HUE_AUTO_CONTROL: U8 = 0x10;
pub const UVC_PU_ANALOG_VIDEO_STANDARD_CONTROL: U8 = 0x11;
pub const UVC_PU_ANALOG_LOCK_STATUS_CONTROL: U8 = 0x12;

// ─── UVC Format Descriptor ─────────────────────

#[repr(C)]
pub struct UvcFormatDescriptor {
    pub bLength: U8,
    pub bDescriptorType: U8,
    pub bDescriptorSubtype: U8,
    pub bFormatIndex: U8,
    pub bNumFrameDescriptors: U8,
    pub guidFormat: [U8; 16],
    pub bBitsPerPixel: U8,
    pub bDefaultFrameIndex: U8,
    pub bAspectRatioX: U8,
    pub bAspectRatioY: U8,
    pub bmInterfaceFlags: U8,
    pub bCopyProtect: U8,
}

impl UvcFormatDescriptor {
    pub const fn new() -> Self {
        UvcFormatDescriptor {
            bLength: 0,
            bDescriptorType: 0,
            bDescriptorSubtype: 0,
            bFormatIndex: 0,
            bNumFrameDescriptors: 0,
            guidFormat: [0; 16],
            bBitsPerPixel: 0,
            bDefaultFrameIndex: 0,
            bAspectRatioX: 0,
            bAspectRatioY: 0,
            bmInterfaceFlags: 0,
            bCopyProtect: 0,
        }
    }
}

// ─── UVC Frame Descriptor ─────────────────────

#[repr(C)]
pub struct UvcFrameDescriptor {
    pub bLength: U8,
    pub bDescriptorType: U8,
    pub bDescriptorSubtype: U8,
    pub bFrameIndex: U8,
    pub bmCapabilities: U8,
    pub wWidth: U16,
    pub wHeight: U16,
    pub dwMinBitRate: U32,
    pub dwMaxBitRate: U32,
    pub dwMaxVideoFrameBufferSize: U32,
    pub dwDefaultFrameInterval: U32,
    pub bFrameIntervalType: U8,
    pub dwFrameInterval: [U32; 1],
}

impl UvcFrameDescriptor {
    pub const fn new() -> Self {
        UvcFrameDescriptor {
            bLength: 0,
            bDescriptorType: 0,
            bDescriptorSubtype: 0,
            bFrameIndex: 0,
            bmCapabilities: 0,
            wWidth: 0,
            wHeight: 0,
            dwMinBitRate: 0,
            dwMaxBitRate: 0,
            dwMaxVideoFrameBufferSize: 0,
            dwDefaultFrameInterval: 0,
            bFrameIntervalType: 0,
            dwFrameInterval: [0; 1],
        }
    }
}

// ─── UVC Camera Structure ─────────────────────

pub struct UvcCamera {
    pub usb_device: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub streaming: bool,
    pub format: CameraFormat,
    pub capabilities: CameraCapability,
    pub buffers: [CameraBuffer; 8],
    pub buffer_count: U32,
    pub current_format: U8,
    pub formats: [UvcFormatDescriptor; 16],
    pub format_count: U8,
    pub frames: [UvcFrameDescriptor; 32],
    pub frame_count: U8,
}

impl UvcCamera {
    pub const fn new() -> Self {
        UvcCamera {
            usb_device: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            streaming: false,
            format: CameraFormat::new(),
            capabilities: CameraCapability::new(),
            buffers: [CameraBuffer::new(); 8],
            buffer_count: 0,
            current_format: 0,
            formats: [UvcFormatDescriptor::new(); 16],
            format_count: 0,
            frames: [UvcFrameDescriptor::new(); 32],
            frame_count: 0,
        }
    }

    /// Initialize UVC camera
    fn init_uvc(&mut self, usb_device: U64, device_id: U16, vendor_id: U16) -> I32 {
        self.usb_device = usb_device;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        // In a real implementation, this would:
        // 1. Probe USB device for UVC interface
        // 2. Parse video control interface descriptor
        // 3. Parse video streaming interface descriptor
        // 4. Enumerate supported formats and frames
        // 5. Set default format

        // Stub: set default values
        self.format.width = 640;
        self.format.height = 480;
        self.format.pixel_format = PixelFormat::YUYV;
        self.format.sizeimage = 640 * 480 * 2;

        self.capabilities.version = 0x00010001; // V4L2 version
        self.capabilities.capabilities = 0x08000001; // VIDEO_CAPTURE | STREAMING

        self.initialized = true;

        CAMERA_OK
    }

    /// Send UVC control request
    unsafe fn send_uvc_control(&self, unit: U8, selector: U8, request: U8, data: *mut U8, length: U16) -> I32 {
        // In a real implementation, send USB control request
        CAMERA_OK
    }

    /// Get current control value
    unsafe fn get_control(&self, unit: U8, selector: U8, value: *mut I32) -> I32 {
        let mut data: [U8; 4] = [0; 4];
        let result = self.send_uvc_control(unit, selector, 0x81, data.as_mut_ptr(), 4);
        
        if result == CAMERA_OK && !value.is_null() {
            *value = (data[0] as I32) | ((data[1] as I32) << 8) | ((data[2] as I32) << 16) | ((data[3] as I32) << 24);
        }

        result
    }

    /// Set control value
    unsafe fn set_control(&mut self, unit: U8, selector: U8, value: I32) -> I32 {
        let mut data: [U8; 4] = [0; 4];
        data[0] = (value & 0xFF) as U8;
        data[1] = ((value >> 8) & 0xFF) as U8;
        data[2] = ((value >> 16) & 0xFF) as U8;
        data[3] = ((value >> 24) & 0xFF) as U8;

        self.send_uvc_control(unit, selector, 0x01, data.as_mut_ptr(), 4)
    }
}

// ─── Implement CameraDevice Trait ─────────────────

impl CameraDevice for UvcCamera {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        let vendor_id = match device_id {
            _ => LOGITECH_VENDOR_ID,
        };
        
        self.init_uvc(pci_bar, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        match self.vendor_id {
            LOGITECH_VENDOR_ID => "Logitech UVC Webcam",
            MICROSOFT_VENDOR_ID => "Microsoft UVC Webcam",
            REALTEK_VENDOR_ID => "Realtek UVC Webcam",
            SUNPLUS_VENDOR_ID => "Sunplus UVC Webcam",
            CHICONY_VENDOR_ID => "Chicony UVC Webcam",
            _ => "UVC Webcam",
        }
    }

    fn get_camera_type(&self) -> CameraType {
        CameraType::USB
    }

    fn get_capabilities(&self, caps: *mut CameraCapability) -> I32 {
        if caps.is_null() {
            return CAMERA_ERR_INIT_FAILED;
        }

        unsafe {
            *caps = self.capabilities;
        }

        CAMERA_OK
    }

    fn query_formats(&self, index: U32, format: *mut PixelFormat) -> I32 {
        if format.is_null() {
            return CAMERA_ERR_INIT_FAILED;
        }

        if index >= 16 {
            return CAMERA_ERR_INVALID_PARAM;
        }

        unsafe {
            // In a real implementation, return supported formats
            if index == 0 {
                *format = PixelFormat::YUYV;
            } else if index == 1 {
                *format = PixelFormat::MJPEG;
            } else {
                return CAMERA_ERR_INVALID_PARAM;
            }
        }

        CAMERA_OK
    }

    fn get_format(&self, format: *mut CameraFormat) -> I32 {
        if format.is_null() {
            return CAMERA_ERR_INIT_FAILED;
        }

        unsafe {
            *format = self.format;
        }

        CAMERA_OK
    }

    fn set_format(&mut self, format: *const CameraFormat) -> I32 {
        if format.is_null() {
            return CAMERA_ERR_INIT_FAILED;
        }

        unsafe {
            self.format = *format;
        }

        CAMERA_OK
    }

    fn try_format(&mut self, format: *mut CameraFormat) -> I32 {
        if format.is_null() {
            return CAMERA_ERR_INIT_FAILED;
        }

        unsafe {
            // In a real implementation, adjust format to nearest supported
            if (*format).width > 1920 {
                (*format).width = 1920;
            }
            if (*format).height > 1080 {
                (*format).height = 1080;
            }
        }

        CAMERA_OK
    }

    fn request_buffers(&mut self, count: U32, memory: MemoryType) -> I32 {
        if !self.initialized {
            return CAMERA_ERR_INIT_FAILED;
        }

        if count > 8 {
            return CAMERA_ERR_INVALID_PARAM;
        }

        self.buffer_count = count;
        
        for i in 0..count as usize {
            self.buffers[i].index = i as U32;
            self.buffers[i].type_ = BufferType::VideoCapture;
            self.buffers[i].memory = memory;
            self.buffers[i].length = self.format.sizeimage;
        }

        CAMERA_OK
    }

    fn query_buffer(&self, buffer: *mut CameraBuffer) -> I32 {
        if buffer.is_null() {
            return CAMERA_ERR_INIT_FAILED;
        }

        unsafe {
            let index = (*buffer).index as usize;
            if index >= self.buffer_count as usize {
                return CAMERA_ERR_INVALID_PARAM;
            }

            *buffer = self.buffers[index];
        }

        CAMERA_OK
    }

    fn queue_buffer(&mut self, buffer: *mut CameraBuffer) -> I32 {
        if buffer.is_null() {
            return CAMERA_ERR_INIT_FAILED;
        }

        unsafe {
            let index = (*buffer).index as usize;
            if index >= self.buffer_count as usize {
                return CAMERA_ERR_INVALID_PARAM;
            }

            self.buffers[index].flags |= 0x00000001; // MAPPED
        }

        CAMERA_OK
    }

    fn dequeue_buffer(&mut self, buffer: *mut CameraBuffer) -> I32 {
        if buffer.is_null() {
            return CAMERA_ERR_INIT_FAILED;
        }

        // In a real implementation, wait for frame to be captured
        CAMERA_OK
    }

    fn stream_on(&mut self, type_: BufferType) -> I32 {
        if !self.initialized {
            return CAMERA_ERR_INIT_FAILED;
        }

        if type_ != BufferType::VideoCapture {
            return CAMERA_ERR_NOT_SUPPORTED;
        }

        self.streaming = true;
        CAMERA_OK
    }

    fn stream_off(&mut self, type_: BufferType) -> I32 {
        if !self.initialized {
            return CAMERA_ERR_INIT_FAILED;
        }

        if type_ != BufferType::VideoCapture {
            return CAMERA_ERR_NOT_SUPPORTED;
        }

        self.streaming = false;
        CAMERA_OK
    }

    fn get_control(&self, id: U32, value: *mut I32) -> I32 {
        if value.is_null() {
            return CAMERA_ERR_INIT_FAILED;
        }

        unsafe {
            // Map V4L2 control IDs to UVC processing unit selectors
            let selector = match id {
                super::camera_device_base::V4L2_CID_BRIGHTNESS => UVC_PU_BRIGHTNESS_CONTROL,
                super::camera_device_base::V4L2_CID_CONTRAST => UVC_PU_CONTRAST_CONTROL,
                super::camera_device_base::V4L2_CID_SATURATION => UVC_PU_SATURATION_CONTROL,
                super::camera_device_base::V4L2_CID_HUE => UVC_PU_HUE_CONTROL,
                super::camera_device_base::V4L2_CID_SHARPNESS => UVC_PU_SHARPNESS_CONTROL,
                super::camera_device_base::V4L2_CID_GAMMA => UVC_PU_GAMMA_CONTROL,
                super::camera_device_base::V4L2_CID_GAIN => UVC_PU_GAIN_CONTROL,
                _ => return CAMERA_ERR_NOT_SUPPORTED,
            };

            self.get_control(0x02, selector, value) // Processing unit 2
        }
    }

    fn set_control(&mut self, id: U32, value: I32) -> I32 {
        unsafe {
            let selector = match id {
                super::camera_device_base::V4L2_CID_BRIGHTNESS => UVC_PU_BRIGHTNESS_CONTROL,
                super::camera_device_base::V4L2_CID_CONTRAST => UVC_PU_CONTRAST_CONTROL,
                super::camera_device_base::V4L2_CID_SATURATION => UVC_PU_SATURATION_CONTROL,
                super::camera_device_base::V4L2_CID_HUE => UVC_PU_HUE_CONTROL,
                super::camera_device_base::V4L2_CID_SHARPNESS => UVC_PU_SHARPNESS_CONTROL,
                super::camera_device_base::V4L2_CID_GAMMA => UVC_PU_GAMMA_CONTROL,
                super::camera_device_base::V4L2_CID_GAIN => UVC_PU_GAIN_CONTROL,
                _ => return CAMERA_ERR_NOT_SUPPORTED,
            };

            self.set_control(0x02, selector, value)
        }
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return CAMERA_ERR_INIT_FAILED;
        }

        self.streaming = false;
        CAMERA_OK
    }

    fn shutdown(&mut self) -> I32 {
        self.reset();
        self.initialized = false;
        CAMERA_OK
    }
}

// ─── Global UVC Camera ─────────────────────────

static mut G_UVC: UvcCamera = UvcCamera::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn uvc_init(usb_device: U64, device_id: U16) -> I32 {
    G_UVC.init(usb_device, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn uvc_is_initialized() -> I32 {
    if G_UVC.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uvc_shutdown() -> I32 {
    G_UVC.shutdown()
}
