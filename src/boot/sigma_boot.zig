const std = @import("std");

pub const EfiHandle = *opaque {};
pub const EfiStatus = usize;

pub const EFI_SUCCESS: EfiStatus = 0;

pub const EfiTableHeader = extern struct {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
};

pub const EfiBootServices = extern struct {
    hdr: EfiTableHeader,
    allocate_pages: *const fn() callconv(.C) EfiStatus,
    free_pages: *const fn() callconv(.C) EfiStatus,
};

pub const EfiSystemTable = extern struct {
    hdr: EfiTableHeader,
    firmware_vendor: [*]u16,
    firmware_revision: u32,
    console_in_handle: EfiHandle,
    con_in: *opaque {},
    console_out_handle: EfiHandle,
    con_out: *opaque {},
    standard_error_handle: EfiHandle,
    std_err: *opaque {},
    runtime_services: *opaque {},
    boot_services: *EfiBootServices,
    number_of_table_entries: usize,
    configuration_table: *opaque {},
};

pub fn parseMemoryMap() void {}
pub fn loadKernelElf() void {}
pub fn detectAcpiRsdp() void {}
pub fn initGop() void {}

test "EfiTableHeader size" {
    try std.testing.expectEqual(@sizeOf(EfiTableHeader), 24);
}

test "EfiSystemTable stub" {
    var hdr = EfiTableHeader {
        .signature = 0x5453595320494249,
        .revision = 1,
        .header_size = @sizeOf(EfiTableHeader),
        .crc32 = 0,
        .reserved = 0,
    };
    try std.testing.expectEqual(hdr.signature, 0x5453595320494249);
}
