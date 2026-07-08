// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/input/elan.rs — ELAN Touchpad Driver
//
// Implements the ELAN touchpad driver.
// Supports ELAN touchpads with advanced features.
// Based on Linux kernel elan_i2c driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::input_device_base::{InputDevice, TouchpadDevice, InputType, InputEvent, AbsoluteAxis, INPUT_OK, INPUT_ERR_NO_DEVICE, INPUT_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── ELAN Vendor ID ─────────────────────────────────

pub const ELAN_VENDOR_ID: U16 = 0x04F3;

// ─── ELAN Register Offsets ─────────────────────────

pub const ELAN_REG_VERSION: U8 = 0x00;
pub const ELAN_REG_RESOLUTION: U8 = 0x03;
pub const ELAN_REG_REPORT: U8 = 0x04;
pub const ELAN_REG_SLEEP: U8 = 0x05;
pub const ELAN_REG_CONTROL: U8 = 0x10;
pub const ELAN_REG_MAX_X: U8 = 0x11;
pub const ELAN_REG_MAX_Y: U8 = 0x12;
pub const ELAN_REG_TRACENUM: U8 = 0x13;
pub const ELAN_REG_IC_TYPE: U8 = 0x20;

// ─── ELAN Control Bits ─────────────────────────

pub const ELAN_ENABLE_ABS: U8 = 0x01;
pub const ELAN_ENABLE_TRACKPAD: U8 = 0x02;
pub const ELAN_ENABLE_WAKE: U8 = 0x04;
pub const ELAN_ENABLE_SLEEP: U8 = 0x08;

// ─── ELAN Touchpad Structure ─────────────────────

pub struct ElanTouchpad {
    pub enabled: bool,
    pub initialized: bool,
    pub tap_to_click: bool,
    pub palm_detection: bool,
    pub sensitivity: U8,
    pub two_finger_scroll: bool,
    pub absolute_axis: AbsoluteAxis,
    pub finger_count: U8,
    pub width: U32,
    pub height: U32,
    pub resolution: U32,
    pub event_handler: Option<extern "C" fn(*mut InputEvent)>,
}

impl ElanTouchpad {
    pub const fn new() -> Self {
        ElanTouchpad {
            enabled: false,
            initialized: false,
            tap_to_click: true,
            palm_detection: true,
            sensitivity: 128,
            two_finger_scroll: true,
            absolute_axis: AbsoluteAxis::new(),
            finger_count: 0,
            width: 0,
            height: 0,
            resolution: 0,
            event_handler: None,
        }
    }

    /// Initialize ELAN touchpad
    fn init_elan(&mut self) -> I32 {
        // In a real implementation, this would:
        // 1. Identify the touchpad via I2C
        // 2. Read version and capabilities
        // 3. Set control register (enable absolute mode)
        // 4. Configure resolution
        // 5. Enable tracking

        // Stub: set default values
        self.width = 3200;
        self.height = 2000;
        self.resolution = 31; // dots per mm

        self.initialized = true;
        self.enabled = true;

        INPUT_OK
    }

    /// Process touchpad packet
    fn process_packet(&mut self, packet: [U8; 6]) {
        // In a real implementation, this would parse the ELAN packet format
        // and extract finger position, pressure, finger count, etc.

        // Stub: generate absolute motion events
        let x = packet[4] as I32;
        let y = packet[5] as I32;

        self.absolute_axis.x = x;
        self.absolute_axis.y = y;

        // Generate events
        if let Some(handler) = self.event_handler {
            unsafe {
                let mut event = InputEvent::new();
                event.event_type = 0x03; // EV_ABS
                event.code = 0x00; // ABS_X
                event.value = x;
                handler(&mut event);

                let mut event2 = InputEvent::new();
                event2.event_type = 0x03; // EV_ABS
                event2.code = 0x01; // ABS_Y
                event2.value = y;
                handler(&mut event2);
            }
        }
    }
}

// ─── Implement InputDevice Trait ─────────────────

impl InputDevice for ElanTouchpad {
    fn init(&mut self, _pci_bar: U64, _device_id: U16) -> I32 {
        self.init_elan()
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        "ELAN Touchpad"
    }

    fn get_input_type(&self) -> InputType {
        InputType::Touchpad
    }

    fn enable(&mut self) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.enabled = true;
        INPUT_OK
    }

    fn disable(&mut self) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.enabled = false;
        INPUT_OK
    }

    fn get_event(&mut self, event: *mut InputEvent) -> I32 {
        if !self.enabled {
            return INPUT_ERR_INIT_FAILED;
        }

        // In a real implementation, read and process touchpad packet
        INPUT_OK
    }

    fn set_event_handler(&mut self, handler: extern "C" fn(*mut InputEvent)) {
        self.event_handler = Some(handler);
    }

    fn get_key_state(&self, _key: super::input_device_base::KeyCode) -> bool {
        false
    }

    fn get_relative_axis(&self) -> super::input_device_base::RelativeAxis {
        super::input_device_base::RelativeAxis::new()
    }

    fn get_absolute_axis(&self) -> AbsoluteAxis {
        self.absolute_axis
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.absolute_axis = AbsoluteAxis::new();
        INPUT_OK
    }

    fn shutdown(&mut self) -> I32 {
        self.disable();
        self.initialized = false;
        INPUT_OK
    }
}

// ─── Implement TouchpadDevice Trait ─────────────────

impl TouchpadDevice for ElanTouchpad {
    fn set_tap_to_click(&mut self, enable: bool) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.tap_to_click = enable;
        INPUT_OK
    }

    fn set_palm_detection(&mut self, enable: bool) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.palm_detection = enable;
        INPUT_OK
    }

    fn set_sensitivity(&mut self, sensitivity: U8) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.sensitivity = sensitivity;
        INPUT_OK
    }

    fn set_two_finger_scroll(&mut self, enable: bool) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.two_finger_scroll = enable;
        INPUT_OK
    }
}

// ─── Global ELAN Touchpad ─────────────────

static mut G_ELAN: ElanTouchpad = ElanTouchpad::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn elan_init() -> I32 {
    G_ELAN.init(0, 0)
}

#[no_mangle]
pub unsafe extern "C" fn elan_is_initialized() -> I32 {
    if G_ELAN.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn elan_enable() -> I32 {
    G_ELAN.enable()
}

#[no_mangle]
pub unsafe extern "C" fn elan_disable() -> I32 {
    G_ELAN.disable()
}

#[no_mangle]
pub unsafe extern "C" fn elan_shutdown() -> I32 {
    G_ELAN.shutdown()
}
