// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/input/input_device_base.rs — Base Device Trait for Input Drivers
//
// Defines the OOP base class for all input devices using Rust traits.
// This provides a common interface for input operations with evdev compatibility.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Input Error Codes ─────────────────────────────────────────────────

pub const INPUT_OK: I32 = 0;
pub const INPUT_ERR_NO_DEVICE: I32 = -1;
pub const INPUT_ERR_INIT_FAILED: I32 = -2;
pub const INPUT_ERR_OUT_OF_MEM: I32 = -3;
pub const INPUT_ERR_NOT_SUPPORTED: I32 = -4;
pub const INPUT_ERR_IO: I32 = -5;

// ─── Input Device Types ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputType {
    Keyboard,
    Mouse,
    Touchpad,
    Touchscreen,
    Joystick,
    Tablet,
    Switch,
    Unknown,
}

// ─── Input Event Types ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEventType {
    Sync,
    Key,
    Relative,
    Absolute,
    Misc,
    Switch,
    Led,
    Sound,
    ForceFeedback,
    ForceFeedbackStatus,
}

// ─── Key Codes ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyCode {
    Reserved,
    Esc,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Minus,
    Equal,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    LeftBrace,
    RightBrace,
    Enter,
    LeftCtrl,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Semicolon,
    Apostrophe,
    Grave,
    LeftShift,
    Backslash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Slash,
    RightShift,
    KpAsterisk,
    LeftAlt,
    Space,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    NumLock,
    ScrollLock,
    Kp7,
    Kp8,
    Kp9,
    KpMinus,
    Kp4,
    Kp5,
    Kp6,
    KpPlus,
    Kp1,
    Kp2,
    Kp3,
    Kp0,
    KpDot,
    Unknown,
}

// ─── Input Event ─────────────────────────────────────────────

#[repr(C)]
pub struct InputEvent {
    pub sec: U64,
    pub usec: U64,
    pub event_type: U16,
    pub code: U16,
    pub value: I32,
}

impl InputEvent {
    pub const fn new() -> Self {
        InputEvent {
            sec: 0,
            usec: 0,
            event_type: 0,
            code: 0,
            value: 0,
        }
    }
}

// ─── Relative Axis ─────────────────────────────────────────

#[repr(C)]
pub struct RelativeAxis {
    pub x: I32,
    pub y: I32,
    pub z: I32,
}

impl RelativeAxis {
    pub const fn new() -> Self {
        RelativeAxis {
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

// ─── Absolute Axis ─────────────────────────────────────────

#[repr(C)]
pub struct AbsoluteAxis {
    pub x: I32,
    pub y: I32,
    pub z: I32,
    pub rx: I32,
    pub ry: I32,
    pub rz: I32,
    pub throttle: I32,
    pub rudder: I32,
    pub wheel: I32,
}

impl AbsoluteAxis {
    pub const fn new() -> Self {
        AbsoluteAxis {
            x: 0,
            y: 0,
            z: 0,
            rx: 0,
            ry: 0,
            rz: 0,
            throttle: 0,
            rudder: 0,
            wheel: 0,
        }
    }
}

// ─── Input Device Trait ─────────────────────────────────────

/// Trait for input device operations
pub trait InputDevice {
    /// Initialize the input device
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get device name
    fn get_device_name(&self) -> &'static str;
    
    /// Get input type
    fn get_input_type(&self) -> InputType;
    
    /// Enable device
    fn enable(&mut self) -> I32;
    
    /// Disable device
    fn disable(&mut self) -> I32;
    
    /// Get event
    fn get_event(&mut self, event: *mut InputEvent) -> I32;
    
    /// Set event handler
    fn set_event_handler(&mut self, handler: extern "C" fn(*mut InputEvent));
    
    /// Get key state
    fn get_key_state(&self, key: KeyCode) -> bool;
    
    /// Get relative axis
    fn get_relative_axis(&self) -> RelativeAxis;
    
    /// Get absolute axis
    fn get_absolute_axis(&self) -> AbsoluteAxis;
    
    /// Reset the device
    fn reset(&mut self) -> I32;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> I32;
}

// ─── Keyboard Trait ─────────────────────────────────────

/// Trait for keyboard-specific operations
pub trait KeyboardDevice: InputDevice {
    /// Set LED state
    fn set_led(&mut self, num_lock: bool, caps_lock: bool, scroll_lock: bool) -> I32;
    
    /// Get LED state
    fn get_led_state(&self) -> (bool, bool, bool);
    
    /// Set repeat rate
    fn set_repeat_rate(&mut self, delay_ms: U32, period_ms: U32) -> I32;
}

// ─── Mouse Trait ─────────────────────────────────────

/// Trait for mouse-specific operations
pub trait MouseDevice: InputDevice {
    /// Set DPI
    fn set_dpi(&mut self, dpi: U32) -> I32;
    
    /// Get DPI
    fn get_dpi(&self) -> U32;
    
    /// Set polling rate
    fn set_polling_rate(&mut self, rate_hz: U32) -> I32;
    
    /// Get polling rate
    fn get_polling_rate(&self) -> U32;
}

// ─── Touchpad Trait ─────────────────────────────────

/// Trait for touchpad-specific operations
pub trait TouchpadDevice: InputDevice {
    /// Enable/disable tap-to-click
    fn set_tap_to_click(&mut self, enable: bool) -> I32;
    
    /// Enable/disable palm detection
    fn set_palm_detection(&mut self, enable: bool) -> I32;
    
    /// Set sensitivity
    fn set_sensitivity(&mut self, sensitivity: U8) -> I32;
    
    /// Enable/disable two-finger scroll
    fn set_two_finger_scroll(&mut self, enable: bool) -> I32;
}
