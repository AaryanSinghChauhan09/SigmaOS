// Intel e1000 Gigabit Network Interface Card Driver
// Conforms to SigmaOS UnifiedPeripheral interface
// Enhanced with MSI-X interrupt support and advanced descriptor management

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use crate::security::CapabilityToken;
use core::ptr::{read_volatile, write_volatile};

use std::boxed::Box;
use std::vec::Vec;

// Register Offsets (MMIO)
const REG_CTRL: u32 = 0x0000; // Device Control Register
const REG_STATUS: u32 = 0x0008; // Device Status Register
const REG_IMS: u32 = 0x00D0; // Interrupt Mask Set Register
const REG_IMC: u32 = 0x00D8; // Interrupt Mask Clear Register
const REG_RCTL: u32 = 0x0100; // Receive Control Register
const REG_TCTL: u32 = 0x0400; // Transmit Control Register
const REG_RDBAL: u32 = 0x2800; // Receive Descriptor Base Address Low
const REG_RDBAH: u32 = 0x2804; // Receive Descriptor Base Address High
const REG_RDLEN: u32 = 0x2808; // Receive Descriptor Length
const REG_RDH: u32 = 0x2810; // Receive Descriptor Head
const REG_RDT: u32 = 0x2818; // Receive Descriptor Tail
const REG_TDBAL: u32 = 0x3800; // Transmit Descriptor Base Address Low
const REG_TDBAH: u32 = 0x3804; // Transmit Descriptor Base Address High
const REG_TDLEN: u32 = 0x3808; // Transmit Descriptor Length
const REG_TDH: u32 = 0x3810; // Transmit Descriptor Head
const REG_TDT: u32 = 0x3818; // Transmit Descriptor Tail

// Descriptor count
const NUM_RX_DESCRIPTORS: usize = 128;
const NUM_TX_DESCRIPTORS: usize = 128;
const RX_BUFFER_SIZE: usize = 2048;

/// Receive Descriptor Layout
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct RxDescriptor {
    pub buffer_addr: u64,
    pub length: u16,
    pub checksum: u16,
    pub status: u8,
    pub errors: u8,
    pub special: u16,
}

/// Transmit Descriptor Layout
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct TxDescriptor {
    pub buffer_addr: u64,
    pub length: u16,
    pub ccmd: u8,
    pub status: u8,
    pub special: u16,
}

/// MSI-X interrupt vector configuration
#[derive(Debug, Clone, Copy)]
pub struct MsixVector {
    pub vector: u16,
    pub enabled: bool,
    pub masked: bool,
}

/// Advanced descriptor ring with zero-copy support
pub struct DescriptorRing<T> {
    pub descriptors: Vec<T>,
    pub head: usize,
    pub tail: usize,
    pub size: usize,
}

impl<T: Clone + Copy> DescriptorRing<T> {
    pub fn new(size: usize, default_desc: T) -> Self {
        Self {
            descriptors: vec![default_desc; size],
            head: 0,
            tail: 0,
            size,
        }
    }

    pub fn enqueue(&mut self, desc: T) -> Result<usize, &'static str> {
        let next_tail = (self.tail + 1) % self.size;
        if next_tail == self.head {
            return Err("Descriptor ring is full");
        }
        self.descriptors[self.tail] = desc;
        let idx = self.tail;
        self.tail = next_tail;
        Ok(idx)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.head == self.tail {
            return None;
        }
        let desc = self.descriptors[self.head];
        self.head = (self.head + 1) % self.size;
        Some(desc)
    }

    pub fn available(&self) -> usize {
        if self.tail >= self.head {
            self.size - self.tail + self.head
        } else {
            self.head - self.tail
        }
    }
}

/// Intel E1000 network card driver state
pub struct E1000Driver {
    pub mmio_base: usize,
    pub rx_ring: DescriptorRing<RxDescriptor>,
    pub tx_ring: DescriptorRing<TxDescriptor>,
    pub rx_buffers: Vec<[u8; RX_BUFFER_SIZE]>,
    pub msix_vectors: Vec<MsixVector>,
    pub tx_tail: usize,
    pub power_state: PowerState,
    pub capabilities: CapabilityToken,
}

