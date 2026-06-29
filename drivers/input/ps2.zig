// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: PS/2 Keyboard Driver (Zig, no stdlib, no libc)
//! Replaces: drivers/input/sigma_keyboard.cpp, sigma_ps2.cpp
//! =========================================================================

const PS2_DATA_PORT: u16   = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;
const PS2_CMD_PORT: u16    = 0x64;
const PS2_INPUT_FULL: u8   = 0x02;
const PS2_OUTPUT_FULL: u8  = 0x01;

/// Read a byte from an I/O port (inline asm)
fn inb(port: u16) u8 {
    return asm volatile ("inb %[port], %[result]"
        : [result] "={al}" (-> u8),
        : [port] "N{dx}" (port));
}

/// Write a byte to an I/O port
fn outb(port: u16, val: u8) void {
    asm volatile ("outb %[val], %[port]"
        : : [port] "N{dx}" (port), [val] "{al}" (val));
}

/// Wait until PS/2 input buffer is empty
fn wait_write() void {
    var retries: usize = 1000;
    while (retries > 0) : (retries -= 1) {
        if ((inb(PS2_STATUS_PORT) & PS2_INPUT_FULL) == 0) return;
    }
}

/// Wait until PS/2 output buffer is full
fn wait_read() void {
    var retries: usize = 1000;
    while (retries > 0) : (retries -= 1) {
        if ((inb(PS2_STATUS_PORT) & PS2_OUTPUT_FULL) != 0) return;
    }
}

/// Scancode set 2 entry
pub const KeyEvent = struct {
    scancode:  u8,
    released:  bool,
    extended:  bool,
};

/// PS/2 Keyboard Controller — OOP struct
pub const PS2Keyboard = struct {
    enabled:     bool,
    last_scan:   u8,
    ext_pending: bool,

    pub fn new() PS2Keyboard {
        return PS2Keyboard{ .enabled = false, .last_scan = 0, .ext_pending = false };
    }

    pub fn initialize(self: *PS2Keyboard) void {
        // Disable devices during init
        wait_write();
        outb(PS2_CMD_PORT, 0xAD); // disable keyboard
        // Flush output buffer
        _ = inb(PS2_DATA_PORT);
        // Enable keyboard
        wait_write();
        outb(PS2_CMD_PORT, 0xAE);
        self.enabled = true;
    }

    pub fn poll(self: *PS2Keyboard) ?KeyEvent {
        if (!self.enabled) return null;
        if ((inb(PS2_STATUS_PORT) & PS2_OUTPUT_FULL) == 0) return null;
        const scan = inb(PS2_DATA_PORT);
        self.last_scan = scan;
        if (scan == 0xE0) {
            self.ext_pending = true;
            return null;
        }
        const released = (scan & 0x80) != 0;
        const code = scan & 0x7F;
        const ext = self.ext_pending;
        self.ext_pending = false;
        return KeyEvent{ .scancode = code, .released = released, .extended = ext };
    }
};

/// PS/2 Mouse Controller — OOP struct
pub const PS2Mouse = struct {
    enabled: bool,
    dx: i8,
    dy: i8,
    buttons: u8,

    pub fn new() PS2Mouse {
        return PS2Mouse{ .enabled = false, .dx = 0, .dy = 0, .buttons = 0 };
    }

    pub fn initialize(self: *PS2Mouse) void {
        wait_write();
        outb(PS2_CMD_PORT, 0xA8); // enable aux device (mouse)
        self.enabled = true;
    }

    pub fn poll(self: *PS2Mouse) bool {
        if (!self.enabled) return false;
        if ((inb(PS2_STATUS_PORT) & PS2_OUTPUT_FULL) == 0) return false;
        self.buttons = inb(PS2_DATA_PORT);
        self.dx      = @bitCast(i8, inb(PS2_DATA_PORT));
        self.dy      = @bitCast(i8, inb(PS2_DATA_PORT));
        return true;
    }
};
