// ZenithNet - Zero-copy networking stack
// Polymorphic network driver interface and zero-copy packet processing

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkError {
    DmaError,
    BufferFull,
    InvalidPacket,
    DriverNotFound,
    TransmissionFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NetworkDriverType {
    E1000,
    Rtl8139,
    VirtioNet,
    Realtek,
}

#[derive(Debug, Clone)]
pub struct NetworkPacketFrame {
    pub data: Vec<u8>,
    pub length: usize,
    pub timestamp: u64,
}

/// Polymorphic network driver interface
pub trait NetworkDriverDevice {
    fn transmit_packet(&mut self, payload: &[u8]) -> Result<(), NetworkError>;
    fn poll_receive_ring(&mut self) -> Option<NetworkPacketFrame>;
    fn configure_dma_ring(&mut self, rx_base: u64, tx_base: u64) -> Result<(), NetworkError>;
    fn get_driver_type(&self) -> NetworkDriverType;
}

/// E1000 Network Driver implementation
pub struct E1000NetworkDriver {
    pub rx_base: u64,
    pub tx_base: u64,
    pub initialized: bool,
}

impl E1000NetworkDriver {
    pub fn new() -> Self {
        Self {
            rx_base: 0,
            tx_base: 0,
            initialized: false,
        }
    }
}

impl NetworkDriverDevice for E1000NetworkDriver {
    fn transmit_packet(&mut self, payload: &[u8]) -> Result<(), NetworkError> {
        if !self.initialized {
            return Err(NetworkError::DmaError);
        }
        // In real implementation, write to E1000 TX registers
        Ok(())
    }

    fn poll_receive_ring(&mut self) -> Option<NetworkPacketFrame> {
        if !self.initialized {
            return None;
        }
        // In real implementation, read from E1000 RX ring
        None
    }

    fn configure_dma_ring(&mut self, rx_base: u64, tx_base: u64) -> Result<(), NetworkError> {
        self.rx_base = rx_base;
        self.tx_base = tx_base;
        self.initialized = true;
        Ok(())
    }

    fn get_driver_type(&self) -> NetworkDriverType {
        NetworkDriverType::E1000
    }
}

/// RTL8139 Network Driver implementation
pub struct Rtl8139NetworkDriver {
    pub rx_base: u64,
    pub tx_base: u64,
    pub initialized: bool,
}

impl Rtl8139NetworkDriver {
    pub fn new() -> Self {
        Self {
            rx_base: 0,
            tx_base: 0,
            initialized: false,
        }
    }
}

impl NetworkDriverDevice for Rtl8139NetworkDriver {
    fn transmit_packet(&mut self, payload: &[u8]) -> Result<(), NetworkError> {
        if !self.initialized {
            return Err(NetworkError::DmaError);
        }
        // In real implementation, write to RTL8139 TX registers
        Ok(())
    }

    fn poll_receive_ring(&mut self) -> Option<NetworkPacketFrame> {
        if !self.initialized {
            return None;
        }
        // In real implementation, read from RTL8139 RX ring
        None
    }

    fn configure_dma_ring(&mut self, rx_base: u64, tx_base: u64) -> Result<(), NetworkError> {
        self.rx_base = rx_base;
        self.tx_base = tx_base;
        self.initialized = true;
        Ok(())
    }

    fn get_driver_type(&self) -> NetworkDriverType {
        NetworkDriverType::Rtl8139
    }
}

/// Zero-copy packet ring buffer interface
pub struct ZeroCopyPacketRing {
    rx_buffer: Vec<Option<NetworkPacketFrame>>,
    tx_buffer: Vec<Option<NetworkPacketFrame>>,
    rx_head: usize,
    rx_tail: usize,
    tx_head: usize,
    tx_tail: usize,
    capacity: usize,
}

impl ZeroCopyPacketRing {
    pub fn new(capacity: usize) -> Self {
        Self {
            rx_buffer: vec![None; capacity],
            tx_buffer: vec![None; capacity],
            rx_head: 0,
            rx_tail: 0,
            tx_head: 0,
            tx_tail: 0,
            capacity,
        }
    }

    /// Enqueue packet to TX ring
    pub fn enqueue_tx(&mut self, packet: NetworkPacketFrame) -> Result<(), NetworkError> {
        if (self.tx_head + 1) % self.capacity == self.tx_tail {
            return Err(NetworkError::BufferFull);
        }

        self.tx_buffer[self.tx_head] = Some(packet);
        self.tx_head = (self.tx_head + 1) % self.capacity;
        Ok(())
    }

    /// Dequeue packet from TX ring
    pub fn dequeue_tx(&mut self) -> Option<NetworkPacketFrame> {
        if self.tx_tail == self.tx_head {
            return None;
        }

        let packet = self.tx_buffer[self.tx_tail].take();
        self.tx_tail = (self.tx_tail + 1) % self.capacity;
        packet
    }

    /// Enqueue packet to RX ring
    pub fn enqueue_rx(&mut self, packet: NetworkPacketFrame) -> Result<(), NetworkError> {
        if (self.rx_head + 1) % self.capacity == self.rx_tail {
            return Err(NetworkError::BufferFull);
        }

        self.rx_buffer[self.rx_head] = Some(packet);
        self.rx_head = (self.rx_head + 1) % self.capacity;
        Ok(())
    }

    /// Dequeue packet from RX ring
    pub fn dequeue_rx(&mut self) -> Option<NetworkPacketFrame> {
        if self.rx_tail == self.rx_head {
            return None;
        }

        let packet = self.rx_buffer[self.rx_tail].take();
        self.rx_tail = (self.rx_tail + 1) % self.capacity;
        packet
    }

    /// Get RX ring count
    pub fn rx_count(&self) -> usize {
        (self.rx_head + self.capacity - self.rx_tail) % self.capacity
    }

    /// Get TX ring count
    pub fn tx_count(&self) -> usize {
        (self.tx_head + self.capacity - self.tx_tail) % self.capacity
    }
}

impl Default for ZeroCopyPacketRing {
    fn default() -> Self {
        Self::new(256)
    }
}

/// Network driver manager
pub struct NetworkDriverManager {
    drivers: BTreeMap<NetworkDriverType, Box<dyn NetworkDriverDevice>>,
    active_driver: Option<NetworkDriverType>,
}

impl NetworkDriverManager {
    pub fn new() -> Self {
        Self {
            drivers: BTreeMap::new(),
            active_driver: None,
        }
    }

    /// Register a network driver
    pub fn register_driver(&mut self, driver_type: NetworkDriverType, driver: Box<dyn NetworkDriverDevice>) {
        self.drivers.insert(driver_type, driver);
    }

    /// Set active driver
    pub fn set_active_driver(&mut self, driver_type: NetworkDriverType) -> Result<(), NetworkError> {
        if !self.drivers.contains_key(&driver_type) {
            return Err(NetworkError::DriverNotFound);
        }
        self.active_driver = Some(driver_type);
        Ok(())
    }

    /// Transmit packet using active driver
    pub fn transmit(&mut self, payload: &[u8]) -> Result<(), NetworkError> {
        let driver_type = self.active_driver.ok_or(NetworkError::DriverNotFound)?;
        let driver = self.drivers.get_mut(&driver_type).ok_or(NetworkError::DriverNotFound)?;
        driver.transmit_packet(payload)
    }

    /// Poll receive ring using active driver
    pub fn poll_receive(&mut self) -> Option<NetworkPacketFrame> {
        let driver_type = self.active_driver?;
        let driver = self.drivers.get_mut(&driver_type)?;
        driver.poll_receive_ring()
    }

    /// Configure DMA ring for active driver
    pub fn configure_dma(&mut self, rx_base: u64, tx_base: u64) -> Result<(), NetworkError> {
        let driver_type = self.active_driver.ok_or(NetworkError::DriverNotFound)?;
        let driver = self.drivers.get_mut(&driver_type).ok_or(NetworkError::DriverNotFound)?;
        driver.configure_dma_ring(rx_base, tx_base)
    }

    /// Get active driver type
    pub fn active_driver_type(&self) -> Option<NetworkDriverType> {
        self.active_driver
    }
}

impl Default for NetworkDriverManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e1000_driver() {
        let mut driver = E1000NetworkDriver::new();
        assert!(!driver.initialized);
        
        driver.configure_dma_ring(0x1000, 0x2000).unwrap();
        assert!(driver.initialized);
        assert_eq!(driver.get_driver_type(), NetworkDriverType::E1000);
    }

    #[test]
    fn test_rtl8139_driver() {
        let mut driver = Rtl8139NetworkDriver::new();
        assert!(!driver.initialized);
        
        driver.configure_dma_ring(0x3000, 0x4000).unwrap();
        assert!(driver.initialized);
        assert_eq!(driver.get_driver_type(), NetworkDriverType::Rtl8139);
    }

    #[test]
    fn test_zero_copy_ring() {
        let mut ring = ZeroCopyPacketRing::new(4);
        
        let packet = NetworkPacketFrame {
            data: vec![0xDE, 0xAD, 0xBE, 0xEF],
            length: 4,
            timestamp: 100,
        };
        
        ring.enqueue_tx(packet.clone()).unwrap();
        assert_eq!(ring.tx_count(), 1);
        
        let retrieved = ring.dequeue_tx().unwrap();
        assert_eq!(retrieved.data, packet.data);
    }

    #[test]
    fn test_ring_buffer_full() {
        let mut ring = ZeroCopyPacketRing::new(2);
        
        let packet = NetworkPacketFrame {
            data: vec![0x01],
            length: 1,
            timestamp: 0,
        };
        
        ring.enqueue_tx(packet.clone()).unwrap();
        ring.enqueue_tx(packet.clone()).unwrap();
        
        assert!(ring.enqueue_tx(packet).is_err());
    }

    #[test]
    fn test_network_driver_manager() {
        let mut manager = NetworkDriverManager::new();
        
        let e1000: Box<dyn NetworkDriverDevice> = Box::new(E1000NetworkDriver::new());
        manager.register_driver(NetworkDriverType::E1000, e1000);
        
        manager.set_active_driver(NetworkDriverType::E1000).unwrap();
        assert_eq!(manager.active_driver_type(), Some(NetworkDriverType::E1000));
    }

    #[test]
    fn test_driver_not_found() {
        let mut manager = NetworkDriverManager::new();
        
        let result = manager.set_active_driver(NetworkDriverType::VirtioNet);
        assert!(result.is_err());
    }
}
