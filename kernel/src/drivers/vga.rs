use super::{Driver, DriverStatus};

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

/// Represents the VGA Framebuffer driver.
pub struct VgaDriver {
    status: DriverStatus,
    column_position: usize,
    row_position: usize,
}

impl VgaDriver {
    pub fn new() -> Self {
        Self {
            status: DriverStatus::Uninitialized,
            column_position: 0,
            row_position: 0,
        }
    }

    /// Write a single byte to the VGA buffer with a specific color.
    fn write_byte(&mut self, byte: u8, color_code: u8) {
        match byte {
            b'\n' => {
                self.row_position += 1;
                self.column_position = 0;
            }
            byte => {
                if self.column_position >= VGA_WIDTH {
                    self.row_position += 1;
                    self.column_position = 0;
                }

                if self.row_position >= VGA_HEIGHT {
                    self.clear_screen();
                }

                let row = self.row_position;
                let col = self.column_position;

                unsafe {
                    *VGA_BUFFER.offset((row * VGA_WIDTH + col) as isize * 2) = byte;
                    *VGA_BUFFER.offset((row * VGA_WIDTH + col) as isize * 2 + 1) = color_code;
                }
                self.column_position += 1;
            }
        }
    }

    /// Print a string to the VGA buffer.
    pub fn print_str(&mut self, s: &str) {
        for byte in s.bytes() {
            // Light cyan color (0xb) on black background (0x0)
            self.write_byte(byte, 0x0b);
        }
    }

    /// Clear the entire VGA screen.
    pub fn clear_screen(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                unsafe {
                    *VGA_BUFFER.offset((row * VGA_WIDTH + col) as isize * 2) = b' ';
                    *VGA_BUFFER.offset((row * VGA_WIDTH + col) as isize * 2 + 1) = 0x0;
                }
            }
        }
        self.row_position = 0;
        self.column_position = 0;
    }
}

impl Driver for VgaDriver {
    fn init(&mut self) -> Result<(), &'static str> {
        self.clear_screen();
        self.status = DriverStatus::Ready;
        Ok(())
    }

    fn status(&self) -> DriverStatus {
        // Rust does not allow returning references to fields like this easily without 
        // copying or cloning if it was complex, but since our enum is simple we can derive PartialEq.
        // For now, we'll return a copy of the status.
        match self.status {
            DriverStatus::Uninitialized => DriverStatus::Uninitialized,
            DriverStatus::Ready => DriverStatus::Ready,
            DriverStatus::Error(e) => DriverStatus::Error(e),
        }
    }

    fn name(&self) -> &'static str {
        "VGA Framebuffer Driver"
    }
}
