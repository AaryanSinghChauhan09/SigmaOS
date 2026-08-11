// Intel High Definition Audio (HDA) Controller Driver
// Conforms to SigmaOS UnifiedPeripheral interface

use crate::drivers::peripheral::{DeviceGeneration, PeripheralDevice, PowerState};
use crate::security::capability::CapabilityToken;
use core::ptr::{read_volatile, write_volatile};

extern crate alloc;
use alloc::boxed::Box;

// Global Controller Registers (MMIO)
const REG_GCAP: u16 = 0x0000; // Global Capabilities
const REG_GCTL: u16 = 0x0008; // Global Control
const REG_WAKEEN: u16 = 0x000C; // Wake Enable
const REG_STATESTS: u16 = 0x000E; // State Status (Codec Wake detection)
const REG_INTCTL: u16 = 0x0020; // Interrupt Control
const REG_INTSTS: u16 = 0x0024; // Interrupt Status
const REG_CORBBLBASE: u16 = 0x0040; // CORB Lower Base Address
const REG_CORBBUBASE: u16 = 0x0044; // CORB Upper Base Address
const REG_CORBWP: u16 = 0x0048; // CORB Write Pointer
const REG_CORBRP: u16 = 0x004A; // CORB Read Pointer
const REG_CORBCTL: u16 = 0x004C; // CORB Control
const REG_RIRBBLBASE: u16 = 0x0050; // RIRB Lower Base Address
const REG_RIRBBUBASE: u16 = 0x0054; // RIRB Upper Base Address
const REG_RIRBWP: u16 = 0x0058; // RIRB Write Pointer
const REG_RIRBCTL: u16 = 0x005C; // RIRB Control

// Stream Descriptor Registers (Stream 0: Output 1)
const REG_SD_CTL: u16 = 0x0080; // Stream Descriptor Control
const REG_SD_STS: u16 = 0x0083; // Stream Descriptor Status
const REG_SD_LPIB: u16 = 0x0084; // Link Position In Buffer
const REG_SD_CBL: u16 = 0x0088; // Cyclic Buffer Length
const REG_SD_LVI: u16 = 0x008C; // Last Valid Index (LVI)
const REG_SD_BDPL: u16 = 0x0090; // Buffer Descriptor List Pointer Low
const REG_SD_BDPU: u16 = 0x0094; // Buffer Descriptor List Pointer High

// CORB/RIRB Ring sizes
const CORB_SIZE: usize = 256;
const RIRB_SIZE: usize = 256;
const NUM_BUFFERS: usize = 2;
const BUFFER_SIZE: usize = 16384; // 16KB double-buffered stream

/// Buffer Descriptor List Entry (BDLE)
#[repr(C, packed)]
#[derive(Clone, Copy, Default)]
pub struct Bdle {
    pub address: u64,
    pub length: u32,
    pub ioc: u32, // Interrupt On Completion bit
}

/// Intel HDA audio hardware state
pub struct IntelHdaDriver {
    pub mmio_base: usize,
    pub corb_ring: &'static mut [u32; CORB_SIZE],
    pub rirb_ring: &'static mut [u64; RIRB_SIZE],
    pub bdl_list: &'static mut [Bdle; NUM_BUFFERS],
    pub audio_buffers: [[u8; BUFFER_SIZE]; NUM_BUFFERS],
    pub corb_write_idx: u16,
    pub current_buffer: usize,
    pub power_state: PowerState,
    pub capabilities: CapabilityToken,
}

impl IntelHdaDriver {
    /// Creates a new uninitialized HDA driver mapped to a specific MMIO memory address
    pub unsafe fn new(mmio_base: usize, capabilities: CapabilityToken) -> Self {
        // Transmute page-aligned DMA physical addresses
        #[cfg(target_os = "none")]
        let corb_ring = core::mem::transmute(0x00300000usize as *mut [u32; CORB_SIZE]);
        #[cfg(not(target_os = "none"))]
        let corb_ring = Box::leak(Box::new([0u32; CORB_SIZE]));

        #[cfg(target_os = "none")]
        let rirb_ring = core::mem::transmute(0x00400000usize as *mut [u64; RIRB_SIZE]);
        #[cfg(not(target_os = "none"))]
        let rirb_ring = Box::leak(Box::new([0u64; RIRB_SIZE]));

        #[cfg(target_os = "none")]
        let bdl_list = core::mem::transmute(0x00500000usize as *mut [Bdle; NUM_BUFFERS]);
        #[cfg(not(target_os = "none"))]
        let bdl_list = Box::leak(Box::new([Bdle::default(); NUM_BUFFERS]));

        Self {
            mmio_base,
            corb_ring,
            rirb_ring,
            bdl_list,
            audio_buffers: [[0u8; BUFFER_SIZE]; NUM_BUFFERS],
            corb_write_idx: 0,
            current_buffer: 0,
            power_state: PowerState::Off,
            capabilities,
        }
    }

