/// Scaffold for Network Interface Controller (NIC) subsystem
/// This defines the standard abstraction for all networking hardware in SigmaOS.

use super::Driver;

pub trait NetworkInterface: Driver {
    /// Send a packet over the network interface.
    fn send_packet(&mut self, data: &[u8]) -> Result<(), &'static str>;

    /// Receive a packet from the network interface.
    fn receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>;
    
    /// Get the MAC address of the interface.
    fn get_mac_address(&self) -> [u8; 6];
}

// Future implementations: E1000Driver, VirtioNetDriver
