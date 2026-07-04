// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// drivers/hal/sigma_pci.zig — PCI/PCIe Bus Enumeration + MSI-X
// Language: Zig — port I/O, comptime config space layout, no libc

// ── PCI Config Space Access (PIO method) ─────────────────────────────────────
const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA:    u16 = 0xCFC;

pub fn config_address(bus: u8, dev: u5, func: u3, off: u8) u32 {
    return 0x8000_0000
        | (@as(u32, bus)  << 16)
        | (@as(u32, dev)  << 11)
        | (@as(u32, func) <<  8)
        | (@as(u32, off & 0xFC));
}

fn outl(port: u16, val: u32) void {
    asm volatile ("outl %[val], %[port]"
        : : [val] "a" (val), [port] "Nd" (port) : "memory");
}
fn inl(port: u16) u32 {
    return asm volatile ("inl %[port], %[ret]"
        : [ret] "=a" (-> u32)
        : [port] "Nd" (port) : "memory");
}

pub fn pci_read32(bus: u8, dev: u5, func: u3, off: u8) u32 {
    outl(CONFIG_ADDRESS, config_address(bus, dev, func, off));
    return inl(CONFIG_DATA);
}
pub fn pci_read16(bus: u8, dev: u5, func: u3, off: u8) u16 {
    const v = pci_read32(bus, dev, func, off);
    return @truncate((v >> @as(u5, @truncate((off & 2) * 8))));
}
pub fn pci_read8(bus: u8, dev: u5, func: u3, off: u8) u8 {
    const v = pci_read32(bus, dev, func, off);
    return @truncate((v >> @as(u5, @truncate((off & 3) * 8))));
}
pub fn pci_write32(bus: u8, dev: u5, func: u3, off: u8, val: u32) void {
    outl(CONFIG_ADDRESS, config_address(bus, dev, func, off));
    outl(CONFIG_DATA, val);
}

// ── PCI Device Descriptor ─────────────────────────────────────────────────────
pub const PciDevice = struct {
    bus:      u8,
    dev:      u5,
    func:     u3,
    vendor:   u16,
    device:   u16,
    class:    u8,
    subclass: u8,
    prog_if:  u8,
    rev:      u8,
    bar:      [6]u32,
    irq_line: u8,
    irq_pin:  u8,
    header:   u8,

    pub fn read(bus: u8, dev: u5, func: u3) ?PciDevice {
        const vid = pci_read16(bus, dev, func, 0x00);
        if (vid == 0xFFFF) return null;
        const did = pci_read16(bus, dev, func, 0x02);
        const class_raw = pci_read32(bus, dev, func, 0x08);
        var d = PciDevice{
            .bus = bus, .dev = dev, .func = func,
            .vendor   = vid,  .device = did,
            .rev      = @truncate(class_raw),
            .prog_if  = @truncate(class_raw >> 8),
            .subclass = @truncate(class_raw >> 16),
            .class    = @truncate(class_raw >> 24),
            .bar      = .{0,0,0,0,0,0},
            .irq_line = pci_read8(bus, dev, func, 0x3C),
            .irq_pin  = pci_read8(bus, dev, func, 0x3D),
            .header   = pci_read8(bus, dev, func, 0x0E) & 0x7F,
        };
        if (d.header == 0) { // type 0 = endpoint
            for (0..6) |i| {
                d.bar[i] = pci_read32(bus, dev, func, @intCast(0x10 + i * 4));
            }
        }
        return d;
    }

    /// Return the memory-mapped BAR base address (mask off type bits)
    pub fn bar_addr(self: *const PciDevice, idx: usize) u64 {
        const b = self.bar[idx];
        if (b & 1 != 0) return 0; // I/O BAR, not supported here
        const is_64bit = (b >> 1) & 3 == 2;
        var addr: u64 = b & 0xFFFF_FFF0;
        if (is_64bit and idx + 1 < 6) {
            addr |= @as(u64, self.bar[idx + 1]) << 32;
        }
        return addr;
    }

    /// Enable bus mastering (needed for DMA)
    pub fn enable_bus_master(self: *const PciDevice) void {
        const cmd = pci_read16(self.bus, self.dev, self.func, 0x04);
        pci_write32(self.bus, self.dev, self.func, 0x04,
                    @as(u32, cmd) | 0x04);
    }

    /// Enable memory space
    pub fn enable_mem(self: *const PciDevice) void {
        const cmd = pci_read16(self.bus, self.dev, self.func, 0x04);
        pci_write32(self.bus, self.dev, self.func, 0x04,
                    @as(u32, cmd) | 0x02);
    }
};

