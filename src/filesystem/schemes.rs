use std::vec;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
// Redox OS-inspired URL Scheme subsystem for SigmaOS.
// Replaces standard Unix pathnames with URL-based resources (e.g., "shm://buffer", "log://kernel", "rand://stream")
// where "everything is a URL resource".

use crate::klib::HashMap;

pub trait Scheme {
    fn open(&mut self, path: &str) -> Result<usize, &'static str>;
    fn read(&mut self, handle: usize, buf: &mut [u8]) -> Result<usize, &'static str>;
    fn write(&mut self, handle: usize, buf: &[u8]) -> Result<usize, &'static str>;
    fn close(&mut self, handle: usize) -> Result<(), &'static str>;
}

/// null: scheme - returns EOF on read, discards write.
pub struct NullScheme {
    next_handle: usize,
}

impl NullScheme {
    pub fn new() -> Self {
        Self { next_handle: 1 }
    }
}

impl Scheme for NullScheme {
    fn open(&mut self, _path: &str) -> Result<usize, &'static str> {
        let h = self.next_handle;
        self.next_handle += 1;
        Ok(h)
    }

    fn read(&mut self, _handle: usize, _buf: &mut [u8]) -> Result<usize, &'static str> {
        Ok(0) // EOF
    }

    fn write(&mut self, _handle: usize, buf: &[u8]) -> Result<usize, &'static str> {
        Ok(buf.len()) // Accept and discard
    }

    fn close(&mut self, _handle: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

/// rand: scheme - yields random streams of data.
pub struct RandScheme {
    next_handle: usize,
    seed: u32,
}

impl RandScheme {
    pub fn new() -> Self {
        Self {
            next_handle: 1,
            seed: 12345,
        }
    }

    fn next_rand(&mut self) -> u8 {
        // Simple LCG randomizer
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        (self.seed >> 16) as u8
    }
}

impl Scheme for RandScheme {
    fn open(&mut self, _path: &str) -> Result<usize, &'static str> {
        let h = self.next_handle;
        self.next_handle += 1;
        Ok(h)
    }

    fn read(&mut self, _handle: usize, buf: &mut [u8]) -> Result<usize, &'static str> {
        for byte in buf.iter_mut() {
            *byte = self.next_rand();
        }
        Ok(buf.len())
    }

    fn write(&mut self, _handle: usize, _buf: &[u8]) -> Result<usize, &'static str> {
        Err("Cannot write to read-only random stream scheme!")
    }

    fn close(&mut self, _handle: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

/// log: scheme - system in-memory log buffer.
pub struct LogScheme {
    next_handle: usize,
    pub logs: Vec<String>,
}

impl LogScheme {
    pub fn new() -> Self {
        Self {
            next_handle: 1,
            logs: vec!["[log:] Scheme Active".to_string()],
        }
    }
}

impl Scheme for LogScheme {
    fn open(&mut self, _path: &str) -> Result<usize, &'static str> {
        let h = self.next_handle;
        self.next_handle += 1;
        Ok(h)
    }

    fn read(&mut self, _handle: usize, buf: &mut [u8]) -> Result<usize, &'static str> {
        let merged_logs = self.format!("{}/{}", logs, "\n");
        let bytes = merged_logs.as_bytes();
        let len = buf.len().min(bytes.len());
        buf[..len].copy_from_slice(&bytes[..len]);
        Ok(len)
    }

    fn write(&mut self, _handle: usize, buf: &[u8]) -> Result<usize, &'static str> {
        if let Ok(s) = std:: String::from_utf8(buf) {
            self.logs.push(s.to_string());
            Ok(buf.len())
        } else {
            Err("Log input must be valid UTF-8 string!")
        }
    }

    fn close(&mut self, _handle: usize) -> Result<(), &'static str> {
        Ok(())
    }
}

/// shm: scheme - shared memory resource mapping.
pub struct ShmScheme {
    next_handle: usize,
    pub buffers: HashMap<String, Vec<u8>>,
    open_paths: HashMap<usize, String>,
}

impl ShmScheme {
    pub fn new() -> Self {
        Self {
            next_handle: 1,
            buffers: HashMap::new(),
            open_paths: HashMap::new(),
        }
    }
}

impl Scheme for ShmScheme {
    fn open(&mut self, path: &str) -> Result<usize, &'static str> {
        let h = self.next_handle;
        self.next_handle += 1;
        self.open_paths.insert(h, path.to_string());
        if !self.buffers.contains_key(path) {
            self.buffers.insert(path.to_string(), Vec::new());
        }
        Ok(h)
    }

    fn read(&mut self, handle: usize, buf: &mut [u8]) -> Result<usize, &'static str> {
        if let Some(path) = self.open_paths.get(&handle) {
            if let Some(storage) = self.buffers.get(path) {
                let len = buf.len().min(storage.len());
                buf[..len].copy_from_slice(&storage[..len]);
                Ok(len)
            } else {
                Err("Storage for path disappeared!")
            }
        } else {
            Err("Invalid SHM scheme handle!")
        }
    }

    fn write(&mut self, handle: usize, buf: &[u8]) -> Result<usize, &'static str> {
        if let Some(path) = self.open_paths.get(&handle) {
            if let Some(storage) = self.buffers.get_mut(path) {
                storage.clear();
                storage.extend_from_slice(buf);
                Ok(buf.len())
            } else {
                Err("Storage for path disappeared!")
            }
        } else {
            Err("Invalid SHM scheme handle!")
        }
    }

    fn close(&mut self, handle: usize) -> Result<(), &'static str> {
        self.open_paths.remove(&handle);
        Ok(())
    }
}

