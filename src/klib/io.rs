// SigmaOS Custom I/O Implementation
// Reduces dependency on std::io by providing custom implementations

use crate::klib::{SigmaString, Vec};

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
            data: Vec::new(),
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
        self.data = Vec::new();
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
    pub fn format_int(&mut self, value: i32) -> SigmaString {
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
        
        let mut string_result = SigmaString::new();
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
    pub fn get_string(&self) -> SigmaString {
        let mut result = SigmaString::new();
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

/// Custom Read trait for klib
pub trait KlibRead {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, SigmaIoError>;
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), SigmaIoError>;
    fn read_to_string(&mut self, s: &mut SigmaString) -> Result<usize, SigmaIoError>;
}

/// Custom Write trait for klib
pub trait KlibWrite {
    fn write(&mut self, buf: &[u8]) -> Result<usize, SigmaIoError>;
    fn write_all(&mut self, buf: &[u8]) -> Result<(), SigmaIoError>;
    fn flush(&mut self) -> Result<(), SigmaIoError>;
}

/// Default implementation for SigmaBuffer
impl KlibRead for SigmaBuffer {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, SigmaIoError> {
        let bytes_read = self.read(buf);
        if bytes_read == 0 && !buf.is_empty() {
            Err(SigmaIoError::UnexpectedEof)
        } else {
            Ok(bytes_read)
        }
    }
    
    fn read_exact(&mut self, buf: &mut [u8]) -> Result<(), SigmaIoError> {
        let mut total_read = 0;
        while total_read < buf.len() {
            let bytes_read = KlibRead::read(self, &mut buf[total_read..])?;
            if bytes_read == 0 {
                return Err(SigmaIoError::UnexpectedEof);
            }
            total_read += bytes_read;
        }
        Ok(())
    }
    
    fn read_to_string(&mut self, s: &mut SigmaString) -> Result<usize, SigmaIoError> {
        let mut buffer = [0u8; 4096];
        let mut total_read = 0;
        
        loop {
            let bytes_read = KlibRead::read(self, &mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            
            for byte in &buffer[..bytes_read] {
                s.push(*byte as char);
            }
            total_read += bytes_read;
        }
        
        Ok(total_read)
    }
}

impl KlibWrite for SigmaBuffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, SigmaIoError> {
        let bytes_written = self.write(buf);
        Ok(bytes_written)
    }
    
    fn write_all(&mut self, buf: &[u8]) -> Result<(), SigmaIoError> {
        let mut total_written = 0;
        while total_written < buf.len() {
            let bytes_written = self.write(&buf[total_written..]);
            if bytes_written == 0 {
                return Err(SigmaIoError::WriteZero);
            }
            total_written += bytes_written;
        }
        Ok(())
    }
    
    fn flush(&mut self) -> Result<(), SigmaIoError> {
        // Buffer-based, no flush needed
        Ok(())
    }
}