impl E1000Driver {
    /// Creates a new uninitialized E1000 driver mapped to a specific MMIO memory address
    pub unsafe fn new(mmio_base: usize, capabilities: CapabilityToken) -> Self {
        let rx_default = RxDescriptor {
            buffer_addr: 0,
            length: 0,
            checksum: 0,
            status: 0,
            errors: 0,
            special: 0,
        };

        let tx_default = TxDescriptor {
            buffer_addr: 0,
            length: 0,
            ccmd: 0,
            status: 0,
            special: 0,
        };

        let rx_ring = DescriptorRing::new(NUM_RX_DESCRIPTORS, rx_default);
        let tx_ring = DescriptorRing::new(NUM_TX_DESCRIPTORS, tx_default);

        let mut rx_buffers = Vec::with_capacity(NUM_RX_DESCRIPTORS);
        for _ in 0..NUM_RX_DESCRIPTORS {
            rx_buffers.push([0u8; RX_BUFFER_SIZE]);
        }

        Self {
            mmio_base,
            rx_ring,
            tx_ring,
            rx_buffers,
            msix_vectors: Vec::new(),
            tx_tail: 0,
            power_state: PowerState::Off,
            capabilities,
        }
    }

    unsafe fn read_reg(&self, offset: u32) -> u32 {
        #[cfg(target_os = "none")]
        {
            read_volatile((self.mmio_base + offset as usize) as *const u32)
        }
        #[cfg(not(target_os = "none"))]
        {
            0
        }
    }

    unsafe fn write_reg(&self, offset: u32, value: u32) {
        #[cfg(target_os = "none")]
        {
            write_volatile((self.mmio_base + offset as usize) as *mut u32, value);
        }
    }

    /// Configure MSI-X interrupt vector
    pub fn configure_msix(&mut self, vector: u16) -> Result<(), &'static str> {
        if self.msix_vectors.len() >= 64 {
            return Err("Maximum MSI-X vectors reached");
        }
        self.msix_vectors.push(MsixVector {
            vector,
            enabled: true,
            masked: false,
        });
        Ok(())
    }

    /// Get number of available MSI-X vectors
    pub fn msix_count(&self) -> usize {
        self.msix_vectors.len()
    }

    /// Get RX ring status
    pub fn get_rx_status(&self) -> (usize, usize, usize) {
        (self.rx_ring.head, self.rx_ring.tail, self.rx_ring.available())
    }

    /// Get TX ring status
    pub fn get_tx_status(&self) -> (usize, usize, usize) {
        (self.tx_ring.head, self.tx_ring.tail, self.tx_ring.available())
    }
}

impl PeripheralDevice for E1000Driver {
    fn name(&self) -> &'static str {
        "Intel e1000 Gigabit NIC"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Enforce network configuration capabilities
        if self.capabilities.bits() & 0x02 == 0 {
            return Err("E1000: PermissionDenied - Missing Network capability");
        }

