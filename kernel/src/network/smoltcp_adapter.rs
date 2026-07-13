#![no_std]

/// Sovereign network packet device trait (OOP-based interface)
pub trait NicDevice {
    fn poll_rx(&mut self) -> Option<[u8; 1500]>;
    fn transmit(&mut self, buf: &[u8]) -> Result<(), NicError>;
}

#[derive(Debug)]
pub enum NicError {
    TransmitFailed,
    BufferFull,
}

/// Dynamic OOP Adapter wrapping the NIC Device
pub struct SigmaNetworkAdapter<'a> {
    pub name: &'a str,
    pub device: &'a mut dyn NicDevice,
    pub ip_address: [u8; 4],
}

impl<'a> SigmaNetworkAdapter<'a> {
    pub fn new(name: &'a str, device: &'a mut dyn NicDevice) -> Self {
        Self {
            name,
            device,
            ip_address: [10, 0, 2, 15],
        }
    }

    /// Process incoming and outgoing network traffic without external dependencies
    pub fn poll(&mut self) {
        if let Some(packet) = self.device.poll_rx() {
            crate::log::info("network_adapter", "Received packet in SigmaOS network stack");
            // Perform basic protocol inspection
            let ether_type = ((packet[12] as u16) << 8) | (packet[13] as u16);
            if ether_type == 0x0800 {
                crate::log::info("network_adapter", "Packet protocol: IPv4");
            }
        }
    }

    /// Send packet out of the interface
    pub fn send(&mut self, data: &[u8]) -> Result<(), NicError> {
        crate::log::info("network_adapter", "Sending packet out of interface");
        self.device.transmit(data)
    }
}
