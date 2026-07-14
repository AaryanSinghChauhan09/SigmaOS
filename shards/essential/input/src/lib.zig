// Sigma Input Driver - Input Device Driver Prototype
// Implements keyboard, mouse, and touchscreen input handling
// No external dependencies - implementing from first principles

const std = @import("std");

/// Input device type
pub const InputDeviceType = enum {
    Keyboard,
    Mouse,
    Touchscreen,
    Unknown,

    pub fn asStr(self: InputDeviceType) []const u8 {
        return switch (self) {
            .Keyboard => "Keyboard",
            .Mouse => "Mouse",
            .Touchscreen => "Touchscreen",
            .Unknown => "Unknown",
        };
    }
};

/// Key code
pub const KeyCode = enum(u16) {
    KeyA = 0x04,
    KeyB = 0x05,
    KeyC = 0x06,
    KeyD = 0x07,
    KeyE = 0x08,
    KeyF = 0x09,
    KeyG = 0x0a,
    KeyH = 0x0b,
    KeyI = 0x0c,
    KeyJ = 0x0d,
    KeyK = 0x0e,
    KeyL = 0x0f,
    KeyM = 0x10,
    KeyN = 0x11,
    KeyO = 0x12,
    KeyP = 0x13,
    KeyQ = 0x14,
    KeyR = 0x15,
    KeyS = 0x16,
    KeyT = 0x17,
    KeyU = 0x18,
    KeyV = 0x19,
    KeyW = 0x1a,
    KeyX = 0x1b,
    KeyY = 0x1c,
    KeyZ = 0x1d,
    Key0 = 0x27,
    Key1 = 0x1e,
    Key2 = 0x1f,
    Key3 = 0x20,
    Key4 = 0x21,
    Key5 = 0x22,
    Key6 = 0x23,
    Key7 = 0x24,
    Key8 = 0x25,
    Key9 = 0x26,
    Space = 0x2c,
    Enter = 0x28,
    Escape = 0x29,
    Backspace = 0x2a,
    Tab = 0x2b,
    LeftShift = 0x2d,
    RightShift = 0x36,
    LeftCtrl = 0x3a,
    RightCtrl = 0x3b,
    LeftAlt = 0x3c,
    RightAlt = 0x3d,
    Unknown = 0x00,

    pub fn asStr(self: KeyCode) []const u8 {
        return switch (self) {
            .KeyA => "A",
            .KeyB => "B",
            .KeyC => "C",
            .KeyD => "D",
            .KeyE => "E",
            .KeyF => "F",
            .KeyG => "G",
            .KeyH => "H",
            .KeyI => "I",
            .KeyJ => "J",
            .KeyK => "K",
            .KeyL => "L",
            .KeyM => "M",
            .KeyN => "N",
            .KeyO => "O",
            .KeyP => "P",
            .KeyQ => "Q",
            .KeyR => "R",
            .KeyS => "S",
            .KeyT => "T",
            .KeyU => "U",
            .KeyV => "V",
            .KeyW => "W",
            .KeyX => "X",
            .KeyY => "Y",
            .KeyZ => "Z",
            .Key0 => "0",
            .Key1 => "1",
            .Key2 => "2",
            .Key3 => "3",
            .Key4 => "4",
            .Key5 => "5",
            .Key6 => "6",
            .Key7 => "7",
            .Key8 => "8",
            .Key9 => "9",
            .Space => "Space",
            .Enter => "Enter",
            .Escape => "Escape",
            .Backspace => "Backspace",
            .Tab => "Tab",
            .LeftShift => "Left Shift",
            .RightShift => "Right Shift",
            .LeftCtrl => "Left Ctrl",
            .RightCtrl => "Right Ctrl",
            .LeftAlt => "Left Alt",
            .RightAlt => "Right Alt",
            .Unknown => "Unknown",
        };
    }
};

/// Key event
pub const KeyEvent = struct {
    key_code: KeyCode,
    pressed: bool,
    timestamp: u64,

    pub fn init(key_code: KeyCode, pressed: bool) KeyEvent {
        const timestamp = std.time.timestamp();
        return KeyEvent{
            .key_code = key_code,
            .pressed = pressed,
            .timestamp = @intCast(timestamp),
        };
    }
};

/// Mouse button
pub const MouseButton = enum(u8) {
    Left = 0x01,
    Right = 0x02,
    Middle = 0x03,
    Unknown = 0x00,

    pub fn asStr(self: MouseButton) []const u8 {
        return switch (self) {
            .Left => "Left",
            .Right => "Right",
            .Middle => "Middle",
            .Unknown => "Unknown",
        };
    }
};

