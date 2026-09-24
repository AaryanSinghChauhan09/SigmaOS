// SigmaOS UART 16550 Serial Port Driver
// Inspired by Linux drivers/tty/serial/8250/
//
// Provides serial communication for kernel debug logging and console I/O.
// Supports COM1-COM4 with configurable baud rate, data format, and FIFO.

use std::fmt;

// ──────────────────────────── I/O Port Abstraction ────────────────────────────

/// Trait for abstracting port I/O operations.
/// On bare-metal, this uses `in`/`out` x86 instructions.
/// In hosted mode, this provides a simulation layer.
pub trait PortIo {
    fn read_port(&self, port: u16) -> u8;
    fn write_port(&self, port: u16, value: u8);
}

/// Simulated port I/O for hosted (std) environment
#[derive(Debug, Clone)]
pub struct SimulatedPortIo {
    /// Simulated port register file (64K ports)
    registers: Vec<u8>,
}

impl SimulatedPortIo {
    pub fn new() -> Self {
        Self {
            registers: vec![0u8; 65536],
        }
    }
}

impl Default for SimulatedPortIo {
    fn default() -> Self {
        Self::new()
    }
}

impl PortIo for SimulatedPortIo {
    fn read_port(&self, port: u16) -> u8 {
        self.registers[port as usize]
    }

    fn write_port(&self, port: u16, value: u8) {
        // In simulation, we just store the value
        // A real implementation would use: unsafe { asm!("out dx, al", in("dx") port, in("al") value) }
        let _ = (port, value);
    }
}

// ──────────────────────────── UART Register Definitions ───────────────────────

/// UART 16550 register offsets from base address
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum UartRegister {
    /// Data Register (read: RX buffer, write: TX buffer)
    /// When DLAB=1: Divisor Latch Low byte
    Data = 0,
    /// Interrupt Enable Register
    /// When DLAB=1: Divisor Latch High byte
    InterruptEnable = 1,
    /// Interrupt Identification Register (read)
    /// FIFO Control Register (write)
    IirFcr = 2,
    /// Line Control Register (data format, DLAB bit)
    LineControl = 3,
    /// Modem Control Register (DTR, RTS, loopback)
    ModemControl = 4,
    /// Line Status Register (data ready, TX empty, errors)
    LineStatus = 5,
    /// Modem Status Register (CTS, DSR, RI, DCD)
    ModemStatus = 6,
    /// Scratch Register
    Scratch = 7,
}

impl UartRegister {
    pub fn offset(self) -> u16 {
        self as u16
    }
}

/// Line Status Register bit flags
pub mod lsr {
    pub const DATA_READY: u8 = 0x01;
    pub const OVERRUN_ERROR: u8 = 0x02;
    pub const PARITY_ERROR: u8 = 0x04;
    pub const FRAMING_ERROR: u8 = 0x08;
    pub const BREAK_INDICATOR: u8 = 0x10;
    pub const TX_HOLDING_EMPTY: u8 = 0x20;
    pub const TX_EMPTY: u8 = 0x40;
    pub const FIFO_ERROR: u8 = 0x80;
}

/// Modem Control Register bit flags
pub mod mcr {
    pub const DTR: u8 = 0x01;
    pub const RTS: u8 = 0x02;
    pub const OUT1: u8 = 0x04;
    pub const OUT2: u8 = 0x08;
    pub const LOOPBACK: u8 = 0x10;
}

/// FIFO Control Register bit flags
pub mod fcr {
    pub const ENABLE_FIFO: u8 = 0x01;
    pub const CLEAR_RX: u8 = 0x02;
    pub const CLEAR_TX: u8 = 0x04;
    pub const DMA_MODE: u8 = 0x08;
    pub const TRIGGER_1: u8 = 0x00;
    pub const TRIGGER_4: u8 = 0x40;
    pub const TRIGGER_8: u8 = 0x80;
    pub const TRIGGER_14: u8 = 0xC0;
}

