/// SigmaOS: Kernel Panic Handler
/// Enhanced panic handler with detailed error reporting

#![no_std]
#![allow(dead_code)]

use core::panic::PanicInfo;
use core::fmt::Write;

// ─── Panic Handler ───────────────────────────────────────────────────────

/// Enhanced panic handler with detailed error reporting
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Try to print panic information
    let mut writer = PanicWriter::new();
    
    writer.write_str("!!! KERNEL PANIC !!!\n");
    writer.write_str("==================\n");
    
    if let Some(location) = info.location() {
        writer.write_str("Location: ");
        writer.write_str(location.file());
        writer.write_str(":");
        // Write line number
        let mut line_buf = [0u8; 20];
        let mut line = location.line();
        let mut i = 0;
        while line > 0 {
            line_buf[i] = (line % 10) as u8 + b'0';
            line /= 10;
            i += 1;
        }
        for j in (0..i).rev() {
            writer.write_str(core::str::from_utf8(&[line_buf[j]]).unwrap_or("?"));
        }
        writer.write_str("\n");
    }
    
    if let Some(message) = info.message() {
        writer.write_str("Message: ");
        // Try to format the message
        let _ = write!(writer, "{}", message);
        writer.write_str("\n");
    }
    
    writer.write_str("==================\n");
    writer.write_str("System halted.\n");
    
    loop {}
}

// ─── Panic Writer ─────────────────────────────────────────────────────────

struct PanicWriter;

impl PanicWriter {
    const fn new() -> Self {
        Self
    }
}

impl Write for PanicWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // TODO: Implement actual output to VGA/serial
        // For now, this is a stub
        let _ = s;
        Ok(())
    }
    
    fn write_char(&mut self, c: char) -> core::fmt::Result {
        // TODO: Implement actual output
        let _ = c;
        Ok(())
    }
}

// ─── Kernel Abort Function ───────────────────────────────────────────────

/// Abort function for unrecoverable errors
#[no_mangle]
pub extern "C" fn abort() -> ! {
    loop {}
}

// ─── Kernel Assert Macro ─────────────────────────────────────────────────

/// Kernel assert macro
#[macro_export]
macro_rules! kassert {
    ($cond:expr) => {
        if !$cond {
            panic!("Assertion failed: {}", stringify!($cond));
        }
    };
    ($cond:expr, $msg:expr) => {
        if !$cond {
            panic!("Assertion failed: {} - {}", stringify!($cond), $msg);
        }
    };
}

// ─── Kernel Unreachable Macro ────────────────────────────────────────────

/// Kernel unreachable macro
#[macro_export]
macro_rules! kunreachable {
    () => {
        unreachable!("Unreachable code reached");
    };
    ($msg:expr) => {
        unreachable!("{}", $msg);
    };
}
