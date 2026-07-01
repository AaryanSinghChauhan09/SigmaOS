/// Sovereign Minimal I/O API
/// Bypasses all standard libraries and libc dependencies by using direct assembly for Port I/O.

use core::arch::asm;

pub struct Port {
    port: u16,
}

impl Port {
    pub const fn new(port: u16) -> Self {
        Self { port }
    }

    /// Read an 8-bit value from the I/O port.
    pub unsafe fn read_u8(&self) -> u8 {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") self.port, options(nomem, nostack, preserves_flags));
        value
    }

    /// Write an 8-bit value to the I/O port.
    pub unsafe fn write_u8(&mut self, value: u8) {
        asm!("out dx, al", in("dx") self.port, in("al") value, options(nomem, nostack, preserves_flags));
    }
    
    // Additional methods for 16-bit and 32-bit I/O can be added here
}

/// A basic, sovereign abstraction for handling hardware interrupts.
/// In the future, this will hook into the formally verified Ada/SPARK interrupt handlers.
pub trait InterruptHandler {
    fn handle_interrupt(&mut self, irq_number: u8);
}