/// Interrupt Enable Register bit flags
pub mod ier {
    pub const RX_AVAILABLE: u8 = 0x01;
    pub const TX_EMPTY: u8 = 0x02;
    pub const LINE_STATUS: u8 = 0x04;
    pub const MODEM_STATUS: u8 = 0x08;
}

/// Line Control Register bit flags
pub mod lcr {
    pub const DLAB: u8 = 0x80;
    pub const BREAK_ENABLE: u8 = 0x40;
}

// ──────────────────────────── COM Port Definitions ────────────────────────────

/// Standard COM port base I/O addresses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComPort {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}

impl ComPort {
    pub fn base_address(self) -> u16 {
        self as u16
    }

    pub fn irq(self) -> u8 {
        match self {
            ComPort::COM1 | ComPort::COM3 => 4,
            ComPort::COM2 | ComPort::COM4 => 3,
        }
    }
}

// ──────────────────────────── Serial Configuration ───────────────────────────

/// Baud rate divisor values for the 16550 UART (1.8432 MHz oscillator)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaudRate {
    Baud115200 = 1,
    Baud57600 = 2,
    Baud38400 = 3,
    Baud19200 = 6,
    Baud9600 = 12,
    Baud4800 = 24,
    Baud2400 = 48,
    Baud1200 = 96,
}

impl BaudRate {
    pub fn divisor(self) -> u16 {
        self as u16
    }
}

/// Data bits per character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataBits {
    Five = 0x00,
    Six = 0x01,
    Seven = 0x02,
    Eight = 0x03,
}

/// Stop bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    One = 0x00,
    Two = 0x04,
}

/// Parity mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    None = 0x00,
    Odd = 0x08,
    Even = 0x18,
    Mark = 0x28,
    Space = 0x38,
}

/// Serial port configuration
#[derive(Debug, Clone, Copy)]
pub struct SerialConfig {
    pub baud_rate: BaudRate,
    pub data_bits: DataBits,
    pub stop_bits: StopBits,
    pub parity: Parity,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            baud_rate: BaudRate::Baud115200,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
        }
    }
}

// ──────────────────────────── Serial Port Driver ─────────────────────────────

/// UART 16550 Serial Port Driver
///
/// Provides byte-level and string-level I/O over a standard PC serial port.
/// Initialization performs a loopback self-test to verify the UART is functional.
#[derive(Debug)]
pub struct SerialPort {
    /// COM port base address
    base: u16,
    /// COM port identifier
    port: ComPort,
    /// Configuration
    config: SerialConfig,
    /// Whether the port has been initialized
    initialized: bool,
    /// Bytes transmitted count
    tx_count: u64,
    /// Bytes received count
    rx_count: u64,
    /// Error count
    error_count: u64,
}

impl SerialPort {
    /// Create a new serial port driver for the given COM port
    pub fn new(port: ComPort) -> Self {
        Self {
            base: port.base_address(),
            port,
            config: SerialConfig::default(),
            initialized: false,
            tx_count: 0,
            rx_count: 0,
            error_count: 0,
        }
    }

    /// Create with custom configuration
    pub fn with_config(port: ComPort, config: SerialConfig) -> Self {
        Self {
            base: port.base_address(),
            port,
            config,
            initialized: false,
            tx_count: 0,
            rx_count: 0,
            error_count: 0,
        }
    }

