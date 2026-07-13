// Sigma Network Driver - Network Interface Card Driver Prototype
// Implements NIC initialization and packet transmission/reception
// No external dependencies - implementing from first principles

const std = @import("std");

/// Network interface type
pub const InterfaceType = enum {
    Ethernet,
    WiFi,
    Unknown,

    pub fn asStr(self: InterfaceType) []const u8 {
        return switch (self) {
            .Ethernet => "Ethernet",
            .WiFi => "WiFi",
            .Unknown => "Unknown",
        };
    }
};

/// Link speed
pub const LinkSpeed = enum {
    Speed10,
    Speed100,
    Speed1000,
    Speed10000,
    Unknown,

    pub fn asStr(self: LinkSpeed) []const u8 {
        return switch (self) {
            .Speed10 => "10 Mbps",
            .Speed100 => "100 Mbps",
            .Speed1000 => "1 Gbps",
            .Speed10000 => "10 Gbps",
            .Unknown => "Unknown",
        };
    }
};

/// MAC address
pub const MACAddress = struct {
    bytes: [6]u8,

    pub fn init(bytes: [6]u8) MACAddress {
        return MACAddress{ .bytes = bytes };
    }

    pub fn format(self: MACAddress, allocator: std.mem.Allocator) ![]u8 {
        return std.fmt.allocPrint(allocator, "{x:0>2}:{x:0>2}:{x:0>2}:{x:0>2}:{x:0>2}:{x:0>2}", .{
            self.bytes[0], self.bytes[1], self.bytes[2],
            self.bytes[3], self.bytes[4], self.bytes[5],
        });
    }

    pub fn isBroadcast(self: MACAddress) bool {
        for (self.bytes) |byte| {
            if (byte != 0xff) return false;
        }
        return true;
    }

    pub fn isMulticast(self: MACAddress) bool {
        return (self.bytes[0] & 0x01) != 0;
    }
};

/// Network interface
pub const NetworkInterface = struct {
    interface_id: [32]u8,
    name: []const u8,
    interface_type: InterfaceType,
    mac_address: MACAddress,
    link_speed: LinkSpeed,
    mtu: u16,
    initialized: bool,
    link_up: bool,

    pub fn init(name: []const u8, interface_type: InterfaceType, mac_address: MACAddress) NetworkInterface {
        var interface_id = [_]u8{0} ** 32;
        const name_bytes = name;
        for (name_bytes, 0..) |byte, i| {
            interface_id[i % 32] +%= byte;
        }
        const type_bytes = interface_type.asStr();
        for (type_bytes, 0..) |byte, i| {
            interface_id[(i + 16) % 32] +%= byte;
        }

        return NetworkInterface{
            .interface_id = interface_id,
            .name = name,
            .interface_type = interface_type,
            .mac_address = mac_address,
            .link_speed = .Unknown,
            .mtu = 1500,
            .initialized = false,
            .link_up = false,
        };
    }

    pub fn getInterfaceId(self: NetworkInterface, allocator: std.mem.Allocator) ![]u8 {
        var result = try allocator.alloc(u8, 64);
        for (self.interface_id, 0..) |byte, i| {
            std.fmt.formatIntBuf(result[i * 2 ..][0..2], byte, 16, .lower, .{ .fill = '0', .width = 2 });
        }
        return result;
    }

    pub fn initialize(self: *NetworkInterface) !void {
        if (self.initialized) {
            return error.AlreadyInitialized;
        }

        self.initialized = true;
        self.link_up = true;
        self.link_speed = .Speed1000;
    }

    pub fn setMTU(self: *NetworkInterface, mtu: u16) !void {
        if (mtu < 576 or mtu > 9000) {
            return error.InvalidMTU;
        }
        self.mtu = mtu;
    }

    pub fn getMTU(self: NetworkInterface) u16 {
        return self.mtu;
    }

    pub fn isUp(self: NetworkInterface) bool {
        return self.link_up;
    }

    pub fn getInfo(self: NetworkInterface, allocator: std.mem.Allocator) !InterfaceInfo {
        const mac_str = try self.mac_address.format(allocator);
        const id_str = try self.getInterfaceId(allocator);

        return InterfaceInfo{
            .interface_id = id_str,
            .name = self.name,
            .interface_type = self.interface_type,
            .mac_address = mac_str,
            .link_speed = self.link_speed,
            .mtu = self.mtu,
            .initialized = self.initialized,
            .link_up = self.link_up,
        };
    }
};