        unsafe {
            // 1. Reset controller
            self.write_reg(REG_CTRL, self.read_reg(REG_CTRL) | 0x04000000); // RST bit
            core::hint::spin_loop(); // Allow reset circuit to settle

            // 2. Disable interrupts
            self.write_reg(REG_IMC, 0xFFFFFFFF);

            // 3. Set up Receive Descriptors
            let rx_ring_physical = self.rx_ring.descriptors.as_ptr() as u64;
            self.write_reg(REG_RDBAL, (rx_ring_physical & 0xFFFFFFFF) as u32);
            self.write_reg(REG_RDBAH, (rx_ring_physical >> 32) as u32);
            self.write_reg(
                REG_RDLEN,
                (NUM_RX_DESCRIPTORS * core::mem::size_of::<RxDescriptor>()) as u32,
            );
            self.write_reg(REG_RDH, self.rx_ring.head as u32);
            self.write_reg(REG_RDT, self.rx_ring.tail as u32);

            // Initialize RX Descriptors with mapped buffers
            for i in 0..NUM_RX_DESCRIPTORS {
                self.rx_ring.descriptors[i].buffer_addr = self.rx_buffers[i].as_ptr() as u64;
                self.rx_ring.descriptors[i].status = 0;
            }

            // Enable RX (RCTL = EN | BAM (Broadcast Accept) | SZ_2048)
            self.write_reg(REG_RCTL, 0x00000002 | 0x00008000 | 0x00000000);

            // 4. Set up Transmit Descriptors
            let tx_ring_physical = self.tx_ring.descriptors.as_ptr() as u64;
            self.write_reg(REG_TDBAL, (tx_ring_physical & 0xFFFFFFFF) as u32);
            self.write_reg(REG_TDBAH, (tx_ring_physical >> 32) as u32);
            self.write_reg(
                REG_TDLEN,
                (NUM_TX_DESCRIPTORS * core::mem::size_of::<TxDescriptor>()) as u32,
            );
            self.write_reg(REG_TDH, self.tx_ring.head as u32);
            self.write_reg(REG_TDT, self.tx_ring.tail as u32);

            // Enable TX (TCTL = EN | PSP (Pad Short Packets))
            self.write_reg(REG_TCTL, 0x00000002 | 0x00000008);

            // Enable selected interrupts (LSC = Link Status Change, RXT0 = Receiver Timer Interrupt)
            self.write_reg(REG_IMS, 0x04 | 0x80);
        }

        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("E1000: Device is powered off");
        }

        if let Some(desc) = self.rx_ring.dequeue() {
            // Check DD (Descriptor Done) bit in status
            if (desc.status & 0x01) == 0 {
                return Ok(0); // No new packet
            }

            let length = desc.length as usize;
            if length > buffer.len() {
                return Err("E1000: Buffer overflow - package size exceeds input");
            }

            // Find the buffer index based on head position
            let buffer_idx = (self.rx_ring.head - 1) % NUM_RX_DESCRIPTORS;
            
            // Copy received data to user buffer
            buffer[..length].copy_from_slice(&self.rx_buffers[buffer_idx][..length]);

            // Update register
            unsafe {
                self.write_reg(REG_RDT, self.rx_ring.tail as u32);
            }

            Ok(length)
        } else {
            Ok(0)
        }
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("E1000: Device is powered off");
        }

        if data.len() > RX_BUFFER_SIZE {
            return Err("E1000: Packet too large");
        }

        let mmio = self.mmio_base;
        let desc = TxDescriptor {
            buffer_addr: data.as_ptr() as u64,
            length: data.len() as u16,
            ccmd: 0x01 | 0x08, // EOP (End of Packet) | RS (Report Status)
            status: 0,
            special: 0,
        };

        // Enqueue descriptor
        self.tx_ring.enqueue(desc)?;
        self.tx_tail = self.tx_ring.tail;

        unsafe {
            // Update Tail pointer to initiate transmission
            #[cfg(target_os = "none")]
            {
                write_volatile((mmio + REG_TDT as usize) as *mut u32, self.tx_tail as u32);
            }

            // Spin-wait until hardware completes transmission
            let mut timeout = 1000;
            while (read_volatile(&self.tx_ring.descriptors[self.tx_ring.head].status) & 0x01) == 0 && timeout > 0 {
                core::hint::spin_loop();
                timeout -= 1;
            }
        }

        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        unsafe {
            self.write_reg(REG_RCTL, 0); // Disable receiver
            self.write_reg(REG_TCTL, 0); // Disable transmitter
            self.write_reg(REG_IMC, 0xFFFFFFFF); // Disable all interrupts
        }
        self.power_state = PowerState::Off;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_descriptor_ring() {
        let default_desc = RxDescriptor {
            buffer_addr: 0,
            length: 0,
            checksum: 0,
            status: 0,
            errors: 0,
            special: 0,
        };
        
        let mut ring = DescriptorRing::new(64, default_desc);
        assert_eq!(ring.available(), 64);
        
        let desc = RxDescriptor {
            buffer_addr: 0x1000,
            length: 512,
            checksum: 0,
            status: 1,
            errors: 0,
            special: 0,
        };
        
        assert!(ring.enqueue(desc).is_ok());
        assert_eq!(ring.available(), 63);
        
        let dequeued = ring.dequeue();
        assert!(dequeued.is_some());
        assert_eq!(dequeued.unwrap().buffer_addr, 0x1000);
    }

    #[test]
    fn test_msix_configuration() {
        let mut driver = unsafe { E1000Driver::new(0xF0000000, CapabilityToken::new(0x02)) };
        
        assert!(driver.configure_msix(1).is_ok());
        assert_eq!(driver.msix_count(), 1);
        
        assert!(driver.configure_msix(2).is_ok());
        assert_eq!(driver.msix_count(), 2);
    }

    #[test]
    fn test_ring_status() {
        let mut driver = unsafe { E1000Driver::new(0xF0000000, CapabilityToken::new(0x02)) };
        
        let (head, tail, available) = driver.get_rx_status();
        assert_eq!(head, 0);
        assert_eq!(tail, 0);
        assert_eq!(available, 128);
    }
}
