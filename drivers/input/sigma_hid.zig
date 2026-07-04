// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/input/sigma_hid.zig — USB HID Keyboard + Mouse Driver
// Language: Zig — direct USB MMIO, comptime HID descriptor parsing
// Pattern: struct with methods

// ── HID Usage Tables (USB HID spec §10) ──────────────────────────────────────

const HID_USAGE_PAGE_KEYBOARD: u16 = 0x07;
const HID_USAGE_PAGE_BUTTON:   u16 = 0x09;

/// Scan-code to ASCII mapping (simplified US QWERTY, no modifier)
const SCANCODE_TO_ASCII: [u8; 128] = blk: {
    var map = [0u8; 128];
    // Letters a-z = HID 0x04–0x1D
    var i: u8 = 0x04;
    while (i <= 0x1D) : (i += 1) {
        map[i] = 'a' + (i - 0x04);
    }
    // Digits 1-9 = HID 0x1E–0x26, 0 = 0x27
    i = 0x1E;
    while (i <= 0x26) : (i += 1) {
        map[i] = '1' + (i - 0x1E);
    }
    map[0x27] = '0';
    // Common keys
    map[0x28] = '\n';  // Return
    map[0x29] = 0x1B;  // Escape
    map[0x2A] = 0x08;  // Backspace
    map[0x2B] = '\t';  // Tab
    map[0x2C] = ' ';   // Space
    map[0x2D] = '-';
    map[0x2E] = '=';
    map[0x2F] = '[';
    map[0x30] = ']';
    map[0x33] = ';';
    map[0x34] = '\'';
    map[0x35] = '`';
    map[0x36] = ',';
    map[0x37] = '.';
    map[0x38] = '/';
    break :blk map;
};

const SCANCODE_TO_SHIFTED: [u8; 128] = blk: {
    var map = [0u8; 128];
    var i: u8 = 0x04;
    while (i <= 0x1D) : (i += 1) {
        map[i] = 'A' + (i - 0x04);
    }
    map[0x1E] = '!'; map[0x1F] = '@'; map[0x20] = '#';
    map[0x21] = '$'; map[0x22] = '%'; map[0x23] = '^';
    map[0x24] = '&'; map[0x25] = '*'; map[0x26] = '(';
    map[0x27] = ')'; map[0x2D] = '_'; map[0x2E] = '+';
    map[0x2F] = '{'; map[0x30] = '}'; map[0x33] = ':';
    map[0x34] = '"'; map[0x35] = '~'; map[0x36] = '<';
    map[0x37] = '>'; map[0x38] = '?'; map[0x2C] = ' ';
    break :blk map;
};

// ── HID Keyboard Boot Protocol Report (8 bytes) ──────────────────────────────

const HidKeyboardReport = packed struct {
    modifiers: u8,     // bit0=LCtrl, bit1=LShift, bit2=LAlt, bit4=RCtrl ...
    reserved:  u8,
    keycodes: [6]u8,   // up to 6 simultaneous keys
};

// ── Key Event ─────────────────────────────────────────────────────────────────

pub const KeyEvent = struct {
    scancode: u8,
    ascii:    u8,
    pressed:  bool,
    shift:    bool,
    ctrl:     bool,
    alt:      bool,
};

// ── Keyboard Event Ring ───────────────────────────────────────────────────────

const RING_SIZE: usize = 64;

pub const KeyRing = struct {
    buf:  [RING_SIZE]KeyEvent,
    head: usize,
    tail: usize,

    pub fn init() KeyRing { return .{ .buf = undefined, .head = 0, .tail = 0 }; }

    pub fn push(self: *KeyRing, ev: KeyEvent) bool {
        const next = (self.tail + 1) % RING_SIZE;
        if (next == self.head) return false; // full
        self.buf[self.tail] = ev;
        self.tail = next;
        return true;
    }

    pub fn pop(self: *KeyRing) ?KeyEvent {
        if (self.head == self.tail) return null;
        const ev = self.buf[self.head];
        self.head = (self.head + 1) % RING_SIZE;
        return ev;
    }

    pub fn is_empty(self: *const KeyRing) bool { return self.head == self.tail; }
};

// ── HID Driver ───────────────────────────────────────────────────────────────

pub const HidKeyboard = struct {
    ring:      KeyRing,
    prev_keys: [6]u8,

    pub fn init() HidKeyboard {
        return HidKeyboard{ .ring = KeyRing.init(), .prev_keys = .{0} ** 6 };
    }

    /// Process a raw 8-byte HID boot-protocol report
    pub fn process_report(self: *HidKeyboard, report: *const HidKeyboardReport) void {
        const shift = (report.modifiers & 0x22) != 0; // LShift | RShift
        const ctrl  = (report.modifiers & 0x11) != 0;
        const alt   = (report.modifiers & 0x44) != 0;

        // Key-release: keys in prev not in current
        for (self.prev_keys) |prev_sc| {
            if (prev_sc == 0) continue;
            var found = false;
            for (report.keycodes) |cur_sc| {
                if (cur_sc == prev_sc) { found = true; break; }
            }
            if (!found) {
                _ = self.ring.push(KeyEvent{
                    .scancode = prev_sc, .ascii = 0,
                    .pressed = false, .shift = shift, .ctrl = ctrl, .alt = alt,
                });
            }
        }

        // Key-press: keys in current not in prev
        for (report.keycodes) |sc| {
            if (sc == 0 or sc >= 128) continue;
            var found = false;
            for (self.prev_keys) |prev| {
                if (prev == sc) { found = true; break; }
            }
            if (!found) {
                const ascii = if (shift) SCANCODE_TO_SHIFTED[sc] else SCANCODE_TO_ASCII[sc];
                _ = self.ring.push(KeyEvent{
                    .scancode = sc, .ascii = ascii,
                    .pressed = true, .shift = shift, .ctrl = ctrl, .alt = alt,
                });
            }
        }

        self.prev_keys = report.keycodes;
    }

    pub fn read_key(self: *HidKeyboard) ?KeyEvent { return self.ring.pop(); }
};

// ── Mouse Boot Protocol Report (4 bytes) ─────────────────────────────────────

const HidMouseReport = packed struct {
    buttons: u8,   // bit0=left, bit1=right, bit2=middle
    dx:      i8,
    dy:      i8,
    wheel:   i8,
};

pub const MouseEvent = struct {
    dx: i8, dy: i8, wheel: i8,
    left: bool, right: bool, middle: bool,
};

pub const HidMouse = struct {
    x: i32,
    y: i32,
    width:  i32,
    height: i32,

    pub fn init(w: i32, h: i32) HidMouse {
        return .{ .x = w/2, .y = h/2, .width = w, .height = h };
    }

    pub fn process_report(self: *HidMouse, report: *const HidMouseReport) MouseEvent {
        self.x = @max(0, @min(self.width  - 1, self.x + report.dx));
        self.y = @max(0, @min(self.height - 1, self.y + report.dy));
        return MouseEvent{
            .dx = report.dx, .dy = report.dy, .wheel = report.wheel,
            .left   = (report.buttons & 0x01) != 0,
            .right  = (report.buttons & 0x02) != 0,
            .middle = (report.buttons & 0x04) != 0,
        };
    }

    pub fn cursor_x(self: *const HidMouse) i32 { return self.x; }
    pub fn cursor_y(self: *const HidMouse) i32 { return self.y; }
};