/// Interface information
pub const InterfaceInfo = struct {
    interface_id: []const u8,
    name: []const u8,
    interface_type: InterfaceType,
    mac_address: []const u8,
    link_speed: LinkSpeed,
    mtu: u16,
    initialized: bool,
    link_up: bool,

    pub fn format(self: InterfaceInfo, allocator: std.mem.Allocator) ![]u8 {
        return std.fmt.allocPrint(allocator, 
            \\Network Interface Information
            \\Interface ID: {s}
            \\Name: {s}
            \\Type: {s}
            \\MAC Address: {s}
            \\Link Speed: {s}
            \\MTU: {d}
            \\Initialized: {}
            \\Link Up: {}
        , .{
            self.interface_id,
            self.name,
            self.interface_type.asStr(),
            self.mac_address,
            self.link_speed.asStr(),
            self.mtu,
            self.initialized,
            self.link_up,
        });
    }
};

/// Network packet
pub const NetworkPacket = struct {
    packet_id: [32]u8,
    data: []u8,
    length: usize,
    interface_id: []const u8,

    pub fn init(data: []u8, interface_id: []const u8) NetworkPacket {
        var packet_id = [_]u8{0} ** 32;
        for (data, 0..) |byte, i| {
            packet_id[i % 32] +%= byte;
        }
        for (interface_id, 0..) |byte, i| {
            packet_id[(i + 16) % 32] +%= byte;
        }

        return NetworkPacket{
            .packet_id = packet_id,
            .data = data,
            .length = data.len,
            .interface_id = interface_id,
        };
    }

    pub fn getPacketId(self: NetworkPacket, allocator: std.mem.Allocator) ![]u8 {
        var result = try allocator.alloc(u8, 64);
        for (self.packet_id, 0..) |byte, i| {
            std.fmt.formatIntBuf(result[i * 2 ..][0..2], byte, 16, .lower, .{ .fill = '0', .width = 2 });
        }
        return result;
    }
};