    unsafe fn read_reg32(&self, offset: u16) -> u32 {
        #[cfg(target_os = "none")]
        {
            read_volatile((self.mmio_base + offset as usize) as *const u32)
        }
        #[cfg(not(target_os = "none"))]
        {
            if offset == REG_GCTL {
                1 // Return enabled GCTL state on test platforms
            } else {
                0
            }
        }
    }

    unsafe fn write_reg32(&self, offset: u16, value: u32) {
        #[cfg(target_os = "none")]
        {
            write_volatile((self.mmio_base + offset as usize) as *mut u32, value);
        }
    }

    unsafe fn read_reg16(&self, offset: u16) -> u16 {
        #[cfg(target_os = "none")]
        {
            read_volatile((self.mmio_base + offset as usize) as *const u16)
        }
        #[cfg(not(target_os = "none"))]
        {
            0
        }
    }

    unsafe fn write_reg16(&self, offset: u16, value: u16) {
        #[cfg(target_os = "none")]
        {
            write_volatile((self.mmio_base + offset as usize) as *mut u16, value);
        }
    }

    unsafe fn read_reg8(&self, offset: u16) -> u8 {
        #[cfg(target_os = "none")]
        {
            read_volatile((self.mmio_base + offset as usize) as *const u8)
        }
        #[cfg(not(target_os = "none"))]
        {
            0
        }
    }

    unsafe fn write_reg8(&self, offset: u16, value: u8) {
        #[cfg(target_os = "none")]
        {
            write_volatile((self.mmio_base + offset as usize) as *mut u8, value);
        }
    }

    /// Sends a verb command to a specific codec and parses the RIRB response
    pub unsafe fn send_verb(
        &mut self,
        codec_id: u8,
        node_id: u8,
        verb: u32,
    ) -> Result<u64, &'static str> {
        let cmd = ((codec_id as u32) << 28) | ((node_id as u32) << 20) | verb;

        // 1. Write command to CORB ring
        self.corb_write_idx = (self.corb_write_idx + 1) % (CORB_SIZE as u16);
        self.corb_ring[self.corb_write_idx as usize] = cmd;

        // Update hardware write pointer
        self.write_reg16(REG_CORBWP, self.corb_write_idx);

        // 2. Wait until processed by checking RIRB Write Pointer (RIRBWP)
        let mut timeout = 1000;
        while timeout > 0 {
            let rirb_wp = self.read_reg16(REG_RIRBWP);
            if rirb_wp == self.corb_write_idx {
                // Command processed, fetch response from RIRB
                return Ok(self.rirb_ring[rirb_wp as usize]);
            }
            core::hint::spin_loop();
            timeout -= 1;
        }

        Err("Intel HDA: Codec communication timeout")
    }
}

