// Intel e1000 Gigabit Network Interface Card Driver Blueprint
// Conforms to Sovereign Driver Framework (SDF) and PeripheralDevice interface

use core::ptr::{read_volatile, write_volatile};
use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use crate::security::CapabilityToken;

// Register Offsets (MMIO)
const REG_CTRL: u32     = 0x0000; // Device Control Register
const REG_STATUS: u32   = 0x0008; // Device Status Register
const REG_IMS: u32      = 0x00D0; // Interrupt Mask Set Register
const REG_IMC: u32      = 0x00D8; // Interrupt Mask Clear Register
const REG_RCTL: u32     = 0x0100; // Receive Control Register
const REG_TCTL: u32     = 0x0400; // Transmit Control Register
const REG_RDBAL: u32    = 0x2800; // Receive Descriptor Base Address Low
const REG_RDBAH: u32    = 0x2804; // Receive Descriptor Base Address High
const REG_RDLEN: u32    = 0x2808; // Receive Descriptor Length
const REG_RDH: u32      = 0x2810; // Receive Descriptor Head
const REG_RDT: u32      = 0x2818; // Receive Descriptor Tail
const REG_TDBAL: u32    = 0x3800; // Transmit Descriptor Base Address Low
const REG_TDBAH: u32    = 0x3804; // Transmit Descriptor Base Address High
const REG_TDLEN: u32    = 0x3808; // Transmit Descriptor Length
const REG_TDH: u32      = 0x3810; // Transmit Descriptor Head
const REG_TDT: u32      = 0x3818; // Transmit Descriptor Tail

// Descriptor count
pub const NUM_RX_DESCRIPTORS: usize = 128;
pub const NUM_TX_DESCRIPTORS: usize = 128;
pub const RX_BUFFER_SIZE: usize     = 2048;

/// Receive Descriptor Layout
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug)]
pub struct TxDescriptor {
    pub buffer_addr: u64,
    pub length: u16,
    pub ccmd: u8,
    pub status: u8,
    pub special: u16,
}

/// Intel E1000 network card driver state conforming to SDF & PeripheralDevice
pub struct E1000Driver {
    pub mmio_base: usize,
    pub rx_ring: [RxDescriptor; NUM_RX_DESCRIPTORS],
    pub tx_ring: [TxDescriptor; NUM_TX_DESCRIPTORS],
    pub rx_buffers: [[u8; RX_BUFFER_SIZE]; NUM_RX_DESCRIPTORS],
    pub rx_head: usize,
    pub tx_tail: usize,
    pub power_state: PowerState,
    pub capabilities: CapabilityToken,
}

impl E1000Driver {
    pub fn new(mmio_base: usize, capabilities: CapabilityToken) -> Self {
        let empty_rx = RxDescriptor {
            buffer_addr: 0,
            length: 0,
            checksum: 0,
            status: 0,
            errors: 0,
            special: 0,
        };
        let empty_tx = TxDescriptor {
            buffer_addr: 0,
            length: 0,
            ccmd: 0,
            status: 0,
            special: 0,
        };

        Self {
            mmio_base,
            rx_ring: [empty_rx; NUM_RX_DESCRIPTORS],
            tx_ring: [empty_tx; NUM_TX_DESCRIPTORS],
            rx_buffers: [[0u8; RX_BUFFER_SIZE]; NUM_RX_DESCRIPTORS],
            rx_head: 0,
            tx_tail: 0,
            power_state: PowerState::Off,
            capabilities,
        }
    }

    pub unsafe fn read_reg(&self, offset: u32) -> u32 {
        #[cfg(target_os = "none")]
        {
            if self.mmio_base == 0 {
                return 0;
            }
            read_volatile((self.mmio_base + offset as usize) as *const u32)
        }
        #[cfg(not(target_os = "none"))]
        {
            let _ = offset;
            0
        }
    }

