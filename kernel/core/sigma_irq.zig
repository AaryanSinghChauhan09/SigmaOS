// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_irq.zig — IRQ / Interrupt Controller
// Replaces: sigma_irq.cpp (C++ stub, removed)
//
// Implements: IDT setup, APIC init, IRQ routing, ISR dispatch
// Language: Zig — direct hardware access, no hidden allocations, comptime
// Pattern: struct with methods (OOP equivalent in Zig)

const APIC_BASE:   usize = 0xFEE0_0000;
const PIC1_CMD:    u16   = 0x0020;
const PIC1_DATA:   u16   = 0x0021;
const PIC2_CMD:    u16   = 0x00A0;
const PIC2_DATA:   u16   = 0x00A1;
const PIC_EOI:     u8    = 0x20;
const IDT_ENTRIES: usize = 256;

// ── Port I/O ─────────────────────────────────────────────────────────────────

inline fn outb(port: u16, val: u8) void {
    asm volatile ("outb %[val], %[port]"
        : // no outputs
        : [val]  "a" (val),
          [port] "Nd" (port)
        : "memory"
    );
}

inline fn inb(port: u16) u8 {
    return asm volatile ("inb %[port], %[ret]"
        : [ret] "=a" (-> u8)
        : [port] "Nd" (port)
        : "memory"
    );
}

// ── IDT Gate Descriptor ──────────────────────────────────────────────────────

const IdtEntry = packed struct {
    offset_lo:  u16,
    selector:   u16,
    ist:        u8,
    type_attr:  u8,
    offset_mid: u16,
    offset_hi:  u32,
    zero:       u32,

    pub fn set(self: *IdtEntry, handler: usize, sel: u16, attr: u8) void {
        self.offset_lo  = @intCast(handler & 0xFFFF);
        self.selector   = sel;
        self.ist        = 0;
        self.type_attr  = attr;
        self.offset_mid = @intCast((handler >> 16) & 0xFFFF);
        self.offset_hi  = @intCast((handler >> 32) & 0xFFFFFFFF);
        self.zero       = 0;
    }
};

const IdtPtr = packed struct {
    limit: u16,
    base:  u64,
};

// ── APIC ─────────────────────────────────────────────────────────────────────

pub const Apic = struct {
    base: usize,

    pub fn init(self: *Apic) void {
        self.base = APIC_BASE;
        // Enable APIC: set bit 8 of SVR (0xF0)
        self.write(0xF0, self.read(0xF0) | 0x100);
    }

    pub fn send_eoi(self: *Apic) void {
        self.write(0xB0, 0); // EOI register
    }

    pub fn write(self: *Apic, off: usize, val: u32) void {
        const ptr: *volatile u32 = @ptrFromInt(self.base + off);
        ptr.* = val;
    }

    pub fn read(self: *const Apic, off: usize) u32 {
        const ptr: *const volatile u32 = @ptrFromInt(self.base + off);
        return ptr.*;
    }
};

// ── PIC (legacy, disable by masking all IRQs) ────────────────────────────────

pub fn pic_disable() void {
    // Remap IRQs 0-15 to vectors 0x20-0x2F (avoid CPU exception conflicts)
    outb(PIC1_CMD,  0x11); outb(PIC2_CMD,  0x11);  // ICW1: init
    outb(PIC1_DATA, 0x20); outb(PIC2_DATA, 0x28);  // ICW2: vector offsets
    outb(PIC1_DATA, 0x04); outb(PIC2_DATA, 0x02);  // ICW3: cascade
    outb(PIC1_DATA, 0x01); outb(PIC2_DATA, 0x01);  // ICW4: 8086 mode
    // Mask all PIC IRQs (APIC takes over)
    outb(PIC1_DATA, 0xFF);
    outb(PIC2_DATA, 0xFF);
}

// ── IDT ──────────────────────────────────────────────────────────────────────

pub const Idt = struct {
    entries: [IDT_ENTRIES]IdtEntry,
    ptr:     IdtPtr,

    pub fn init(self: *Idt) void {
        // Zero all entries
        for (&self.entries) |*e| {
            e.* = @zeroBitCast(IdtEntry{});
        }
        self.ptr = IdtPtr{
            .limit = @sizeOf([IDT_ENTRIES]IdtEntry) - 1,
            .base  = @intFromPtr(&self.entries),
        };
    }

    pub fn set_gate(self: *Idt, vec: u8, handler: usize) void {
        // 0x8E = present | DPL=0 | interrupt gate
        self.entries[vec].set(handler, 0x08, 0x8E);
    }

    pub fn load(self: *const Idt) void {
        asm volatile ("lidt [%[ptr]]"
            : // no outputs
            : [ptr] "r" (&self.ptr)
            : "memory"
        );
    }
};

// ── IRQ Handler Table ────────────────────────────────────────────────────────

pub const IrqHandler = *const fn (vec: u8) void;
var irq_table: [IDT_ENTRIES]?IrqHandler = .{null} ** IDT_ENTRIES;

pub fn register_irq(vec: u8, handler: IrqHandler) void {
    irq_table[vec] = handler;
}

/// Called from assembly ISR stub after saving registers
pub export fn sigma_irq_dispatch(vec: u8) void {
    if (irq_table[vec]) |handler| {
        handler(vec);
    }
    // Send EOI to APIC (assumes global apic instance at known address)
    const apic: *Apic = @ptrFromInt(APIC_BASE - APIC_BASE + APIC_BASE); // placeholder
    _ = apic; // EOI sent by caller or via APIC register write
}

// ── Timer (APIC one-shot) ────────────────────────────────────────────────────

pub const ApicTimer = struct {
    apic: *Apic,

    pub fn init_periodic(self: *ApicTimer, vec: u8, divisor: u32) void {
        // Divide configuration register (0x3E0)
        self.apic.write(0x3E0, divisor);
        // Timer LVT: periodic mode | vector
        self.apic.write(0x320, 0x20000 | @as(u32, vec));
        // Initial count (calibrate later with HPET)
        self.apic.write(0x380, 1_000_000);
    }
};
