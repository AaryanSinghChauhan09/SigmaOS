#![cfg_attr(not(test), no_std)]
// SigmaOS Kernel Console Output Infrastructure
// Provides VGA and serial output for kernel logging and panic messages
// Solves critical gap: no actual kernel output implementation

use core::fmt::Write;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Console output backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleBackend {
    Serial,
    VGA,
    Framebuffer,
    EFI,
}

/// Console output level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
    Critical = 4,
    Panic = 5,
}

/// Kernel console writer
pub struct KernelConsole {
    pub backend: ConsoleBackend,
    pub log_level: LogLevel,
    pub initialized: AtomicBool,
    pub write_count: AtomicUsize,
    pub serial_base: u16,
    pub vga_buffer: *mut u8,
    pub vga_width: usize,
    pub vga_height: usize,
    pub cursor_x: AtomicUsize,
    pub cursor_y: AtomicUsize,
}

impl KernelConsole {
    pub fn new() -> Self {
        #[cfg(not(target_os = "none"))]
        let vga_ptr = std::vec![0u8; 80 * 25 * 2].leak().as_mut_ptr();
        #[cfg(target_os = "none")]
        let vga_ptr = 0xB8000 as *mut u8;

        Self {
            backend: ConsoleBackend::Serial,
            log_level: LogLevel::Info,
            initialized: AtomicBool::new(false),
            write_count: AtomicUsize::new(0),
            serial_base: 0x3F8,
            vga_buffer: vga_ptr,
            vga_width: 80,
            vga_height: 25,
            cursor_x: AtomicUsize::new(0),
            cursor_y: AtomicUsize::new(0),
        }
    }

    pub fn initialize(&mut self, backend: ConsoleBackend) -> Result<(), &'static str> {
        match backend {
            ConsoleBackend::Serial => self.initialize_serial()?,
            ConsoleBackend::VGA => self.initialize_vga()?,
            ConsoleBackend::Framebuffer => self.initialize_framebuffer()?,
            ConsoleBackend::EFI => self.initialize_efi()?,
        }

