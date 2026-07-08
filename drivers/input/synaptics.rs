// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/input/synaptics.rs — Synaptics Touchpad Driver
//
// Implements the Synaptics touchpad driver.
// Supports Synaptics touchpaps with advanced features.
// Based on Linux kernel synaptics driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::input_device_base::{InputDevice, TouchpadDevice, InputType, InputEvent, AbsoluteAxis, INPUT_OK, INPUT_ERR_NO_DEVICE, INPUT_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Synaptics Vendor ID ─────────────────────────────────

pub const SYNAPTICS_VENDOR_ID: U16 = 0x06CB;

// ─── Synaptics Register Offsets ─────────────────────────

pub const SYNAPTICS_IDENTIFY: U8 = 0x00;
pub const SYNAPTICS_MODE: U8 = 0x01;
pub const SYNAPTICS_CAPABILITIES: U8 = 0x02;
pub const SYNAPTICS_EXT_CAPABILITIES: U8 = 0x0C;
pub const SYNAPTICS_SAMPLE_RATE: U8 = 0x0D;
pub const SYNAPTICS_RESOLUTION: U8 = 0x0E;
pub const SYNAPTICS_EXTENDED_MODE: U8 = 0x0F;

// ─── Synaptics Mode Bits ─────────────────────────

pub const SYNAPTICS_MODE_ABSOLUTE: U8 = 0x80;
pub const SYNAPTICS_MODE_HIGH_RATE: U8 = 0x40;
pub const SYNAPTICS_MODE_SLEEP: U8 = 0x08;
pub const SYNAPTICS_MODE_DISABLE_G: U8 = 0x04;
pub const SYNAPTICS_MODE_PACKET_RATE: U8 = 0x02;
pub const SYNAPTICS_MODE_W_MODE: U8 = 0x01;

// ─── Synaptics Capabilities ─────────────────────

pub const SYNAPTICS_CAP_EXTENDED: U8 = 0x80;
pub const SYNAPTICS_CAP_PASS_THROUGH: U8 = 0x40;
pub const SYNAPTICS_CAP_SLEEP: U8 = 0x20;
pub const SYNAPTICS_CAP_FOUR_BUTTON: U8 = 0x10;
pub const SYNAPTICS_CAP_MULTIFINGER: U8 = 0x08;
pub const SYNAPTICS_CAP_PALMDETECT: U8 = 0x04;
pub const SYNAPTICS_CAP_PEN: U8 = 0x02;

// ─── Synaptics Touchpad Structure ─────────────────

pub struct SynapticsTouchpad {
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

impl SynapticsTouchpad {
    pub const fn new() -> Self {
        SynapticsTouchpad {
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

    /// Initialize Synaptics touchpad
    fn init_synaptics(&mut self) -> I32 {
        // In a real implementation, this would:
        // 1. Identify the touchpad
        // 2. Read capabilities
        // 3. Set mode (absolute mode, high rate)
        // 4. Configure resolution
        // 5. Enable extended features

        // Stub: set default values
        self.width = 3200;
        self.height = 2000;
        self.resolution = 80; // dots per mm

        self.initialized = true;
        self.enabled = true;

        INPUT_OK
    }

    /// Process touchpad packet
    fn process_packet(&mut self, packet: [U8; 6]) {
        // In a real implementation, this would parse the Synaptics packet format
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

impl InputDevice for SynapticsTouchpad {
    fn init(&mut self, _pci_bar: U64, _device_id: U16) -> I32 {
        self.init_synaptics()
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        "Synaptics Touchpad"
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

impl TouchpadDevice for SynapticsTouchpad {
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

// ─── Global Synaptics Touchpad ─────────────────

static mut G_SYNAPTICS: SynapticsTouchpad = SynapticsTouchpad::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn synaptics_init() -> I32 {
    G_SYNAPTICS.init(0, 0)
}

#[no_mangle]
pub unsafe extern "C" fn synaptics_is_initialized() -> I32 {
    if G_SYNAPTICS.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn synaptics_enable() -> I32 {
    G_SYNAPTICS.enable()
}

#[no_mangle]
pub unsafe extern "C" fn synaptics_disable() -> I32 {
    G_SYNAPTICS.disable()
}

#[no_mangle]
pub unsafe extern "C" fn synaptics_shutdown() -> I32 {
    G_SYNAPTICS.shutdown()
}
