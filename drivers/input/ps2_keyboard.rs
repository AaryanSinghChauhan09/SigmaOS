// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/input/ps2_keyboard.rs — PS/2 Keyboard Driver
//
// Implements the PS/2 keyboard driver.
// Supports standard PS/2 keyboards.
// Based on Linux kernel atkbd driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::input_device_base::{InputDevice, KeyboardDevice, InputType, InputEvent, KeyCode, INPUT_OK, INPUT_ERR_NO_DEVICE, INPUT_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── PS/2 Keyboard I/O Ports ─────────────────────────────

pub const PS2_DATA_PORT: U16 = 0x60;
pub const PS2_COMMAND_PORT: U16 = 0x64;

// ─── PS/2 Commands ─────────────────────────────────

pub const PS2_CMD_READ_CONFIG: U8 = 0x20;
pub const PS2_CMD_WRITE_CONFIG: U8 = 0x60;
pub const PS2_CMD_DISABLE_KEYBOARD: U8 = 0xAD;
pub const PS2_CMD_ENABLE_KEYBOARD: U8 = 0xAE;
pub const PS2_CMD_TEST_KEYBOARD: U8 = 0xAB;
pub const PS2_CMD_SELF_TEST: U8 = 0xAA;
pub const PS2_CMD_SCANCODE_SET: U8 = 0xF0;
pub const PS2_CMD_ECHO: U8 = 0xEE;
pub const PS2_CMD_SET_LEDS: U8 = 0xED;
pub const PS2_CMD_SET_REPEAT_RATE: U8 = 0xF3;

// ─── PS/2 Response Codes ─────────────────────────────

pub const PS2_RESP_ACK: U8 = 0xFA;
pub const PS2_RESP_RESEND: U8 = 0xFE;
pub const PS2_RESP_SELF_TEST_OK: U8 = 0x55;

// ─── PS/2 Keyboard Structure ─────────────────────────

pub struct Ps2Keyboard {
    pub enabled: bool,
    pub initialized: bool,
    pub num_lock: bool,
    pub caps_lock: bool,
    pub scroll_lock: bool,
    pub repeat_delay: U32,
    pub repeat_rate: U32,
    pub event_handler: Option<extern "C" fn(*mut InputEvent)>,
    pub key_state: [bool; 256],
}

impl Ps2Keyboard {
    pub const fn new() -> Self {
        Ps2Keyboard {
            enabled: false,
            initialized: false,
            num_lock: false,
            caps_lock: false,
            scroll_lock: false,
            repeat_delay: 500,
            repeat_rate: 30,
            event_handler: None,
            key_state: [false; 256],
        }
    }

    /// Read data from PS/2 data port
    unsafe fn read_data(&self) -> U8 {
        inb(PS2_DATA_PORT)
    }

    /// Write data to PS/2 data port
    unsafe fn write_data(&self, data: U8) {
        outb(PS2_DATA_PORT, data)
    }

    /// Write command to PS/2 command port
    unsafe fn write_command(&self, command: U8) {
        outb(PS2_COMMAND_PORT, command)
    }

    /// Wait for output buffer to be ready
    unsafe fn wait_for_output(&self) -> bool {
        let mut timeout = 10000;
        while timeout > 0 {
            let status = inb(PS2_COMMAND_PORT);
            if status & 0x01 != 0 {
                return true;
            }
            timeout -= 1;
        }
        false
    }

    /// Wait for input buffer to be ready
    unsafe fn wait_for_input(&self) -> bool {
        let mut timeout = 10000;
        while timeout > 0 {
            let status = inb(PS2_COMMAND_PORT);
            if status & 0x02 == 0 {
                return true;
            }
            timeout -= 1;
        }
        false
    }

    /// Send command to keyboard
    unsafe fn send_command(&self, command: U8) -> I32 {
        if !self.wait_for_input() {
            return INPUT_ERR_IO;
        }

        self.write_data(command);

        if !self.wait_for_output() {
            return INPUT_ERR_IO;
        }

        let response = self.read_data();
        if response != PS2_RESP_ACK {
            return INPUT_ERR_IO;
        }

        INPUT_OK
    }

    /// Initialize PS/2 keyboard
    fn init_ps2(&mut self) -> I32 {
        unsafe {
            // Disable keyboard
            self.write_command(PS2_CMD_DISABLE_KEYBOARD);

            // Flush output buffer
            while self.wait_for_output() {
                self.read_data();
            }

            // Perform self-test
            self.write_command(PS2_CMD_SELF_TEST);
            if !self.wait_for_output() {
                return INPUT_ERR_INIT_FAILED;
            }

            let response = self.read_data();
            if response != PS2_RESP_SELF_TEST_OK {
                return INPUT_ERR_INIT_FAILED;
            }

            // Test keyboard
            self.write_command(PS2_CMD_TEST_KEYBOARD);
            if !self.wait_for_output() {
                return INPUT_ERR_INIT_FAILED;
            }

            let response = self.read_data();
            if response != 0x00 {
                return INPUT_ERR_INIT_FAILED;
            }

            // Set scancode set 2
            self.send_command(PS2_CMD_SCANCODE_SET);
            self.send_command(0x02);

            // Enable keyboard
            self.write_command(PS2_CMD_ENABLE_KEYBOARD);
        }

        self.initialized = true;
        self.enabled = true;

        INPUT_OK
    }