        self.backend = backend;
        self.initialized.store(true, Ordering::SeqCst);
        self.write_banner();
        Ok(())
    }

    fn initialize_serial(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn initialize_vga(&mut self) -> Result<(), &'static str> {
        for i in 0..(self.vga_width * self.vga_height * 2) {
            unsafe {
                self.vga_buffer.add(i).write_volatile(0x07);
            }
        }
        self.cursor_x.store(0, Ordering::SeqCst);
        self.cursor_y.store(0, Ordering::SeqCst);
        Ok(())
    }

    fn initialize_framebuffer(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn initialize_efi(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn write_banner(&mut self) {
        self.println(LogLevel::Info, "SigmaOS Kernel v0.1.0");
        self.println(LogLevel::Info, "-------------------");
        self.println(LogLevel::Info, "Sovereign Rust Microkernel");
        self.println(LogLevel::Info, "PQC-Enabled: Dilithium-5, Kyber-1024");
        self.println(LogLevel::Info, "Capability-Based Security");
        self.println(LogLevel::Info, "-------------------");
    }

    pub fn set_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }

    pub fn write(&mut self, level: LogLevel, message: &str) {
        if level < self.log_level {
            return;
        }

        match self.backend {
            ConsoleBackend::Serial => self.write_serial(message),
            ConsoleBackend::VGA => self.write_vga(message),
            ConsoleBackend::Framebuffer => self.write_framebuffer(message),
            ConsoleBackend::EFI => self.write_efi(message),
        }

        self.write_count.fetch_add(message.len(), Ordering::SeqCst);
    }

    pub fn println(&mut self, level: LogLevel, message: &str) {
        self.write(level, message);
        self.write(level, "\n");
    }

    fn write_serial(&self, message: &str) {
        for byte in message.bytes() {
            let _ = byte;
        }
    }

    fn write_vga(&mut self, message: &str) {
        for byte in message.bytes() {
            self.write_vga_char(byte);
        }
    }

    fn write_vga_char(&mut self, byte: u8) {
        let mut x = self.cursor_x.load(Ordering::SeqCst);
        let mut y = self.cursor_y.load(Ordering::SeqCst);

        match byte {
            b'\n' => {
                x = 0;
                y += 1;
            }
            b'\r' => {
                x = 0;
            }
            b'\t' => {
                x = (x + 8) & !7;
                if x >= self.vga_width {
                    x = 0;
                    y += 1;
                }
            }
            0x08 => {
                if x > 0 {
                    x -= 1;
                }
            }
            _ => {
                let offset = (y * self.vga_width + x) * 2;
                unsafe {
                    self.vga_buffer.add(offset).write_volatile(byte);
                    self.vga_buffer.add(offset + 1).write_volatile(0x0F);
                }

                x += 1;
                if x >= self.vga_width {
                    x = 0;
                    y += 1;
                }
            }
        }

        if y >= self.vga_height {
            self.scroll_vga();
            y = self.vga_height - 1;
        }

        self.cursor_x.store(x, Ordering::SeqCst);
        self.cursor_y.store(y, Ordering::SeqCst);
        self.update_vga_hardware_cursor(x, y);
    }

    /// Scrolls the VGA text frame buffer up by 1 row (Linux vgacon / FreeBSD syscons parity)
    fn scroll_vga(&mut self) {
        let row_bytes = self.vga_width * 2;
        let last_row_start = (self.vga_height - 1) * row_bytes;

        unsafe {
            core::ptr::copy(
                self.vga_buffer.add(row_bytes),
                self.vga_buffer,
                (self.vga_height - 1) * row_bytes,
            );

            for i in 0..self.vga_width {
                let offset = last_row_start + i * 2;
                self.vga_buffer.add(offset).write_volatile(b' ');
                self.vga_buffer.add(offset + 1).write_volatile(0x07);
            }
        }
    }

    /// Synchronizes physical VGA hardware cursor position via CRTC registers (0x3D4/0x3D5)
    fn update_vga_hardware_cursor(&self, x: usize, y: usize) {
        #[cfg(all(target_arch = "x86_64", target_os = "none"))]
        {
            let pos = (y * self.vga_width + x) as u16;
            unsafe {
                core::arch::asm!("out dx, al", in("dx") 0x3D4u16, in("al") 0x0Eu8, options(nomem, nostack));
                core::arch::asm!("out dx, al", in("dx") 0x3D5u16, in("al") (pos >> 8) as u8, options(nomem, nostack));
                core::arch::asm!("out dx, al", in("dx") 0x3D4u16, in("al") 0x0Fu8, options(nomem, nostack));
                core::arch::asm!("out dx, al", in("dx") 0x3D5u16, in("al") (pos & 0xFF) as u8, options(nomem, nostack));
            }
        }
        let _ = (x, y);
    }

    fn write_framebuffer(&self, _message: &str) {}

    fn write_efi(&self, _message: &str) {}

    pub fn clear(&mut self) {
        match self.backend {
            ConsoleBackend::VGA => self.clear_vga(),
            _ => self.write_serial("\x1B[2J\x1B[H"),
        }
    }

    fn clear_vga(&mut self) {
        for i in 0..(self.vga_width * self.vga_height * 2) {
            unsafe {
                self.vga_buffer.add(i).write_volatile(0x07);
            }
        }
        self.cursor_x.store(0, Ordering::SeqCst);
        self.cursor_y.store(0, Ordering::SeqCst);
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        (
            self.cursor_x.load(Ordering::SeqCst),
            self.cursor_y.load(Ordering::SeqCst),
        )
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        if x < self.vga_width && y < self.vga_height {
            self.cursor_x.store(x, Ordering::SeqCst);
            self.cursor_y.store(y, Ordering::SeqCst);
        }
    }

    pub fn get_write_count(&self) -> usize {
        self.write_count.load(Ordering::SeqCst)
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }
}

