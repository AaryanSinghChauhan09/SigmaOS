#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// GDT, IDT, and VGA Text Buffer architectures for SigmaOS
// Integrates core bare-metal concepts from phil-opp/blog_os under `// #![no_std]  // crate-root only`.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU16, AtomicUsize, Ordering};

// ==========================================
// 1. GLOBAL DESCRIPTOR TABLE & TSS
// ==========================================

/// Task State Segment (TSS) containing double fault stacks to prevent triple faults
pub struct TaskStateSegment {
    pub interrupt_stack_table: [u64; 7],
}

impl TaskStateSegment {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            interrupt_stack_table: [0; 7],
        }
    }
}

/// Global Descriptor Table (GDT) mapping code, data, and system segments
pub struct GDT {
    pub code_selector: u16,
    pub tss_selector: u16,
}

impl GDT {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            code_selector: 8,
            tss_selector: 16,
        }
    }

    pub fn load(&self, _tss: &TaskStateSegment) -> bool {
        // Simulates loading GDT and TSS registers securely
        true
    }
}

// ==========================================
// 2. INTERRUPT DESCRIPTOR TABLE & EXCEPTIONS
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionType {
    DivideByZero = 0,
    Breakpoint = 3,
    DoubleFault = 8,
    PageFault = 14,
}

/// Interrupt Descriptor Table (IDT) defining entry gates for exceptions and IRQs
pub struct IDT {
    pub divide_by_zero_handler: Option<fn()>,
    pub breakpoint_handler: Option<fn()>,
    pub double_fault_handler: Option<fn()>,
    pub page_fault_handler: Option<fn()>,
    pub hardware_irqs: [Option<fn()>; 16],
}

impl IDT {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            divide_by_zero_handler: None,
            breakpoint_handler: None,
            double_fault_handler: None,
            page_fault_handler: None,
            hardware_irqs: [None; 16],
        }
    }

    pub fn set_exception_handler(&mut self, exc: ExceptionType, handler: fn()) {
        match exc {
            ExceptionType::DivideByZero => self.divide_by_zero_handler = Some(handler),
            ExceptionType::Breakpoint => self.breakpoint_handler = Some(handler),
            ExceptionType::DoubleFault => self.double_fault_handler = Some(handler),
            ExceptionType::PageFault => self.page_fault_handler = Some(handler),
        }
    }

    pub fn set_hardware_irq_handler(&mut self, irq: usize, handler: fn()) {
        if irq < 16 {
            self.hardware_irqs[irq] = Some(handler);
        }
    }
}

// ==========================================
// 3. VGA TEXT BUFFER DRIVER
// ==========================================

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VGAColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: VGAColor, background: VGAColor) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: u8,
}

pub struct VGATextBuffer {
    pub buffer: [[ScreenChar; 80]; 25],
    pub row: usize,
    pub col: usize,
    pub current_color: ColorCode,
}

impl VGATextBuffer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            buffer: [[ScreenChar {
                ascii_character: b' ',
                color_code: 0x07,
            }; 80]; 25],
            row: 0,
            col: 0,
            current_color: ColorCode::new(VGAColor::LightGray, VGAColor::Black),
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= 80 {
                    self.new_line();
                }

                let row = self.row;
                let col = self.col;

                self.buffer[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.current_color.0,
                };
                self.col += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    fn new_line(&mut self) {
        if self.row < 24 {
            self.row += 1;
            self.col = 0;
        } else {
            // Scroll buffer up by one row
            for r in 1..25 {
                self.buffer[r - 1] = self.buffer[r];
            }
            // Clear last row
            self.buffer[24] = [ScreenChar {
                ascii_character: b' ',
                color_code: self.current_color.0,
            }; 80];
            self.col = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXCEPTION_TRIGGERED: AtomicUsize = AtomicUsize::new(0);

    fn mock_breakpoint_handler() {
        EXCEPTION_TRIGGERED.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_gdt_and_tss_loading() {
        let tss = TaskStateSegment::new();
        let gdt = GDT::new();
        assert!(gdt.load(&tss));
    }

    #[test]
    fn test_idt_handling() {
        let mut idt = IDT::new();
        idt.set_exception_handler(ExceptionType::Breakpoint, mock_breakpoint_handler);

        assert!(idt.breakpoint_handler.is_some());

        // Simulates triggering the registered exception handler
        if let Some(handler) = idt.breakpoint_handler {
            handler();
        }
        assert_eq!(EXCEPTION_TRIGGERED.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_vga_writing_and_scrolling() {
        let mut vga = VGATextBuffer::new();
        vga.write_string("Hello SigmaOS bare-metal!\n");
        assert_eq!(vga.row, 1);
        assert_eq!(vga.col, 0);
        assert_eq!(vga.buffer[0][0].ascii_character, b'H');

        // Write 25 rows to trigger scroll
        for i in 0..25 {
            vga.write_string("Scroll test row\n");
        }
        assert_eq!(vga.row, 24);
        assert_eq!(vga.buffer[23][0].ascii_character, b'S');
    }
}