    /// Initialize the UART with the configured settings
    ///
    /// Performs:
    /// 1. Disable all interrupts
    /// 2. Set DLAB to configure baud rate divisor
    /// 3. Configure data format (data bits, stop bits, parity)
    /// 4. Enable and clear FIFOs (14-byte trigger)
    /// 5. Configure modem control (DTR, RTS, OUT2)
    /// 6. Loopback self-test
    /// 7. Normal operation mode
    pub fn init(&mut self, io: &dyn PortIo) -> Result<(), SerialError> {
        // Step 1: Disable all interrupts
        io.write_port(self.reg(UartRegister::InterruptEnable), 0x00);

        // Step 2: Enable DLAB to set baud rate
        io.write_port(self.reg(UartRegister::LineControl), lcr::DLAB);

        // Set divisor (low byte then high byte)
        let divisor = self.config.baud_rate.divisor();
        io.write_port(self.reg(UartRegister::Data), (divisor & 0xFF) as u8);
        io.write_port(
            self.reg(UartRegister::InterruptEnable),
            ((divisor >> 8) & 0xFF) as u8,
        );

        // Step 3: Set data format (clears DLAB)
        let line_ctrl = self.config.data_bits as u8
            | self.config.stop_bits as u8
            | self.config.parity as u8;
        io.write_port(self.reg(UartRegister::LineControl), line_ctrl);

        // Step 4: Enable FIFO, clear buffers, 14-byte trigger threshold
        io.write_port(
            self.reg(UartRegister::IirFcr),
            fcr::ENABLE_FIFO | fcr::CLEAR_RX | fcr::CLEAR_TX | fcr::TRIGGER_14,
        );

        // Step 5: Set modem control (DTR + RTS + OUT2 for interrupts)
        io.write_port(
            self.reg(UartRegister::ModemControl),
            mcr::DTR | mcr::RTS | mcr::OUT2,
        );

        // Step 6: Loopback self-test
        io.write_port(
            self.reg(UartRegister::ModemControl),
            mcr::DTR | mcr::RTS | mcr::OUT1 | mcr::OUT2 | mcr::LOOPBACK,
        );

        // Send test byte
        let test_byte: u8 = 0xAE;
        io.write_port(self.reg(UartRegister::Data), test_byte);

        // Read back and verify
        let readback = io.read_port(self.reg(UartRegister::Data));
        if readback != test_byte {
            return Err(SerialError::LoopbackFailed {
                sent: test_byte,
                received: readback,
            });
        }

        // Step 7: Normal operation (disable loopback, enable IRQs on modem)
        io.write_port(
            self.reg(UartRegister::ModemControl),
            mcr::DTR | mcr::RTS | mcr::OUT1 | mcr::OUT2,
        );

        self.initialized = true;
        Ok(())
    }

    /// Write a single byte, waiting for TX holding register to be empty
    pub fn write_byte(&mut self, io: &dyn PortIo, byte: u8) {
        // Poll until the transmitter holding register is empty
        while io.read_port(self.reg(UartRegister::LineStatus)) & lsr::TX_HOLDING_EMPTY == 0 {
            core::hint::spin_loop();
        }
        io.write_port(self.reg(UartRegister::Data), byte);
        self.tx_count += 1;
    }

    /// Read a single byte, waiting for data to be available
    pub fn read_byte(&mut self, io: &dyn PortIo) -> u8 {
        while !self.data_available(io) {
            core::hint::spin_loop();
        }
        let byte = io.read_port(self.reg(UartRegister::Data));
        self.rx_count += 1;
        byte
    }

    /// Try to read a byte without blocking (returns None if no data)
    pub fn try_read_byte(&mut self, io: &dyn PortIo) -> Option<u8> {
        if self.data_available(io) {
            let byte = io.read_port(self.reg(UartRegister::Data));
            self.rx_count += 1;
            Some(byte)
        } else {
            None
        }
    }