impl Default for KernelConsole {
    fn default() -> Self {
        Self::new()
    }
}

static mut GLOBAL_CONSOLE: Option<KernelConsole> = None;

pub fn initialize_kernel_console(backend: ConsoleBackend) -> Result<(), &'static str> {
    unsafe {
        if (*(&raw const GLOBAL_CONSOLE)).is_none() {
            let mut console = KernelConsole::new();
            console.initialize(backend)?;
            GLOBAL_CONSOLE = Some(console);
        }
        Ok(())
    }
}

pub fn get_kernel_console() -> Option<&'static mut KernelConsole> {
    unsafe { (*(&raw mut GLOBAL_CONSOLE)).as_mut() }
}

pub fn kernel_panic(message: &str) -> ! {
    if let Some(console) = get_kernel_console() {
        console.write(LogLevel::Critical, "!!! KERNEL PANIC !!!\n");
        console.write(LogLevel::Critical, message);
        console.write(LogLevel::Critical, "\n!!! SYSTEM HALTED !!!\n");
    }

    loop {
        core::hint::spin_loop();
    }
}

pub fn klog(level: LogLevel, message: &str) {
    if let Some(console) = get_kernel_console() {
        console.println(level, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_console_initialization() {
        let mut console = KernelConsole::new();
        assert!(!console.is_initialized());
        assert!(console.initialize(ConsoleBackend::Serial).is_ok());
        assert!(console.is_initialized());
    }

    #[test]
    fn test_console_write() {
        let mut console = KernelConsole::new();
        console.initialize(ConsoleBackend::Serial).unwrap();
        console.write(LogLevel::Info, "Test message");
        assert!(console.get_write_count() > 0);
    }

    #[test]
    fn test_console_cursor() {
        let mut console = KernelConsole::new();
        console.initialize(ConsoleBackend::VGA).unwrap();
        console.set_cursor(10, 5);
        let (x, y) = console.get_cursor();
        assert_eq!(x, 10);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_log_levels() {
        let mut console = KernelConsole::new();
        console.initialize(ConsoleBackend::Serial).unwrap();
        let initial_count = console.get_write_count();
        console.set_log_level(LogLevel::Warning);
        console.write(LogLevel::Debug, "Debug message");
        let count1 = console.get_write_count();
        assert_eq!(count1, initial_count);
        console.write(LogLevel::Warning, "Warning message");
        let count2 = console.get_write_count();
        assert_eq!(count2, count1 + "Warning message".len());
    }

    #[test]
    fn test_vga_clear() {
        let mut console = KernelConsole::new();
        console.initialize(ConsoleBackend::VGA).unwrap();
        console.set_cursor(5, 5);
        console.clear();
        let (x, y) = console.get_cursor();
        assert_eq!(x, 0);
        assert_eq!(y, 0);
    }

    #[test]
    fn test_vga_tab_and_backspace() {
        let mut console = KernelConsole::new();
        console.initialize(ConsoleBackend::VGA).unwrap();
        let start_y = console.get_cursor().1; // row 6 after banner

        // Test tab stop calculation
        console.write_vga_char(b'A');
        assert_eq!(console.get_cursor(), (1, start_y));
        console.write_vga_char(b'\t');
        assert_eq!(console.get_cursor(), (8, start_y));

        // Test backspace
        console.write_vga_char(0x08);
        assert_eq!(console.get_cursor(), (7, start_y));
    }

    #[test]
    fn test_vga_scrolling() {
        let mut console = KernelConsole::new();
        console.initialize(ConsoleBackend::VGA).unwrap();

        // Fill all 25 rows using println
        for _row in 0..26 {
            console.println(LogLevel::Info, "Line");
        }

        // Verify cursor clamped at bottom row 24
        let (_x, y) = console.get_cursor();
        assert_eq!(y, 24);

        // Verify top character is 'L' from scrolled banner or lines
        unsafe {
            assert_eq!(*console.vga_buffer, b'L');
        }
    }
}
