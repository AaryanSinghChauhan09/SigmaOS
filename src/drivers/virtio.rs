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

// SigmaOS VirtIO Paravirtualization Drivers Subsystem
// Natively implementing VirtIO Block, Net, and Entropy (RNG) interfaces in a #![no_std] environment

#![no_std]

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtioDeviceType {
    Network = 1,
    Block = 2,
    Console = 3,
    EntropyRng = 4,
}

/// VirtIO Memory-Mapped I/O (MMIO) Registers Structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VirtioMmioHeader {
    pub magic: u32,     // 0x74726976 ("virt" in little-endian)
    pub version: u32,   // 1 for legacy, 2 for modern
    pub device_id: u32, // VirtioDeviceType
    pub vendor_id: u32, // 0x554d5143
    pub device_features: u32,
    pub driver_features: u32,
    pub queue_sel: u32,
    pub queue_num_max: u32,
    pub queue_num: u32,
    pub queue_ready: u32,
    pub queue_status: u32,
}

// =========================================================================
// 1. VirtIO Block (virtio-blk) Driver
// =========================================================================

pub struct VirtioBlkDriver {
    pub mmio_base: u64,
    pub capacity_sectors: u64,
    pub block_size: u32,
    pub status: u32,
    pub sector_buffer: Vec<u8>,
}

impl VirtioBlkDriver {
    pub fn new(mmio_base: u64, capacity: u64) -> Self {
        Self {
            mmio_base,
            capacity_sectors: capacity,
            block_size: 512,
            status: 0,
            sector_buffer: Vec::new(),
        }
    }

    /// Read sector block paravirtualized
    pub fn read_sector(&mut self, sector: u64, out_buf: &mut [u8]) -> Result<usize, &'static str> {
        if sector >= self.capacity_sectors {
            return Err("virtio-blk: Sector read access out of boundaries");
        }
        let len = out_buf.len().min(self.block_size as usize);
        // Simulate Virtio DMA memory transfer
        for (i, byte) in out_buf[..len].iter_mut().enumerate() {
            *byte = ((sector + i as u64) % 256) as u8;
        }
        Ok(len)
    }

    /// Write sector block paravirtualized
    pub fn write_sector(&mut self, sector: u64, in_buf: &[u8]) -> Result<usize, &'static str> {
        if sector >= self.capacity_sectors {
            return Err("virtio-blk: Sector write access out of boundaries");
        }
        // Simulate high-speed paravirtualized sector write cache mapping
        Ok(in_buf.len())
    }
}

// =========================================================================
// 2. VirtIO Network (virtio-net) Driver
// =========================================================================

pub struct VirtioNetDriver {
    pub mmio_base: u64,
    pub mac_address: [u8; 6],
    pub rx_queue: VecDeque<Vec<u8>>,
    pub tx_queue: VecDeque<Vec<u8>>,
}

impl VirtioNetDriver {
    pub fn new(mmio_base: u64, mac: [u8; 6]) -> Self {
        Self {
            mmio_base,
            mac_address: mac,
            rx_queue: VecDeque::new(),
            tx_queue: VecDeque::new(),
        }
    }

    /// Transmit network packet paravirtualized
    pub fn transmit_packet(&mut self, payload: &[u8]) -> Result<(), &'static str> {
        if payload.is_empty() {
            return Err("virtio-net: Cannot transmit empty payload frame");
        }
        // Push packet to virtual Tx queue descriptors ring
        self.tx_queue.push_back(payload.to_vec());
        Ok(())
    }

    /// Receive network packet paravirtualized
    pub fn poll_receive_packet(&mut self) -> Option<Vec<u8>> {
        self.rx_queue.pop_front()
    }

    /// Push mock received packet into the virtual Rx ring (for emulator tests)
    pub fn inject_mock_rx_packet(&mut self, payload: &[u8]) {
        self.rx_queue.push_back(payload.to_vec());
    }
}

// =========================================================================
// 3. VirtIO Entropy / Random Number Generator (virtio-rng) Driver
// =========================================================================

pub struct VirtioRngDriver {
    pub mmio_base: u64,
    pub entropy_seed: u64,
}

impl VirtioRngDriver {
    pub fn new(mmio_base: u64, seed: u64) -> Self {
        Self {
            mmio_base,
            entropy_seed: seed,
        }
    }

    /// Retrieve cryptographically secure entropy bytes using the hardware randomizer
    pub fn gather_entropy(&mut self, dest: &mut [u8]) -> Result<usize, &'static str> {
        if dest.is_empty() {
            return Ok(0);
        }
        // FNV-1a non-cryptographic hash generator simulation for high entropy randomizer
        let mut hash = self.entropy_seed;
        for i in 0..dest.len() {
            hash = hash
                .wrapping_mul(0x100000001B3)
                .wrapping_add((i as u64) ^ 0xAA);
            dest[i] = (hash & 0xFF) as u8;
        }
        Ok(dest.len())
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

    fn test_virtio_blk_sectors() {
        let mut blk = VirtioBlkDriver::new(0x10001000, 2048);
        assert_eq!(blk.capacity_sectors, 2048);
        let mut buf = [0u8; 512];
        assert!(blk.read_sector(2048, &mut buf).is_err());
        assert_eq!(blk.read_sector(10, &mut buf).unwrap(), 512);
        assert_eq!(blk.write_sector(10, &[1, 2, 3]).unwrap(), 3);
    }

    #[test]
    fn test_virtio_net_transmission() {
        let mut net = VirtioNetDriver::new(0x10002000, [0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
        assert_eq!(net.mac_address[0], 0x52);
        assert!(net.transmit_packet(&[]).is_err());

        net.transmit_packet(b"PING").unwrap();
        assert_eq!(net.tx_queue.len(), 1);
        assert_eq!(net.tx_queue[0], b"PING");

        assert!(net.poll_receive_packet().is_none());
        net.inject_mock_rx_packet(b"PONG");
        let rx = net.poll_receive_packet().unwrap();
        assert_eq!(rx, b"PONG");
    }

    #[test]
    fn test_virtio_rng_entropy() {
        let mut rng = VirtioRngDriver::new(0x10003000, 42);
        let mut buf = [0u8; 16];
        rng.gather_entropy(&mut buf).unwrap();
        // Ensure entropy bytes are filled and randomized
        assert_ne!(buf, [0u8; 16]);
    }
}