// ── PCI Bus Scanner ───────────────────────────────────────────────────────────
pub const MAX_PCI_DEVICES: usize = 64;

pub const PciBus = struct {
    devices: [MAX_PCI_DEVICES]?PciDevice,
    count:   usize,

    pub fn scan() PciBus {
        var bus = PciBus{ .devices = .{null} ** MAX_PCI_DEVICES, .count = 0 };
        for (0..256) |b| {
            for (0..32) |d| {
                for (0..8) |f| {
                    if (bus.count >= MAX_PCI_DEVICES) break;
                    if (PciDevice.read(@intCast(b), @intCast(d), @intCast(f))) |dev| {
                        bus.devices[bus.count] = dev;
                        bus.count += 1;
                        // If multi-function (bit 7 of header type), scan all funcs
                        if (f == 0 and pci_read8(@intCast(b), @intCast(d), 0, 0x0E) & 0x80 == 0) break;
                    } else if (f == 0) break; // no device at func 0 → skip other funcs
                }
            }
        }
        return bus;
    }

    pub fn find_by_class(self: *const PciBus, class: u8, subclass: u8) ?*const PciDevice {
        for (self.devices[0..self.count]) |*opt| {
            if (opt.*) |*dev| {
                if (dev.class == class and dev.subclass == subclass) return dev;
            }
        }
        return null;
    }

    pub fn find_by_id(self: *const PciBus, vendor: u16, device: u16) ?*const PciDevice {
        for (self.devices[0..self.count]) |*opt| {
            if (opt.*) |*dev| {
                if (dev.vendor == vendor and dev.device == device) return dev;
            }
        }
        return null;
    }
};

// ── MSI-X Setup ───────────────────────────────────────────────────────────────
pub fn setup_msix(dev: *const PciDevice, vector: u8, dest_cpu: u8) bool {
    // Find MSI-X capability (cap ID = 0x11)
    var cap_ptr = pci_read8(dev.bus, dev.dev, dev.func, 0x34) & 0xFC;
    while (cap_ptr != 0) {
        const cap_id = pci_read8(dev.bus, dev.dev, dev.func, cap_ptr);
        if (cap_id == 0x11) { // MSI-X
            const msg_ctrl = pci_read16(dev.bus, dev.dev, dev.func, cap_ptr + 2);
            const tbl_off_bir = pci_read32(dev.bus, dev.dev, dev.func, cap_ptr + 4);
            const bir = tbl_off_bir & 0x7;
            const tbl_offset = tbl_off_bir & ~@as(u32, 0x7);
            const tbl_base = dev.bar_addr(bir) + tbl_offset;
            // Write first table entry: address + data
            const addr_lo: u64 = 0xFEE0_0000 | (@as(u64, dest_cpu) << 12);
            const ptr_lo: *volatile u32 = @ptrFromInt(tbl_base);
            const ptr_hi: *volatile u32 = @ptrFromInt(tbl_base + 4);
            const ptr_data: *volatile u32 = @ptrFromInt(tbl_base + 8);
            const ptr_ctrl: *volatile u32 = @ptrFromInt(tbl_base + 12);
            ptr_lo.* = @truncate(addr_lo);
            ptr_hi.* = @truncate(addr_lo >> 32);
            ptr_data.* = vector;
            ptr_ctrl.* = 0; // unmask
            // Enable MSI-X
            const new_ctrl: u16 = (msg_ctrl | 0x8000) & ~@as(u16, 0x4000);
            pci_write32(dev.bus, dev.dev, dev.func, cap_ptr + 2, new_ctrl);
            return true;
        }
        cap_ptr = pci_read8(dev.bus, dev.dev, dev.func, cap_ptr + 1) & 0xFC;
    }
    return false;
}