impl PeripheralDevice for IntelHdaDriver {
    fn name(&self) -> &'static str {
        "Intel HD Audio"
    }

    fn generation(&self) -> DeviceGeneration {
        DeviceGeneration::Modern
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        // Audio devices require explicit hardware execution capabilities
        if self.capabilities.bits() & 0x01 == 0 {
            return Err("Intel HDA: PermissionDenied - Missing Device capabilities");
        }

        unsafe {
            // 1. Reset the HDA Controller (GCTL reset bit to 0, wait, then to 1)
            let gctl = self.read_reg32(REG_GCTL);
            self.write_reg32(REG_GCTL, gctl & !1);
            let mut timeout = 1000;
            while (self.read_reg32(REG_GCTL) & 1) != 0 && timeout > 0 {
                core::hint::spin_loop();
                timeout -= 1;
            }

            self.write_reg32(REG_GCTL, self.read_reg32(REG_GCTL) | 1);
            let mut timeout = 1000;
            while (self.read_reg32(REG_GCTL) & 1) == 0 && timeout > 0 {
                core::hint::spin_loop();
                timeout -= 1;
            }

            // 2. Set up CORB Base Address
            let corb_phys = self.corb_ring.as_ptr() as u64;
            self.write_reg32(REG_CORBBLBASE, (corb_phys & 0xFFFFFFFF) as u32);
            self.write_reg32(REG_CORBBUBASE, (corb_phys >> 32) as u32);

            // Reset CORB Read/Write pointers
            self.write_reg16(REG_CORBWP, 0);
            self.write_reg16(REG_CORBRP, 0x8000); // Set Reset bit in Read Pointer
            self.write_reg16(REG_CORBRP, 0); // Clear Reset

            // Start CORB engine (CORBCTL = CORBRUN)
            self.write_reg8(REG_CORBCTL, 0x02);

            // 3. Set up RIRB Base Address
            let rirb_phys = self.rirb_ring.as_ptr() as u64;
            self.write_reg32(REG_RIRBBLBASE, (rirb_phys & 0xFFFFFFFF) as u32);
            self.write_reg32(REG_RIRBBUBASE, (rirb_phys >> 32) as u32);

            // Reset RIRB write pointer
            self.write_reg16(REG_RIRBWP, 0x8000); // Reset bit
            self.write_reg16(REG_RIRBWP, 0);

            // Start RIRB engine (RIRBCTL = RIRBRUN)
            self.write_reg8(REG_RIRBCTL, 0x02);

            // 4. Initialize stream descriptor double-buffering
            let bdl_phys = self.bdl_list.as_ptr() as u64;
            self.write_reg32(REG_SD_BDPL, (bdl_phys & 0xFFFFFFFF) as u32);
            self.write_reg32(REG_SD_BDPU, (bdl_phys >> 32) as u32);

            for i in 0..NUM_BUFFERS {
                self.bdl_list[i].address = self.audio_buffers[i].as_ptr() as u64;
                self.bdl_list[i].length = BUFFER_SIZE as u32;
                self.bdl_list[i].ioc = 1; // Trigger interrupt on buffer empty
            }

            // Set Cyclic Buffer Length (CBL)
            self.write_reg32(REG_SD_CBL, (BUFFER_SIZE * NUM_BUFFERS) as u32);
            // Set Last Valid Index (LVI) to count - 1
            self.write_reg8(REG_SD_LVI, (NUM_BUFFERS - 1) as u8);

            // Enable stream interrupt mask
            self.write_reg32(REG_INTCTL, self.read_reg32(REG_INTCTL) | 0x01 | 0x80000000);
        }

        self.power_state = PowerState::On;
        Ok(())
    }

    fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, &'static str> {
        // Input streams use similar descriptor indices. This template models write playback
        Err("Intel HDA: Read operation not supported on playback descriptor")
    }

    fn write(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.power_state != PowerState::On {
            return Err("Intel HDA: Device is powered off");
        }

        // Write streaming frames into the inactive cyclic DMA buffer
        let length = data.len().min(BUFFER_SIZE);
        self.audio_buffers[self.current_buffer][..length].copy_from_slice(&data[..length]);

        unsafe {
            // Start playback if currently stopped (SD_CTL |= SRUN)
            let ctrl = self.read_reg8(REG_SD_CTL);
            if (ctrl & 0x02) == 0 {
                self.write_reg8(REG_SD_CTL, ctrl | 0x02);
            }
        }

        // Toggle double buffer pointer
        self.current_buffer = (self.current_buffer + 1) % NUM_BUFFERS;
        Ok(length)
    }

    fn set_power_state(&mut self, state: PowerState) -> Result<(), &'static str> {
        self.power_state = state;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        unsafe {
            // Stop stream (SD_CTL &= !SRUN)
            self.write_reg8(REG_SD_CTL, self.read_reg8(REG_SD_CTL) & !0x02);
            // Disable global interrupts
            self.write_reg32(REG_INTCTL, 0);
        }
        self.power_state = PowerState::Off;
        Ok(())
    }
}
