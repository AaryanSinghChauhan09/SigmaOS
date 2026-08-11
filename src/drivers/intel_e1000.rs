// Intel 8254x Gigabit Ethernet Network Controller Driver (e1000)
// Extremely ubiquitous virtual and hardware gigabit networking driver used by Linux and BSD guest environments

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};

/// Intel e1000 Tx and Rx Descriptor structures
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct E1000RxDesc {
    pub addr: u64,
    pub length: u16,
    pub checksum: u16,
    pub status: u8,
    pub errors: u8,
    pub special: u16,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct E1000TxDesc {
    pub addr: u64,
    pub length: u16,
    pub cso: u8,
    pub cmd: u8,
    pub status: u8,
    pub css: u8,
    pub special: u16,
}

/// Intel E1000 Driver matching Intel 8254x controller specification
pub struct IntelE1000Driver {
    pub is_initialized: bool,
    pub power_state: PowerState,
    pub rx_descriptors: Vec<E1000RxDesc>,
    pub tx_descriptors: Vec<E1000TxDesc>,
    pub rx_head: u32,
    pub rx_tail: u32,
    pub tx_head: u32,
    pub tx_tail: u32,
    pub mac_address: [u8; 6],
}

impl IntelE1000Driver {
    pub fn new(mac: [u8; 6]) -> Self {
        Self {
            is_initialized: false,
            power_state: PowerState::Off,
            rx_descriptors: vec![E1000RxDesc::default(); 128],
            tx_descriptors: vec![E1000TxDesc::default(); 128],
            rx_head: 0,
            rx_tail: 127,
            tx_head: 0,
            tx_tail: 127,
            mac_address: mac,
        }
    }
}

impl PeripheralDevice for IntelE1000Driver {
    fn name(&self) -> &'static str {
        "Intel 8254x Gigabit Ethernet Controller Driver (e1000)"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Legacy // Broad compatibility, categorized as legacy-standard compatible
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        self.is_initialized = true;
        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("e1000: Driver is not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("e1000: Card power state is not online");
        }

        // Mock gigabit frame capture
        let mock_ethernet_frame = [
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, // Broad DST
            0x00, 0x1a, 0xa0, 0x11, 0x22, 0x33, // SRC
            0x08, 0x00,                         // IPv4
            0x45, 0x00, 0x00, 0x14, 0x00, 0x01, // IP Payload
        ];

        let read_len = std::cmp::min(buffer.len(), mock_ethernet_frame.len());
        buffer[..read_len].copy_from_slice(&mock_ethernet_frame[..read_len]);
        Ok(read_len)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("e1000: Driver is not initialized");
        }
        if self.power_state != PowerState::On {
            return Err("e1000: Card power state is not online");
        }

        // Queue data to simulated Tx descriptors
        let descriptor_idx = (self.tx_tail % 128) as usize;
        self.tx_descriptors[descriptor_idx] = E1000TxDesc {
            addr: data.as_ptr() as u64,
            length: data.len() as u16,
            cso: 0,
            cmd: 0x01 | 0x08, // End of Packet (EOP) and Insert FCS (IFCS)
            status: 0,
            css: 0,
            special: 0,
        };
        self.tx_tail += 1;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intel_e1000_driver() {
        let mut e1000 = IntelE1000Driver::new([0x00, 0x1a, 0xa0, 0x11, 0x22, 0x33]);
        assert_eq!(e1000.name(), "Intel 8254x Gigabit Ethernet Controller Driver (e1000)");
        assert_eq!(e1000.mac_address, [0x00, 0x1a, 0xa0, 0x11, 0x22, 0x33]);

        assert!(e1000.read(&mut [0; 10]).is_err());

        e1000.initialize().unwrap();
        let mut buf = vec![0; 64];
        let bytes_received = e1000.read(&mut buf).unwrap();
        assert_eq!(bytes_received, 20);
        assert_eq!(buf[0], 0xff);
        assert_eq!(buf[6], 0x00);
        assert_eq!(buf[12], 0x08);

        let tx_data = [0xaa, 0xbb, 0xcc];
        assert_eq!(e1000.write(&tx_data).unwrap(), 3);
        assert_eq!(e1000.tx_tail, 128);

        e1000.shutdown().unwrap();
        assert!(e1000.write(&tx_data).is_err());
    }
}
