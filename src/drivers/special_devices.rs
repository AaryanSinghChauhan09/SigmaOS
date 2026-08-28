extern crate alloc;
/// Special Virtual and Loopback Device Drivers for SigmaOS
/// Inspired by Linux and BSD distribution drivers (/dev/null, /dev/zero, /dev/urandom, /dev/loop).

use crate::drivers::peripheral::{PeripheralDevice, DeviceGeneration, PowerState};
use alloc::vec::Vec;

/// Standard /dev/null device discarding all writes and returning EOF (0 bytes) on read.
pub struct NullDevice {
    pub power_state: PowerState,
}

impl Default for NullDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl NullDevice {
    pub fn new() -> Self {
        NullDevice {
            power_state: PowerState::On,
        }
    }
}

impl PeripheralDevice for NullDevice {
    fn name(&self) -> &'static str {
        "/dev/null"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, &'static str> {
        // null device always returns EOF (0 bytes read)
        Ok(0)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        // null device successfully discards all input data
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        Ok(())
    }
}

/// Standard /dev/zero device returning an infinite stream of zeroed bytes on read.
pub struct ZeroDevice {
    pub power_state: PowerState,
}

impl Default for ZeroDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroDevice {
    pub fn new() -> Self {
        ZeroDevice {
            power_state: PowerState::On,
        }
    }
}

impl PeripheralDevice for ZeroDevice {
    fn name(&self) -> &'static str {
        "/dev/zero"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        // Zero device fills the requested read buffer entirely with zeroes
        for byte in buffer.iter_mut() {
            *byte = 0;
        }
        Ok(buffer.len())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        // zero device successfully discards all input data
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        Ok(())
    }
}

/// Standard /dev/urandom pseudo-random generator device (Linear Congruential Generator).
pub struct RandomDevice {
    pub power_state: PowerState,
    seed: u64,
}

impl Default for RandomDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl RandomDevice {
    pub fn new() -> Self {
        RandomDevice {
            power_state: PowerState::On,
            seed: 0xDEADBEEFC0FEBABE,
        }
    }

    /// Set random entropy seed dynamically
    pub fn seed_entropy(&mut self, seed: u64) {
        self.seed = seed;
    }

    fn next_byte(&mut self) -> u8 {
        // Numerical Recipes parameters LCG multiplier and increment
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.seed >> 32) as u8
    }
}

impl PeripheralDevice for RandomDevice {
    fn name(&self) -> &'static str {
        "/dev/urandom"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        for byte in buffer.iter_mut() {
            *byte = self.next_byte();
        }
        Ok(buffer.len())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        // Accepts user-space data as added entropy to seed the generator
        let mut added_seed = 0u64;
        for (i, &byte) in data.iter().enumerate().take(8) {
            added_seed |= (byte as u64) << (i * 8);
        }
        self.seed ^= added_seed;
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        Ok(())
    }
}

/// Standard block loop-device (/dev/loop0) mapping a block file overlay statically.
pub struct LoopDevice {
    pub power_state: PowerState,
    storage: Vec<u8>,
    block_size: usize,
}

impl LoopDevice {
    pub fn new(capacity: usize, block_size: usize) -> Self {
        let mut storage = Vec::new();
        for _ in 0..capacity {
            storage.push(0u8);
        }
        LoopDevice {
            power_state: PowerState::On,
            storage,
            block_size,
        }
    }

    pub fn capacity(&self) -> usize {
        self.storage.len()
    }

    pub fn block_size(&self) -> usize {
        self.block_size
    }
}

impl PeripheralDevice for LoopDevice {
    fn name(&self) -> &'static str {
        "/dev/loop0"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        let len = buffer.len().min(self.storage.len());
        buffer[..len].copy_from_slice(&self.storage.as_slice()[..len]);
        Ok(len)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        let len = data.len().min(self.storage.len());
        self.storage.as_mut_slice()[..len].copy_from_slice(&data[..len]);
        Ok(len)
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_device() {
        let mut null = NullDevice::new();
        assert_eq!(null.name(), "/dev/null");
        assert!(null.initialize().is_ok());

        let mut read_buf = [0xFFu8; 10];
        let bytes_read = null.read(&mut read_buf).unwrap();
        assert_eq!(bytes_read, 0); // null always returns EOF

        let bytes_written = null.write(&[1, 2, 3, 4, 5]).unwrap();
        assert_eq!(bytes_written, 5); // null swallows everything
    }

    #[test]
    fn test_zero_device() {
        let mut zero = ZeroDevice::new();
        assert_eq!(zero.name(), "/dev/zero");

        let mut read_buf = [0xFFu8; 10];
        let bytes_read = zero.read(&mut read_buf).unwrap();
        assert_eq!(bytes_read, 10);
        assert_eq!(read_buf, [0u8; 10]); // zero fills buffers completely with 0
    }

    #[test]
    fn test_random_device() {
        let mut rand_dev = RandomDevice::new();
        assert_eq!(rand_dev.name(), "/dev/urandom");

        let mut read_buf1 = [0u8; 16];
        let mut read_buf2 = [0u8; 16];

        rand_dev.read(&mut read_buf1).unwrap();
        rand_dev.read(&mut read_buf2).unwrap();

        // High probability that they are different
        assert_ne!(read_buf1, read_buf2);

        // Test writing entropy to adjust seed
        let bytes_written = rand_dev.write(&[0x11, 0x22]).unwrap();
        assert_eq!(bytes_written, 2);
    }

    #[test]
    fn test_loop_device() {
        let mut loop_dev = LoopDevice::new(1024, 512);
        assert_eq!(loop_dev.name(), "/dev/loop0");
        assert_eq!(loop_dev.capacity(), 1024);
        assert_eq!(loop_dev.block_size(), 512);

        let data = [0x55u8; 128];
        let bytes_written = loop_dev.write(&data).unwrap();
        assert_eq!(bytes_written, 128);

        let mut read_buf = [0u8; 128];
        let bytes_read = loop_dev.read(&mut read_buf).unwrap();
        assert_eq!(bytes_read, 128);
        assert_eq!(read_buf, data);
    }
}
