use std::vec::Vec;
use std::string::String;

pub struct DevFs {
    devices: Vec<DeviceNode>,
    kernel_log_ring: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DeviceNode {
    pub name: &'static str,
    pub major: u32,
    pub minor: u32,
    pub is_block: bool,
    pub mode: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevFsError {
    DeviceNotFound,
    NoSpaceLeft,
    PermissionDenied,
    IoError,
}

impl DevFs {
    pub fn new() -> Self {
        let mut devfs = Self {
            devices: Vec::new(),
            kernel_log_ring: Vec::new(),
        };

        // Character devices (Linux & BSD inspired)
        devfs.register_device("null", 1, 3, false, 0o666);
        devfs.register_device("zero", 1, 5, false, 0o666);
        devfs.register_device("full", 1, 7, false, 0o666);
        devfs.register_device("random", 1, 8, false, 0o666);
        devfs.register_device("urandom", 1, 9, false, 0o666);
        devfs.register_device("kmsg", 1, 11, false, 0o644);
        devfs.register_device("tty", 5, 0, false, 0o666);
        devfs.register_device("console", 5, 1, false, 0o600);
        devfs.register_device("ptmx", 5, 2, false, 0o666);
        devfs.register_device("stdin", 0, 0, false, 0o666);
        devfs.register_device("stdout", 0, 1, false, 0o666);
        devfs.register_device("stderr", 0, 2, false, 0o666);

        // Block devices
        devfs.register_device("loop0", 7, 0, true, 0o660);
        devfs.register_device("sda", 8, 0, true, 0o660);
        devfs.register_device("sda1", 8, 1, true, 0o660);
        devfs.register_device("zram0", 251, 0, true, 0o660);
        devfs.register_device("md0", 9, 0, true, 0o660);
        devfs.register_device("nvme0n1", 259, 0, true, 0o660);

        devfs
    }

    pub fn register_device(&mut self, name: &'static str, major: u32, minor: u32, is_block: bool, mode: u32) {
        if !self.devices.iter().any(|d| d.name == name) {
            self.devices.push(DeviceNode {
                name,
                major,
                minor,
                is_block,
                mode,
            });
        }
    }

    pub fn find_device(&self, name: &str) -> Option<&DeviceNode> {
        self.devices.iter().find(|d| d.name == name)
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Simulates reading from character devices (`/dev/null`, `/dev/zero`, `/dev/urandom`, `/dev/kmsg`)
    pub fn read_char_device(&self, name: &str, buf: &mut [u8]) -> Result<usize, DevFsError> {
        match name {
            "null" => Ok(0), // EOF immediately
            "zero" => {
                for byte in buf.iter_mut() {
                    *byte = 0;
                }
                Ok(buf.len())
            }
            "full" => Ok(buf.len()), // Reading from /dev/full yields infinite zero bytes
            "random" | "urandom" => {
                let mut seed = 0x85eb_ca6b_u32;
                for byte in buf.iter_mut() {
                    seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                    *byte = (seed >> 16) as u8;
                }
                Ok(buf.len())
            }
            "kmsg" => {
                let mut written = 0;
                for msg in &self.kernel_log_ring {
                    let bytes = msg.as_bytes();
                    let len = bytes.len().min(buf.len() - written);
                    if len == 0 {
                        break;
                    }
                    buf[written..written + len].copy_from_slice(&bytes[..len]);
                    written += len;
                }
                Ok(written)
            }
            _ => {
                if self.find_device(name).is_some() {
                    Ok(0)
                } else {
                    Err(DevFsError::DeviceNotFound)
                }
            }
        }
    }

    /// Simulates writing to character devices (`/dev/null`, `/dev/zero`, `/dev/full`, `/dev/kmsg`)
    pub fn write_char_device(&mut self, name: &str, buf: &[u8]) -> Result<usize, DevFsError> {
        match name {
            "null" | "zero" => Ok(buf.len()), // Discard all data successfully
            "full" => Err(DevFsError::NoSpaceLeft), // Writing to /dev/full yields ENOSPC
            "kmsg" => {
                if let Ok(msg) = String::from_utf8(buf.to_vec()) {
                    self.kernel_log_ring.push(msg);
                }
                Ok(buf.len())
            }
            _ => {
                if self.find_device(name).is_some() {
                    Ok(buf.len())
                } else {
                    Err(DevFsError::DeviceNotFound)
                }
            }
        }
    }
}

impl Default for DevFs {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devfs_device_registration() {
        let devfs = DevFs::new();
        assert!(devfs.find_device("null").is_some());
        assert!(devfs.find_device("zero").is_some());
        assert!(devfs.find_device("zram0").is_some());
        assert!(devfs.find_device("kmsg").is_some());
        assert_eq!(devfs.find_device("null").unwrap().major, 1);
        assert_eq!(devfs.find_device("null").unwrap().minor, 3);
    }

    #[test]
    fn test_char_devices_behavior() {
        let mut devfs = DevFs::new();

        // /dev/null
        let mut buf = [0xffu8; 16];
        let n = devfs.read_char_device("null", &mut buf).unwrap();
        assert_eq!(n, 0); // EOF

        let w = devfs.write_char_device("null", b"hello").unwrap();
        assert_eq!(w, 5);

        // /dev/zero
        let n = devfs.read_char_device("zero", &mut buf).unwrap();
        assert_eq!(n, 16);
        assert_eq!(buf, [0u8; 16]);

        // /dev/full
        assert_eq!(devfs.write_char_device("full", b"test"), Err(DevFsError::NoSpaceLeft));

        // /dev/urandom
        let n = devfs.read_char_device("urandom", &mut buf).unwrap();
        assert_eq!(n, 16);
        assert_ne!(buf, [0u8; 16]);

        // /dev/kmsg
        let _ = devfs.write_char_device("kmsg", b"Kernel booted successfully");
        let mut kmsg_buf = [0u8; 64];
        let n = devfs.read_char_device("kmsg", &mut kmsg_buf).unwrap();
        assert!(n > 0);
        assert!(String::from_utf8_lossy(&kmsg_buf[..n]).contains("Kernel booted"));
    }
}
