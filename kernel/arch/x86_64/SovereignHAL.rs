/// SigmaOS Sovereign Hardware Abstraction Layer (HAL)
/// Migrated from C++ to Rust — no_std, no alloc, no external crates.
/// Provides silicon-direct port I/O, MMIO, and platform reset.

#![no_std]
#![allow(dead_code)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;

// ─── Error Type ─────────────────────────────────────────────────────────────

/// Errors that can occur during HAL operations.
#[derive(Copy, Clone)]
enum HalError {
    /// The device is not responding.
    DeviceNotResponding,
    /// An invalid port or address was specified.
    InvalidAddress,
    /// The operation timed out.
    Timeout,
}

// ─── HalDevice Trait (OOP Interface) ────────────────────────────────────────

/// Trait that all HAL-managed devices must implement.
/// This is the OOP interface replacing the C++ virtual class hierarchy.
trait HalDevice {
    /// Initialize the device. Returns Ok(()) on success.
    fn init(&mut self) -> Result<(), HalError>;
    /// Return the human-readable device type name.
    fn type_name(&self) -> &'static str;
}

// ─── Port I/O ───────────────────────────────────────────────────────────────

/// Low-level x86 port I/O operations via inline assembly.
/// These replace the C++ outb/inb methods on SovereignHAL.
struct PortIO;

impl PortIO {
    /// Write a byte to an I/O port.
    #[inline(always)]
    unsafe fn outb(port: SigmaU16, val: SigmaU8) {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(nomem, nostack, preserves_flags),
        );
    }

    /// Read a byte from an I/O port.
    #[inline(always)]
    unsafe fn inb(port: SigmaU16) -> SigmaU8 {
        let val: SigmaU8;
        core::arch::asm!(
            "in al, dx",
            out("al") val,
            in("dx") port,
            options(nomem, nostack, preserves_flags),
        );
        val
    }

    /// Write a 16-bit word to an I/O port.
    #[inline(always)]
    unsafe fn outw(port: SigmaU16, val: SigmaU16) {
        core::arch::asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") val,
            options(nomem, nostack, preserves_flags),
        );
    }

    /// Read a 16-bit word from an I/O port.
    #[inline(always)]
    unsafe fn inw(port: SigmaU16) -> SigmaU16 {
        let val: SigmaU16;
        core::arch::asm!(
            "in ax, dx",
            out("ax") val,
            in("dx") port,
            options(nomem, nostack, preserves_flags),
        );
        val
    }

    /// Write a 32-bit dword to an I/O port.
    #[inline(always)]
    unsafe fn outl(port: SigmaU16, val: SigmaU32) {
        core::arch::asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") val,
            options(nomem, nostack, preserves_flags),
        );
    }

    /// Read a 32-bit dword from an I/O port.
    #[inline(always)]
    unsafe fn inl(port: SigmaU16) -> SigmaU32 {
        let val: SigmaU32;
        core::arch::asm!(
            "in eax, dx",
            out("eax") val,
            in("dx") port,
            options(nomem, nostack, preserves_flags),
        );
        val
    }
}

// ─── MMIO ───────────────────────────────────────────────────────────────────

/// Memory-Mapped I/O operations.
struct Mmio;

impl Mmio {
    /// Read a 32-bit value from a memory-mapped register.
    #[inline(always)]
    unsafe fn read32(addr: SigmaU64) -> SigmaU32 {
        core::ptr::read_volatile(addr as *const SigmaU32)
    }

    /// Write a 32-bit value to a memory-mapped register.
    #[inline(always)]
    unsafe fn write32(addr: SigmaU64, val: SigmaU32) {
        core::ptr::write_volatile(addr as *mut SigmaU32, val);
    }

    /// Read a 64-bit value from a memory-mapped register.
    #[inline(always)]
    unsafe fn read64(addr: SigmaU64) -> SigmaU64 {
        core::ptr::read_volatile(addr as *const SigmaU64)
    }

    /// Write a 64-bit value to a memory-mapped register.
    #[inline(always)]
    unsafe fn write64(addr: SigmaU64, val: SigmaU64) {
        core::ptr::write_volatile(addr as *mut SigmaU64, val);
    }
}

// ─── Serial Port ────────────────────────────────────────────────────────────

/// COM1 serial port constants and initialization.
struct SerialPort;
impl SerialPort {
    const COM1: SigmaU16 = 0x3F8;
}

/// Initialize the COM1 serial port for debug output.
unsafe fn serial_init() {
    PortIO::outb(SerialPort::COM1 + 1, 0x00); // Disable interrupts
    PortIO::outb(SerialPort::COM1 + 3, 0x80); // Enable DLAB
    PortIO::outb(SerialPort::COM1 + 0, 0x01); // Divisor low (115200 baud)
    PortIO::outb(SerialPort::COM1 + 1, 0x00); // Divisor high
    PortIO::outb(SerialPort::COM1 + 3, 0x03); // 8N1
    PortIO::outb(SerialPort::COM1 + 2, 0xC7); // Enable FIFO
    PortIO::outb(SerialPort::COM1 + 4, 0x0B); // RTS/DSR set
}

// ─── SovereignHAL ───────────────────────────────────────────────────────────

/// The Sovereign Hardware Abstraction Layer.
/// Orchestrates hardware shard initialization and provides platform primitives.
struct SovereignHAL {
    initialized: bool,
}

impl SovereignHAL {
    const fn new() -> Self {
        Self { initialized: false }
    }

    /// Initialize the HAL — sets up serial I/O and marks the HAL as online.
    unsafe fn init(&mut self) {
        serial_init();
        self.initialized = true;
    }

    /// Perform a platform reset via the PS/2 keyboard controller.
    /// Writes 0xFE to port 0x64 which triggers a CPU reset.
    unsafe fn reboot(&self) {
        PortIO::outb(0x64, 0xFE);
    }
}

impl HalDevice for SovereignHAL {
    fn init(&mut self) -> Result<(), HalError> {
        unsafe { serial_init(); }
        self.initialized = true;
        Ok(())
    }

    fn type_name(&self) -> &'static str {
        "SovereignHAL"
    }
}

// ─── Global Singleton ───────────────────────────────────────────────────────

static mut HAL: SovereignHAL = SovereignHAL::new();

// ─── C-ABI Bridge ───────────────────────────────────────────────────────────

/// Initialize the Hardware Abstraction Layer.
/// Replaces the original `extern "C" void hal_init()`.
#[no_mangle]
pub unsafe extern "C" fn hal_init() {
    HAL.init();
}

/// Perform a system reboot.
/// Replaces the original `extern "C" void hal_reboot()`.
#[no_mangle]
pub unsafe extern "C" fn hal_reboot() {
    HAL.reboot();
}
