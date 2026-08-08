/// SigmaOS: VGA Text Mode Driver
/// Basic VGA text mode driver for kernel output

#[allow(dead_code)]

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaUsize = usize;

const VGA_BUFFER: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VgaColor {
    pub value: SigmaU8,
}

impl VgaColor {
    pub const fn new(fg: SigmaU8, bg: SigmaU8) -> Self {
        Self {
            value: (bg << 4) | (fg & 0xF),
        }
    }

    pub const BLACK: Self = Self::new(0, 0);
    pub const BLUE: Self = Self::new(1, 0);
    pub const GREEN: Self = Self::new(2, 0);
    pub const CYAN: Self = Self::new(3, 0);
    pub const RED: Self = Self::new(4, 0);
    pub const MAGENTA: Self = Self::new(5, 0);
    pub const BROWN: Self = Self::new(6, 0);
    pub const LIGHT_GREY: Self = Self::new(7, 0);
    pub const DARK_GREY: Self = Self::new(8, 0);
    pub const LIGHT_BLUE: Self = Self::new(9, 0);
    pub const LIGHT_GREEN: Self = Self::new(10, 0);
    pub const LIGHT_CYAN: Self = Self::new(11, 0);
    pub const LIGHT_RED: Self = Self::new(12, 0);
    pub const LIGHT_MAGENTA: Self = Self::new(13, 0);
    pub const YELLOW: Self = Self::new(14, 0);
    pub const WHITE: Self = Self::new(15, 0);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VgaEntry {
    pub character: SigmaU8,
    pub color: SigmaU8,
}

pub struct VgaDriver {
    buffer: *mut VgaEntry,
    row: SigmaUsize,
    column: SigmaUsize,
    color: SigmaU8,
}

impl VgaDriver {
    pub const fn new() -> Self {
        Self {
            buffer: VGA_BUFFER as *mut VgaEntry,
            row: 0,
            column: 0,
            color: VgaColor::WHITE.value,
        }
    }

    pub unsafe fn init(&mut self) -> Result<(), &'static str> {
        self.clear_screen();
        Ok(())
    }

    unsafe fn clear_screen(&mut self) {
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            (*self.buffer.add(i)).character = b' ';
            (*self.buffer.add(i)).color = self.color;
        }
        self.row = 0;
        self.column = 0;
    }

    pub fn print_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                b'\n' => unsafe { self.newline() },
                byte => unsafe { self.put_byte(byte) },
            }
        }
    }

    unsafe fn put_byte(&mut self, byte: SigmaU8) {
        if self.column >= VGA_WIDTH {
            self.newline();
        }

        let offset = self.row * VGA_WIDTH + self.column;
        (*self.buffer.add(offset)).character = byte;
        (*self.buffer.add(offset)).color = self.color;

        self.column += 1;
    }

    unsafe fn newline(&mut self) {
        self.column = 0;
        if self.row < VGA_HEIGHT - 1 {
            self.row += 1;
        } else {
            self.scroll_up();
        }
    }

    unsafe fn scroll_up(&mut self) {
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let src_offset = row * VGA_WIDTH + col;
                let dst_offset = (row - 1) * VGA_WIDTH + col;
                (*self.buffer.add(dst_offset)) = *self.buffer.add(src_offset);
            }
        }

        // Clear last row
        for col in 0..VGA_WIDTH {
            let offset = (VGA_HEIGHT - 1) * VGA_WIDTH + col;
            (*self.buffer.add(offset)).character = b' ';
            (*self.buffer.add(offset)).color = self.color;
        }

        self.row = VGA_HEIGHT - 1;
    }

    pub unsafe fn set_color(&mut self, color: SigmaU8) {
        self.color = color;
    }
}

impl super::Driver for VgaDriver {
    fn init(&mut self) -> Result<(), &'static str> {
        unsafe { self.init() }
    }

    fn status(&self) -> super::DriverStatus {
        super::DriverStatus::Ready
    }

    fn name(&self) -> &'static str {
        "VGA Text Mode Driver"
    }

    fn class(&self) -> super::DeviceClass {
        super::DeviceClass::Display
    }
}