/// Mouse event
pub const MouseEvent = struct {
    x: i32,
    y: i32,
    button: MouseButton,
    pressed: bool,
    timestamp: u64,

    pub fn init(x: i32, y: i32, button: MouseButton, pressed: bool) MouseEvent {
        const timestamp = std.time.timestamp();
        return MouseEvent{
            .x = x,
            .y = y,
            .button = button,
            .pressed = pressed,
            .timestamp = @intCast(timestamp),
        };
    }
};

/// Touch event
pub const TouchEvent = struct {
    x: i32,
    y: i32,
    touch_id: u32,
    pressed: bool,
    timestamp: u64,

    pub fn init(x: i32, y: i32, touch_id: u32, pressed: bool) TouchEvent {
        const timestamp = std.time.timestamp();
        return TouchEvent{
            .x = x,
            .y = y,
            .touch_id = touch_id,
            .pressed = pressed,
            .timestamp = @intCast(timestamp),
        };
    }
};

/// Input event
pub const InputEvent = union(enum) {
    Key: KeyEvent,
    Mouse: MouseEvent,
    Touch: TouchEvent,
};

/// Input device
pub const InputDevice = struct {
    device_id: [32]u8,
    name: []const u8,
    device_type: InputDeviceType,
    initialized: bool,
    enabled: bool,

    pub fn init(name: []const u8, device_type: InputDeviceType) InputDevice {
        var device_id = [_]u8{0} ** 32;
        const name_bytes = name;
        for (name_bytes, 0..) |byte, i| {
            device_id[i % 32] +%= byte;
        }
        const type_bytes = device_type.asStr();
        for (type_bytes, 0..) |byte, i| {
            device_id[(i + 16) % 32] +%= byte;
        }

        return InputDevice{
            .device_id = device_id,
            .name = name,
            .device_type = device_type,
            .initialized = false,
            .enabled = false,
        };
    }

    pub fn getDeviceId(self: InputDevice, allocator: std.mem.Allocator) ![]u8 {
        var result = try allocator.alloc(u8, 64);
        for (self.device_id, 0..) |byte, i| {
            std.fmt.formatIntBuf(result[i * 2 ..][0..2], byte, 16, .lower, .{ .fill = '0', .width = 2 });
        }
        return result;
    }

    pub fn initialize(self: *InputDevice) !void {
        if (self.initialized) {
            return error.AlreadyInitialized;
        }

        self.initialized = true;
        self.enabled = true;
    }

    pub fn enable(self: *InputDevice) !void {
        if (!self.initialized) {
            return error.DeviceNotInitialized;
        }
        self.enabled = true;
    }

    pub fn disable(self: *InputDevice) !void {
        if (!self.initialized) {
            return error.DeviceNotInitialized;
        }
        self.enabled = false;
    }

    pub fn isEnabled(self: InputDevice) bool {
        return self.enabled;
    }

    pub fn getInfo(self: InputDevice, allocator: std.mem.Allocator) !DeviceInfo {
        const id_str = try self.getDeviceId(allocator);

        return DeviceInfo{
            .device_id = id_str,
            .name = self.name,
            .device_type = self.device_type,
            .initialized = self.initialized,
            .enabled = self.enabled,
        };
    }
};

/// Device information
pub const DeviceInfo = struct {
    device_id: []const u8,
    name: []const u8,
    device_type: InputDeviceType,
    initialized: bool,
    enabled: bool,

    pub fn format(self: DeviceInfo, allocator: std.mem.Allocator) ![]u8 {
        return std.fmt.allocPrint(allocator, 
            \\Input Device Information
            \\Device ID: {s}
            \\Name: {s}
            \\Type: {s}
            \\Initialized: {}
            \\Enabled: {}
        , .{
            self.device_id,
            self.name,
            self.device_type.asStr(),
            self.initialized,
            self.enabled,
        });
    }
};

