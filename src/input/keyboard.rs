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

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Keyboard Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 71
/// Implements keyboard input handling and key mapping

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type KeyCode = u16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyState { Released = 0, Pressed = 1, Repeated = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Modifier { Shift = 1, Ctrl = 2, Alt = 4, Super = 8 }

pub trait KeyboardDevice {
    fn read_key(&mut self) -> Option<(KeyCode, KeyState)>;
    fn get_modifiers(&self) -> u8;
    fn set_leds(&mut self, caps: bool, num: bool, scroll: bool);
}

#[repr(C)]
pub struct SimpleKeyboardDevice {
    pub modifiers: AtomicUsize,
    pub leds: AtomicUsize,
}

impl SimpleKeyboardDevice {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleKeyboardDevice {
            modifiers: AtomicUsize::new(0),
            leds: AtomicUsize::new(0),
        }
    }
}

impl KeyboardDevice for SimpleKeyboardDevice {
    fn read_key(&mut self) -> Option<(KeyCode, KeyState)> {
        None
    }

    fn get_modifiers(&self) -> u8 { self.modifiers.load(Ordering::SeqCst) as u8 }

    fn set_leds(&mut self, caps: bool, num: bool, scroll: bool) {
        let mut leds = 0;
        if caps { leds |= 1; }
        if num { leds |= 2; }
        if scroll { leds |= 4; }
        self.leds.store(leds, Ordering::SeqCst);
    }
}

pub trait KeyMapper {
    fn map_scancode(&self, scancode: KeyCode) -> char;
    fn set_layout(&mut self, layout: &[u8]);
}

#[repr(C)]
pub struct SimpleKeyMapper {
    pub layout: [u8; 32],
}

impl SimpleKeyMapper {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut layout = [0u8; 32];
        let default_name = b"us-qwerty";
        layout[..default_name.len()].copy_from_slice(default_name);
        SimpleKeyMapper {
            layout,
        }
    }
}

impl KeyMapper for SimpleKeyMapper {
    fn map_scancode(&self, scancode: KeyCode) -> char {
        match scancode {
            4 => 'a',
            5 => 'b',
            6 => 'c',
            16 => 'q',
            17 => 'w',
            18 => 'e',
            30 => '1',
            31 => '2',
            32 => '3',
            _ => '\0',
        }
    }

    fn set_layout(&mut self, layout: &[u8]) {
        let mut layout_array = [0u8; 32];
        let layout_len = layout.len().min(31);
        for i in 0..layout_len {
            layout_array[i] = layout[i];
        }
        self.layout = layout_array;
    }
}

pub trait InputBuffer {
    fn push_key(&mut self, key: char);
    fn pop_key(&mut self) -> Option<char>;
    fn peek_key(&self) -> Option<char>;
    fn is_empty(&self) -> bool;
}

#[repr(C)]
pub struct SimpleInputBuffer {
    pub buffer: Vec<char>,
    pub size: AtomicUsize,
}

impl SimpleInputBuffer {
    pub fn new(size: usize) -> Self {
        SimpleInputBuffer {
            buffer: Vec::new(),
            size: AtomicUsize::new(size),
        }
    }
}

impl InputBuffer for SimpleInputBuffer {
    fn push_key(&mut self, key: char) {
        let max = self.size.load(Ordering::SeqCst);
        if self.buffer.len() < max {
            self.buffer.push(key);
        }
    }

    fn pop_key(&mut self) -> Option<char> {
        if !self.buffer.is_empty() {
            Some(self.buffer.remove(0))
        } else {
            None
        }
    }

