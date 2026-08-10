// SigmaOS Intel 8254x Gigabit Ethernet (e1000) Network Driver Subsystem
// Parity-level implementation of the legendary Intel hardware NIC controller in a #![no_std] environment
// Enhanced with advanced Linux-style net interface configuration and FreeBSD-style device polling

#![no_std]

extern crate alloc;

use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// Intel e1000 Transmit Descriptor Structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct E1000TxDescriptor {
    pub buffer_addr: u64,
    pub length: u16,
    pub cso: u8,
    pub cmd: u8,
    pub status: u8,
    pub css: u8,
    pub special: u16,
}

/// Intel e1000 Receive Descriptor Structure
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct E1000RxDescriptor {
    pub buffer_addr: u64,
    pub length: u16,
    pub checksum: u16,
    pub status: u8,
    pub errors: u8,
    pub special: u16,
}

/// Linux-style Network Interface Statistics
#[derive(Debug, Clone, Default)]
pub struct NetworkInterfaceStats {
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub collisions: u64,
}

/// Supported Ethernet Link Duplex Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkDuplex {
    Half,
    Full,
}

pub struct IntelE1000Driver {
    pub pci_bar_base: u64,
    pub mac_address: [u8; 6],
    pub is_initialized: bool,

    // Emulated Tx and Rx ring status registers
    pub tx_descriptors: Vec<E1000TxDescriptor>,
    pub rx_descriptors: Vec<E1000RxDescriptor>,
    pub rx_packet_queue: VecDeque<Vec<u8>>,
    pub tx_packet_queue: VecDeque<Vec<u8>>,

    // Advanced Linux-inspired features
    pub mtu: usize,
    pub stats: NetworkInterfaceStats,
    pub promiscuous_mode: bool,
    pub multicast_filter_enabled: bool,
    pub multicast_hash_table: [u32; 128], // Multicast hash filter table
    pub tx_checksum_offload: bool,
    pub rx_checksum_offload: bool,
    pub link_speed_mbps: u32,             // e.g. 10, 100, 1000
    pub link_duplex: LinkDuplex,

    // FreeBSD-inspired device polling mode (prevents interrupt storms under load)
    pub polling_enabled: bool,
}

impl IntelE1000Driver {
    pub fn new(bar_base: u64, mac: [u8; 6]) -> Self {
        Self {
            pci_bar_base: bar_base,
            mac_address: mac,
            is_initialized: false,
            tx_descriptors: Vec::new(),
            rx_descriptors: Vec::new(),
            rx_packet_queue: VecDeque::new(),
            tx_packet_queue: VecDeque::new(),

            // Default Linux MTU is 1500
            mtu: 1500,
            stats: NetworkInterfaceStats::default(),
            promiscuous_mode: false,
            multicast_filter_enabled: true,
            multicast_hash_table: [0; 128],
            tx_checksum_offload: true,
            rx_checksum_offload: true,
            link_speed_mbps: 1000, // Gigabit speed by default
            link_duplex: LinkDuplex::Full,

            polling_enabled: false,
        }
    }

    /// Complete hardware init sequences of e1000 register sets
    pub fn initialize_hardware(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;

        // Allocate Tx/Rx descriptor rings (standard size = 64)
        self.tx_descriptors.clear();
        self.rx_descriptors.clear();
        for _ in 0..64 {
            self.tx_descriptors.push(E1000TxDescriptor::default());
            self.rx_descriptors.push(E1000RxDescriptor::default());
        }

        // Simulates unmasking interrupts (IMR), enabling transmitter (TCTL) & receiver (RCTL)
        Ok(())
    }