/// Network driver
pub const NetworkDriver = struct {
    interfaces: std.ArrayList(NetworkInterface),
    packet_queue: std.ArrayList(NetworkPacket),

    pub fn init(allocator: std.mem.Allocator) NetworkDriver {
        return NetworkDriver{
            .interfaces = std.ArrayList(NetworkInterface).init(allocator),
            .packet_queue = std.ArrayList(NetworkPacket).init(allocator),
        };
    }

    pub fn deinit(self: *NetworkDriver) void {
        self.interfaces.deinit();
        self.packet_queue.deinit();
    }

    /// Detect network interfaces
    pub fn detectInterfaces(self: *NetworkDriver) !void {
        // Simulate interface detection
        const mac1 = MACAddress.init([_]u8{ 0x00, 0x11, 0x22, 0x33, 0x44, 0x55 });
        const eth0 = NetworkInterface.init("eth0", .Ethernet, mac1);
        try self.interfaces.append(eth0);

        const mac2 = MACAddress.init([_]u8{ 0x00, 0xaa, 0xbb, 0xcc, 0xdd, 0xee });
        const wlan0 = NetworkInterface.init("wlan0", .WiFi, mac2);
        try self.interfaces.append(wlan0);
    }

    /// Get interface by ID
    pub fn getInterface(self: NetworkDriver, interface_id: []const u8) ?*NetworkInterface {
        for (self.interfaces.items) |*interface| {
            const id = interface.getInterfaceId(std.heap.page_allocator) catch continue;
            defer std.heap.page_allocator.free(id);
            if (std.mem.eql(u8, id, interface_id)) {
                return interface;
            }
        }
        return null;
    }

    /// Get interface by name
    pub fn getInterfaceByName(self: NetworkDriver, name: []const u8) ?*NetworkInterface {
        for (self.interfaces.items) |*interface| {
            if (std.mem.eql(u8, interface.name, name)) {
                return interface;
            }
        }
        return null;
    }

    /// Initialize interface
    pub fn initializeInterface(self: *NetworkDriver, name: []const u8) !void {
        const interface = self.getInterfaceByName(name) orelse return error.InterfaceNotFound;
        try interface.initialize();
    }

    /// Set MTU
    pub fn setMTU(self: *NetworkDriver, name: []const u8, mtu: u16) !void {
        const interface = self.getInterfaceByName(name) orelse return error.InterfaceNotFound;
        try interface.setMTU(mtu);
    }

    /// Send packet
    pub fn sendPacket(self: *NetworkDriver, interface_name: []const u8, data: []u8) !void {
        const interface = self.getInterfaceByName(interface_name) orelse return error.InterfaceNotFound;
        if (!interface.initialized) {
            return error.InterfaceNotInitialized;
        }

        const interface_id = try interface.getInterfaceId(std.heap.page_allocator);
        defer std.heap.page_allocator.free(interface_id);

        const packet = NetworkPacket.init(data, interface_id);
        try self.packet_queue.append(packet);
    }

    /// Receive packet
    pub fn receivePacket(self: *NetworkDriver, interface_name: []const u8) !NetworkPacket {
        const interface = self.getInterfaceByName(interface_name) orelse return error.InterfaceNotFound;
        if (!interface.initialized) {
            return error.InterfaceNotInitialized;
        }

        // Simulate packet reception
        const data = try std.heap.page_allocator.alloc(u8, 1500);
        @memset(data, 0);

        const interface_id = try interface.getInterfaceId(std.heap.page_allocator);
        defer std.heap.page_allocator.free(interface_id);

        return NetworkPacket.init(data, interface_id);
    }

    /// List all interfaces
    pub fn listInterfaces(self: NetworkDriver) []const NetworkInterface {
        return self.interfaces.items;
    }

    /// Get interface count
    pub fn interfaceCount(self: NetworkDriver) usize {
        return self.interfaces.items.len;
    }
};

test "network interface creation" {
    const mac = MACAddress.init([_]u8{ 0x00, 0x11, 0x22, 0x33, 0x44, 0x55 });
    const interface = NetworkInterface.init("eth0", .Ethernet, mac);

    try std.testing.expectEqualStrings("eth0", interface.name);
    try std.testing.expectEqual(InterfaceType.Ethernet, interface.interface_type);
    try std.testing.expect(!interface.initialized);
}

test "mac address formatting" {
    const mac = MACAddress.init([_]u8{ 0x00, 0x11, 0x22, 0x33, 0x44, 0x55 });
    const mac_str = try mac.format(std.testing.allocator);
    defer std.testing.allocator.free(mac_str);

    try std.testing.expectEqualStrings("00:11:22:33:44:55", mac_str);
}

test "interface initialization" {
    const mac = MACAddress.init([_]u8{ 0x00, 0x11, 0x22, 0x33, 0x44, 0x55 });
    var interface = NetworkInterface.init("eth0", .Ethernet, mac);

    try interface.initialize();
    try std.testing.expect(interface.initialized);
    try std.testing.expect(interface.link_up);
}

test "mtu setting" {
    const mac = MACAddress.init([_]u8{ 0x00, 0x11, 0x22, 0x33, 0x44, 0x55 });
    var interface = NetworkInterface.init("eth0", .Ethernet, mac);

    try interface.setMTU(9000);
    try std.testing.expectEqual(@as(u16, 9000), interface.getMTU());

    try std.testing.expectError(error.InvalidMTU, interface.setMTU(500));
}

test "network driver" {
    var driver = NetworkDriver.init(std.testing.allocator);
    defer driver.deinit();

    try driver.detectInterfaces();

    try std.testing.expectEqual(@as(usize, 2), driver.interfaceCount());
}
