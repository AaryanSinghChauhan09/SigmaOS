// SigmaOS Advanced Driver & Hardware Shims Subsystem
// Independent, zero-dependency implementations of Intel e1000, Intel HDA, and VirtIO hardware drivers

use std::collections::BTreeMap;
use std::vec::Vec;

// =========================================================================
// 1. INTEL E1000 ETHERNET DRIVER SHIM
// =========================================================================

pub struct IntelE1000Driver {
    pub mac_address: [u8; 6],
    pub rx_enabled: bool,
    pub tx_enabled: bool,
    pub tx_descriptors: Vec<Vec<u8>>, // Simulates TX descriptor ring
    pub rx_descriptors: Vec<Vec<u8>>, // Simulates RX descriptor ring
    pub link_up: bool,
}

impl IntelE1000Driver {
    pub fn new(mac: [u8; 6]) -> Self {
        Self {
            mac_address: mac,
            rx_enabled: false,
            tx_enabled: false,
            tx_descriptors: Vec::new(),
            rx_descriptors: Vec::new(),
            link_up: true,
        }
    }

    pub fn enable_interfaces(&mut self) {
        self.rx_enabled = true;
        self.tx_enabled = true;
    }

    pub fn transmit_packet(&mut self, packet_data: &[u8]) -> Result<(), &'static str> {
        if !self.link_up {
            return Err("e1000 Error: Link down");
        }
        if !self.tx_enabled {
            return Err("e1000 Error: TX disabled");
        }
        if packet_data.len() > 1518 {
            return Err("e1000 Error: Packet size exceeds Ethernet MTU of 1518");
        }

        self.tx_descriptors.push(packet_data.to_vec());
        Ok(())
    }

    pub fn receive_packet(&mut self, packet_data: &[u8]) -> Result<(), &'static str> {
        if !self.rx_enabled {
            return Err("e1000 Error: RX disabled");
        }
        self.rx_descriptors.push(packet_data.to_vec());
        Ok(())
    }
}

// =========================================================================
// 2. INTEL HIGH DEFINITION AUDIO (HDA) CODEC SHIM
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HdaSampleRate {
    Rate44100Hz,
    Rate48000Hz,
    Rate96000Hz,
}

pub struct IntelHdaDriver {
    pub active_stream_id: Option<u32>,
    pub sample_rate: HdaSampleRate,
    pub stream_buffers: BTreeMap<u32, Vec<u8>>, // stream_id -> audio buffer
    pub volume_level: u8,                       // 0 to 100
}

impl IntelHdaDriver {
    pub fn new() -> Self {
        Self {
            active_stream_id: None,
            sample_rate: HdaSampleRate::Rate44100Hz,
            stream_buffers: BTreeMap::new(),
            volume_level: 80,
        }
    }

    pub fn configure_audio_stream(&mut self, stream_id: u32, rate: HdaSampleRate) {
        self.active_stream_id = Some(stream_id);
        self.sample_rate = rate;
        if !self.stream_buffers.contains_key(&stream_id) {
            self.stream_buffers.insert(stream_id, Vec::new());
        }
    }

    pub fn write_audio_samples(
        &mut self,
        stream_id: u32,
        samples: &[u8],
    ) -> Result<(), &'static str> {
        if let Some(buf) = self.stream_buffers.get_mut(&stream_id) {
            buf.extend_from_slice(samples);
            Ok(())
        } else {
            Err("IntelHDA Error: Stream not configured")
        }
    }

    pub fn set_volume(&mut self, level: u8) {
        self.volume_level = level.min(100);
    }
}

impl Default for IntelHdaDriver {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. VIRTIO BLOCK SPECIFICATION DRIVER SHIM
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtioBlockOp {
    Read,
    Write,
}

pub struct VirtioBlockRequest {
    pub sector: u64,
    pub operation: VirtioBlockOp,
    pub buffer: Vec<u8>,
}

pub struct VirtioBlockDriver {
    pub request_queue: Vec<VirtioBlockRequest>,
    pub processed_queue_count: usize,
    pub disk_size_sectors: u64,
}

impl VirtioBlockDriver {
    pub fn new(size_sectors: u64) -> Self {
        Self {
            request_queue: Vec::new(),
            processed_queue_count: 0,
            disk_size_sectors: size_sectors,
        }
    }

    pub fn push_block_request(
        &mut self,
        sector: u64,
        op: VirtioBlockOp,
        data_buffer: &[u8],
    ) -> Result<(), &'static str> {
        if sector >= self.disk_size_sectors {
            return Err("VirtioBlock Error: Sector index out of bounds");
        }

        let request = VirtioBlockRequest {
            sector,
            operation: op,
            buffer: data_buffer.to_vec(),
        };

        self.request_queue.push(request);
        Ok(())
    }

    pub fn execute_next_request(&mut self) -> Option<VirtioBlockRequest> {
        if self.request_queue.is_empty() {
            None
        } else {
            self.processed_queue_count += 1;
            Some(self.request_queue.remove(0))
        }
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_e1000_ethernet_driver() {
        let mut nic = IntelE1000Driver::new([0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E]);
        nic.enable_interfaces();

        // Transmit packet
        assert!(nic.transmit_packet(b"GET / HTTP/1.1").is_ok());
        assert_eq!(nic.tx_descriptors.len(), 1);
        assert_eq!(nic.tx_descriptors[0], b"GET / HTTP/1.1");

        // Receive packet
        assert!(nic.receive_packet(b"HTTP/1.1 200 OK").is_ok());
        assert_eq!(nic.rx_descriptors.len(), 1);
        assert_eq!(nic.rx_descriptors[0], b"HTTP/1.1 200 OK");

        // Transmit packet with oversized MTU (fails)
        let oversized = [1u8; 2000];
        assert!(nic.transmit_packet(&oversized).is_err());
    }

    #[test]
    fn test_intel_hda_audio_driver() {
        let mut hda = IntelHdaDriver::new();
        hda.configure_audio_stream(5, HdaSampleRate::Rate96000Hz);
        assert_eq!(hda.sample_rate, HdaSampleRate::Rate96000Hz);

        // Write audio samples
        assert!(hda.write_audio_samples(5, b"PCM_DATA_CH1_CH2").is_ok());
        assert_eq!(hda.stream_buffers.get(&5).unwrap(), b"PCM_DATA_CH1_CH2");

        // Write to unconfigured stream (fails)
        assert!(hda.write_audio_samples(99, b"PCM_DATA").is_err());

        // Volume control
        hda.set_volume(90);
        assert_eq!(hda.volume_level, 90);
        hda.set_volume(150); // Clamped
        assert_eq!(hda.volume_level, 100);
    }

    #[test]
    fn test_virtio_block_driver() {
        let mut vblk = VirtioBlockDriver::new(100);
        assert_eq!(vblk.disk_size_sectors, 100);

        // Push Block request (Write)
        assert!(vblk
            .push_block_request(10, VirtioBlockOp::Write, b"SECTOR_DATA_10")
            .is_ok());
        assert_eq!(vblk.request_queue.len(), 1);

        // Push Block request out of bounds (fails)
        assert!(vblk
            .push_block_request(150, VirtioBlockOp::Read, b"")
            .is_err());

        // Execute next request
        let req = vblk.execute_next_request().unwrap();
        assert_eq!(req.sector, 10);
        assert_eq!(req.operation, VirtioBlockOp::Write);
        assert_eq!(req.buffer, b"SECTOR_DATA_10");
        assert_eq!(vblk.processed_queue_count, 1);

        assert!(vblk.execute_next_request().is_none());
    }
}
