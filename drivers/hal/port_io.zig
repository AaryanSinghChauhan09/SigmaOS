// SPDX-License-Identifier: GPL-2.0-or-later
//
// =========================================================================
// SIGMAOS: x86-64 PORT I/O PRIMITIVES (Zig — freestanding, no std)
// =========================================================================
//
// Language: Zig (freestanding)
//
// ZERO standard library imports. ZERO predefined functions.
// All I/O port instructions implemented via Zig inline assembly directly.
// No @import("std"), no @import("os"), no builtin helper functions.
//
// Compile with:
//   zig build-obj -target x86_64-freestanding-none -O ReleaseFast \
//                 drivers/hal/port_io.zig -femit-bin=build/port_io.o
// =========================================================================

// No imports — this file is entirely self-contained.

// ═══════════════════════════════════════════════════════════════════════════
// § 1. 8-bit port I/O
// ═══════════════════════════════════════════════════════════════════════════

/// Read one byte from an x86-64 I/O port.
/// Uses `inb` instruction via Zig inline assembly — no library wrapper.
pub fn inb(port: u16) u8 {
    return asm volatile ("inb %[port], %[ret]"
        : [ret] "={al}" (-> u8)
        : [port] "N{dx}" (port)
        : "memory"
    );
}

/// Write one byte to an x86-64 I/O port.
pub fn outb(port: u16, val: u8) void {
    asm volatile ("outb %[val], %[port]"
        :
        : [val]  "{al}"   (val),
          [port] "N{dx}"  (port)
        : "memory"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// § 2. 16-bit port I/O
// ═══════════════════════════════════════════════════════════════════════════

/// Read one 16-bit word from an x86-64 I/O port.
pub fn inw(port: u16) u16 {
    return asm volatile ("inw %[port], %[ret]"
        : [ret] "={ax}" (-> u16)
        : [port] "N{dx}" (port)
        : "memory"
    );
}

/// Write one 16-bit word to an x86-64 I/O port.
pub fn outw(port: u16, val: u16) void {
    asm volatile ("outw %[val], %[port]"
        :
        : [val]  "{ax}"   (val),
          [port] "N{dx}"  (port)
        : "memory"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// § 3. 32-bit port I/O
// ═══════════════════════════════════════════════════════════════════════════

/// Read one 32-bit dword from an x86-64 I/O port.
pub fn inl(port: u16) u32 {
    return asm volatile ("inl %[port], %[ret]"
        : [ret] "={eax}" (-> u32)
        : [port] "N{dx}" (port)
        : "memory"
    );
}

/// Write one 32-bit dword to an x86-64 I/O port.
pub fn outl(port: u16, val: u32) void {
    asm volatile ("outl %[val], %[port]"
        :
        : [val]  "{eax}"  (val),
          [port] "N{dx}"  (port)
        : "memory"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// § 4. CPU control instructions
// ═══════════════════════════════════════════════════════════════════════════

/// Disable hardware interrupts (CLI).
pub fn cli() void {
    asm volatile ("cli" ::: "memory");
}

/// Enable hardware interrupts (STI).
pub fn sti() void {
    asm volatile ("sti" ::: "memory");
}

/// Halt the CPU until the next interrupt (HLT).
pub fn hlt() void {
    asm volatile ("hlt" ::: "memory");
}

/// CPU pause hint — reduces power in spin-wait loops.
pub fn pause() void {
    asm volatile ("pause" ::: "memory");
}

/// Full memory fence (MFENCE).
pub fn mfence() void {
    asm volatile ("mfence" ::: "memory");
}

/// Store fence (SFENCE).
pub fn sfence() void {
    asm volatile ("sfence" ::: "memory");
}

/// Load fence (LFENCE).
pub fn lfence() void {
    asm volatile ("lfence" ::: "memory");
}

// ═══════════════════════════════════════════════════════════════════════════
// § 5. MSR (Model-Specific Register) read/write
// ═══════════════════════════════════════════════════════════════════════════

/// Read a 64-bit Model-Specific Register.
/// `ecx` = MSR address.  Returns EDX:EAX concatenated as u64.
pub fn rdmsr(msr: u32) u64 {
    var lo: u32 = undefined;
    var hi: u32 = undefined;
    asm volatile ("rdmsr"
        : [lo] "={eax}" (lo),
          [hi] "={edx}" (hi)
        : [msr] "{ecx}" (msr)
        : "memory"
    );
    return (@as(u64, hi) << 32) | @as(u64, lo);
}

/// Write a 64-bit Model-Specific Register.
pub fn wrmsr(msr: u32, val: u64) void {
    const lo: u32 = @truncate(val);
    const hi: u32 = @truncate(val >> 32);
    asm volatile ("wrmsr"
        :
        : [lo]  "{eax}" (lo),
          [hi]  "{edx}" (hi),
          [msr] "{ecx}" (msr)
        : "memory"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// § 6. CPUID
// ═══════════════════════════════════════════════════════════════════════════

/// CPUID result — four 32-bit general-purpose registers.
pub const CpuidResult = struct {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
};

/// Execute the CPUID instruction with leaf `leaf` and sub-leaf `subleaf`.
pub fn cpuid(leaf: u32, subleaf: u32) CpuidResult {
    var eax: u32 = undefined;
    var ebx: u32 = undefined;
    var ecx: u32 = undefined;
    var edx: u32 = undefined;
    asm volatile ("cpuid"
        : [eax] "={eax}" (eax),
          [ebx] "={ebx}" (ebx),
          [ecx] "={ecx}" (ecx),
          [edx] "={edx}" (edx)
        : [leaf]    "{eax}" (leaf),
          [subleaf] "{ecx}" (subleaf)
        : "memory"
    );
    return CpuidResult{ .eax = eax, .ebx = ebx, .ecx = ecx, .edx = edx };
}

// ═══════════════════════════════════════════════════════════════════════════
// § 7. CR register access
// ═══════════════════════════════════════════════════════════════════════════

/// Read CR0 (control register 0).
pub fn readCr0() u64 {
    return asm volatile ("mov %%cr0, %[ret]"
        : [ret] "=r" (-> u64)
        :
        : "memory"
    );
}

/// Write CR0.
pub fn writeCr0(val: u64) void {
    asm volatile ("mov %[val], %%cr0"
        :
        : [val] "r" (val)
        : "memory"
    );
}

/// Read CR3 (page table base register).
pub fn readCr3() u64 {
    return asm volatile ("mov %%cr3, %[ret]"
        : [ret] "=r" (-> u64)
        :
        : "memory"
    );
}

/// Write CR3 (triggers TLB flush).
pub fn writeCr3(val: u64) void {
    asm volatile ("mov %[val], %%cr3"
        :
        : [val] "r" (val)
        : "memory"
    );
}