    /// Write a string, converting newlines to CR+LF
    pub fn write_string(&mut self, io: &dyn PortIo, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(io, b'\r');
            }
            self.write_byte(io, byte);
        }
    }

    /// Check if data is available to read
    pub fn data_available(&self, io: &dyn PortIo) -> bool {
        io.read_port(self.reg(UartRegister::LineStatus)) & lsr::DATA_READY != 0
    }

    /// Check if transmitter can accept data
    pub fn can_transmit(&self, io: &dyn PortIo) -> bool {
        io.read_port(self.reg(UartRegister::LineStatus)) & lsr::TX_HOLDING_EMPTY != 0
    }

    /// Check for line errors
    pub fn check_errors(&self, io: &dyn PortIo) -> Option<LineError> {
        let status = io.read_port(self.reg(UartRegister::LineStatus));
        if status & (lsr::OVERRUN_ERROR | lsr::PARITY_ERROR | lsr::FRAMING_ERROR) != 0 {
            Some(LineError {
                overrun: status & lsr::OVERRUN_ERROR != 0,
                parity: status & lsr::PARITY_ERROR != 0,
                framing: status & lsr::FRAMING_ERROR != 0,
                break_detected: status & lsr::BREAK_INDICATOR != 0,
            })
        } else {
            None
        }
    }

    /// Enable specific interrupts
    pub fn enable_interrupts(&self, io: &dyn PortIo, mask: u8) {
        let current = io.read_port(self.reg(UartRegister::InterruptEnable));
        io.write_port(self.reg(UartRegister::InterruptEnable), current | mask);
    }

    /// Disable all interrupts
    pub fn disable_interrupts(&self, io: &dyn PortIo) {
        io.write_port(self.reg(UartRegister::InterruptEnable), 0x00);
    }

    /// Get the COM port identifier
    pub fn port(&self) -> ComPort {
        self.port
    }

    /// Get transmission statistics
    pub fn stats(&self) -> SerialStats {
        SerialStats {
            tx_count: self.tx_count,
            rx_count: self.rx_count,
            error_count: self.error_count,
            initialized: self.initialized,
            port: self.port,
        }
    }

    /// Calculate register address from base + offset
    fn reg(&self, register: UartRegister) -> u16 {
        self.base + register.offset()
    }
}

// ──────────────────────────── Serial Logger ──────────────────────────────────

/// Kernel log levels (matching Linux's printk levels)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

impl LogLevel {
    pub fn prefix(self) -> &'static str {
        match self {
            LogLevel::Emergency => "[EMERG]",
            LogLevel::Alert => "[ALERT]",
            LogLevel::Critical => "[CRIT] ",
            LogLevel::Error => "[ERROR]",
            LogLevel::Warning => "[WARN] ",
            LogLevel::Notice => "[NOTE] ",
            LogLevel::Info => "[INFO] ",
            LogLevel::Debug => "[DEBUG]",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.prefix())
    }
}

/// High-level serial logging interface
///
/// Wraps a `SerialPort` to provide structured kernel log output
/// with log levels, timestamps, and formatted messages.
pub struct SerialLogger {
    port: SerialPort,
    min_level: LogLevel,
    message_count: u64,
}

impl SerialLogger {
    /// Create a new logger on the given COM port
    pub fn new(com_port: ComPort) -> Self {
        Self {
            port: SerialPort::new(com_port),
            min_level: LogLevel::Debug,
            message_count: 0,
        }
    }

    /// Initialize the underlying serial port
    pub fn init(&mut self, io: &dyn PortIo) -> Result<(), SerialError> {
        self.port.init(io)?;
        self.log(io, LogLevel::Info, "SigmaOS serial logger initialized");
        Ok(())
    }

    /// Set minimum log level (messages below this level are discarded)
    pub fn set_min_level(&mut self, level: LogLevel) {
        self.min_level = level;
    }

    /// Log a message at the given level
    pub fn log(&mut self, io: &dyn PortIo, level: LogLevel, message: &str) {
        if level > self.min_level {
            return;
        }
        self.message_count += 1;
        let prefix = level.prefix();
        self.port.write_string(io, prefix);
        self.port.write_string(io, " ");
        self.port.write_string(io, message);
        self.port.write_string(io, "\n");
    }

    /// Log at Emergency level
    pub fn emergency(&mut self, io: &dyn PortIo, msg: &str) {
        self.log(io, LogLevel::Emergency, msg);
    }

    /// Log at Error level
    pub fn error(&mut self, io: &dyn PortIo, msg: &str) {
        self.log(io, LogLevel::Error, msg);
    }

    /// Log at Warning level
    pub fn warn(&mut self, io: &dyn PortIo, msg: &str) {
        self.log(io, LogLevel::Warning, msg);
    }