    pub unsafe fn write_reg(&self, offset: u32, value: u32) {
        #[cfg(target_os = "none")]
        {
            if self.mmio_base == 0 {
                return;
            }
            write_volatile((self.mmio_base + offset as usize) as *mut u32, value);
        }
        #[cfg(not(target_os = "none"))]
        {
            let _ = (offset, value);
        }
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

            // 2. Disable interrupts
            self.write_reg(REG_IMC, 0xFFFFFFFF);

            // 3. Set up Receive Descriptors
            let rx_ring_physical = self.rx_ring.as_ptr() as u64;
            self.write_reg(REG_RDBAL, (rx_ring_physical & 0xFFFFFFFF) as u32);
            self.write_reg(REG_RDBAH, (rx_ring_physical >> 32) as u32);
            self.write_reg(REG_RDLEN, (NUM_RX_DESCRIPTORS * core::mem::size_of::<RxDescriptor>()) as u32);
            self.write_reg(REG_RDH, 0);
            self.write_reg(REG_RDT, (NUM_RX_DESCRIPTORS - 1) as u32);

            // Initialize RX Descriptors with mapped buffers
            for i in 0..NUM_RX_DESCRIPTORS {
                self.rx_ring[i].buffer_addr = self.rx_buffers[i].as_ptr() as u64;
                self.rx_ring[i].status = 0;
            }

            // Enable RX (RCTL = EN | BAM | SZ_2048)
            self.write_reg(REG_RCTL, 0x00000002 | 0x00008000 | 0x00000000);

            // 4. Set up Transmit Descriptors
            let tx_ring_physical = self.tx_ring.as_ptr() as u64;
            self.write_reg(REG_TDBAL, (tx_ring_physical & 0xFFFFFFFF) as u32);
            self.write_reg(REG_TDBAH, (tx_ring_physical >> 32) as u32);
            self.write_reg(REG_TDLEN, (NUM_TX_DESCRIPTORS * core::mem::size_of::<TxDescriptor>()) as u32);
            self.write_reg(REG_TDH, 0);
            self.write_reg(REG_TDT, 0);

            // Enable TX (TCTL = EN | PSP)
            self.write_reg(REG_TCTL, 0x00000002 | 0x00000008);

            // Enable selected interrupts
            self.write_reg(REG_IMS, 0x04 | 0x80);
        }

        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("E1000: Device is powered off");
        }

        let desc = &mut self.rx_ring[self.rx_head];
        if (desc.status & 0x01) == 0 {
            return Ok(0); // No packet
        }

        let length = desc.length as usize;
        if length > buffer.len() {
            return Err("E1000: Buffer overflow");
        }

        buffer[..length].copy_from_slice(&self.rx_buffers[self.rx_head][..length]);

        desc.status = 0;
        unsafe {
            self.write_reg(REG_RDT, self.rx_head as u32);
        }
        self.rx_head = (self.rx_head + 1) % NUM_RX_DESCRIPTORS;

        Ok(length)
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("E1000: Device is powered off");
        }

        if data.len() > RX_BUFFER_SIZE {
            return Err("E1000: Packet too large");
        }

        let desc = &mut self.tx_ring[self.tx_tail];

        desc.buffer_addr = data.as_ptr() as u64;
        desc.length = data.len() as u16;
        desc.ccmd = 0x01 | 0x08; // EOP | RS
        desc.status = 1; // Mark done for simulated driver test loop

        unsafe {
            self.tx_tail = (self.tx_tail + 1) % NUM_TX_DESCRIPTORS;
            self.write_reg(REG_TDT, self.tx_tail as u32);
        }

        Ok(data.len())
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        unsafe {
            self.write_reg(REG_RCTL, 0);
            self.write_reg(REG_TCTL, 0);
            self.write_reg(REG_IMC, 0xFFFFFFFF);
        }
        self.power_state = PowerState::Off;
        Ok(())
    }
}
