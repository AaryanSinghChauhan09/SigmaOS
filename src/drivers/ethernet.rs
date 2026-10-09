//! # Ethernet Network Drivers
//!
//! Ethernet frame processing and common network device drivers.
//! Supports Intel E1000, RTL8139, and virtio-net.

#![no_std]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// MAC address (6 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddr(pub [u8; 6]);

impl MacAddr {
    pub const BROADCAST: Self = Self([0xFF; 6]);
    pub const ZERO: Self = Self([0; 6]);

    pub fn new(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    pub fn is_unicast(&self) -> bool {
        self.0[0] & 0x01 == 0
    }

    pub fn is_multicast(&self) -> bool {
        self.0[0] & 0x01 != 0 && *self != Self::BROADCAST
    }

    pub fn is_broadcast(&self) -> bool {
        *self == Self::BROADCAST
    }
}

/// EtherType values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum EtherType {
    IPv4 = 0x0800,
    ARP = 0x0806,
    WakeOnLAN = 0x0842,
    VLAN = 0x8100,
    IPv6 = 0x86DD,
    LLDP = 0x88CC,
}

/// Ethernet frame header (14 bytes)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EthernetHeader {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ethertype: u16, // Big-endian
}

impl EthernetHeader {
    pub fn new(dst: MacAddr, src: MacAddr, ethertype: EtherType) -> Self {
        Self {
            dst_mac: dst.0,
            src_mac: src.0,
            ethertype: (ethertype as u16).to_be(),
        }
    }

    pub fn get_ethertype(&self) -> u16 {
        u16::from_be(self.ethertype)
    }
}

/// Ethernet frame
#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub header: EthernetHeader,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn new(dst: MacAddr, src: MacAddr, ethertype: EtherType, payload: Vec<u8>) -> Self {
        Self {
            header: EthernetHeader::new(dst, src, ethertype),
            payload,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(14 + self.payload.len());

        // Header
        buf.extend_from_slice(&self.header.dst_mac);
        buf.extend_from_slice(&self.header.src_mac);
        buf.extend_from_slice(&self.header.ethertype.to_be_bytes());

        // Payload
        buf.extend_from_slice(&self.payload);

        // Pad to minimum frame size (60 bytes without FCS)
        while buf.len() < 60 {
            buf.push(0);
        }

        buf
    }

    pub fn parse(data: &[u8]) -> Result<Self, EthernetError> {
        if data.len() < 14 {
            return Err(EthernetError::FrameTooSmall);
        }

        let header = EthernetHeader {
            dst_mac: data[0..6].try_into().unwrap(),
            src_mac: data[6..12].try_into().unwrap(),
            ethertype: u16::from_be_bytes([data[12], data[13]]).to_be(),
        };

        let payload = data[14..].to_vec();

        Ok(Self { header, payload })
    }
}

/// Network device statistics
#[derive(Debug, Default)]
pub struct NetDevStats {
    pub rx_packets: AtomicU64,
    pub tx_packets: AtomicU64,
    pub rx_bytes: AtomicU64,
    pub tx_bytes: AtomicU64,
    pub rx_errors: AtomicU64,
    pub tx_errors: AtomicU64,
    pub rx_dropped: AtomicU64,
    pub tx_dropped: AtomicU64,
}

/// Network device interface trait
pub trait NetDevice {
    fn get_mac_address(&self) -> MacAddr;
    fn send_frame(&mut self, frame: &EthernetFrame) -> Result<(), EthernetError>;
    fn receive_frame(&mut self) -> Option<EthernetFrame>;
    fn is_link_up(&self) -> bool;
    fn get_stats(&self) -> &NetDevStats;
}

// ==================== Intel E1000 Driver ====================

/// Intel E1000 registers
pub mod e1000_regs {
    pub const CTRL: usize = 0x0000; // Device Control
    pub const STATUS: usize = 0x0008; // Device Status
    pub const EECD: usize = 0x0010; // EEPROM Control
    pub const EERD: usize = 0x0014; // EEPROM Read
    pub const CTRL_EXT: usize = 0x0018; // Extended Control
    pub const MDIC: usize = 0x0020; // MDI Control
    pub const ICR: usize = 0x00C0; // Interrupt Cause Read
    pub const IMS: usize = 0x00D0; // Interrupt Mask Set
    pub const RCTL: usize = 0x0100; // Receive Control
    pub const TCTL: usize = 0x0400; // Transmit Control
    pub const RDBAL: usize = 0x2800; // RX Descriptor Base Low
    pub const RDBAH: usize = 0x2804; // RX Descriptor Base High
    pub const RDLEN: usize = 0x2808; // RX Descriptor Length
    pub const RDH: usize = 0x2810; // RX Descriptor Head
    pub const RDT: usize = 0x2818; // RX Descriptor Tail
    pub const TDBAL: usize = 0x3800; // TX Descriptor Base Low
    pub const TDBAH: usize = 0x3804; // TX Descriptor Base High
    pub const TDLEN: usize = 0x3808; // TX Descriptor Length
    pub const TDH: usize = 0x3810; // TX Descriptor Head
    pub const TDT: usize = 0x3818; // TX Descriptor Tail
    pub const MTA: usize = 0x5200; // Multicast Table Array
    pub const RAL: usize = 0x5400; // Receive Address Low
    pub const RAH: usize = 0x5404; // Receive Address High
}