    /// Convert scancode to keycode
    fn scancode_to_keycode(&self, scancode: U8) -> KeyCode {
        match scancode {
            0x01 => KeyCode::Esc,
            0x02 => KeyCode::Key1,
            0x03 => KeyCode::Key2,
            0x04 => KeyCode::Key3,
            0x05 => KeyCode::Key4,
            0x06 => KeyCode::Key5,
            0x07 => KeyCode::Key6,
            0x08 => KeyCode::Key7,
            0x09 => KeyCode::Key8,
            0x0A => KeyCode::Key9,
            0x0B => KeyCode::Key0,
            0x0E => KeyCode::Backspace,
            0x0F => KeyCode::Tab,
            0x10 => KeyCode::Q,
            0x11 => KeyCode::W,
            0x12 => KeyCode::E,
            0x13 => KeyCode::R,
            0x14 => KeyCode::T,
            0x15 => KeyCode::Y,
            0x16 => KeyCode::U,
            0x17 => KeyCode::I,
            0x18 => KeyCode::O,
            0x19 => KeyCode::P,
            0x1A => KeyCode::LeftBrace,
            0x1B => KeyCode::RightBrace,
            0x1C => KeyCode::Enter,
            0x1D => KeyCode::LeftCtrl,
            0x1E => KeyCode::A,
            0x1F => KeyCode::S,
            0x20 => KeyCode::D,
            0x21 => KeyCode::F,
            0x22 => KeyCode::G,
            0x23 => KeyCode::H,
            0x24 => KeyCode::J,
            0x25 => KeyCode::K,
            0x26 => KeyCode::L,
            0x27 => KeyCode::Semicolon,
            0x28 => KeyCode::Apostrophe,
            0x29 => KeyCode::Grave,
            0x2A => KeyCode::LeftShift,
            0x2B => KeyCode::Backslash,
            0x2C => KeyCode::Z,
            0x2D => KeyCode::X,
            0x2E => KeyCode::C,
            0x2F => KeyCode::V,
            0x30 => KeyCode::B,
            0x31 => KeyCode::N,
            0x32 => KeyCode::M,
            0x33 => KeyCode::Comma,
            0x34 => KeyCode::Dot,
            0x35 => KeyCode::Slash,
            0x36 => KeyCode::RightShift,
            0x37 => KeyCode::KpAsterisk,
            0x38 => KeyCode::LeftAlt,
            0x39 => KeyCode::Space,
            0x3A => KeyCode::CapsLock,
            0x3B => KeyCode::F1,
            0x3C => KeyCode::F2,
            0x3D => KeyCode::F3,
            0x3E => KeyCode::F4,
            0x3F => KeyCode::F5,
            0x40 => KeyCode::F6,
            0x41 => KeyCode::F7,
            0x42 => KeyCode::F8,
            0x43 => KeyCode::F9,
            0x44 => KeyCode::F10,
            0x45 => KeyCode::NumLock,
            0x46 => KeyCode::ScrollLock,
            0x47 => KeyCode::Kp7,
            0x48 => KeyCode::Kp8,
            0x49 => KeyCode::Kp9,
            0x4A => KeyCode::KpMinus,
            0x4B => KeyCode::Kp4,
            0x4C => KeyCode::Kp5,
            0x4D => KeyCode::Kp6,
            0x4E => KeyCode::KpPlus,
            0x4F => KeyCode::Kp1,
            0x50 => KeyCode::Kp2,
            0x51 => KeyCode::Kp3,
            0x52 => KeyCode::Kp0,
            0x53 => KeyCode::KpDot,
            _ => KeyCode::Unknown,
        }
    }

    /// Process scancode
    fn process_scancode(&mut self, scancode: U8) {
        let is_break = scancode & 0x80 != 0;
        let scancode = scancode & 0x7F;

        let keycode = self.scancode_to_keycode(scancode);
        let key_index = keycode as usize;

        if is_break {
            self.key_state[key_index] = false;
        } else {
            self.key_state[key_index] = true;

            // Handle toggle keys
            match keycode {
                KeyCode::NumLock => {
                    self.num_lock = !self.num_lock;
                    self.update_leds();
                }
                KeyCode::CapsLock => {
                    self.caps_lock = !self.caps_lock;
                    self.update_leds();
                }
                KeyCode::ScrollLock => {
                    self.scroll_lock = !self.scroll_lock;
                    self.update_leds();
                }
                _ => {}
            }
        }

        // Generate event
        if let Some(handler) = self.event_handler {
            let mut event = InputEvent::new();
            event.event_type = 0x01; // EV_KEY
            event.code = keycode as U16;
            event.value = if is_break { 0 } else { 1 };

            unsafe {
                handler(&mut event);
            }
        }
    }

