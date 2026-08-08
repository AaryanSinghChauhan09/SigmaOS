/// SigmaOS: I/O Module
/// Placeholder for I/O operations

#[allow(dead_code)]

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaBool = bool;
type SigmaUsize = usize;

/// Read byte from I/O port
pub unsafe fn inb(port: SigmaU16) -> SigmaU8 {
    let value: SigmaU8;
    core::arch::asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack)
    );
    value
}

/// Write byte to I/O port
pub unsafe fn outb(port: SigmaU16, value: SigmaU8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack)
    );
}

/// Read word from I/O port
pub unsafe fn inw(port: SigmaU16) -> SigmaU16 {
    let value: SigmaU16;
    core::arch::asm!(
        "in ax, dx",
        out("ax") value,
        in("dx") port,
        options(nomem, nostack)
    );
    value
}

/// Write word to I/O port
pub unsafe fn outw(port: SigmaU16, value: SigmaU16) {
    core::arch::asm!(
        "out dx, ax",
        in("dx") port,
        in("ax") value,
        options(nomem, nostack)
    );
}

/// Read double word from I/O port
pub unsafe fn inl(port: SigmaU16) -> SigmaU32 {
    let value: SigmaU32;
    core::arch::asm!(
        "in eax, dx",
        out("eax") value,
        in("dx") port,
        options(nomem, nostack)
    );
    value
}

/// Write double word to I/O port
pub unsafe fn outl(port: SigmaU16, value: SigmaU32) {
    core::arch::asm!(
        "out dx, eax",
        in("dx") port,
        in("eax") value,
        options(nomem, nostack)
    );
}