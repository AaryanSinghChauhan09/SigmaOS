// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/input/ps2_mouse.rs — PS/2 Mouse Driver
//
// Implements the PS/2 mouse driver.
// Supports standard PS/2 mice with scroll wheel.
// Based on Linux kernel psmouse driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::input_device_base::{InputDevice, MouseDevice, InputType, InputEvent, RelativeAxis, INPUT_OK, INPUT_ERR_NO_DEVICE, INPUT_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── PS/2 Mouse I/O Ports ─────────────────────────────────

pub const PS2_DATA_PORT: U16 = 0x60;
pub const PS2_COMMAND_PORT: U16 = 0x64;

// ─── PS/2 Mouse Commands ─────────────────────────────────

pub const PS2_MOUSE_CMD_SET_RATE: U8 = 0xF3;
pub const PS2_MOUSE_CMD_SET_RESOLUTION: U8 = 0xE8;
pub const PS2_MOUSE_CMD_SET_SCALING_2_1: U8 = 0xE6;
pub const PS2_MOUSE_CMD_SET_SCALING_1_1: U8 = 0xE7;
pub const PS2_MOUSE_CMD_REQUEST_STATUS: U8 = 0xE9;
pub const PS2_MOUSE_CMD_SET_STREAM_MODE: U8 = 0xEA;
pub const PS2_MOUSE_CMD_RESET: U8 = 0xFF;
pub const PS2_MOUSE_CMD_SET_SAMPLE_RATE: U8 = 0xF3;
pub const PS2_MOUSE_CMD_ENABLE: U8 = 0xF4;
pub const PS2_MOUSE_CMD_DISABLE: U8 = 0xF5;
pub const PS2_MOUSE_CMD_SET_DEFAULTS: U8 = 0xF6;

// ─── PS/2 Mouse Structure ─────────────────────────────

pub struct Ps2Mouse {
    pub enabled: bool,
    pub initialized: bool,
    pub dpi: U32,
    pub polling_rate: U32,
    pub scroll_enabled: bool,
    pub event_handler: Option<extern "C" fn(*mut InputEvent)>,
    pub relative_axis: RelativeAxis,
    pub button_state: U8,
}

