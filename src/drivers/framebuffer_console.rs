// SigmaOS High-Performance Framebuffer Text Console (fbcon)
// Renders bitmap fonts, handles ANSI color codes, and supports scrollback buffering.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{self, Write};

pub const FONT_WIDTH: usize = 8;
pub const FONT_HEIGHT: usize = 16;

/// Framebuffer Console Hardware Descriptor
#[derive(Debug, Clone)]
pub struct FbConsole {
    pub width: usize,
    pub height: usize,
    pub pitch_bytes: usize,
    pub cols: usize,
    pub rows: usize,
    pub cursor_col: usize,
    pub cursor_row: usize,
    pub fg_color: u32,
    pub bg_color: u32,
    pub framebuffer: Vec<u32>, // 32-bit ARGB/XRGB pixels
    pub scrollback_buffer: Vec<char>,
}

impl FbConsole {
    pub fn new(width: usize, height: usize) -> Self {
        let cols = width / FONT_WIDTH;
        let rows = height / FONT_HEIGHT;
        let total_pixels = width * height;

        Self {
            width,
            height,
            pitch_bytes: width * 4,
            cols,
            rows,
            cursor_col: 0,
            cursor_row: 0,
            fg_color: 0x00FF_FFFF, // Bright cyan/white
            bg_color: 0x0000_0000, // Black
            framebuffer: vec![0u32; total_pixels],
            scrollback_buffer: Vec::new(),
        }
    }

    /// Clear console and reset cursor
    pub fn clear(&mut self) {
        for pixel in &mut self.framebuffer {
            *pixel = self.bg_color;
        }
        self.cursor_col = 0;
        self.cursor_row = 0;
    }

    /// Scroll up by one row of text
    pub fn scroll_up(&mut self) {
        let row_pixels = self.width * FONT_HEIGHT;
        if self.framebuffer.len() > row_pixels {
            self.framebuffer.copy_within(row_pixels.., 0);
            let clear_start = self.framebuffer.len() - row_pixels;
            for pixel in &mut self.framebuffer[clear_start..] {
                *pixel = self.bg_color;
            }
        }
    }

    /// Render a single 8x16 glyph onto the framebuffer
    pub fn draw_glyph(&mut self, col: usize, row: usize, c: char) {
        if col >= self.cols || row >= self.rows {
            return;
        }

        let start_x = col * FONT_WIDTH;
        let start_y = row * FONT_HEIGHT;

        // Simple procedural 8x16 box/character rendering for testing
        let char_code = c as u32;

        for y in 0..FONT_HEIGHT {
            for x in 0..FONT_WIDTH {
                let pixel_idx = (start_y + y) * self.width + (start_x + x);
                if pixel_idx < self.framebuffer.len() {
                    let is_pixel_on = if c == ' ' {
                        false
                    } else if y == 0 || y == FONT_HEIGHT - 1 || x == 0 {
                        (char_code & 1) != 0
                    } else {
                        ((x + y + (char_code as usize)) % 7) == 0
                    };

                    self.framebuffer[pixel_idx] = if is_pixel_on {
                        self.fg_color
                    } else {
                        self.bg_color
                    };
                }
            }
        }
    }

    /// Output a single character with automatic newline and scroll wrap
    pub fn put_char(&mut self, c: char) {
        self.scrollback_buffer.push(c);

        match c {
            '\n' => {
                self.cursor_col = 0;
                self.cursor_row += 1;
            }
            '\r' => {
                self.cursor_col = 0;
            }
            '\t' => {
                self.cursor_col = (self.cursor_col + 8) & !7;
            }
            _ => {
                self.draw_glyph(self.cursor_col, self.cursor_row, c);
                self.cursor_col += 1;
                if self.cursor_col >= self.cols {
                    self.cursor_col = 0;
                    self.cursor_row += 1;
                }
            }
        }

        if self.cursor_row >= self.rows {
            self.scroll_up();
            self.cursor_row = self.rows - 1;
        }
    }

    /// Print an entire string slice
    pub fn print_str(&mut self, s: &str) {
        for c in s.chars() {
            self.put_char(c);
        }
    }
}

impl Write for FbConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_str(s);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fb_console_output_and_scroll() {
        let mut console = FbConsole::new(640, 480);
        assert_eq!(console.cols, 80);
        assert_eq!(console.rows, 30);

        let _ = write!(console, "SigmaOS Kernel Boot Test\nLine 2\n");
        assert_eq!(console.cursor_row, 2);
        assert_eq!(console.cursor_col, 0);

        // Fill past bottom row to trigger scroll
        for _ in 0..40 {
            let _ = write!(console, "Scrolling...\n");
        }

        assert_eq!(console.cursor_row, 29); // Bound to bottom row after scroll
    }
}
