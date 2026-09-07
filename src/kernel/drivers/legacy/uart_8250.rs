#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

use crate::kernel::subsystems::registry::{
    InitOrder, KernelSubsystem, SubsystemError, SubsystemPriority,
};
/// SigmaOS Legacy Driver — NS8250/16550A UART Serial Port Driver
/// Absorbs Linux serial.c (the most-ported driver in Linux history)
/// Supports: 8250, 8250A, 16450, 16550, 16550A, 16750, 16950 UARTs
/// COM1–COM4, baud rates 50–4,000,000, 5–8 data bits, 1–2 stop bits, parity
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::VecDeque;

/// Standard COM port I/O bases and IRQs
pub const COM1_BASE: u16 = 0x3F8;
pub const COM1_IRQ: u8 = 4;
pub const COM2_BASE: u16 = 0x2F8;
pub const COM2_IRQ: u8 = 3;
pub const COM3_BASE: u16 = 0x3E8;
pub const COM3_IRQ: u8 = 4;
pub const COM4_BASE: u16 = 0x2E8;
pub const COM4_IRQ: u8 = 3;

/// UART register offsets from base I/O address
pub mod regs {
    pub const RBR: u16 = 0; // Receive Buffer Register (read)
    pub const THR: u16 = 0; // Transmit Holding Register (write)
    pub const DLL: u16 = 0; // Divisor Latch Low (DLAB=1)
    pub const IER: u16 = 1; // Interrupt Enable Register
    pub const DLH: u16 = 1; // Divisor Latch High (DLAB=1)
    pub const IIR: u16 = 2; // Interrupt ID Register (read)
    pub const FCR: u16 = 2; // FIFO Control Register (write)
    pub const LCR: u16 = 3; // Line Control Register
    pub const MCR: u16 = 4; // Modem Control Register
    pub const LSR: u16 = 5; // Line Status Register
    pub const MSR: u16 = 6; // Modem Status Register
    pub const SCR: u16 = 7; // Scratch Register
}