    fn peek_key(&self) -> Option<char> {
        if !self.buffer.is_empty() {
            Some(self.buffer[0])
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool { self.buffer.is_empty() }
}

pub trait KeyboardHandler {
    fn handle_key_event(&mut self, key: KeyCode, state: KeyState, modifiers: u8);
    fn register_callback(&mut self, callback: fn(KeyCode, KeyState, u8));
}

#[repr(C)]
pub struct SimpleKeyboardHandler {
    pub callbacks: Vec<fn(KeyCode, KeyState, u8)>,
}

impl SimpleKeyboardHandler {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleKeyboardHandler {
            callbacks: Vec::new(),
        }
    }
}

impl KeyboardHandler for SimpleKeyboardHandler {
    fn handle_key_event(&mut self, key: KeyCode, state: KeyState, modifiers: u8) {
        for &callback in &self.callbacks {
            callback(key, state, modifiers);
        }
    }

    fn register_callback(&mut self, callback: fn(KeyCode, KeyState, u8)) {
        self.callbacks.push(callback);
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    fn is_empty(&self) -> bool { self.len == 0 }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }


// =========================================================================
// Linux & BSD Inspired USB HID Keyboard Driver Engine
// (Parity with Linux hid-input.c & FreeBSD ukbd.c)
// =========================================================================

/// USB HID Keyboard Modifier Bitmask flags (Standard USB HID spec)
pub const HID_MODIFIER_LCTRL: u8   = 1 << 0;
pub const HID_MODIFIER_LSHIFT: u8  = 1 << 1;
pub const HID_MODIFIER_LALT: u8    = 1 << 2;
pub const HID_MODIFIER_LMETA: u8   = 1 << 3;
pub const HID_MODIFIER_RCTRL: u8   = 1 << 4;
pub const HID_MODIFIER_RSHIFT: u8  = 1 << 5;
pub const HID_MODIFIER_RALT: u8    = 1 << 6;
pub const HID_MODIFIER_RMETA: u8   = 1 << 7;

/// USB HID Keyboard LED Indicator Flags
pub const HID_LED_NUM_LOCK: u8    = 1 << 0;
pub const HID_LED_CAPS_LOCK: u8   = 1 << 1;
pub const HID_LED_SCROLL_LOCK: u8 = 1 << 2;
pub const HID_LED_COMPOSE: u8     = 1 << 3;
pub const HID_LED_KANA: u8        = 1 << 4;

/// Standard USB HID 8-byte Boot Protocol Keyboard Input Report
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsbHidBootReport {
    pub modifiers: u8,
    pub reserved: u8,
    pub keycodes: [u8; 6],
}

impl UsbHidBootReport {
    pub fn new(modifiers: u8, keycodes: [u8; 6]) -> Self {
        Self {
            modifiers,
            reserved: 0,
            keycodes,
        }
    }

    pub fn is_phantom_state(&self) -> bool {
        // HID ErrorRollOver (0x01) indicated across keys
        self.keycodes.iter().all(|&k| k == 0x01)
    }
}

/// Linux & FreeBSD inspired USB HID Keyboard Driver State
pub struct UsbHidKeyboardDriver {
    pub current_report: UsbHidBootReport,
    pub previous_report: UsbHidBootReport,
    pub active_leds: u8,
    pub repeat_delay_ms: u32,
    pub repeat_rate_hz: u32,
    pub nkro_enabled: bool,
    pub key_press_events: [u16; 16],
    pub key_press_count: usize,
}

impl UsbHidKeyboardDriver {
    pub fn new() -> Self {
        Self {
            current_report: UsbHidBootReport::new(0, [0; 6]),
            previous_report: UsbHidBootReport::new(0, [0; 6]),
            active_leds: 0,
            repeat_delay_ms: 250, // Standard 250ms repeat delay
            repeat_rate_hz: 30,   // Standard 30Hz repeat rate
            nkro_enabled: false,
            key_press_events: [0; 16],
            key_press_count: 0,
        }
    }

    /// Set typematic repeat delay and rate (Linux kbd / FreeBSD kbd control)
    pub fn set_repeat_rate(&mut self, delay_ms: u32, rate_hz: u32) {
        self.repeat_delay_ms = delay_ms;
        self.repeat_rate_hz = rate_hz;
    }