/// SchemeRegistry - parses URLs and routes calls to Redox schemes.
pub struct SchemeRegistry {
    pub null_scheme: NullScheme,
    pub rand_scheme: RandScheme,
    pub log_scheme: LogScheme,
    pub shm_scheme: ShmScheme,
    /// Maps full routing descriptor to scheme type and inner handle.
    /// Format: (scheme_name, inner_handle)
    pub open_resources: HashMap<usize, (String, usize)>,
    next_global_handle: usize,
}

impl SchemeRegistry {
    pub fn new() -> Self {
        Self {
            null_scheme: NullScheme::new(),
            rand_scheme: RandScheme::new(),
            log_scheme: LogScheme::new(),
            shm_scheme: ShmScheme::new(),
            open_resources: HashMap::new(),
            next_global_handle: 1000,
        }
    }

    /// Resolves URL format (e.g. "shm://buffer_name") and registers open handle
    pub fn open(&mut self, url: &str) -> Result<usize, &'static str> {
        let parts: Vec<&str> = url.split("://").collect();
        if parts.len() != 2 {
            return Err("Invalid URL format! Must be scheme://path");
        }

        let scheme_name = parts[0];
        let path = parts[1];

        let inner_handle = match scheme_name {
            "null" => self.null_scheme.open(path)?,
            "rand" => self.rand_scheme.open(path)?,
            "log" => self.log_scheme.open(path)?,
            "shm" => self.shm_scheme.open(path)?,
            _ => return Err("Unknown scheme identifier!"),
        };

        let g_handle = self.next_global_handle;
        self.next_global_handle += 1;
        self.open_resources.insert(g_handle, (scheme_name.to_string(), inner_handle));

        Ok(g_handle)
    }

    pub fn read(&mut self, handle: usize, buf: &mut [u8]) -> Result<usize, &'static str> {
        if let Some((scheme, inner)) = self.open_resources.get(&handle).cloned() {
            match scheme.as_str() {
                "null" => self.null_scheme.read(inner, buf),
                "rand" => self.rand_scheme.read(inner, buf),
                "log" => self.log_scheme.read(inner, buf),
                "shm" => self.shm_scheme.read(inner, buf),
                _ => Err("Invalid resource scheme association!"),
            }
        } else {
            Err("Handle not found in registry!")
        }
    }

    pub fn write(&mut self, handle: usize, buf: &[u8]) -> Result<usize, &'static str> {
        if let Some((scheme, inner)) = self.open_resources.get(&handle).cloned() {
            match scheme.as_str() {
                "null" => self.null_scheme.write(inner, buf),
                "rand" => self.rand_scheme.write(inner, buf),
                "log" => self.log_scheme.write(inner, buf),
                "shm" => self.shm_scheme.write(inner, buf),
                _ => Err("Invalid resource scheme association!"),
            }
        } else {
            Err("Handle not found in registry!")
        }
    }

    pub fn close(&mut self, handle: usize) -> Result<(), &'static str> {
        if let Some((scheme, inner)) = self.open_resources.remove(&handle) {
            match scheme.as_str() {
                "null" => self.null_scheme.close(inner),
                "rand" => self.rand_scheme.close(inner),
                "log" => self.log_scheme.close(inner),
                "shm" => self.shm_scheme.close(inner),
                _ => Err("Invalid resource scheme association!"),
            }
        } else {
            Err("Handle not found in registry!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_scheme() {
        let mut scheme = NullScheme::new();
        let h = scheme.open("foo").unwrap();

        let mut buf = [1u8, 2u8, 3u8];
        let bytes_read = scheme.read(h, &mut buf).unwrap();
        assert_eq!(bytes_read, 0); // Always EOF

        let bytes_written = scheme.write(h, b"hello").unwrap();
        assert_eq!(bytes_written, 5); // Successfully discarded
    }

    #[test]
    fn test_rand_scheme() {
        let mut scheme = RandScheme::new();
        let h = scheme.open("stream").unwrap();

        let mut buf = [0u8; 10];
        let bytes_read = scheme.read(h, &mut buf).unwrap();
        assert_eq!(bytes_read, 10);
        assert_ne!(buf, [0u8; 10]); // Non-zero randomized content
    }

    #[test]
    fn test_log_and_shm_schemes() {
        let mut registry = SchemeRegistry::new();

        // Test Shared Memory Scheme
        let h_shm = registry.open("shm://test_buf").unwrap();
        let write_len = registry.write(h_shm, b"Redox Scheme data").unwrap();
        assert_eq!(write_len, 17);

        let mut read_buf = [0u8; 100];
        let read_len = registry.read(h_shm, &mut read_buf).unwrap();
        assert_eq!(read_len, 17);
        assert_eq!(&read_buf[..17], b"Redox Scheme data");

        // Test log Scheme
        let h_log = registry.open("log://kernel").unwrap();
        registry.write(h_log, b"New Kernel Warning!").unwrap();

        let mut log_buf = [0u8; 200];
        let log_len = registry.read(h_log, &mut log_buf).unwrap();
        let log_str = std:: String::from_utf8(&log_buf[..log_len]).unwrap();
        assert!(log_str.contains("New Kernel Warning!"));

        registry.close(h_shm).unwrap();
        registry.close(h_log).unwrap();
    }
}
