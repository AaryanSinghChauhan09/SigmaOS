use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// Console I/O module for SigmaOS
// Replaces std::io functionality for terminal I/O

use crate::klib::custom_string::SigmaString;

pub struct SigmaConsole {
    stdout_fd: i32,
    stdin_fd: i32,
    stderr_fd: i32,
}

#[derive(Debug)]
pub enum IoError {
    ReadFailed,
    WriteFailed,
    InvalidInput,
}

impl SigmaConsole {
    pub const fn new() -> Self {
        Self {
            stdout_fd: 1,
            stdin_fd: 0,
            stderr_fd: 2,
        }
    }

    pub fn print(&self, s: &str) {
        let bytes = s.as_bytes();
        unsafe {
            Self::syscall_write(self.stdout_fd, bytes.as_ptr(), bytes.len());
        }
    }

    pub fn println(&self, s: &str) {
        self.print(s);
        self.print("\n");
    }

    pub fn read_line(&self) -> Result<String, IoError> {
        let mut buffer = [0u8; 1024];
        let bytes_read =
            unsafe { Self::syscall_read(self.stdin_fd, buffer.as_mut_ptr(), buffer.len()) };

        if bytes_read < 0 {
            return Err(IoError::ReadFailed);
        }

        Ok(String::from_utf8_lossy(&buffer[..bytes_read as usize]).to_string())
    }

    pub fn eprint(&self, s: &str) {
        let bytes = s.as_bytes();
        unsafe {
            Self::syscall_write(self.stderr_fd, bytes.as_ptr(), bytes.len());
        }
    }

    pub fn eprintln(&self, s: &str) {
        self.eprint(s);
        self.eprint("\n");
    }

    unsafe fn syscall_write(fd: i32, buffer: *const u8, count: usize) -> isize {
        // Placeholder for actual syscall implementation
        // This would be replaced with actual syscall when running on SigmaOS
        count as isize
    }

    unsafe fn syscall_read(fd: i32, buffer: *mut u8, count: usize) -> isize {
        // Placeholder for actual syscall implementation
        // This would be replaced with actual syscall when running on SigmaOS
        0
    }
}

// Global console instance
pub static CONSOLE: SigmaConsole = SigmaConsole::new();

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let msg = alloc::format!($($arg)*);
        $crate::klib::console::CONSOLE.print(&msg);
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::klib::console::CONSOLE.print("\n");
    };
    ($($arg:tt)*) => {
        let msg = alloc::format!($($arg)*);
        $crate::klib::console::CONSOLE.println(&msg);
    };
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {
        let msg = alloc::format!($($arg)*);
        $crate::klib::console::CONSOLE.eprint(&msg);
    };
}

#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::klib::console::CONSOLE.eprint("\n");
    };
    ($($arg:tt)*) => {
        let msg = alloc::format!($($arg)*);
        $crate::klib::console::CONSOLE.eprintln(&msg);
    };
}