    /// Update LED indicators and generate LED Output Report byte
    pub fn update_led_state(&mut self, num_lock: bool, caps_lock: bool, scroll_lock: bool) -> u8 {
        let mut leds = 0u8;
        if num_lock { leds |= HID_LED_NUM_LOCK; }
        if caps_lock { leds |= HID_LED_CAPS_LOCK; }
        if scroll_lock { leds |= HID_LED_SCROLL_LOCK; }
        self.active_leds = leds;
        leds
    }

    /// Process incoming 8-byte HID Boot Protocol input report
    pub fn process_input_report(&mut self, report_bytes: &[u8]) -> Result<usize, &'static str> {
        if report_bytes.len() < 8 {
            return Err("USB HID: Invalid report length (minimum 8 bytes required)");
        }

        let mut keycodes = [0u8; 6];
        keycodes.copy_from_slice(&report_bytes[2..8]);

        let new_report = UsbHidBootReport::new(report_bytes[0], keycodes);
        if new_report.is_phantom_state() {
            return Err("USB HID: Phantom state / ErrorRollOver detected");
        }

        self.previous_report = self.current_report;
        self.current_report = new_report;
        self.key_press_count = 0;

        // Detect newly pressed keycodes (present in current, missing in previous)
        for &key in &self.current_report.keycodes {
            if key != 0 && !self.previous_report.keycodes.contains(&key) {
                if self.key_press_count < self.key_press_events.len() {
                    self.key_press_events[self.key_press_count] = key as u16;
                    self.key_press_count += 1;
                }
            }
        }

        Ok(self.key_press_count)
    }

    /// Decodes USB HID scancode to ASCII char considering Shift & CapsLock modifiers
    pub fn decode_hid_key_to_ascii(&self, hid_code: u8) -> char {
        let is_shift = (self.current_report.modifiers & (HID_MODIFIER_LSHIFT | HID_MODIFIER_RSHIFT)) != 0;
        let is_caps = (self.active_leds & HID_LED_CAPS_LOCK) != 0;
        let uppercase = is_shift ^ is_caps;

        match hid_code {
            0x04..=0x1D => {
                let base = if uppercase { b'A' } else { b'a' };
                (base + (hid_code - 0x04)) as char
            }
            0x1E..=0x26 => {
                // Number keys 1-9
                if is_shift {
                    let shift_numbers = [b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'('];
                    shift_numbers[(hid_code - 0x1E) as usize] as char
                } else {
                    (b'1' + (hid_code - 0x1E)) as char
                }
            }
            0x27 => if is_shift { ')' } else { '0' },
            0x28 => '\n', // Enter
            0x29 => '\x1B', // Escape
            0x2A => '\x08', // Backspace
            0x2B => '\t', // Tab
            0x2C => ' ', // Space
            _ => '\0',
        }
    }
}

impl Default for UsbHidKeyboardDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usb_hid_keyboard_driver_report_parsing() {
        let mut driver = UsbHidKeyboardDriver::new();
        driver.update_led_state(true, false, false);
        assert_eq!(driver.active_leds, HID_LED_NUM_LOCK);

        // Simulate pressing 'A' (HID 0x04) with Left Shift (modifier 0x02)
        let report = [HID_MODIFIER_LSHIFT, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00];
        let pressed = driver.process_input_report(&report).unwrap();
        assert_eq!(pressed, 1);
        assert_eq!(driver.key_press_events[0], 0x04);

        let ch = driver.decode_hid_key_to_ascii(0x04);
        assert_eq!(ch, 'A');
    }

    #[test]
    fn test_usb_hid_phantom_state_detection() {
        let mut driver = UsbHidKeyboardDriver::new();
        let phantom_report = [0x00, 0x00, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01];
        assert!(driver.process_input_report(&phantom_report).is_err());
    }
}

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