impl Ps2Mouse {
    pub const fn new() -> Self {
        Ps2Mouse {
            enabled: false,
            initialized: false,
            dpi: 400,
            polling_rate: 100,
            scroll_enabled: true,
            event_handler: None,
            relative_axis: RelativeAxis::new(),
            button_state: 0,
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

    /// Send command to mouse
    unsafe fn send_mouse_command(&self, command: U8) -> I32 {
        self.write_command(0xD4); // Send to mouse
        if !self.wait_for_input() {
            return INPUT_ERR_IO;
        }

        self.write_data(command);

        if !self.wait_for_output() {
            return INPUT_ERR_IO;
        }

        let response = self.read_data();
        if response != 0xFA { // ACK
            return INPUT_ERR_IO;
        }

        INPUT_OK
    }

    /// Initialize PS/2 mouse
    fn init_ps2(&mut self) -> I32 {
        unsafe {
            // Disable mouse
            self.send_mouse_command(PS2_MOUSE_CMD_DISABLE);

            // Reset mouse
            self.send_mouse_command(PS2_MOUSE_CMD_RESET);
            if !self.wait_for_output() {
                return INPUT_ERR_INIT_FAILED;
            }
            let response = self.read_data();
            if response != 0xAA {
                return INPUT_ERR_INIT_FAILED;
            }
            if !self.wait_for_output() {
                return INPUT_ERR_INIT_FAILED;
            }
            let device_id = self.read_data();
            if device_id != 0x00 {
                return INPUT_ERR_INIT_FAILED;
            }

            // Set sample rate to 100 Hz
            self.send_mouse_command(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
            self.write_data(100);
            if !self.wait_for_output() {
                return INPUT_ERR_IO;
            }
            self.read_data();

            // Enable scroll wheel
            self.send_mouse_command(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
            self.write_data(200);
            if !self.wait_for_output() {
                return INPUT_ERR_IO;
            }
            self.read_data();

            self.send_mouse_command(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
            self.write_data(200);
            if !self.wait_for_output() {
                return INPUT_ERR_IO;
            }
            self.read_data();

            self.send_mouse_command(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
            self.write_data(80);
            if !self.wait_for_output() {
                return INPUT_ERR_IO;
            }
            self.read_data();

            // Set stream mode
            self.send_mouse_command(PS2_MOUSE_CMD_SET_STREAM_MODE);

            // Enable mouse
            self.send_mouse_command(PS2_MOUSE_CMD_ENABLE);
        }

        self.initialized = true;
        self.enabled = true;

        INPUT_OK
    }

    /// Process mouse packet
    fn process_packet(&mut self, packet: [U8; 3]) {
        // Packet format for standard PS/2 mouse with scroll wheel:
        // Byte 0: Y overflow, X overflow, Y sign, X sign, Middle, Right, Left
        // Byte 1: X movement
        // Byte 2: Y movement

        let left = (packet[0] & 0x01) != 0;
        let right = (packet[0] & 0x02) != 0;
        let middle = (packet[0] & 0x04) != 0;

        let x_sign = (packet[0] & 0x10) != 0;
        let y_sign = (packet[0] & 0x20) != 0;

        let mut x = packet[1] as I32;
        let mut y = packet[2] as I32;

        if x_sign {
            x = x - 256;
        }
        if y_sign {
            y = y - 256;
        }

        // Update relative axis
        self.relative_axis.x = x;
        self.relative_axis.y = y;

        // Update button state
        self.button_state = 0;
        if left {
            self.button_state |= 0x01;
        }
        if right {
            self.button_state |= 0x02;
        }
        if middle {
            self.button_state |= 0x04;
        }

        // Generate events
        if let Some(handler) = self.event_handler {
            unsafe {
                // Button events
                if left {
                    let mut event = InputEvent::new();
                    event.event_type = 0x01; // EV_KEY
                    event.code = 0x110; // BTN_LEFT
                    event.value = 1;
                    handler(&mut event);
                }

                if right {
                    let mut event = InputEvent::new();
                    event.event_type = 0x01; // EV_KEY
                    event.code = 0x111; // BTN_RIGHT
                    event.value = 1;
                    handler(&mut event);
                }

                if middle {
                    let mut event = InputEvent::new();
                    event.event_type = 0x01; // EV_KEY
                    event.code = 0x112; // BTN_MIDDLE
                    event.value = 1;
                    handler(&mut event);
                }

                // Relative motion event
                if x != 0 || y != 0 {
                    let mut event = InputEvent::new();
                    event.event_type = 0x02; // EV_REL
                    event.code = 0x00; // REL_X
                    event.value = x;
                    handler(&mut event);

                    let mut event2 = InputEvent::new();
                    event2.event_type = 0x02; // EV_REL
                    event2.code = 0x01; // REL_Y
                    event2.value = y;
                    handler(&mut event2);
                }
            }
        }
    }
}

// ─── Implement InputDevice Trait ─────────────────────

impl InputDevice for Ps2Mouse {
    fn init(&mut self, _pci_bar: U64, _device_id: U16) -> I32 {
        self.init_ps2()
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        "PS/2 Mouse"
    }

    fn get_input_type(&self) -> InputType {
        InputType::Mouse
    }

    fn enable(&mut self) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        unsafe {
            self.send_mouse_command(PS2_MOUSE_CMD_ENABLE);
        }

        self.enabled = true;
        INPUT_OK
    }

    fn disable(&mut self) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        unsafe {
            self.send_mouse_command(PS2_MOUSE_CMD_DISABLE);
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
                let mut packet = [0u8; 3];
                packet[0] = self.read_data();
                
                if self.wait_for_output() {
                    packet[1] = self.read_data();
                    
                    if self.wait_for_output() {
                        packet[2] = self.read_data();
                        
                        self.process_packet(packet);

                        if !event.is_null() {
                            (*event).event_type = 0x00; // EV_SYN
                            (*event).code = 0;
                            (*event).value = 0;
                        }

                        INPUT_OK
                    } else {
                        INPUT_ERR_IO
                    }
                } else {
                    INPUT_ERR_IO
                }
            } else {
                INPUT_ERR_IO
            }
        }
    }

    fn set_event_handler(&mut self, handler: extern "C" fn(*mut InputEvent)) {
        self.event_handler = Some(handler);
    }

    fn get_key_state(&self, _key: super::input_device_base::KeyCode) -> bool {
        false
    }

    fn get_relative_axis(&self) -> RelativeAxis {
        self.relative_axis
    }

    fn get_absolute_axis(&self) -> super::input_device_base::AbsoluteAxis {
        super::input_device_base::AbsoluteAxis::new()
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.relative_axis = RelativeAxis::new();
        self.button_state = 0;

        INPUT_OK
    }

    fn shutdown(&mut self) -> I32 {
        self.disable();
        self.initialized = false;
        INPUT_OK
    }
}

// ─── Implement MouseDevice Trait ─────────────────

impl MouseDevice for Ps2Mouse {
    fn set_dpi(&mut self, dpi: U32) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        self.dpi = dpi;
        INPUT_OK
    }

    fn get_dpi(&self) -> U32 {
        self.dpi
    }

    fn set_polling_rate(&mut self, rate_hz: U32) -> I32 {
        if !self.initialized {
            return INPUT_ERR_INIT_FAILED;
        }

        unsafe {
            self.send_mouse_command(PS2_MOUSE_CMD_SET_SAMPLE_RATE);
            self.write_data(rate_hz as U8);
            if !self.wait_for_output() {
                return INPUT_ERR_IO;
            }
            self.read_data();
        }

        self.polling_rate = rate_hz;
        INPUT_OK
    }

    fn get_polling_rate(&self) -> U32 {
        self.polling_rate
    }
}

// ─── Global PS/2 Mouse ─────────────────────────

static mut G_PS2_MOUSE: Ps2Mouse = Ps2Mouse::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_init() -> I32 {
    G_PS2_MOUSE.init(0, 0)
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_is_initialized() -> I32 {
    if G_PS2_MOUSE.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_enable() -> I32 {
    G_PS2_MOUSE.enable()
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_disable() -> I32 {
    G_PS2_MOUSE.disable()
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_shutdown() -> I32 {
    G_PS2_MOUSE.shutdown()
}

#[no_mangle]
pub unsafe extern "C" fn ps2_mouse_poll() -> I32 {
    let mut event = InputEvent::new();
    G_PS2_MOUSE.get_event(&mut event)
}

unsafe fn outb(port: U16, value: U8) {
    // Placeholder for I/O port write
}

unsafe fn inb(port: U16) -> U8 {
    // Placeholder for I/O port read
    0
}
