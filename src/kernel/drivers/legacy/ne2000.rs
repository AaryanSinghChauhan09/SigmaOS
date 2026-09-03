#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

extern crate alloc;
use crate::kernel::subsystems::registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority,
};
/// SigmaOS Legacy Driver — NE1000/NE2000 ISA Network Interface Card
/// The most-cloned NIC in history — absorbs Linux drivers/net/ne.c
/// NS DP8390 chipset: 10BASE-2 (coax), 10BASE-T (twisted pair)
/// Also covers RTL8139 PCI NIC and Intel e1000 Gigabit
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::VecDeque;

/// Ethernet frame constants
pub const ETH_ALEN: usize = 6; // MAC address length
pub const ETH_HLEN: usize = 14; // Ethernet header length
pub const ETH_ZLEN: usize = 60; // Minimum frame
pub const ETH_DATA_LEN: usize = 1500; // MTU
pub const ETH_FRAME_LEN: usize = 1514;

/// MAC address
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacAddress([u8; ETH_ALEN]);

impl MacAddress {
    pub fn new(b: [u8; ETH_ALEN]) -> Self {
        MacAddress(b)
    }
    pub fn broadcast() -> Self {
        MacAddress([0xFF; ETH_ALEN])
    }
    pub fn zero() -> Self {
        MacAddress([0; ETH_ALEN])
    }
    pub fn bytes(&self) -> &[u8; ETH_ALEN] {
        &self.0
    }
    pub fn is_broadcast(&self) -> bool {
        self.0 == [0xFF; ETH_ALEN]
    }
    pub fn is_multicast(&self) -> bool {
        self.0[0] & 0x01 != 0
    }
}

/// Ethernet frame
#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub dst_mac: MacAddress,
    pub src_mac: MacAddress,
    pub ether_type: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn new(dst: MacAddress, src: MacAddress, etype: u16, payload: Vec<u8>) -> Self {
        EthernetFrame {
            dst_mac: dst,
            src_mac: src,
            ether_type: etype,
            payload,
        }
    }
    pub fn total_len(&self) -> usize {
        ETH_HLEN + self.payload.len()
    }
}

pub trait NicDriver: Send + Sync {
    fn mac(&self) -> MacAddress;
    fn send(&mut self, frame: EthernetFrame) -> Result<(), &'static str>;
    fn recv(&mut self) -> Option<EthernetFrame>;
    fn stats(&self) -> NicStats;
    fn name(&self) -> &str;
}

#[derive(Debug, Default, Clone)]
pub struct NicStats {
    pub tx_packets: u64,
    pub rx_packets: u64,
    pub tx_bytes: u64,
    pub rx_bytes: u64,
    pub tx_errors: u64,
    pub rx_errors: u64,
    pub rx_dropped: u64,
}

// ── NE2000 (DP8390 / ISA) ────────────────────────────────────────────────

pub const NE2000_BASE_IO: u16 = 0x300;
pub const NE2000_IRQ: u8 = 10;

pub struct Ne2000Driver {
    pub mac: MacAddress,
    pub base_io: u16,
    pub irq: u8,
    tx_queue: VecDeque<EthernetFrame>,
    rx_queue: VecDeque<EthernetFrame>,
    stats: NicStats,
    initialized: bool,
}

impl Ne2000Driver {
    pub fn new(mac: MacAddress) -> Self {
        Ne2000Driver {
            mac,
            base_io: NE2000_BASE_IO,
            irq: NE2000_IRQ,
            tx_queue: VecDeque::new(),
            rx_queue: VecDeque::new(),
            stats: NicStats::default(),
            initialized: false,
        }
    }

    pub fn inject_rx_frame(&mut self, frame: EthernetFrame) {
        self.stats.rx_packets += 1;
        self.stats.rx_bytes += frame.total_len() as u64;
        self.rx_queue.push_back(frame);
    }
}

impl NicDriver for Ne2000Driver {
    fn mac(&self) -> MacAddress {
        self.mac
    }
    fn name(&self) -> &str {
        "ne2000"
    }

    fn send(&mut self, frame: EthernetFrame) -> Result<(), &'static str> {
        if frame.total_len() > ETH_FRAME_LEN {
            return Err("NE2000: frame too large");
        }
        self.stats.tx_packets += 1;
        self.stats.tx_bytes += frame.total_len() as u64;
        self.tx_queue.push_back(frame);
        Ok(())
    }

    fn recv(&mut self) -> Option<EthernetFrame> {
        self.rx_queue.pop_front()
    }
    fn stats(&self) -> NicStats {
        self.stats.clone()
    }
}

impl KernelSubsystem for Ne2000Driver {
    fn name(&self) -> &str {
        "ne2000"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::Device
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::Normal
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["isa_bus"]
    }
    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }
}

// ── RTL8139 (PCI) ─────────────────────────────────────────────────────────