    /// Update LED state
    unsafe fn update_leds(&mut self) {
        let mut led_byte: U8 = 0;
        if self.num_lock {
            led_byte |= 0x02;
        }
        if self.caps_lock {
            led_byte |= 0x04;
        }
        if self.scroll_lock {
            led_byte |= 0x01;
        }

        self.send_command(PS2_CMD_SET_LEDS);
        self.write_data(led_byte);
    }
}

// ─── Implement InputDevice Trait ─────────────────────

impl InputDevice for Ps2Keyboard {
    fn init(&mut self, _pci_bar: U64, _device_id: U16) -> I32 {
        self.init_ps2()
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        "PS/2 Keyboard"
    }

    fn get_input_type(&self) -> InputType {
        InputType::Keyboard
    }

    fn enable(&mut self) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        unsafe {
            self.write_command(PS2_CMD_ENABLE_KEYBOARD);
        }

        self.enabled = true;
        INPUT_OK
    }

    fn disable(&mut self) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        unsafe {
            self.write_command(PS2_CMD_DISABLE_KEYBOARD);
        }

        self.enabled = false;
        INPUT_OK
    }

    fn get_event(&mut self, event: *mut InputEvent) -> I32 {
        if !self.enabled {
            return INPUT_ERR_INIT_FAILED;
        }

        unsafe {
            if self.wait_for_output() {
                let scancode = self.read_data();
                self.process_scancode(scancode);

                if !event.is_null() {
                    (*event).event_type = 0x00; // EV_SYN
                    (*event).code = 0;
                    (*event).value = 0;
                }

                INPUT_OK
            } else {
                INPUT_ERR_IO
            }
        }
    }

    fn set_event_handler(&mut self, handler: extern "C" fn(*mut InputEvent)) {
        self.event_handler = Some(handler);
    }

    fn get_key_state(&self, key: KeyCode) -> bool {
        self.key_state[key as usize]
    }

    fn get_relative_axis(&self) -> super::input_device_base::RelativeAxis {
        super::input_device_base::RelativeAxis::new()
    }

    fn get_absolute_axis(&self) -> super::input_device_base::AbsoluteAxis {
        super::input_device_base::AbsoluteAxis::new()
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.num_lock = false;
        self.caps_lock = false;
        self.scroll_lock = false;

        unsafe {
            self.update_leds();
        }

        INPUT_OK
    }

    fn shutdown(&mut self) -> I32 {
        self.disable();
        self.initialized = false;
        INPUT_OK
    }
}

// ─── Implement KeyboardDevice Trait ─────────────────

impl KeyboardDevice for Ps2Keyboard {
    fn set_led(&mut self, num_lock: bool, caps_lock: bool, scroll_lock: bool) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.num_lock = num_lock;
        self.caps_lock = caps_lock;
        self.scroll_lock = scroll_lock;

        unsafe {
            self.update_leds();
        }

        INPUT_OK
    }

    fn get_led_state(&self) -> (bool, bool, bool) {
        (self.num_lock, self.caps_lock, self.scroll_lock)
    }

    fn set_repeat_rate(&mut self, delay_ms: U32, period_ms: U32) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.repeat_delay = delay_ms;
        self.repeat_rate = period_ms;

        unsafe {
            self.send_command(PS2_CMD_SET_REPEAT_RATE);
            // Convert period to PS/2 format
            let rate = match period_ms {
                0..=30 => 0,
                31..=60 => 1,
                61..=120 => 2,
                121..=240 => 3,
                _ => 0,
            };
            self.write_data(rate);
        }

        INPUT_OK
    }
}

// ─── Global PS/2 Keyboard ─────────────────────────

static mut G_PS2_KEYBOARD: Ps2Keyboard = Ps2Keyboard::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn ps2_keyboard_init() -> I32 {
    G_PS2_KEYBOARD.init(0, 0)
}

#[no_mangle]
pub unsafe extern "C" fn ps2_keyboard_is_initialized() -> I32 {
    if G_PS2_KEYBOARD.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn ps2_keyboard_enable() -> I32 {
    G_PS2_KEYBOARD.enable()
}

#[no_mangle]
pub unsafe extern "C" fn ps2_keyboard_disable() -> I32 {
    G_PS2_KEYBOARD.disable()
}

#[no_mangle]
pub unsafe extern "C" fn ps2_keyboard_shutdown() -> I32 {
    G_PS2_KEYBOARD.shutdown()
}

#[no_mangle]
pub unsafe extern "C" fn ps2_keyboard_poll() -> I32 {
    let mut event = InputEvent::new();
    G_PS2_KEYBOARD.get_event(&mut event)
}

unsafe fn outb(port: U16, value: U8) {
    // Placeholder for I/O port write
}

unsafe fn inb(port: U16) -> U8 {
    // Placeholder for I/O port read
    0
}
