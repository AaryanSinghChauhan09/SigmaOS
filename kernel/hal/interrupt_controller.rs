/// SigmaOS: APIC + PIC Initialization
/// Phase G Blocker #5: APIC + PIC init
/// Migrated from C/C++ to Rust — no_std, no alloc, no external crates.


#[allow(dead_code)]

use core::arch::asm;

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── I/O Ports ─────────────────────────────────────────────────────────────

const PIC1_CMD: SigmaU16 = 0x20;
const PIC1_DATA: SigmaU16 = 0x21;
const PIC2_CMD: SigmaU16 = 0xA0;
const PIC2_DATA: SigmaU16 = 0xA1;

const ICW1_INIT: SigmaU8 = 0x10;
const ICW1_ICW4: SigmaU8 = 0x01;
const ICW4_8086: SigmaU8 = 0x01;

// ─── APIC Registers ─────────────────────────────────────────────────────────

const APIC_BASE: SigmaU64 = 0xFEE00000;
const APIC_ID: SigmaU64 = 0x020;
const APIC_VERSION: SigmaU64 = 0x030;
const APIC_TPR: SigmaU64 = 0x080;
const APIC_APR: SigmaU64 = 0x090;
const APIC_PPR: SigmaU64 = 0x0A0;
const APIC_EOI: SigmaU64 = 0x0B0;
const APIC_LDR: SigmaU64 = 0x0D0;
const APIC_DFR: SigmaU64 = 0x0E0;
const APIC_SVR: SigmaU64 = 0x0F0;
const APIC_ISR: SigmaU64 = 0x100;
const APIC_TMR: SigmaU64 = 0x180;
const APIC_IRR: SigmaU64 = 0x200;
const APIC_ERROR: SigmaU64 = 0x280;
const APIC_ICR: SigmaU64 = 0x300;
const APIC_LVT_TIMER: SigmaU64 = 0x320;
const APIC_LVT_THERMAL: SigmaU64 = 0x330;
const APIC_LVT_PERF: SigmaU64 = 0x340;
const APIC_LVT_LINT0: SigmaU64 = 0x350;
const APIC_LVT_LINT1: SigmaU64 = 0x360;
const APIC_LVT_ERROR: SigmaU64 = 0x370;
const APIC_TIMER_INITIAL: SigmaU64 = 0x380;
const APIC_TIMER_CURRENT: SigmaU64 = 0x390;
const APIC_TIMER_DIVIDE: SigmaU64 = 0x3E0;

// ─── APIC Flags ───────────────────────────────────────────────────────────

const APIC_SVR_ENABLE: SigmaU32 = 1 << 8;
const APIC_SVR_FOCUS: SigmaU32 = 1 << 9;
const APIC_EOI_ACK: SigmaU32 = 0;

// ─── Interrupt Controller State ───────────────────────────────────────────

pub struct InterruptController {
    pic_enabled: SigmaBool,
    apic_enabled: SigmaBool,
    apic_base: SigmaU64,
    apic_id: SigmaU32,
    apic_version: SigmaU32,
}

impl InterruptController {
    pub const fn new() -> Self {
        Self {
            pic_enabled: false,
            apic_enabled: false,
            apic_base: APIC_BASE,
            apic_id: 0,
            apic_version: 0,
        }
    }

    /// Initialize PIC (8259)
    pub unsafe fn init_pic(&mut self) -> Result<(), &'static str> {
        if self.pic_enabled {
            return Err("PIC already initialized");
        }

        // Save masks
        let pic1_mask = self.inb(PIC1_DATA);
        let pic2_mask = self.inb(PIC2_DATA);

        // Start initialization sequence
        self.outb(PIC1_CMD, ICW1_INIT | ICW1_ICW4);
        self.outb_delay();
        self.outb(PIC2_CMD, ICW1_INIT | ICW1_ICW4);
        self.outb_delay();

        // Set vector offsets
        self.outb(PIC1_DATA, 0x20); // Master PIC vector offset
        self.outb_delay();
        self.outb(PIC2_DATA, 0x28); // Slave PIC vector offset
        self.outb_delay();

        // Configure cascade
        self.outb(PIC1_DATA, 4); // Slave PIC at IRQ2
        self.outb_delay();
        self.outb(PIC2_DATA, 2); // Cascade identity
        self.outb_delay();

        // Set 8086 mode
        self.outb(PIC1_DATA, ICW4_8086);
        self.outb_delay();
        self.outb(PIC2_DATA, ICW4_8086);
        self.outb_delay();

        // Restore masks
        self.outb(PIC1_DATA, pic1_mask);
        self.outb(PIC2_DATA, pic2_mask);

        self.pic_enabled = true;