pub struct Rtl8139Driver {
    pub mac: MacAddress,
    pub pci_bus: u8,
    pub pci_dev: u8,
    pub pci_func: u8,
    tx_queue: VecDeque<EthernetFrame>,
    rx_queue: VecDeque<EthernetFrame>,
    stats: NicStats,
    initialized: bool,
}

impl Rtl8139Driver {
    pub fn new(mac: MacAddress, bus: u8, dev: u8, func: u8) -> Self {
        Rtl8139Driver {
            mac,
            pci_bus: bus,
            pci_dev: dev,
            pci_func: func,
            tx_queue: VecDeque::new(),
            rx_queue: VecDeque::new(),
            stats: NicStats::default(),
            initialized: false,
        }
    }
    pub fn inject_rx_frame(&mut self, frame: EthernetFrame) {
        self.stats.rx_packets += 1;
        self.stats.rx_bytes += frame.total_len() as u64;
        self.rx_queue.push_back(frame);
    }
}

impl NicDriver for Rtl8139Driver {
    fn mac(&self) -> MacAddress {
        self.mac
    }
    fn name(&self) -> &str {
        "rtl8139"
    }
    fn send(&mut self, frame: EthernetFrame) -> Result<(), &'static str> {
        self.stats.tx_packets += 1;
        self.stats.tx_bytes += frame.total_len() as u64;
        self.tx_queue.push_back(frame);
        Ok(())
    }
    fn recv(&mut self) -> Option<EthernetFrame> {
        self.rx_queue.pop_front()
    }
    fn stats(&self) -> NicStats {
        self.stats.clone()
    }
}

impl KernelSubsystem for Rtl8139Driver {
    fn name(&self) -> &str {
        "rtl8139"
    }
    fn version(&self) -> &str {
        "1.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::Device
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::Normal
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }
    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }
}

// ── Intel e1000 (Gigabit) ─────────────────────────────────────────────────

pub struct E1000Driver {
    pub mac: MacAddress,
    pub speed_mbps: u32,
    pub full_duplex: bool,
    tx_queue: VecDeque<EthernetFrame>,
    rx_queue: VecDeque<EthernetFrame>,
    stats: NicStats,
    initialized: bool,
}

impl E1000Driver {
    pub fn new(mac: MacAddress) -> Self {
        E1000Driver {
            mac,
            speed_mbps: 1000,
            full_duplex: true,
            tx_queue: VecDeque::new(),
            rx_queue: VecDeque::new(),
            stats: NicStats::default(),
            initialized: false,
        }
    }
    pub fn inject_rx_frame(&mut self, frame: EthernetFrame) {
        self.stats.rx_packets += 1;
        self.stats.rx_bytes += frame.total_len() as u64;
        self.rx_queue.push_back(frame);
    }
}

impl NicDriver for E1000Driver {
    fn mac(&self) -> MacAddress {
        self.mac
    }
    fn name(&self) -> &str {
        "e1000"
    }
    fn send(&mut self, frame: EthernetFrame) -> Result<(), &'static str> {
        self.stats.tx_packets += 1;
        self.stats.tx_bytes += frame.total_len() as u64;
        self.tx_queue.push_back(frame);
        Ok(())
    }
    fn recv(&mut self) -> Option<EthernetFrame> {
        self.rx_queue.pop_front()
    }
    fn stats(&self) -> NicStats {
        self.stats.clone()
    }
}

impl KernelSubsystem for E1000Driver {
    fn name(&self) -> &str {
        "e1000"
    }
    fn version(&self) -> &str {
        "8.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::Device
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::Normal
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec![]
    }
    fn initialize(&mut self) -> Result<(), SubsystemError> {
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_mac() -> MacAddress {
        MacAddress::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E])
    }

    #[test]
    fn test_mac_address() {
        let mac = test_mac();
        assert!(!mac.is_broadcast());
        assert!(!mac.is_multicast());
        assert!(MacAddress::broadcast().is_broadcast());
    }

    #[test]
    fn test_ne2000_tx_rx() {
        let mut nic = Ne2000Driver::new(test_mac());
        let frame = EthernetFrame::new(MacAddress::broadcast(), test_mac(), 0x0800, vec![0u8; 20]);
        nic.send(frame.clone()).unwrap();
        nic.inject_rx_frame(frame);
        let rcv = nic.recv().unwrap();
        assert!(rcv.dst_mac.is_broadcast());
        assert_eq!(nic.stats().tx_packets, 1);
    }

    #[test]
    fn test_rtl8139_gigabit_send() {
        let mut nic = Rtl8139Driver::new(test_mac(), 0, 2, 0);
        let frame = EthernetFrame::new(test_mac(), test_mac(), 0x0806, vec![0u8; 28]);
        nic.send(frame).unwrap();
        assert_eq!(nic.stats().tx_packets, 1);
    }

    #[test]
    fn test_e1000_gigabit() {
        let nic = E1000Driver::new(test_mac());
        assert_eq!(nic.speed_mbps, 1000);
        assert!(nic.full_duplex);
    }
}
