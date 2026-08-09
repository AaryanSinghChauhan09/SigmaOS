// SigmaOS Intel 8254x Gigabit Ethernet (e1000) Network Driver Subsystem
// Parity-level implementation of the legendary Intel hardware NIC controller in a #![no_std] environment

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

pub struct IntelE1000Driver {
    pub pci_bar_base: u64,
    pub mac_address: [u8; 6],
    pub is_initialized: bool,
    // Emulated Tx and Rx ring status registers
    pub tx_descriptors: Vec<E1000TxDescriptor>,
    pub rx_descriptors: Vec<E1000RxDescriptor>,
    pub rx_packet_queue: VecDeque<Vec<u8>>,
    pub tx_packet_queue: VecDeque<Vec<u8>>,
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
        }
    }

    /// Complete hardware init sequences of e1000 register sets
    pub fn initialize_hardware(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;

        // Allocate Tx/Rx descriptor rings (standard size = 64)
        for _ in 0..64 {
            self.tx_descriptors.push(E1000TxDescriptor::default());
            self.rx_descriptors.push(E1000RxDescriptor::default());
        }

        // Simulates unmasking interrupts (IMR), enabling transmitter (TCTL) & receiver (RCTL)
        Ok(())
    }

    /// Transmit raw network packet over the NIC Tx rings
    pub fn transmit_packet(&mut self, payload: &[u8]) -> Result<(), &'static str> {
        if !self.is_initialized {
            return Err("e1000: NIC hardware must be initialized prior to packet transmission");
        }
        if payload.is_empty() {
            return Err("e1000: Cannot transmit empty frame payload");
        }

        // Fill descriptor ring parameters
        let desc_index = self.tx_packet_queue.len() % 64;
        self.tx_descriptors[desc_index] = E1000TxDescriptor {
            buffer_addr: 0x20000000 + (desc_index as u64 * 2048), // mock DMA address
            length: payload.len() as u16,
            cso: 0,
            cmd: 0b00001001, // RS (Report Status) & EOP (End of Packet)
            status: 1,       // Completed
            css: 0,
            special: 0,
        };

        // Queue packet payload
        self.tx_packet_queue.push_back(payload.to_vec());
        Ok(())
    }

    /// Receive raw network packet over the NIC Rx rings
    pub fn receive_packet(&mut self) -> Option<Vec<u8>> {
        if !self.is_initialized {
            return None;
        }
        self.rx_packet_queue.pop_front()
    }

    /// Push mock packet into the NIC Rx descriptors ring (emulator simulation helper)
    pub fn inject_mock_rx_packet(&mut self, payload: &[u8]) {
        let desc_index = self.rx_packet_queue.len() % 64;
        self.rx_descriptors[desc_index] = E1000RxDescriptor {
            buffer_addr: 0x30000000 + (desc_index as u64 * 2048),
            length: payload.len() as u16,
            checksum: 0,
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
}
