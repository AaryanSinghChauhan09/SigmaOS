// usb_core.rs: Core USB Subsystem & xHCI Driver Skeleton for SigmaOS

#![no_std]

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ptr::{read_volatile, write_volatile};

/// 16-byte Transfer Request Block (TRB)
/// The fundamental data structure for xHCI communication
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Trb {
    pub parameter: u64,
    pub status: u32,
    pub control: u32,
}

/// A generic Ring Buffer of TRBs used for Command, Event, and Transfer Rings
pub struct TrbRing {
    pub ring: Vec<Trb>,
    pub enqueue_ptr: usize,
    pub dequeue_ptr: usize,
    pub cycle_state: u32, // Cycle bit toggles when wrapping
}

impl TrbRing {
    pub fn new(size: usize) -> Self {
        Self {
            ring: alloc::vec![Trb::default(); size],
            enqueue_ptr: 0,
            dequeue_ptr: 0,
            cycle_state: 1,
        }
    }
}

/// Device Context Base Address Array (DCBAA)
/// Points to up to 255 Device Contexts for active USB devices
#[repr(C)]
pub struct Dcbaa {
    pub pointers: [u64; 256], // Index 0 is scratchpad array
}

/// Memory-Mapped xHCI Operational Registers
#[repr(C)]
pub struct XhciOperationalRegs {
    pub usbcmd: u32,
    pub usbsts: u32,
    pub pagesize: u32,
    pub rsvd1: [u32; 2],
    pub dnctrl: u32,
    pub crcr: u64, // Command Ring Control Register
    pub rsvd2: [u32; 4],
    pub dcbaap: u64, // DCBAA Pointer
    pub config: u32,
}

const USBCMD_RUN: u32 = 1 << 0;
const USBCMD_HCRST: u32 = 1 << 1;
const USBSTS_HALTED: u32 = 1 << 0;

pub struct XhciController {
    op_regs: *mut XhciOperationalRegs,
    dcbaa: Box<Dcbaa>,
    command_ring: TrbRing,
    event_ring: TrbRing,
}

impl XhciController {
    pub fn new(op_base: usize) -> Self {
        Self {
            op_regs: op_base as *mut XhciOperationalRegs,
            dcbaa: Box::new(Dcbaa { pointers: [0; 256] }),
            command_ring: TrbRing::new(256),
            event_ring: TrbRing::new(256),
        }
    }

    /// Initialize the xHCI Controller
    pub fn init(&mut self) -> Result<(), &'static str> {
        unsafe {
            // 1. Halt the controller
            let mut cmd = read_volatile(&mut (*self.op_regs).usbcmd);
            cmd &= !USBCMD_RUN;
            write_volatile(&mut (*self.op_regs).usbcmd, cmd);

            // Wait for HCHalted
            while (read_volatile(&mut (*self.op_regs).usbsts) & USBSTS_HALTED) == 0 {
                // busy wait
            }

            // 2. Reset the controller
            write_volatile(&mut (*self.op_regs).usbcmd, USBCMD_HCRST);
            while (read_volatile(&mut (*self.op_regs).usbcmd) & USBCMD_HCRST) != 0 {
                // wait for reset to complete
            }

            // 3. Set Device Context Base Address Array (DCBAAP)
            let dcbaa_phys = self.dcbaa.as_ref() as *const _ as u64; // Stub for physical addr
            write_volatile(&mut (*self.op_regs).dcbaap, dcbaa_phys);

            // 4. Set Command Ring Control Register (CRCR)
            let cmd_ring_phys = self.command_ring.ring.as_ptr() as u64; // Stub for physical addr
            // Cycle bit (RCS) is bit 0. We start with Cycle bit = 1.
            write_volatile(&mut (*self.op_regs).crcr, cmd_ring_phys | 1);

            // 5. Initialize Event Rings (Interrupter setup omitted for brevity in MVP skeleton)
            // ...

            // 6. Run the controller
            cmd = read_volatile(&mut (*self.op_regs).usbcmd);
            cmd |= USBCMD_RUN;
            write_volatile(&mut (*self.op_regs).usbcmd, cmd);
            
            // Wait until HCHalted is clear
            while (read_volatile(&mut (*self.op_regs).usbsts) & USBSTS_HALTED) != 0 {
                // busy wait
            }
        }
        
        Ok(())
    }

    pub fn submit_command_trb(&mut self, trb: Trb) {
        // Enqueue the TRB on the command ring, toggle cycle bit, and ring Doorbell 0
        // (Implementation details omitted for brevity)
    }
}
