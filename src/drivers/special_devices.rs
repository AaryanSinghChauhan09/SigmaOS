// Standard Linux/BSD-inspired virtual special devices (/dev/null, /dev/zero, /dev/urandom)
// Implements safe character drivers for simulated hardware interfaces.

pub trait SpecialDevice {
    fn read(&mut self, buf: &mut [u8]) -> usize;
    fn write(&mut self, buf: &[u8]) -> usize;
}

pub struct NullDevice;

impl SpecialDevice for NullDevice {
    fn read(&mut self, _buf: &mut [u8]) -> usize {
        0 // Always EOF
    }

    fn write(&mut self, buf: &[u8]) -> usize {
        buf.len() // Successfully write and discard
    }
}

pub struct ZeroDevice;

impl SpecialDevice for ZeroDevice {
    fn read(&mut self, buf: &mut [u8]) -> usize {
        for byte in buf.iter_mut() {
            *byte = 0;
        }
        buf.len()
    }

    fn write(&mut self, buf: &[u8]) -> usize {
        buf.len() // Discard
    }
}

pub struct RandomDevice {
    seed: u32,
}

impl RandomDevice {
    pub fn new() -> Self {
        Self { seed: 54321 }
    }

    fn next_rand(&mut self) -> u8 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        (self.seed >> 16) as u8
    }
}

impl SpecialDevice for RandomDevice {
    fn read(&mut self, buf: &mut [u8]) -> usize {
        for byte in buf.iter_mut() {
            *byte = self.next_rand();
        }
        buf.len()
    }

    fn write(&mut self, _buf: &[u8]) -> usize {
        0 // Read-only device
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_special_devices() {
        let mut null_dev = NullDevice;
        let mut zero_dev = ZeroDevice;
        let mut rand_dev = RandomDevice::new();

        // 1. NullDevice Test
        let mut buf = [1u8; 10];
        assert_eq!(null_dev.read(&mut buf), 0);
        assert_eq!(null_dev.write(b"discard"), 7);

        // 2. ZeroDevice Test
        assert_eq!(zero_dev.read(&mut buf), 10);
        assert_eq!(buf, [0u8; 10]);

        // 3. RandomDevice Test
        let mut rand_buf = [0u8; 5];
        assert_eq!(rand_dev.read(&mut rand_buf), 5);
        assert_ne!(rand_buf, [0u8; 5]);
    }
}
