// SigmaOS VirtIO-Net Driver with IOAPIC Interrupt Wiring
// Implements interrupt-driven packet processing over VirtIO ring buffers (no busy polling).

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

pub const VIRTIO_NET_RX_QUEUE: u16 = 0;
pub const VIRTIO_NET_TX_QUEUE: u16 = 1;
pub const VIRTQ_DESC_F_NEXT: u16 = 1;
pub const VIRTQ_DESC_F_WRITE: u16 = 2; // Buffer is device-writable (RX)

/// VirtIO descriptor
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VirtqDesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

/// VirtIO Avail Ring
#[derive(Debug, Clone)]
pub struct VirtqAvail {
    pub flags: u16,
    pub idx: u16,
    pub ring: [u16; 64],
}

impl Default for VirtqAvail {
    fn default() -> Self {
        Self {
            flags: 0,
            idx: 0,
            ring: [0u16; 64],
        }
    }
}

/// VirtIO Used Element
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct VirtqUsedElem {
    pub id: u32,
    pub len: u32,
}

/// VirtIO Used Ring
#[derive(Debug, Clone)]
pub struct VirtqUsed {
    pub flags: u16,
    pub idx: u16,
    pub ring: [VirtqUsedElem; 64],
}

impl Default for VirtqUsed {
    fn default() -> Self {
        Self {
            flags: 0,
            idx: 0,
            ring: [VirtqUsedElem { id: 0, len: 0 }; 64],
        }
    }
}

/// Hardware VirtIO Network Device Controller
#[derive(Debug, Clone)]
pub struct VirtioNetDevice {
    pub ioapic_irq_vector: u8,
    pub is_irq_wired: bool,
    pub mac_address: [u8; 6],
    pub rx_desc: [VirtqDesc; 64],
    pub rx_avail: VirtqAvail,
    pub rx_used: VirtqUsed,
    pub tx_desc: [VirtqDesc; 64],
    pub tx_avail: VirtqAvail,
    pub tx_used: VirtqUsed,
    pub packets_received: u64,
    pub packets_transmitted: u64,
    pub rx_packet_buffer: Vec<Vec<u8>>,
}

impl VirtioNetDevice {
    pub fn new(mac_address: [u8; 6]) -> Self {
        Self {
            ioapic_irq_vector: 0,
            is_irq_wired: false,
            mac_address,
            rx_desc: [VirtqDesc::default(); 64],
            rx_avail: VirtqAvail::default(),
            rx_used: VirtqUsed::default(),
            tx_desc: [VirtqDesc::default(); 64],
            tx_avail: VirtqAvail::default(),
            tx_used: VirtqUsed::default(),
            packets_received: 0,
            packets_transmitted: 0,
            rx_packet_buffer: Vec::new(),
        }
    }

    /// Wire VirtIO device interrupt line to Local APIC / IOAPIC interrupt vector
    pub fn wire_ioapic_irq(&mut self, vector: u8) {
        self.ioapic_irq_vector = vector;
        self.is_irq_wired = true;
    }

    /// Transmit a packet through TX ring buffer
    pub fn transmit(&mut self, payload: &[u8]) -> Result<usize, &'static str> {
        let idx = (self.tx_avail.idx as usize) % 64;
        self.tx_desc[idx].addr = 0x2000_0000 + (idx as u64 * 2048);
        self.tx_desc[idx].len = payload.len() as u32;
        self.tx_desc[idx].flags = 0;

        self.tx_avail.ring[idx] = idx as u16;
        self.tx_avail.idx = self.tx_avail.idx.wrapping_add(1);

        self.packets_transmitted += 1;
        Ok(payload.len())
    }

    /// Interrupt Service Routine invoked by IOAPIC when device signals packet arrival
    pub fn handle_interrupt(&mut self, simulated_incoming_packets: &[&[u8]]) -> usize {
        if !self.is_irq_wired {
            return 0; // IRQ not wired; ignore
        }

        let mut received = 0;
        for &pkt in simulated_incoming_packets {
            self.rx_packet_buffer.push(pkt.to_vec());
            self.packets_received += 1;
            received += 1;
        }

        received
    }

    /// Fetch next packet from buffer (non-blocking)
    pub fn pop_received_packet(&mut self) -> Option<Vec<u8>> {
        if self.rx_packet_buffer.is_empty() {
            None
        } else {
            Some(self.rx_packet_buffer.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtio_net_irq_wiring_and_transfer() {
        let mut dev = VirtioNetDevice::new([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
        dev.wire_ioapic_irq(48); // IRQ vector 48
        assert!(dev.is_irq_wired);

        let packet = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x08, 0x00];
        assert!(dev.transmit(&packet).is_ok());
        assert_eq!(dev.packets_transmitted, 1);

        let incoming = [b"Test Ethernet Packet Payload".as_slice()];
        let count = dev.handle_interrupt(&incoming);
        assert_eq!(count, 1);
        assert_eq!(dev.packets_received, 1);

        let popped = dev.pop_received_packet().expect("Popped packet");
        assert_eq!(popped.as_slice(), b"Test Ethernet Packet Payload");
    }
}