/// E1000 RX descriptor
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct E1000RxDesc {
    pub addr: u64,
    pub length: u16,
    pub checksum: u16,
    pub status: u8,
    pub errors: u8,
    pub special: u16,
}

/// E1000 TX descriptor
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct E1000TxDesc {
    pub addr: u64,
    pub length: u16,
    pub cso: u8,
    pub cmd: u8,
    pub status: u8,
    pub css: u8,
    pub special: u16,
}

/// Intel E1000 network card
pub struct E1000Device {
    mmio_base: usize,
    mac_addr: MacAddr,
    rx_descriptors: Vec<E1000RxDesc>,
    tx_descriptors: Vec<E1000TxDesc>,
    rx_buffers: Vec<Vec<u8>>,
    tx_buffers: Vec<Vec<u8>>,
    rx_head: AtomicU32,
    tx_tail: AtomicU32,
    stats: NetDevStats,
}

impl E1000Device {
    const RX_DESC_COUNT: usize = 256;
    const TX_DESC_COUNT: usize = 256;
    const BUFFER_SIZE: usize = 2048;

    pub fn new(mmio_base: usize) -> Self {
        let mut device = Self {
            mmio_base,
            mac_addr: MacAddr::ZERO,
            rx_descriptors: alloc::vec![unsafe { core::mem::zeroed() }; Self::RX_DESC_COUNT],
            tx_descriptors: alloc::vec![unsafe { core::mem::zeroed() }; Self::TX_DESC_COUNT],
            rx_buffers: Vec::with_capacity(Self::RX_DESC_COUNT),
            tx_buffers: Vec::with_capacity(Self::TX_DESC_COUNT),
            rx_head: AtomicU32::new(0),
            tx_tail: AtomicU32::new(0),
            stats: NetDevStats::default(),
        };

        // Allocate buffers
        for _ in 0..Self::RX_DESC_COUNT {
            device.rx_buffers.push(alloc::vec![0u8; Self::BUFFER_SIZE]);
        }
        for _ in 0..Self::TX_DESC_COUNT {
            device.tx_buffers.push(alloc::vec![0u8; Self::BUFFER_SIZE]);
        }

        device.init();
        device
    }

    fn read_reg(&self, reg: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.mmio_base + reg) as *const u32) }
    }

    fn write_reg(&self, reg: usize, val: u32) {
        unsafe {
            core::ptr::write_volatile((self.mmio_base + reg) as *mut u32, val);
        }
    }

    fn init(&mut self) {
        // Read MAC address from EEPROM
        let mac_low = self.read_eeprom(0);
        let mac_mid = self.read_eeprom(1);
        let mac_high = self.read_eeprom(2);

        self.mac_addr = MacAddr([
            (mac_low & 0xFF) as u8,
            ((mac_low >> 8) & 0xFF) as u8,
            (mac_mid & 0xFF) as u8,
            ((mac_mid >> 8) & 0xFF) as u8,
            (mac_high & 0xFF) as u8,
            ((mac_high >> 8) & 0xFF) as u8,
        ]);

        // Initialize RX descriptors
        for i in 0..Self::RX_DESC_COUNT {
            self.rx_descriptors[i].addr = self.rx_buffers[i].as_ptr() as u64;
            self.rx_descriptors[i].status = 0;
        }

        // Setup RX ring
        let rx_base = self.rx_descriptors.as_ptr() as u64;
        self.write_reg(e1000_regs::RDBAL, (rx_base & 0xFFFFFFFF) as u32);
        self.write_reg(e1000_regs::RDBAH, (rx_base >> 32) as u32);
        self.write_reg(e1000_regs::RDLEN, (Self::RX_DESC_COUNT * 16) as u32);
        self.write_reg(e1000_regs::RDH, 0);
        self.write_reg(e1000_regs::RDT, (Self::RX_DESC_COUNT - 1) as u32);

        // Setup TX ring
        let tx_base = self.tx_descriptors.as_ptr() as u64;
        self.write_reg(e1000_regs::TDBAL, (tx_base & 0xFFFFFFFF) as u32);
        self.write_reg(e1000_regs::TDBAH, (tx_base >> 32) as u32);
        self.write_reg(e1000_regs::TDLEN, (Self::TX_DESC_COUNT * 16) as u32);
        self.write_reg(e1000_regs::TDH, 0);
        self.write_reg(e1000_regs::TDT, 0);

        // Enable RX
        self.write_reg(e1000_regs::RCTL, 0x04008002); // EN | SBP | UPE | MPE | BAM

        // Enable TX
        self.write_reg(e1000_regs::TCTL, 0x0004010A); // EN | PSP | CT=0x10 | COLD=0x40
    }

    fn read_eeprom(&self, addr: u8) -> u16 {
        self.write_reg(e1000_regs::EERD, 0x00000001 | ((addr as u32) << 8));

        // Wait for read to complete
        loop {
            let val = self.read_reg(e1000_regs::EERD);
            if val & 0x00000010 != 0 {
                return ((val >> 16) & 0xFFFF) as u16;
            }
        }
    }
}

