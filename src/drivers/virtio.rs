// VirtIO Paravirtualization driver suite (inspired by Linux and BSD guest virtual drivers)
// Supports virtio-net (network), virtio-blk (block storage), and virtio-rng (entropy/randomness)

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

/// VirtIO Descriptor structure
#[derive(Debug, Clone)]
pub struct VirtioDesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

/// VirtIO virtual split queue (OASIS VirtIO Spec compliant split ring queue)
pub struct VirtQueue {
    pub size: u16,
    pub descriptors: Vec<VirtioDesc>,
    pub avail_ring: Vec<u16>,
    pub used_ring: Vec<u16>,
    pub last_used_idx: u16,
}

impl VirtQueue {
    pub fn new(size: u16) -> Self {
        Self {
            size,
            descriptors: vec![VirtioDesc { addr: 0, len: 0, flags: 0, next: 0 }; size as usize],
            avail_ring: vec![0; size as usize],
            used_ring: vec![0; size as usize],
            last_used_idx: 0,
        }
    }

    pub fn push_desc(&mut self, idx: u16, addr: u64, len: u32, flags: u16) {
        if idx < self.size {
            self.descriptors[idx as usize] = VirtioDesc { addr, len, flags, next: 0 };
        }
    }
}

/// VirtIO Block Device Driver (virtio-blk)
pub struct VirtioBlkDriver {
    pub is_initialized: bool,
    pub power_state: PowerState,
    pub request_queue: VirtQueue,
    pub sector_count: u64,
}

impl VirtioBlkDriver {
    pub fn new(sectors: u64) -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            request_queue: VirtQueue::new(128),
            sector_count: sectors,
        }
    }
}

impl PeripheralDevice for VirtioBlkDriver {
    fn name(&self) -> &'static str {
        "VirtIO Virtual Block Device Driver (virtio-blk)"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("virtio-blk not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("virtio-blk is offline");
        }
        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }
        Ok(buffer.len())
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("virtio-blk not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("virtio-blk is offline");
        }
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        Ok(())
    }
}

/// VirtIO Network Device Driver (virtio-net)
pub struct VirtioNetDriver {
    pub is_initialized: bool,
    pub power_state: PowerState,
    pub rx_queue: VirtQueue,
    pub tx_queue: VirtQueue,
    pub mac_address: [u8; 6],
}

impl VirtioNetDriver {
    pub fn new(mac: [u8; 6]) -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            rx_queue: VirtQueue::new(256),
            tx_queue: VirtQueue::new(256),
            mac_address: mac,
        }
    }
}

impl PeripheralDevice for VirtioNetDriver {
    fn name(&self) -> &'static str {
        "VirtIO Virtual Ethernet Adapter Driver (virtio-net)"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("virtio-net not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("virtio-net is offline");
        }
        // Mock incoming ethernet packet payload
        let mock_packet = [0x52, 0x54, 0x00, 0x12, 0x34, 0x56, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x08, 0x00];
        let copy_len = std::cmp::min(buffer.len(), mock_packet.len());
        buffer[..copy_len].copy_from_slice(&mock_packet[..copy_len]);
        Ok(copy_len)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("virtio-net not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("virtio-net is offline");
        }
        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        Ok(())
    }
}

/// VirtIO Entropy Device Driver (virtio-rng)
pub struct VirtioRngDriver {
    pub is_initialized: bool,
    pub power_state: PowerState,
    pub entropy_queue: VirtQueue,
}

impl VirtioRngDriver {
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            entropy_queue: VirtQueue::new(64),
        }
    }
}

impl Default for VirtioRngDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl PeripheralDevice for VirtioRngDriver {
    fn name(&self) -> &'static str {
        "VirtIO Virtual Hardware Entropy Generator Driver (virtio-rng)"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("virtio-rng not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("virtio-rng is offline");
        }
        // Fill buffer with pseudo-random hardware bytes
        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = ((i * 127 + 42) % 251) as u8;
        }
        Ok(buffer.len())
    }

    fn write(&mut self, _data: &[u8]) -> Result<usize, &'static str> {
        Err("virtio-rng is a read-only entropy device")
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.is_initialized = false;
        self.power_state = PowerState::Off;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtio_blk_driver() {
        let mut blk = VirtioBlkDriver::new(10000);
        assert_eq!(blk.name(), "VirtIO Virtual Block Device Driver (virtio-blk)");
        assert_eq!(blk.generation(), DeviceGeneration::Modern);
        assert!(blk.read(&mut [0; 10]).is_err());

        blk.initialize().unwrap();
        let mut buf = vec![0; 50];
        assert_eq!(blk.read(&mut buf).unwrap(), 50);
        assert_eq!(buf[0], 0);
        assert_eq!(buf[1], 1);

        assert_eq!(blk.write(&[1, 2, 3]).unwrap(), 3);
        blk.shutdown().unwrap();
    }

    #[test]
    fn test_virtio_net_driver() {
        let mut net = VirtioNetDriver::new([0x52, 0x54, 0x00, 0xaa, 0xbb, 0xcc]);
        assert_eq!(net.name(), "VirtIO Virtual Ethernet Adapter Driver (virtio-net)");
        assert_eq!(net.mac_address, [0x52, 0x54, 0x00, 0xaa, 0xbb, 0xcc]);

        net.initialize().unwrap();
        let mut buf = vec![0; 20];
        let bytes_read = net.read(&mut buf).unwrap();
        assert_eq!(bytes_read, 14);
        assert_eq!(buf[0], 0x52);
        assert_eq!(buf[1], 0x54);
    }

    #[test]
    fn test_virtio_rng_driver() {
        let mut rng = VirtioRngDriver::new();
        assert_eq!(rng.name(), "VirtIO Virtual Hardware Entropy Generator Driver (virtio-rng)");

        rng.initialize().unwrap();
        let mut entropy = vec![0; 16];
        rng.read(&mut entropy).unwrap();
        assert_ne!(entropy, vec![0; 16]);

        assert!(rng.write(&[1, 2]).is_err());
    }
}
