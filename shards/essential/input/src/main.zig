// Sigma Input Driver CLI
// Command-line interface for input driver management

const std = @import("std");
const sigma_input = @import("lib.zig");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        std.process.exit(1);
    }

    var driver = sigma_input.InputDriver.init(allocator);
    defer driver.deinit();

    const command = args[1];

    if (std.mem.eql(u8, command, "detect")) {
        try handleDetect(&driver);
    } else if (std.mem.eql(u8, command, "list")) {
        try handleList(&driver);
    } else if (std.mem.eql(u8, command, "info")) {
        try handleInfo(&driver, args);
    } else if (std.mem.eql(u8, command, "init")) {
        try handleInit(&driver, args);
    } else if (std.mem.eql(u8, command, "enable")) {
        try handleEnable(&driver, args);
    } else if (std.mem.eql(u8, command, "disable")) {
        try handleDisable(&driver, args);
    } else {
        printUsage();
        std.process.exit(1);
    }
}

fn printUsage() void {
    std.debug.print("Sigma Input Driver CLI\n\n", .{});
    std.debug.print("Usage:\n", .{});
    std.debug.print("  input_driver detect\n", .{});
    std.debug.print("  input_driver list\n", .{});
    std.debug.print("  input_driver info <device_name>\n", .{});
    std.debug.print("  input_driver init <device_name>\n", .{});
    std.debug.print("  input_driver enable <device_name>\n", .{});
    std.debug.print("  input_driver disable <device_name>\n\n", .{});
    std.debug.print("Example:\n", .{});
    std.debug.print("  input_driver detect\n", .{});
    std.debug.print("  input_driver init keyboard0\n", .{});
    std.debug.print("  input_driver enable mouse0\n", .{});
}

fn handleDetect(driver: *sigma_input.InputDriver) !void {
    try driver.detectDevices();

    std.debug.print("Input device detection complete\n", .{});
    std.debug.print("Found {} input device(s)\n\n", .{driver.deviceCount()});

    for (driver.listDevices()) |device| {
        std.debug.print("Device Name: {s}\n", .{device.name});
        std.debug.print("Type: {s}\n", .{device.device_type.asStr()});
        std.debug.print("\n", .{});
    }
}

fn handleList(driver: *sigma_input.InputDriver) !void {
    const devices = driver.listDevices();

    if (devices.len == 0) {
        std.debug.print("No input devices found. Run 'input_driver detect' first.\n", .{});
        return;
    }

    std.debug.print("Input Devices ({}):\n\n", .{devices.len});

    for (devices) |device| {
        std.debug.print("Device Name: {s}\n", .{device.name});
        std.debug.print("Type: {s}\n", .{device.device_type.asStr()});
        std.debug.print("Initialized: {}\n", .{device.initialized});
        std.debug.print("Enabled: {}\n", .{device.enabled});
        std.debug.print("\n", .{});
    }
}

fn handleInfo(driver: *sigma_input.InputDriver, args: [][:]u8) !void {
    if (args.len < 3) {
        std.debug.print("Error: Device name required\n", .{});
        printUsage();
        std.process.exit(1);
    }

    const device_name = args[2];

    const device = driver.getDeviceByName(device_name) orelse {
        std.debug.print("Error: Device not found: {s}\n", .{device_name});
        std.process.exit(1);
    };

    const info = try device.getInfo(std.heap.page_allocator);
    defer std.heap.page_allocator.free(info.device_id);

    const formatted = try info.format(std.heap.page_allocator);
    defer std.heap.page_allocator.free(formatted);

    std.debug.print("{s}\n", .{formatted});
}

fn handleInit(driver: *sigma_input.InputDriver, args: [][:]u8) !void {
    if (args.len < 3) {
        std.debug.print("Error: Device name required\n", .{});
        printUsage();
        std.process.exit(1);
    }

    const device_name = args[2];

    try driver.initializeDevice(device_name);

    std.debug.print("Device initialized successfully\n", .{});

    const device = driver.getDeviceByName(device_name).?;
    std.debug.print("Name: {s}\n", .{device.name});
    std.debug.print("Type: {s}\n", .{device.device_type.asStr()});
}

fn handleEnable(driver: *sigma_input.InputDriver, args: [][:]u8) !void {
    if (args.len < 3) {
        std.debug.print("Error: Device name required\n", .{});
        printUsage();
        std.process.exit(1);
    }

    const device_name = args[2];

    try driver.enableDevice(device_name);

    std.debug.print("Device enabled successfully\n", .{});

    const device = driver.getDeviceByName(device_name).?;
    std.debug.print("Name: {s}\n", .{device.name});
    std.debug.print("Enabled: {}\n", .{device.isEnabled()});
}

fn handleDisable(driver: *sigma_input.InputDriver, args: [][:]u8) !void {
    if (args.len < 3) {
        std.debug.print("Error: Device name required\n", .{});
        printUsage();
        std.process.exit(1);
    }

    const device_name = args[2];

    try driver.disableDevice(device_name);

    std.debug.print("Device disabled successfully\n", .{});

    const device = driver.getDeviceByName(device_name).?;
    std.debug.print("Name: {s}\n", .{device.name});
    std.debug.print("Enabled: {}\n", .{device.isEnabled()});
}
