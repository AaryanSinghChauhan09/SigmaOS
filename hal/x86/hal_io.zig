// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: x86 HAL I/O Port Access (Zig, no stdlib, no libc)
//! Replaces: hal/x86/ C files
//! =========================================================================

/// Read a byte from a hardware I/O port
pub inline fn inb(port: u16) u8 {
    return asm volatile ("inb %[port], %[result]"
        : [result] "={al}" (-> u8),
        : [port] "N{dx}" (port),
    );
}

/// Write a byte to a hardware I/O port
pub inline fn outb(port: u16, value: u8) void {
    asm volatile ("outb %[value], %[port]"
        :
        : [port] "N{dx}" (port),
          [value] "{al}" (value),
    );
}

/// Read a 16-bit word from a hardware I/O port
pub inline fn inw(port: u16) u16 {
    return asm volatile ("inw %[port], %[result]"
        : [result] "={ax}" (-> u16),
        : [port] "N{dx}" (port),
    );
}

/// Write a 16-bit word to a hardware I/O port
pub inline fn outw(port: u16, value: u16) void {
    asm volatile ("outw %[value], %[port]"
        :
        : [port] "N{dx}" (port),
          [value] "{ax}" (value),
    );
}

/// Read a 32-bit dword from a hardware I/O port
pub inline fn inl(port: u16) u32 {
    return asm volatile ("inl %[port], %[result]"
        : [result] "={eax}" (-> u32),
        : [port] "N{dx}" (port),
    );
}

/// Write a 32-bit dword to a hardware I/O port
pub inline fn outl(port: u16, value: u32) void {
    asm volatile ("outl %[value], %[port]"
        :
        : [port] "N{dx}" (port),
          [value] "{eax}" (value),
    );
}

/// MMIO: read a u32 from a memory-mapped register address
pub inline fn mmio_read32(address: usize) u32 {
    const ptr: *volatile u32 = @intToPtr(*volatile u32, address);
    return ptr.*;
}

/// MMIO: write a u32 to a memory-mapped register address
pub inline fn mmio_write32(address: usize, value: u32) void {
    const ptr: *volatile u32 = @intToPtr(*volatile u32, address);
    ptr.* = value;
}

/// HAL I/O Controller — OOP struct encapsulating port access
pub const HalIO = struct {
    base_port: u16,

    pub fn new(base: u16) HalIO {
        return HalIO{ .base_port = base };
    }

    pub fn read_byte(self: *const HalIO, offset: u16) u8 {
        return inb(self.base_port + offset);
    }

    pub fn write_byte(self: *const HalIO, offset: u16, value: u8) void {
        outb(self.base_port + offset, value);
    }

    pub fn read_dword(self: *const HalIO, offset: u16) u32 {
        return inl(self.base_port + offset);
    }

    pub fn write_dword(self: *const HalIO, offset: u16, value: u32) void {
        outl(self.base_port + offset, value);
    }
};
