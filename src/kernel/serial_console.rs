//! # 8250 / 16550A UART Serial Console & Early Debugger
//!
//! Provides bare-metal early serial console output and remote UDP netconsole
//! capabilities for SigmaOS.
//!
//! ## Ports & Registers
//! - COM1: `0x3F8`
//! - COM2: `0x2F8`
//! - Divisor Latch Access Bit (DLAB) programming for baud rate generator (115200 baud)
//! - FIFO Control Register (FCR) enabling 16-byte buffer and trigger levels
//! - Interrupt Enable Register (IER) and Line Status Register (LSR) polling

#![allow(dead_code)]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/// Standard PC COM port base addresses
pub const COM1_BASE: u16 = 0x03F8;
pub const COM2_BASE: u16 = 0x02F8;

/// UART register offsets from base I/O port
pub const UART_DATA: u16 = 0;       // Read/Write Data (DLAB = 0)
pub const UART_DLL: u16 = 0;        // Divisor Latch Low (DLAB = 1)
pub const UART_IER: u16 = 1;        // Interrupt Enable (DLAB = 0)
pub const UART_DLM: u16 = 1;        // Divisor Latch High (DLAB = 1)
pub const UART_IIR_FCR: u16 = 2;    // Interrupt Ident / FIFO Control
pub const UART_LCR: u16 = 3;        // Line Control Register
pub const UART_MCR: u16 = 4;        // Modem Control Register
pub const UART_LSR: u16 = 5;        // Line Status Register
pub const UART_MSR: u16 = 6;        // Modem Status Register

/// Line Status Register bits
pub const LSR_DATA_READY: u8 = 1 << 0;
pub const LSR_OVERRUN_ERROR: u8 = 1 << 1;
pub const LSR_PARITY_ERROR: u8 = 1 << 2;
pub const LSR_FRAMING_ERROR: u8 = 1 << 3;
pub const LSR_BREAK_INTERRUPT: u8 = 1 << 4;
pub const LSR_TX_HOLD_EMPTY: u8 = 1 << 5;
pub const LSR_TX_EMPTY: u8 = 1 << 6;

/// Hardware 16550A Serial Driver
pub struct SerialPort {
    pub base_port: u16,
    pub initialized: bool,
}

impl SerialPort {
    pub const fn new(base_port: u16) -> Self {
        Self {
            base_port,
            initialized: false,
        }
    }

    /// Initialize port to 115200 baud, 8 data bits, no parity, 1 stop bit.
    pub fn init(&mut self) {
        // Disable interrupts
        self.outb(UART_IER, 0x00);

        // Enable DLAB (set baud rate divisor)
        self.outb(UART_LCR, 0x80);

        // Set divisor to 1 (115200 baud)
        self.outb(UART_DLL, 0x01);
        self.outb(UART_DLM, 0x00);

        // 8 bits, no parity, one stop bit
        self.outb(UART_LCR, 0x03);

        // Enable FIFO, clear them, with 14-byte threshold
        self.outb(UART_IIR_FCR, 0xC7);

        // IRQs enabled, RTS/DSR set
        self.outb(UART_MCR, 0x0B);

        self.initialized = true;
    }

    /// Check if transmit holding register is empty.
    pub fn is_transmit_empty(&self) -> bool {
        (self.inb(UART_LSR) & LSR_TX_HOLD_EMPTY) != 0
    }

    /// Write a single byte to the serial port.
    pub fn send(&self, byte: u8) {
        while !self.is_transmit_empty() {
            // spin wait
        }
        self.outb(UART_DATA, byte);
    }

    /// Write a string slice to serial port.
    pub fn write_str(&self, s: &str) {
        for b in s.bytes() {
            if b == b'\n' {
                self.send(b'\r');
            }
            self.send(b);
        }
    }

    /// Low-level port I/O simulation / primitive
    fn outb(&self, offset: u16, val: u8) {
        // In kernel mode with inline asm:
        // unsafe { core::arch::asm!("out dx, al", in("dx") self.base_port + offset, in("al") val); }
        let _ = (offset, val);
    }

    fn inb(&self, offset: u16) -> u8 {
        let _ = offset;
        LSR_TX_HOLD_EMPTY // Mock ready for testing
    }
}

/// Remote Netconsole UDP Logger
pub struct NetconsoleSender {
    pub target_ip: [u8; 4],
    pub target_port: u16,
    pub source_port: u16,
    pub seq: u32,
}

impl NetconsoleSender {
    pub fn new(target_ip: [u8; 4], target_port: u16) -> Self {
        Self {
            target_ip,
            target_port,
            source_port: 6665,
            seq: 0,
        }
    }

    /// Construct and queue early panic UDP packet to remote syslog/netconsole receiver.
    pub fn send_log(&mut self, message: &str) -> Vec<u8> {
        let mut packet = Vec::with_capacity(message.len() + 32);
        packet.extend_from_slice(b"NETCONSOLE:");
        packet.extend_from_slice(&self.seq.to_be_bytes());
        packet.extend_from_slice(b":");
        packet.extend_from_slice(message.as_bytes());
        self.seq = self.seq.wrapping_add(1);
        packet
    }
}
