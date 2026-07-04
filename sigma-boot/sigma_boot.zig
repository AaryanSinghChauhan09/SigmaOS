// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// sigma-boot/sigma_boot.zig — UEFI EFI Boot Stub
// Replaces: sigma_boot.c (C stub, removed)
//
// Language: Zig — direct UEFI access, no libc, no hidden allocations
// Purpose: Load kernel ELF, set up memory map, jump to kernel_main
// Pattern: struct-based UEFI protocol wrappers (OOP equivalent)

const uefi = @import("std").os.uefi;

// ── UEFI Types ────────────────────────────────────────────────────────────────

const EfiStatus = u64;
const EFI_SUCCESS:          EfiStatus = 0;
const EFI_LOAD_ERROR:       EfiStatus = 0x8000000000000001;
const EFI_NOT_FOUND:        EfiStatus = 0x800000000000000E;
const EFI_BUFFER_TOO_SMALL: EfiStatus = 0x8000000000000005;

const EFI_MEMORY_TYPE_LOADER_DATA: u32 = 2;

/// UEFI memory descriptor
const MemoryDescriptor = extern struct {
    type_:          u32,
    physical_start: u64,
    virtual_start:  u64,
    num_pages:      u64,
    attribute:      u64,
};

/// Information passed from boot stub to kernel
pub const BootInfo = extern struct {
    magic:          u64 = 0x5369676D61_424F4F54, // "SigmaBOOT"
    memory_map:     u64,   // physical address of EFI memory map
    memory_map_sz:  usize,
    desc_sz:        usize,
    rsdp_addr:      u64,   // ACPI RSDP physical address
    framebuffer:    u64,   // GOP framebuffer physical address
    fb_width:       u32,
    fb_height:      u32,
    fb_stride:      u32,
    kernel_phys:    u64,
    kernel_virt:    u64,
    kernel_sz:      u64,
    initramfs_phys: u64,
    initramfs_sz:   u64,
};

// ── EFI Entry Point ───────────────────────────────────────────────────────────

pub fn efi_main(
    image_handle: uefi.Handle,
    system_table: *uefi.tables.SystemTable,
) callconv(.C) EfiStatus {
    const bs = system_table.boot_services orelse return EFI_LOAD_ERROR;
    const con_out = system_table.con_out  orelse return EFI_LOAD_ERROR;

    // Clear screen + print banner
    _ = con_out.clearScreen();
    _ = con_out.outputString(std.unicode.utf8ToUtf16LeStringLiteral(
        "\r\nSigmaOS Boot v15.0\r\n"));

    // Locate GOP (Graphics Output Protocol)
    var gop: ?*uefi.protocol.GraphicsOutput = null;
    const gop_guid = uefi.protocol.GraphicsOutput.guid;
    _ = bs.locateProtocol(&gop_guid, null, @ptrCast(&gop));

    var boot_info = BootInfo{};
    if (gop) |g| {
        boot_info.framebuffer = g.mode.frame_buffer_base;
        boot_info.fb_width    = g.mode.info.horizontal_resolution;
        boot_info.fb_height   = g.mode.info.vertical_resolution;
        boot_info.fb_stride   = g.mode.info.pixels_per_scan_line;
    }

    // Load kernel ELF from ESP /boot/sigma-kernel.elf
    var kernel_phys: u64 = 0;
    var kernel_sz:   u64 = 0;
    const load_status = load_kernel(bs, image_handle,
        &kernel_phys, &kernel_sz);
    if (load_status != EFI_SUCCESS) return load_status;

    boot_info.kernel_phys = kernel_phys;
    boot_info.kernel_sz   = kernel_sz;

    // Get memory map and exit boot services
    var map_sz:   usize = 0;
    var map_key:  usize = 0;
    var desc_sz:  usize = 0;
    var desc_ver: u32   = 0;
    var map_buf: [0x8000]u8 = undefined;

    map_sz = map_buf.len;
    _ = bs.getMemoryMap(&map_sz, @ptrCast(&map_buf), &map_key, &desc_sz, &desc_ver);
    _ = bs.exitBootServices(image_handle, map_key);

    boot_info.memory_map    = @intFromPtr(&map_buf);
    boot_info.memory_map_sz = map_sz;
    boot_info.desc_sz       = desc_sz;

    // Jump to kernel entry point (first 8 bytes of loaded ELF = entry addr)
    const entry_addr = kernel_phys;
    const kernel_main: *const fn (*const BootInfo) callconv(.C) noreturn =
        @ptrFromInt(entry_addr);
    kernel_main(&boot_info);
}

// ── Kernel ELF Loader ────────────────────────────────────────────────────────

fn load_kernel(
    bs:           *uefi.tables.BootServices,
    image_handle: uefi.Handle,
    out_phys:     *u64,
    out_sz:       *u64,
) EfiStatus {
    // Locate the loaded image protocol to get device handle
    var loaded_image: ?*uefi.protocol.LoadedImage = null;
    const li_guid = uefi.protocol.LoadedImage.guid;
    _ = bs.openProtocol(image_handle, &li_guid,
        @ptrCast(&loaded_image), image_handle, null,
        uefi.protocol.OpenProtocolAttributes{ .get_protocol = true });
    const device = (loaded_image orelse return EFI_LOAD_ERROR).device_handle;

    // Open simple filesystem on boot device
    var sfs: ?*uefi.protocol.SimpleFileSystem = null;
    const sfs_guid = uefi.protocol.SimpleFileSystem.guid;
    _ = bs.openProtocol(device, &sfs_guid, @ptrCast(&sfs),
        image_handle, null,
        uefi.protocol.OpenProtocolAttributes{ .get_protocol = true });

    var root_dir: ?*uefi.protocol.File = null;
    _ = (sfs orelse return EFI_NOT_FOUND).openVolume(@ptrCast(&root_dir));

    var kernel_file: ?*uefi.protocol.File = null;
    const path = std.unicode.utf8ToUtf16LeStringLiteral(
        "\\boot\\sigma-kernel.elf");
    _ = (root_dir orelse return EFI_NOT_FOUND).open(
        @ptrCast(&kernel_file), path,
        uefi.protocol.File.efi_file_mode_read, 0);

    // Get file size via getInfo
    var file_info_buf: [512]u8 align(8) = undefined;
    var info_sz: usize = file_info_buf.len;
    const fi_guid = uefi.FileInfo.guid;
    _ = (kernel_file orelse return EFI_NOT_FOUND).getInfo(
        &fi_guid, &info_sz, &file_info_buf);
    const file_info: *const uefi.FileInfo = @ptrCast(&file_info_buf);
    const file_size = file_info.file_size;

    // Allocate pages for kernel
    var phys: u64 = 0x200000; // load at 2 MB
    _ = bs.allocatePages(.AllocateAddress, .LoaderData,
        (file_size + 4095) / 4096, &phys);

    // Read kernel into memory
    var read_sz = file_size;
    _ = (kernel_file.?).read(&read_sz, @ptrFromInt(phys));

    out_phys.* = phys;
    out_sz.*   = file_size;
    return EFI_SUCCESS;
}

const std = @import("std");
