// Sigma Network Driver CLI
// Command-line interface for network driver management

const std = @import("std");
const sigma_network = @import("lib.zig");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    if (args.len < 2) {
        printUsage();
        std.process.exit(1);
    }

    var driver = sigma_network.NetworkDriver.init(allocator);
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
    } else if (std.mem.eql(u8, command, "mtu")) {
        try handleMTU(&driver, args);
    } else if (std.mem.eql(u8, command, "send")) {
        try handleSend(&driver, args);
    } else {
        printUsage();
        std.process.exit(1);
    }
}

fn printUsage() void {
    std.debug.print("Sigma Network Driver CLI\n\n", .{});
    std.debug.print("Usage:\n", .{});
    std.debug.print("  network_driver detect\n", .{});
    std.debug.print("  network_driver list\n", .{});
    std.debug.print("  network_driver info <interface_name>\n", .{});
    std.debug.print("  network_driver init <interface_name>\n", .{});
    std.debug.print("  network_driver mtu <interface_name> <mtu>\n", .{});
    std.debug.print("  network_driver send <interface_name> <data>\n\n", .{});
    std.debug.print("Example:\n", .{});
    std.debug.print("  network_driver detect\n", .{});
    std.debug.print("  network_driver init eth0\n", .{});
    std.debug.print("  network_driver mtu eth0 9000\n", .{});
}

fn handleDetect(driver: *sigma_network.NetworkDriver) !void {
    try driver.detectInterfaces();

    std.debug.print("Network interface detection complete\n", .{});
    std.debug.print("Found {} network interface(s)\n\n", .{driver.interfaceCount()});

    for (driver.listInterfaces()) |interface| {
        const mac_str = try interface.mac_address.format(std.heap.page_allocator);
        defer std.heap.page_allocator.free(mac_str);

        std.debug.print("Interface Name: {s}\n", .{interface.name});
        std.debug.print("Type: {s}\n", .{interface.interface_type.asStr()});
        std.debug.print("MAC Address: {s}\n", .{mac_str});
        std.debug.print("\n", .{});
    }
}

fn handleList(driver: *sigma_network.NetworkDriver) !void {
    const interfaces = driver.listInterfaces();

    if (interfaces.len == 0) {
        std.debug.print("No network interfaces found. Run 'network_driver detect' first.\n", .{});
        return;
    }

    std.debug.print("Network Interfaces ({}):\n\n", .{interfaces.len});

    for (interfaces) |interface| {
        const mac_str = try interface.mac_address.format(std.heap.page_allocator);
        defer std.heap.page_allocator.free(mac_str);

        std.debug.print("Interface Name: {s}\n", .{interface.name});
        std.debug.print("Type: {s}\n", .{interface.interface_type.asStr()});
        std.debug.print("MAC Address: {s}\n", .{mac_str});
        std.debug.print("Link Speed: {s}\n", .{interface.link_speed.asStr()});
        std.debug.print("MTU: {d}\n", .{interface.mtu});
        std.debug.print("Initialized: {}\n", .{interface.initialized});
        std.debug.print("Link Up: {}\n", .{interface.link_up});
        std.debug.print("\n", .{});
    }
}

fn handleInfo(driver: *sigma_network.NetworkDriver, args: [][:]u8) !void {
    if (args.len < 3) {
        std.debug.print("Error: Interface name required\n", .{});
        printUsage();
        std.process.exit(1);
    }

    const interface_name = args[2];

    const interface = driver.getInterfaceByName(interface_name) orelse {
        std.debug.print("Error: Interface not found: {s}\n", .{interface_name});
        std.process.exit(1);
    };

    const info = try interface.getInfo(std.heap.page_allocator);
    defer std.heap.page_allocator.free(info.interface_id);
    defer std.heap.page_allocator.free(info.mac_address);

    const formatted = try info.format(std.heap.page_allocator);
    defer std.heap.page_allocator.free(formatted);

    std.debug.print("{s}\n", .{formatted});
}

fn handleInit(driver: *sigma_network.NetworkDriver, args: [][:]u8) !void {
    if (args.len < 3) {
        std.debug.print("Error: Interface name required\n", .{});
        printUsage();
        std.process.exit(1);
    }

    const interface_name = args[2];

    try driver.initializeInterface(interface_name);

    std.debug.print("Interface initialized successfully\n", .{});

    const interface = driver.getInterfaceByName(interface_name).?;
    std.debug.print("Name: {s}\n", .{interface.name});
    std.debug.print("Link Speed: {s}\n", .{interface.link_speed.asStr()});
}

fn handleMTU(driver: *sigma_network.NetworkDriver, args: [][:]u8) !void {
    if (args.len < 4) {
        std.debug.print("Error: Interface name and MTU required\n", .{});
        printUsage();
        std.process.exit(1);
    }

    const interface_name = args[2];
    const mtu = try std.fmt.parseInt(u16, args[3], 10);

    try driver.setMTU(interface_name, mtu);

    std.debug.print("MTU set successfully\n", .{});

    const interface = driver.getInterfaceByName(interface_name).?;
    std.debug.print("New MTU: {d}\n", .{interface.getMTU()});
}

fn handleSend(driver: *sigma_network.NetworkDriver, args: [][:]u8) !void {
    if (args.len < 4) {
        std.debug.print("Error: Interface name and data required\n", .{});
        printUsage();
        std.process.exit(1);
    }

    const interface_name = args[2];
    const data = args[3];

    try driver.sendPacket(interface_name, data);

    std.debug.print("Packet sent successfully\n", .{});
    std.debug.print("Bytes: {d}\n", .{data.len});
}