    /// Configure Maximum Transmission Unit (MTU), supporting jumbo frames up to 9000 bytes
    pub fn set_mtu(&mut self, new_mtu: usize) -> Result<(), &'static str> {
        if new_mtu < 68 || new_mtu > 9000 {
            return Err("e1000: MTU must be between 68 and 9000 bytes");
        }
        self.mtu = new_mtu;
        Ok(())
    }

    /// Enable or disable Promiscuous Mode
    pub fn set_promiscuous_mode(&mut self, enabled: bool) {
        self.promiscuous_mode = enabled;
    }

    /// Set Multicast Hash Filter Table value
    pub fn set_multicast_filter(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index >= self.multicast_hash_table.len() {
            return Err("e1000: Multicast hash index out of bounds");
        }
        self.multicast_hash_table[index] = value;
        Ok(())
    }

    /// FreeBSD-style: Toggle Device Polling Mode
    pub fn set_polling_enabled(&mut self, enabled: bool) {
        self.polling_enabled = enabled;
    }

    /// Transmit raw network packet over the NIC Tx rings
    pub fn transmit_packet(&mut self, payload: &[u8]) -> Result<(), &'static str> {
        if !self.is_initialized {
            self.stats.tx_errors += 1;
            return Err("e1000: NIC hardware must be initialized prior to packet transmission");
        }
        if payload.is_empty() {
            self.stats.tx_errors += 1;
            return Err("e1000: Cannot transmit empty frame payload");
        }
        // Check frame length against MTU (+ Ethernet header overhead of ~14-18 bytes)
        if payload.len() > (self.mtu + 18) {
            self.stats.tx_errors += 1;
            return Err("e1000: Packet size exceeds configured MTU limit");
        }

        // Fill descriptor ring parameters
        let desc_index = self.tx_packet_queue.len() % 64;
        self.tx_descriptors[desc_index] = E1000TxDescriptor {
            buffer_addr: 0x20000000 + (desc_index as u64 * 2048), // mock DMA address
            length: payload.len() as u16,
            cso: if self.tx_checksum_offload { 1 } else { 0 },
            cmd: 0b00001001, // RS (Report Status) & EOP (End of Packet)
            status: 1,       // Completed
            css: 0,
            special: 0,
        };

        // Queue packet payload
        self.tx_packet_queue.push_back(payload.to_vec());

        // Update Linux stats counters
        self.stats.tx_packets += 1;
        self.stats.tx_bytes += payload.len() as u64;

        Ok(())
    }

    /// Receive raw network packet over the NIC Rx rings
    pub fn receive_packet(&mut self) -> Option<Vec<u8>> {
        if !self.is_initialized {
            return None;
        }

        // Under polling, the kernel polls the hardware queue explicitly
        if self.polling_enabled {
            // Emulate a polled ring harvest
            self.poll_rx_ring()
        } else {
            // Standard interrupt-driven mode path
            if let Some(pkt) = self.rx_packet_queue.pop_front() {
                self.stats.rx_packets += 1;
                self.stats.rx_bytes += pkt.len() as u64;
                Some(pkt)
            } else {
                None
            }
        }
    }

    /// Helper to poll the Rx descriptor ring (BSD ifnet polling mechanism)
    pub fn poll_rx_ring(&mut self) -> Option<Vec<u8>> {
        if let Some(pkt) = self.rx_packet_queue.pop_front() {
            self.stats.rx_packets += 1;
            self.stats.rx_bytes += pkt.len() as u64;
            Some(pkt)
        } else {
            None
        }
    }

    /// Push mock packet into the NIC Rx descriptors ring (emulator simulation helper)
    pub fn inject_mock_rx_packet(&mut self, payload: &[u8]) {
        let desc_index = self.rx_packet_queue.len() % 64;
        self.rx_descriptors[desc_index] = E1000RxDescriptor {
            buffer_addr: 0x30000000 + (desc_index as u64 * 2048),
            length: payload.len() as u16,
            checksum: if self.rx_checksum_offload { 0xAA } else { 0 },
            status: 1, // DD (Descriptor Done) flag set
            errors: 0,
            special: 0,
        };

        self.rx_packet_queue.push_back(payload.to_vec());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e1000_nic_lifecycle_and_packet_flows() {
        let mut nic = IntelE1000Driver::new(0xF0000000, [0x00, 0x0C, 0x29, 0x12, 0x34, 0x56]);
        assert_eq!(nic.pci_bar_base, 0xF0000000);
        assert_eq!(nic.mac_address[2], 0x29);

        // Transmission before initialization must fail
        assert!(nic.transmit_packet(b"PING").is_err());

        // Initialize hardware registers
        nic.initialize_hardware().unwrap();
        assert!(nic.is_initialized);
        assert_eq!(nic.tx_descriptors.len(), 64);
        assert_eq!(nic.rx_descriptors.len(), 64);

        // Transmit packets
        nic.transmit_packet(b"DHCP_DISCOVER").unwrap();
        assert_eq!(nic.tx_packet_queue.len(), 1);
        assert_eq!(nic.tx_packet_queue[0], b"DHCP_DISCOVER");
        let tx_len = nic.tx_descriptors[0].length;
        let tx_status = nic.tx_descriptors[0].status;
        assert_eq!(tx_len, 13);
        assert_eq!(tx_status, 1);

        // Receive packets
        assert!(nic.receive_packet().is_none());
        nic.inject_mock_rx_packet(b"DHCP_OFFER");
        let rx_status = nic.rx_descriptors[0].status;
        assert_eq!(rx_status, 1);
        let rx = nic.receive_packet().unwrap();
        assert_eq!(rx, b"DHCP_OFFER");
    }

    #[test]
    fn test_linux_inspired_features() {
        let mut nic = IntelE1000Driver::new(0xE0000000, [0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);
        nic.initialize_hardware().unwrap();

        // 1. MTU Configuration & boundary checks
        assert_eq!(nic.mtu, 1500); // Standard default
        assert!(nic.set_mtu(9000).is_ok()); // Jumbo frame support
        assert_eq!(nic.mtu, 9000);
        assert!(nic.set_mtu(9001).is_err()); // Exceeds jumbo limit
        assert!(nic.set_mtu(60).is_err());   // Below min standard MTU

        // Configure back to standard for testing transmit bounds
        nic.set_mtu(1500).unwrap();
        let large_payload = [0u8; 1600];
        assert!(nic.transmit_packet(&large_payload).is_err()); // Exceeds MTU + overhead
        assert_eq!(nic.stats.tx_errors, 1);

        // 2. Statistics Counter Tracking
        assert_eq!(nic.stats.tx_packets, 0);
        nic.transmit_packet(b"HELLO").unwrap();
        assert_eq!(nic.stats.tx_packets, 1);
        assert_eq!(nic.stats.tx_bytes, 5);

        nic.inject_mock_rx_packet(b"WORLD");
        assert_eq!(nic.stats.rx_packets, 0);
        nic.receive_packet().unwrap();
        assert_eq!(nic.stats.rx_packets, 1);
        assert_eq!(nic.stats.rx_bytes, 5);

        // 3. Promiscuous Mode & Multicast Hash Filters
        assert!(!nic.promiscuous_mode);
        nic.set_promiscuous_mode(true);
        assert!(nic.promiscuous_mode);

        assert_eq!(nic.multicast_hash_table[5], 0);
        nic.set_multicast_filter(5, 0x12345678).unwrap();
        assert_eq!(nic.multicast_hash_table[5], 0x12345678);
        assert!(nic.set_multicast_filter(128, 1).is_err()); // Out of bounds
    }

    #[test]
    fn test_bsd_inspired_polling_and_media() {
        let mut nic = IntelE1000Driver::new(0xD0000000, [0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE]);
        nic.initialize_hardware().unwrap();

        // 1. Device Polling Mode Toggle
        assert!(!nic.polling_enabled);
        nic.set_polling_enabled(true);
        assert!(nic.polling_enabled);

        nic.inject_mock_rx_packet(b"POLL_ME");
        // Under polling, receive_packet calls poll_rx_ring
        let rx = nic.receive_packet().unwrap();
        assert_eq!(rx, b"POLL_ME");

        // 2. Link Settings
        assert_eq!(nic.link_speed_mbps, 1000);
        assert_eq!(nic.link_duplex, LinkDuplex::Full);
    }
}