/// Input driver
pub const InputDriver = struct {
    devices: std.ArrayList(InputDevice),
    event_queue: std.ArrayList(InputEvent),

    pub fn init(allocator: std.mem.Allocator) InputDriver {
        return InputDriver{
            .devices = std.ArrayList(InputDevice).init(allocator),
            .event_queue = std.ArrayList(InputEvent).init(allocator),
        };
    }

    pub fn deinit(self: *InputDriver) void {
        self.devices.deinit();
        self.event_queue.deinit();
    }

    /// Detect input devices
    pub fn detectDevices(self: *InputDriver) !void {
        // Simulate device detection
        const keyboard = InputDevice.init("keyboard0", .Keyboard);
        try self.devices.append(keyboard);

        const mouse = InputDevice.init("mouse0", .Mouse);
        try self.devices.append(mouse);

        const touchscreen = InputDevice.init("touchscreen0", .Touchscreen);
        try self.devices.append(touchscreen);
    }

    /// Get device by ID
    pub fn getDevice(self: InputDriver, device_id: []const u8) ?*InputDevice {
        for (self.devices.items) |*device| {
            const id = device.getDeviceId(std.heap.page_allocator) catch continue;
            defer std.heap.page_allocator.free(id);
            if (std.mem.eql(u8, id, device_id)) {
                return device;
            }
        }
        return null;
    }

    /// Get device by name
    pub fn getDeviceByName(self: InputDriver, name: []const u8) ?*InputDevice {
        for (self.devices.items) |*device| {
            if (std.mem.eql(u8, device.name, name)) {
                return device;
            }
        }
        return null;
    }

    /// Initialize device
    pub fn initializeDevice(self: *InputDriver, name: []const u8) !void {
        const device = self.getDeviceByName(name) orelse return error.DeviceNotFound;
        try device.initialize();
    }

    /// Enable device
    pub fn enableDevice(self: *InputDriver, name: []const u8) !void {
        const device = self.getDeviceByName(name) orelse return error.DeviceNotFound;
        try device.enable();
    }

    /// Disable device
    pub fn disableDevice(self: *InputDriver, name: []const u8) !void {
        const device = self.getDeviceByName(name) orelse return error.DeviceNotFound;
        try device.disable();
    }

    /// Queue key event
    pub fn queueKeyEvent(self: *InputDriver, key_code: KeyCode, pressed: bool) !void {
        const event = KeyEvent.init(key_code, pressed);
        try self.event_queue.append(InputEvent{ .Key = event });
    }

    /// Queue mouse event
    pub fn queueMouseEvent(self: *InputDriver, x: i32, y: i32, button: MouseButton, pressed: bool) !void {
        const event = MouseEvent.init(x, y, button, pressed);
        try self.event_queue.append(InputEvent{ .Mouse = event });
    }

    /// Queue touch event
    pub fn queueTouchEvent(self: *InputDriver, x: i32, y: i32, touch_id: u32, pressed: bool) !void {
        const event = TouchEvent.init(x, y, touch_id, pressed);
        try self.event_queue.append(InputEvent{ .Touch = event });
    }

    /// Process event queue
    pub fn processEventQueue(self: *InputDriver) !void {
        while (self.event_queue.items.len > 0) {
            _ = self.event_queue.orderedRemove(0);
        }
    }

    /// List all devices
    pub fn listDevices(self: InputDriver) []const InputDevice {
        return self.devices.items;
    }

    /// Get device count
    pub fn deviceCount(self: InputDriver) usize {
        return self.devices.items.len;
    }
};

test "input device creation" {
    const device = InputDevice.init("keyboard0", .Keyboard);

    try std.testing.expectEqualStrings("keyboard0", device.name);
    try std.testing.expectEqual(InputDeviceType.Keyboard, device.device_type);
    try std.testing.expect(!device.initialized);
}

test "device initialization" {
    var device = InputDevice.init("mouse0", .Mouse);

    try device.initialize();
    try std.testing.expect(device.initialized);
    try std.testing.expect(device.enabled);
}

test "device enable/disable" {
    var device = InputDevice.init("touchscreen0", .Touchscreen);
    try device.initialize();

    try device.disable();
    try std.testing.expect(!device.isEnabled());

    try device.enable();
    try std.testing.expect(device.isEnabled());
}

test "key event creation" {
    const event = KeyEvent.init(.KeyA, true);

    try std.testing.expectEqual(KeyCode.KeyA, event.key_code);
    try std.testing.expect(event.pressed);
}

test "mouse event creation" {
    const event = MouseEvent.init(100, 200, .Left, true);

    try std.testing.expectEqual(@as(i32, 100), event.x);
    try std.testing.expectEqual(@as(i32, 200), event.y);
    try std.testing.expectEqual(MouseButton.Left, event.button);
}

test "input driver" {
    var driver = InputDriver.init(std.testing.allocator);
    defer driver.deinit();

    try driver.detectDevices();

    try std.testing.expectEqual(@as(usize, 3), driver.deviceCount());
}