        Ok(())
    }

    /// Disable PIC
    pub unsafe fn disable_pic(&mut self) {
        if !self.pic_enabled {
            return;
        }

        // Mask all interrupts
        self.outb(PIC1_DATA, 0xFF);
        self.outb(PIC2_DATA, 0xFF);

        self.pic_enabled = false;
    }

    /// Initialize APIC
    pub unsafe fn init_apic(&mut self) -> Result<(), &'static str> {
        if self.apic_enabled {
            return Err("APIC already initialized");
        }

        // Read APIC ID and version
        self.apic_id = self.apic_read32(APIC_ID);
        self.apic_version = self.apic_read32(APIC_VERSION);

        // Enable APIC
        let svr = self.apic_read32(APIC_SVR);
        self.apic_write32(APIC_SVR, svr | APIC_SVR_ENABLE);

        // Set task priority to 0
        self.apic_write32(APIC_TPR, 0);

        // Configure LVT entries
        self.apic_write32(APIC_LVT_TIMER, 0x10000); // Mask timer
        self.apic_write32(APIC_LVT_THERMAL, 0x10000); // Mask thermal
        self.apic_write32(APIC_LVT_PERF, 0x10000); // Mask performance
        self.apic_write32(APIC_LVT_LINT0, 0x10000); // Mask LINT0
        self.apic_write32(APIC_LVT_LINT1, 0x10000); // Mask LINT1
        self.apic_write32(APIC_LVT_ERROR, 0x10000); // Mask error

        // Set DFR to flat model
        self.apic_write32(APIC_DFR, 0xFFFFFFFF);

        // Set LDR to all ones
        self.apic_write32(APIC_LDR, 0xFFFFFFFF);

        self.apic_enabled = true;

        Ok(())
    }

    /// Disable APIC
    pub unsafe fn disable_apic(&mut self) {
        if !self.apic_enabled {
            return;
        }

        // Disable APIC
        let svr = self.apic_read32(APIC_SVR);
        self.apic_write32(APIC_SVR, svr & !APIC_SVR_ENABLE);

        self.apic_enabled = false;
    }

    /// Send End of Interrupt
    pub unsafe fn send_eoi(&mut self) {
        if self.apic_enabled {
            self.apic_write32(APIC_EOI, APIC_EOI_ACK);
        } else if self.pic_enabled {
            self.outb(PIC1_CMD, 0x20);
            self.outb(PIC2_CMD, 0x20);
        }
    }

    /// Read APIC register
    unsafe fn apic_read32(&self, offset: SigmaU64) -> SigmaU32 {
        let addr = self.apic_base + offset;
        *(addr as *const SigmaU32)
    }

    /// Write APIC register
    unsafe fn apic_write32(&self, offset: SigmaU64, value: SigmaU32) {
        let addr = self.apic_base + offset;
        *(addr as *mut SigmaU32) = value;
    }

    /// Read byte from I/O port
    unsafe fn inb(&self, port: SigmaU16) -> SigmaU8 {
        let mut value: SigmaU8 = 0;
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack));
        value
    }

    /// Write byte to I/O port
    unsafe fn outb(&self, port: SigmaU16, value: SigmaU8) {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack));
    }

    /// I/O delay
    unsafe fn outb_delay(&self) {
        self.outb(0x80, 0);
    }

    /// Get APIC ID
    pub unsafe fn get_apic_id(&mut self) -> SigmaU32 {
        self.apic_id
    }

    /// Get APIC version
    pub unsafe fn get_apic_version(&mut self) -> SigmaU32 {
        self.apic_version
    }

    /// Check if PIC is enabled
    pub unsafe fn is_pic_enabled(&self) -> SigmaBool {
        self.pic_enabled
    }

    /// Check if APIC is enabled
    pub unsafe fn is_apic_enabled(&self) -> SigmaBool {
        self.apic_enabled
    }
}

// ─── Global Interrupt Controller Instance ─────────────────────────────────

static mut INTERRUPT_CONTROLLER: InterruptController = InterruptController::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_pic_init() -> SigmaI32 {
    match INTERRUPT_CONTROLLER.init_pic() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_pic_disable() {
    INTERRUPT_CONTROLLER.disable_pic();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_apic_init() -> SigmaI32 {
    match INTERRUPT_CONTROLLER.init_apic() {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_apic_disable() {
    INTERRUPT_CONTROLLER.disable_apic();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_send_eoi() {
    INTERRUPT_CONTROLLER.send_eoi();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_get_apic_id() -> SigmaU32 {
    INTERRUPT_CONTROLLER.get_apic_id()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_get_apic_version() -> SigmaU32 {
    INTERRUPT_CONTROLLER.get_apic_version()
}
