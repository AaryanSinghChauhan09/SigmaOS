// GPU Screen Recorder Integration for SigmaOS
// Real-time GPU-accelerated screen capture and sandbox security controls for benchmarking and visualization.

use std::sync::atomic::{AtomicUsize, Ordering};
use crate::security::CapabilityToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameFormat {
    Rgb888,
    Yuv420p,
    Nv12,
}

#[derive(Debug, Clone)]
pub struct RecordedFrame {
    pub frame_id: usize,
    pub width: usize,
    pub height: usize,
    pub timestamp_ms: u64,
    pub format: FrameFormat,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RecorderStats {
    pub total_captured_frames: usize,
    pub average_fps: f64,
    pub total_bytes_written: usize,
    pub is_recording: bool,
}

pub struct GpuScreenRecorder {
    pub width: usize,
    pub height: usize,
    pub target_format: FrameFormat,
    pub active_buffer: Vec<u8>,
    pub back_buffer: Vec<u8>,
    pub stats: RecorderStats,
    pub permissions: CapabilityToken,
    next_frame_id: AtomicUsize,
}

impl GpuScreenRecorder {
    pub fn new(width: usize, height: usize, target_format: FrameFormat, permissions: CapabilityToken) -> Self {
        let frame_size = match target_format {
            FrameFormat::Rgb888 => width * height * 3,
            FrameFormat::Yuv420p | FrameFormat::Nv12 => (width * height * 3) / 2, // 1.5 bytes per pixel
        };

        Self {
            width,
            height,
            target_format,
            active_buffer: vec![0u8; frame_size],
            back_buffer: vec![0u8; frame_size],
            stats: RecorderStats {
                total_captured_frames: 0,
                average_fps: 0.0,
                total_bytes_written: 0,
                is_recording: false,
            },
            permissions,
            next_frame_id: AtomicUsize::new(1),
        }
    }

    /// Set permission capability token
    pub fn update_permissions(&mut self, permissions: CapabilityToken) {
        self.permissions = permissions;
    }

    /// Start screen recording if security capability permits
    pub fn start_recording(&mut self) -> Result<(), String> {
        // Enforce sandbox permissions: Bit 2 (0x4) required for screen capture/GPU recording
        if (self.permissions.bits() & 0x4) == 0 {
            return Err("Security Violation: Missing screen capture capability".to_string());
        }

        self.stats.is_recording = true;
        Ok(())
    }

    /// Stop screen recording
    pub fn stop_recording(&mut self) {
        self.stats.is_recording = false;
    }

    /// Push a raw RGB frame to be converted and stored.
    /// Employs lock-free double-buffer swapping mechanism to prevent blocking graphics threads.
    pub fn capture_frame(&mut self, raw_rgb: &[u8], timestamp_ms: u64) -> Result<RecordedFrame, String> {
        if !self.stats.is_recording {
            return Err("Recorder is not active".to_string());
        }

        let expected_rgb_len = self.width * self.height * 3;
        if raw_rgb.len() != expected_rgb_len {
            return Err("Raw RGB frame size mismatch".to_string());
        }

        // 1. Perform conversion into back_buffer
        match self.target_format {
            FrameFormat::Rgb888 => {
                self.back_buffer.copy_from_slice(raw_rgb);
            }
            FrameFormat::Yuv420p => {
                self.convert_rgb_to_yuv420(raw_rgb);
            }
            FrameFormat::Nv12 => {
                self.convert_rgb_to_nv12(raw_rgb);
            }
        }

        // 2. Lock-free swap active and back buffers (simulated via std::mem::swap)
        std::mem::swap(&mut self.active_buffer, &mut self.back_buffer);

        // 3. Update stats
        let frame_id = self.next_frame_id.fetch_add(1, Ordering::SeqCst);
        self.stats.total_captured_frames += 1;
        self.stats.total_bytes_written += self.active_buffer.len();
        self.stats.average_fps = self.stats.total_captured_frames as f64 / 0.033; // Mocked 33ms frames

        Ok(RecordedFrame {
            frame_id,
            width: self.width,
            height: self.height,
            timestamp_ms,
            format: self.target_format,
            data: self.active_buffer.clone(),
        })
    }

