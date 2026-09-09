// V4L2 Webcam Driver Paradigm Implementation for SigmaOS
// Provides zero-dependency Video4Linux2 capture buffer management and format negotiation.

use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    YUYV,
    MJPEG,
    H264,
    RGB24,
}

pub struct V4l2Capability {
    pub driver: String,
    pub card: String,
    pub bus_info: String,
    pub version: u32,
}

pub struct V4l2StreamBuffer {
    pub index: u32,
    pub length: usize,
    pub bytes_used: usize,
    pub data: Vec<u8>,
}

pub struct V4l2WebcamDriver {
    pub device_node: String,
    pub current_format: PixelFormat,
    pub width: u32,
    pub height: u32,
    pub streaming: bool,
    pub buffers: Vec<V4l2StreamBuffer>,
}

impl V4l2WebcamDriver {
    pub fn new(device_node: &str) -> Self {
        Self {
            device_node: String::from(device_node),
            current_format: PixelFormat::YUYV,
            width: 1920,
            height: 1080,
            streaming: false,
            buffers: Vec::new(),
        }
    }

    pub fn set_format(&mut self, format: PixelFormat, width: u32, height: u32) -> bool {
        if self.streaming {
            return false;
        }
        self.current_format = format;
        self.width = width;
        self.height = height;
        true
    }

    pub fn start_stream(&mut self, buffer_count: usize) -> bool {
        self.buffers.clear();
        let frame_size = (self.width * self.height * 2) as usize;
        for idx in 0..buffer_count {
            let mut buf = Vec::new();
            buf.resize(frame_size, 0);
            self.buffers.push(V4l2StreamBuffer {
                index: idx as u32,
                length: frame_size,
                bytes_used: frame_size,
                data: buf,
            });
        }
        self.streaming = true;
        true
    }

    pub fn stop_stream(&mut self) {
        self.streaming = false;
        self.buffers.clear();
    }
}
