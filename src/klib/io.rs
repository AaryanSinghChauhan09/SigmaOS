// SigmaOS Custom I/O Implementation
// Reduces dependency on std::io by providing custom implementations

use crate::klib::{String, Vec};

/// Custom buffer for I/O operations
pub struct SigmaBuffer {
    pub data: Vec<u8>,
    pub position: usize,
}

impl SigmaBuffer {
    pub fn new() -> Self {
        SigmaBuffer {
            data: Vec::new(),
            position: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        SigmaBuffer {
            data: Vec::with_capacity(capacity),
            position: 0,
        }
    }

    /// Write bytes to buffer
    pub fn write(&mut self, bytes: &[u8]) -> usize {
        let bytes_written = bytes.len();
        for &byte in bytes {
            self.data.push(byte);
        }
        bytes_written
    }

    /// Read bytes from buffer
    pub fn read(&mut self, buffer: &mut [u8]) -> usize {
        let bytes_to_read = core::cmp::min(buffer.len(), self.data.len() - self.position);
        for i in 0..bytes_to_read {
            buffer[i] = self.data[self.position + i];
        }
        self.position += bytes_to_read;
        bytes_to_read
    }

    /// Get remaining bytes in buffer
    pub fn remaining(&self) -> usize {
        self.data.len() - self.position
    }

    /// Reset buffer position
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// Clear buffer data
    pub fn clear(&mut self) {
        self.data.clear();
        self.position = 0;
    }
}

/// Custom string formatting for I/O
pub struct SigmaFormatter {
    pub buffer: SigmaBuffer,
}

impl SigmaFormatter {
    pub fn new() -> Self {
        SigmaFormatter {
            buffer: SigmaBuffer::new(),
        }
    }

    /// Format integer to string
    pub fn format_int(&mut self, value: i32) -> String {
        let mut result = Vec::new();
        let mut num = value;
        
        if num == 0 {
            result.push(b'0');
        } else {
            let negative = num < 0;
            if negative {
                num = -num;
            }
            
            while num > 0 {
                let digit = (num % 10) as u8;
                result.push(digit + b'0');
                num /= 10;
            }
            
            if negative {
                result.push(b'-');
            }
            
            // Reverse the result
            let len = result.len();
            for i in 0..len / 2 {
                let temp = result[i];
                result[i] = result[len - 1 - i];
                result[len - 1 - i] = temp;
            }
        }
        
        let mut string_result = String::new();
        for &byte in &result {
            string_result.push(byte as char);
        }
        string_result
    }

    /// Format string to buffer
    pub fn format_str(&mut self, s: &str) {
        self.buffer.write(s.as_bytes());
    }

    /// Get formatted string
    pub fn get_string(&self) -> String {
        let mut result = String::new();
        for &byte in &self.buffer.data {
            result.push(byte as char);
        }
        result
    }
}

/// Custom error type for I/O operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigmaIoError {
    NotFound,
    PermissionDenied,
    InvalidInput,
    UnexpectedEof,
    WriteZero,
    Other,
}

impl Default for SigmaBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SigmaFormatter {
    fn default() -> Self {
        Self::new()
    }
}