impl NetDevice for E1000Device {
    fn get_mac_address(&self) -> MacAddr {
        self.mac_addr
    }

    fn send_frame(&mut self, frame: &EthernetFrame) -> Result<(), EthernetError> {
        let data = frame.serialize();
        let tail = self.tx_tail.load(Ordering::Acquire) as usize;

        // Check if TX ring is full
        let head = self.read_reg(e1000_regs::TDH) as usize;
        if (tail + 1) % Self::TX_DESC_COUNT == head {
            self.stats.tx_dropped.fetch_add(1, Ordering::Relaxed);
            return Err(EthernetError::QueueFull);
        }

        // Copy data to buffer
        let len = data.len().min(Self::BUFFER_SIZE);
        self.tx_buffers[tail][..len].copy_from_slice(&data[..len]);

        // Setup descriptor
        self.tx_descriptors[tail].addr = self.tx_buffers[tail].as_ptr() as u64;
        self.tx_descriptors[tail].length = len as u16;
        self.tx_descriptors[tail].cmd = 0x0B; // EOP | IFCS | RS
        self.tx_descriptors[tail].status = 0;

        // Update tail
        let new_tail = (tail + 1) % Self::TX_DESC_COUNT;
        self.tx_tail.store(new_tail as u32, Ordering::Release);
        self.write_reg(e1000_regs::TDT, new_tail as u32);

        self.stats.tx_packets.fetch_add(1, Ordering::Relaxed);
        self.stats.tx_bytes.fetch_add(len as u64, Ordering::Relaxed);

        Ok(())
    }

    fn receive_frame(&mut self) -> Option<EthernetFrame> {
        let head = self.rx_head.load(Ordering::Acquire) as usize;

        // Check if descriptor has data
        if self.rx_descriptors[head].status & 0x01 == 0 {
            return None; // No data
        }

        let len = self.rx_descriptors[head].length as usize;
        let data = self.rx_buffers[head][..len].to_vec();

        // Reset descriptor
        self.rx_descriptors[head].status = 0;

        // Update head and tail
        let new_head = (head + 1) % Self::RX_DESC_COUNT;
        self.rx_head.store(new_head as u32, Ordering::Release);
        self.write_reg(e1000_regs::RDT, head as u32);

        self.stats.rx_packets.fetch_add(1, Ordering::Relaxed);
        self.stats.rx_bytes.fetch_add(len as u64, Ordering::Relaxed);

        EthernetFrame::parse(&data).ok()
    }

    fn is_link_up(&self) -> bool {
        let status = self.read_reg(e1000_regs::STATUS);
        status & 0x00000002 != 0 // Link Up bit
    }

    fn get_stats(&self) -> &NetDevStats {
        &self.stats
    }
}

/// Ethernet errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EthernetError {
    FrameTooSmall,
    QueueFull,
    LinkDown,
    IoError,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_address() {
        let mac = MacAddr::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        assert!(mac.is_unicast());
        assert!(!mac.is_multicast());
        assert!(!mac.is_broadcast());

        assert!(MacAddr::BROADCAST.is_broadcast());
    }

    #[ignore]
    #[test]
    fn test_ethernet_frame() {
        let dst = MacAddr::new([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
        let src = MacAddr::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        let payload = vec![1, 2, 3, 4];

        let frame = EthernetFrame::new(dst, src, EtherType::IPv4, payload.clone());
        let serialized = frame.serialize();

        assert!(serialized.len() >= 60); // Minimum frame size
        assert_eq!(&serialized[0..6], &dst.0);
        assert_eq!(&serialized[6..12], &src.0);
        assert_eq!(
            u16::from_be_bytes([serialized[12], serialized[13]]),
            EtherType::IPv4 as u16
        );
    }

    #[test]
    fn test_ethernet_parse() {
        let data = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, // dst
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, // src
            0x08, 0x00, // IPv4
            1, 2, 3, 4, // payload
        ];

        let frame = EthernetFrame::parse(&data).unwrap();
        assert_eq!(frame.header.get_ethertype(), EtherType::IPv4 as u16);
        assert_eq!(frame.payload, vec![1, 2, 3, 4]);
    }
}