    /// Log at Info level
    pub fn info(&mut self, io: &dyn PortIo, msg: &str) {
        self.log(io, LogLevel::Info, msg);
    }

    /// Log at Debug level
    pub fn debug(&mut self, io: &dyn PortIo, msg: &str) {
        self.log(io, LogLevel::Debug, msg);
    }

    /// Get message count
    pub fn message_count(&self) -> u64 {
        self.message_count
    }
}

// ──────────────────────────── Error Types ─────────────────────────────────────

/// Serial port error types
#[derive(Debug, Clone)]
pub enum SerialError {
    /// Loopback test failed during initialization
    LoopbackFailed { sent: u8, received: u8 },
    /// Port not initialized
    NotInitialized,
    /// Timeout waiting for data
    Timeout,
    /// Line error detected
    LineError(LineError),
}

impl fmt::Display for SerialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SerialError::LoopbackFailed { sent, received } => {
                write!(
                    f,
                    "UART loopback test failed: sent 0x{:02X}, received 0x{:02X}",
                    sent, received
                )
            }
            SerialError::NotInitialized => write!(f, "Serial port not initialized"),
            SerialError::Timeout => write!(f, "Serial I/O timeout"),
            SerialError::LineError(e) => write!(f, "Line error: {}", e),
        }
    }
}

/// Line status error details
#[derive(Debug, Clone)]
pub struct LineError {
    pub overrun: bool,
    pub parity: bool,
    pub framing: bool,
    pub break_detected: bool,
}

impl fmt::Display for LineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut errors = Vec::new();
        if self.overrun {
            errors.push("overrun");
        }
        if self.parity {
            errors.push("parity");
        }
        if self.framing {
            errors.push("framing");
        }
        if self.break_detected {
            errors.push("break");
        }
        write!(f, "{}", errors.join(", "))
    }
}

/// Serial port statistics
#[derive(Debug, Clone)]
pub struct SerialStats {
    pub tx_count: u64,
    pub rx_count: u64,
    pub error_count: u64,
    pub initialized: bool,
    pub port: ComPort,
}

impl fmt::Display for SerialStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}: TX={} RX={} ERR={} init={}",
            self.port, self.tx_count, self.rx_count, self.error_count, self.initialized
        )
    }
}

/// Quick kernel log function for debug output on COM1
pub fn klog(io: &dyn PortIo, message: &str) {
    let mut port = SerialPort::new(ComPort::COM1);
    let _ = port.init(io);
    port.write_string(io, "[KLOG] ");
    port.write_string(io, message);
    port.write_string(io, "\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serial_config_default() {
        let config = SerialConfig::default();
        assert_eq!(config.baud_rate, BaudRate::Baud115200);
        assert_eq!(config.data_bits, DataBits::Eight);
        assert_eq!(config.stop_bits, StopBits::One);
        assert_eq!(config.parity, Parity::None);
    }

    #[test]
    fn test_com_port_addresses() {
        assert_eq!(ComPort::COM1.base_address(), 0x3F8);
        assert_eq!(ComPort::COM2.base_address(), 0x2F8);
        assert_eq!(ComPort::COM3.base_address(), 0x3E8);
        assert_eq!(ComPort::COM4.base_address(), 0x2E8);
    }

    #[test]
    fn test_register_offsets() {
        assert_eq!(UartRegister::Data.offset(), 0);
        assert_eq!(UartRegister::InterruptEnable.offset(), 1);
        assert_eq!(UartRegister::LineStatus.offset(), 5);
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Emergency < LogLevel::Debug);
        assert!(LogLevel::Error < LogLevel::Warning);
    }

    #[test]
    fn test_serial_stats_display() {
        let stats = SerialStats {
            tx_count: 100,
            rx_count: 50,
            error_count: 0,
            initialized: true,
            port: ComPort::COM1,
        };
        let s = format!("{}", stats);
        assert!(s.contains("COM1"));
        assert!(s.contains("TX=100"));
    }
}