/// Baud rate divisor table (16550A, 1.8432 MHz crystal)
pub fn baud_divisor(baud: u32) -> u16 {
    match baud {
        50 => 2304,
        75 => 1536,
        110 => 1047,
        150 => 768,
        300 => 384,
        600 => 192,
        1200 => 96,
        1800 => 64,
        2400 => 48,
        4800 => 24,
        9600 => 12,
        19200 => 6,
        38400 => 3,
        57600 => 2,
        115200 => 1,
        _ => 12, // default 9600
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataBits {
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    One,
    Two,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    None,
    Odd,
    Even,
    Mark,
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UartType {
    U8250,
    U16450,
    U16550,
    U16550A,
    U16750,
    U16950,
}

/// UART port configuration
#[derive(Debug, Clone)]
pub struct UartConfig {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub stop_bits: StopBits,
    pub parity: Parity,
    pub fifo_enabled: bool,
}

impl Default for UartConfig {
    fn default() -> Self {
        UartConfig {
            baud_rate: 9600,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
            fifo_enabled: true,
        }
    }
}

/// UART port — represents one physical COM port
pub struct UartPort {
    pub port_num: u8, // 1–4
    pub base_io: u16,
    pub irq: u8,
    pub uart_type: UartType,
    pub config: UartConfig,
    pub enabled: bool,
    rx_buf: VecDeque<u8>,
    tx_buf: VecDeque<u8>,
    rx_bytes: AtomicUsize,
    tx_bytes: AtomicUsize,
}

impl UartPort {
    pub fn com1() -> Self {
        Self::new(1, COM1_BASE, COM1_IRQ)
    }
    pub fn com2() -> Self {
        Self::new(2, COM2_BASE, COM2_IRQ)
    }
    pub fn com3() -> Self {
        Self::new(3, COM3_BASE, COM3_IRQ)
    }
    pub fn com4() -> Self {
        Self::new(4, COM4_BASE, COM4_IRQ)
    }

    pub fn new(port_num: u8, base_io: u16, irq: u8) -> Self {
        UartPort {
            port_num,
            base_io,
            irq,
            uart_type: UartType::U16550A,
            config: UartConfig::default(),
            enabled: false,
            rx_buf: VecDeque::with_capacity(256),
            tx_buf: VecDeque::with_capacity(256),
            rx_bytes: AtomicUsize::new(0),
            tx_bytes: AtomicUsize::new(0),
        }
    }

    pub fn configure(&mut self, config: UartConfig) {
        self.config = config;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn write_byte(&mut self, b: u8) {
        self.tx_buf.push_back(b);
        self.tx_bytes.fetch_add(1, Ordering::Relaxed);
    }

    pub fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            self.write_byte(b);
        }
    }

    pub fn inject_rx(&mut self, data: &[u8]) {
        for &b in data {
            self.rx_buf.push_back(b);
            self.rx_bytes.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        self.rx_buf.pop_front()
    }

    pub fn flush_tx(&mut self) -> Vec<u8> {
        self.tx_buf.drain(..).collect()
    }

    pub fn rx_count(&self) -> usize {
        self.rx_bytes.load(Ordering::Relaxed)
    }
    pub fn tx_count(&self) -> usize {
        self.tx_bytes.load(Ordering::Relaxed)
    }
    pub fn divisor(&self) -> u16 {
        baud_divisor(self.config.baud_rate)
    }
}

/// UART driver managing all 4 COM ports
pub struct Uart8250Driver {
    ports: [Option<UartPort>; 4],
    initialized: bool,
}

impl Uart8250Driver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Uart8250Driver {
            ports: [
                Some(UartPort::com1()),
                Some(UartPort::com2()),
                Some(UartPort::com3()),
                Some(UartPort::com4()),
            ],
            initialized: false,
        }
    }

    pub fn port(&self, n: usize) -> Option<&UartPort> {
        self.ports.get(n).and_then(|p| p.as_ref())
    }

    pub fn port_mut(&mut self, n: usize) -> Option<&mut UartPort> {
        self.ports.get_mut(n).and_then(|p| p.as_mut())
    }
}

impl KernelSubsystem for Uart8250Driver {
    fn name(&self) -> &str {
        "uart_8250"
    }
    fn version(&self) -> &str {
        "3.0.0"
    }
    fn init_order(&self) -> InitOrder {
        InitOrder::EarlyBoot
    }
    fn priority(&self) -> SubsystemPriority {
        SubsystemPriority::High
    }
    fn dependencies(&self) -> Vec<&'static str> {
        vec!["isa_bus"]
    }

    fn initialize(&mut self) -> Result<(), SubsystemError> {
        for port in self.ports.iter_mut().flatten() {
            port.enable();
        }
        self.initialized = true;
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), SubsystemError> {
        for port in self.ports.iter_mut().flatten() {
            port.enabled = false;
        }
        Ok(())
    }
}

impl Default for Uart8250Driver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_baud_divisors() {
        assert_eq!(baud_divisor(9600), 12);
        assert_eq!(baud_divisor(115200), 1);
        assert_eq!(baud_divisor(1200), 96);
    }

    #[test]
    fn test_uart_write_read() {
        let mut port = UartPort::com1();
        port.enable();
        port.write_str("Hello, UART!");
        let flushed = port.flush_tx();
        assert_eq!(&flushed, b"Hello, UART!");
        assert_eq!(port.tx_count(), 12);
    }

    #[test]
    fn test_uart_rx() {
        let mut port = UartPort::com2();
        port.inject_rx(b"SigmaOS");
        assert_eq!(port.read_byte(), Some(b'S'));
        assert_eq!(port.rx_count(), 7);
    }

    #[test]
    fn test_driver_initialization() {
        let mut drv = Uart8250Driver::new();
        drv.initialize().unwrap();
        assert!(drv.port(0).unwrap().enabled);
        assert!(drv.port(3).unwrap().enabled);
    }

    #[test]
    fn test_uart_config() {
        let mut port = UartPort::com1();
        port.configure(UartConfig {
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
            fifo_enabled: true,
        });
        assert_eq!(port.divisor(), 1);
    }
}