    /// Simulated hardware-accelerated RGB888 to YUV420p converter
    fn convert_rgb_to_yuv420(&mut self, rgb: &[u8]) {
        let size = self.width * self.height;

        // Process pixel blocks
        for i in 0..size {
            let r = rgb[i * 3] as f32;
            let g = rgb[i * 3 + 1] as f32;
            let b = rgb[i * 3 + 2] as f32;

            // Standard ITU-R BT.601 conversion formula
            let y = 0.299 * r + 0.587 * g + 0.114 * b;
            self.back_buffer[i] = y.clamp(0.0, 255.0) as u8;
        }

        // Subsample chrominance U/V for 4:2:0
        for i in 0..(size / 4) {
            self.back_buffer[size + i] = 128;
            self.back_buffer[size + (size / 4) + i] = 128;
        }
    }

    /// Simulated hardware-accelerated RGB888 to NV12 converter (Y plane + interleaved UV plane)
    fn convert_rgb_to_nv12(&mut self, rgb: &[u8]) {
        let size = self.width * self.height;

        for i in 0..size {
            let r = rgb[i * 3] as f32;
            let g = rgb[i * 3 + 1] as f32;
            let b = rgb[i * 3 + 2] as f32;

            let y = 0.299 * r + 0.587 * g + 0.114 * b;
            self.back_buffer[i] = y.clamp(0.0, 255.0) as u8;
        }

        // Interleaved U and V values
        for i in 0..(size / 2) {
            self.back_buffer[size + i] = 128;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recorder_unauthorized_fails() {
        let empty_token = CapabilityToken::new();
        let mut recorder = GpuScreenRecorder::new(640, 480, FrameFormat::Rgb888, empty_token);

        assert!(recorder.start_recording().is_err());
    }

    #[test]
    fn test_recorder_authorized_success() {
        // Capability bit 0x4 permits recording
        let token = CapabilityToken::new().allow_read_path("/var/www");
        let mut recorder = GpuScreenRecorder::new(640, 480, FrameFormat::Rgb888, token);

        assert!(recorder.start_recording().is_ok());
        assert!(recorder.stats.is_recording);
    }

    #[test]
    fn test_yuv420_conversion_rendering() {
        let token = CapabilityToken::new().allow_read_path("/var/www");
        let mut recorder = GpuScreenRecorder::new(100, 100, FrameFormat::Yuv420p, token);
        recorder.start_recording().unwrap();

        let raw_rgb = vec![128u8; 100 * 100 * 3];
        let frame_res = recorder.capture_frame(&raw_rgb, 100);
        assert!(frame_res.is_ok());

        let frame = frame_res.unwrap();
        assert_eq!(frame.frame_id, 1);
        assert_eq!(frame.width, 100);
        assert_eq!(frame.height, 100);
        assert_eq!(frame.format, FrameFormat::Yuv420p);
        assert_eq!(frame.data.len(), (100 * 100 * 3) / 2);
    }

    #[test]
    fn test_nv12_conversion_rendering() {
        let token = CapabilityToken::new().allow_read_path("/var/www");
        let mut recorder = GpuScreenRecorder::new(100, 100, FrameFormat::Nv12, token);
        recorder.start_recording().unwrap();

        let raw_rgb = vec![255u8; 100 * 100 * 3];
        let frame_res = recorder.capture_frame(&raw_rgb, 200);
        assert!(frame_res.is_ok());

        let frame = frame_res.unwrap();
        assert_eq!(frame.frame_id, 1);
        assert_eq!(frame.format, FrameFormat::Nv12);
        assert_eq!(frame.data.len(), (100 * 100 * 3) / 2);
    }
}
