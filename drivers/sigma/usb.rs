// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: NATIVE SOVEREIGN — USB (xHCI) Driver (Rust, no_std)
//! =========================================================================
//!
//! Replaces: drivers/sigma/sigma_usb.cpp
//! Language: Rust  #![no_std]  #![no_builtins]
//!
//! xHCI (USB 3.x) host controller driver.
//! ZERO standard library. ZERO predefined functions. ZERO external crates.
//! All TRB ring buffers, device slots, and endpoint contexts are
//! stack-resident — no heap, no alloc.
//!
//! Reference: eXtensible Host Controller Interface (xHCI) Specification 1.2
//!
//! Selected at build time with: TARGET_OS=sigma
//! =========================================================================

#![no_std]
#![no_builtins]
#![allow(dead_code)]

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)); } }
}

// ── Primitive types ───────────────────────────────────────────────────────
type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ═══════════════════════════════════════════════════════════════════════════
// § 1. xHCI Capability Register Offsets (xHCI 1.2 §5.3)
// ═══════════════════════════════════════════════════════════════════════════

const XHCI_CAP_CAPLENGTH : U32 = 0x00; // Capability Register Length (1 byte)
const XHCI_CAP_HCIVERSION: U32 = 0x02; // Host Controller Interface Version
const XHCI_CAP_HCSPARAMS1: U32 = 0x04; // Structural Parameters 1
const XHCI_CAP_HCSPARAMS2: U32 = 0x08; // Structural Parameters 2
const XHCI_CAP_HCSPARAMS3: U32 = 0x0C; // Structural Parameters 3
const XHCI_CAP_HCCPARAMS1: U32 = 0x10; // Capability Parameters 1
const XHCI_CAP_DBOFF     : U32 = 0x14; // Doorbell Offset
const XHCI_CAP_RTSOFF    : U32 = 0x18; // Runtime Register Space Offset

// ═══════════════════════════════════════════════════════════════════════════
// § 2. xHCI Operational Register Offsets (relative to cap_length)
// ═══════════════════════════════════════════════════════════════════════════

const XHCI_OP_USBCMD   : U32 = 0x00; // USB Command
const XHCI_OP_USBSTS   : U32 = 0x04; // USB Status
const XHCI_OP_PAGESIZE  : U32 = 0x08; // Page Size
const XHCI_OP_DNCTRL   : U32 = 0x14; // Device Notification Control
const XHCI_OP_CRCR_LO  : U32 = 0x18; // Command Ring Control (low 32)
const XHCI_OP_CRCR_HI  : U32 = 0x1C; // Command Ring Control (high 32)
const XHCI_OP_DCBAAP_LO: U32 = 0x30; // Device Context Base Addr Array Ptr (lo)
const XHCI_OP_DCBAAP_HI: U32 = 0x34; // Device Context Base Addr Array Ptr (hi)
const XHCI_OP_CONFIG    : U32 = 0x38; // Configure

// USBCMD bits
const XHCI_CMD_RUN    : U32 = 1 << 0;  // Run/Stop
const XHCI_CMD_HCRST  : U32 = 1 << 1;  // Host Controller Reset
const XHCI_CMD_INTE   : U32 = 1 << 2;  // Interrupter Enable
const XHCI_CMD_HSEE   : U32 = 1 << 3;  // Host System Error Enable

// USBSTS bits
const XHCI_STS_HCH    : U32 = 1 << 0;  // HC Halted
const XHCI_STS_CNR    : U32 = 1 << 11; // Controller Not Ready
const XHCI_STS_HCE    : U32 = 1 << 12; // Host Controller Error

const POLL_MAX: U32 = 500_000;

// ═══════════════════════════════════════════════════════════════════════════
// § 3. Transfer Request Block (TRB) — 16 bytes, xHCI 1.2 §4.11
// ═══════════════════════════════════════════════════════════════════════════

/// Generic Transfer Request Block — the universal work item in xHCI.
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct Trb {
    param_lo: U32, // Parameter (low 32 bits) / Data Buffer Pointer Lo
    param_hi: U32, // Parameter (high 32 bits) / Data Buffer Pointer Hi
    status  : U32, // Status / Transfer Length / Interrupter Target
    control : U32, // Cycle bit (0), TRB Type (15:10), and flags
}

const _: () = assert!(core::mem::size_of::<Trb>() == 16);

impl Trb {
    const fn zeroed() -> Self {
        Trb { param_lo: 0, param_hi: 0, status: 0, control: 0 }
    }
}

// TRB Type field values (bits 15:10 of control)
const TRB_TYPE_NORMAL       : U32 = 1  << 10;
const TRB_TYPE_SETUP_STAGE  : U32 = 2  << 10;
const TRB_TYPE_DATA_STAGE   : U32 = 3  << 10;
const TRB_TYPE_STATUS_STAGE : U32 = 4  << 10;
const TRB_TYPE_LINK         : U32 = 6  << 10;
const TRB_TYPE_ENABLE_SLOT  : U32 = 9  << 10;
const TRB_TYPE_DISABLE_SLOT : U32 = 10 << 10;
const TRB_TYPE_ADDR_DEVICE  : U32 = 11 << 10;
const TRB_TYPE_CMD_COMPLETE : U32 = 33 << 10;
const TRB_TYPE_PORT_STATUS  : U32 = 34 << 10;

// Command Ring size
const CMD_RING_SIZE: usize = 64;
// Event Ring size
const EVT_RING_SIZE: usize = 64;

// ═══════════════════════════════════════════════════════════════════════════
// § 4. Volatile MMIO helpers (self-contained)
// ═══════════════════════════════════════════════════════════════════════════

#[inline(always)]
unsafe fn read32(base: U64, off: U32) -> U32 {
    core::ptr::read_volatile((base + off as U64) as *const U32)
}

#[inline(always)]
unsafe fn write32(base: U64, off: U32, val: U32) {
    core::ptr::write_volatile((base + off as U64) as *mut U32, val);
}

#[inline]
unsafe fn poll32(base: U64, off: U32, mask: U32, expected: U32) -> bool {
    let mut i: U32 = 0;
    while i < POLL_MAX {
        if (read32(base, off) & mask) == expected { return true; }
        core::arch::asm!("pause", options(nomem, nostack, preserves_flags));
        i += 1;
    }
    false
}

// ═══════════════════════════════════════════════════════════════════════════
// § 5. SigmaUsb driver state — stack-resident, no heap
// ═══════════════════════════════════════════════════════════════════════════

/// Maximum number of USB devices the xHCI can track.
const MAX_USB_DEVICES: usize = 32;

/// xHCI Device Context Base Address Array — one u64 per slot.
#[repr(C, align(64))]
struct Dcbaa {
    entries: [U64; MAX_USB_DEVICES + 1], // slot 0 = scratchpad
}

impl Dcbaa {
    const fn new() -> Self {
        Dcbaa { entries: [0; MAX_USB_DEVICES + 1] }
    }
}

/// Command Ring — stack-resident TRB ring.
#[repr(C, align(64))]
struct CmdRing {
    trbs: [Trb; CMD_RING_SIZE],
    enqueue: usize,
    cycle: U32,
}

impl CmdRing {
    const fn new() -> Self {
        // Macro to create zeroed TRB array at const time
        macro_rules! trb_array {
            ($n:expr) => {{
                let mut arr = [Trb::zeroed(); $n];
                // Last TRB is a Link TRB — we set it up at init time
                arr
            }}
        }
        CmdRing {
            trbs: trb_array!(CMD_RING_SIZE),
            enqueue: 0,
            cycle: 1,
        }
    }
}

/// Full xHCI driver state.
pub struct SigmaUsb {
    mmio_base    : U64,
    op_base      : U64,      // Operational register base (mmio_base + cap_length)
    db_base      : U64,      // Doorbell register base
    rt_base      : U64,      // Runtime register base
    dcbaa        : Dcbaa,
    cmd_ring     : CmdRing,
    max_ports    : U32,
    max_slots    : U32,
    device_count : U32,
    initialized  : bool,
}

impl SigmaUsb {
    pub const fn new() -> Self {
        SigmaUsb {
            mmio_base   : 0,
            op_base     : 0,
            db_base     : 0,
            rt_base     : 0,
            dcbaa       : Dcbaa::new(),
            cmd_ring    : CmdRing::new(),
            max_ports   : 0,
            max_slots   : 0,
            device_count: 0,
            initialized : false,
        }
    }

    // ── init ───────────────────────────────────────────────────────────────

    /// Initialise the xHCI host controller.
    ///
    /// Sequence (xHCI 1.2 §4.2):
    ///   1. Read CAPLENGTH to locate Operational Registers
    ///   2. Read DBOFF / RTSOFF for doorbell / runtime offsets
    ///   3. Stop controller (USBCMD.RUN = 0), wait HCH = 1
    ///   4. Reset controller (USBCMD.HCRST = 1), wait CNR = 0
    ///   5. Program MaxSlotsEn, DCBAAP, CRCR
    ///   6. Start controller (USBCMD.RUN = 1)
    pub unsafe fn init(&mut self, mmio_base: U64) -> I32 {
        self.mmio_base = mmio_base;

        // Step 1: Read capability register length
        let cap_length = read32(mmio_base, XHCI_CAP_CAPLENGTH) & 0xFF;
        self.op_base = mmio_base + cap_length as U64;

        // Read structural parameters
        let hcsparams1 = read32(mmio_base, XHCI_CAP_HCSPARAMS1);
        self.max_slots = hcsparams1 & 0xFF;
        self.max_ports = (hcsparams1 >> 24) & 0xFF;

        // Step 2: Doorbell & Runtime offsets
        let dboff  = read32(mmio_base, XHCI_CAP_DBOFF) & !0x3;
        let rtsoff = read32(mmio_base, XHCI_CAP_RTSOFF) & !0x1F;
        self.db_base = mmio_base + dboff as U64;
        self.rt_base = mmio_base + rtsoff as U64;

        // Step 3: Stop controller
        let cmd = read32(self.op_base, XHCI_OP_USBCMD);
        write32(self.op_base, XHCI_OP_USBCMD, cmd & !XHCI_CMD_RUN);
        if !poll32(self.op_base, XHCI_OP_USBSTS, XHCI_STS_HCH, XHCI_STS_HCH) {
            return -4; // SIGMA_TIMEOUT
        }

        // Step 4: Reset
        write32(self.op_base, XHCI_OP_USBCMD, XHCI_CMD_HCRST);
        if !poll32(self.op_base, XHCI_OP_USBSTS, XHCI_STS_CNR, 0) {
            return -4;
        }
        // Wait for HCRST to self-clear
        if !poll32(self.op_base, XHCI_OP_USBCMD, XHCI_CMD_HCRST, 0) {
            return -4;
        }

        // Step 5a: Max Device Slots Enabled
        let max_en = if self.max_slots > MAX_USB_DEVICES as U32 {
            MAX_USB_DEVICES as U32
        } else {
            self.max_slots
        };
        write32(self.op_base, XHCI_OP_CONFIG, max_en);

        // Step 5b: DCBAAP
        let dcbaap_phys = self.dcbaa.entries.as_ptr() as U64;
        write32(self.op_base, XHCI_OP_DCBAAP_LO, dcbaap_phys as U32);
        write32(self.op_base, XHCI_OP_DCBAAP_HI, (dcbaap_phys >> 32) as U32);

        // Step 5c: Command Ring Control Register
        let crcr_phys = self.cmd_ring.trbs.as_ptr() as U64;
        // Cycle State (bit 0) = 1
        write32(self.op_base, XHCI_OP_CRCR_LO, (crcr_phys as U32) | 1);
        write32(self.op_base, XHCI_OP_CRCR_HI, (crcr_phys >> 32) as U32);

        // Step 5d: Set up Link TRB at end of command ring
        let last = CMD_RING_SIZE - 1;
        self.cmd_ring.trbs[last].param_lo = crcr_phys as U32;
        self.cmd_ring.trbs[last].param_hi = (crcr_phys >> 32) as U32;
        self.cmd_ring.trbs[last].control  = TRB_TYPE_LINK | 1; // Toggle Cycle

        // Step 6: Start controller
        write32(self.op_base, XHCI_OP_USBCMD,
                XHCI_CMD_RUN | XHCI_CMD_INTE | XHCI_CMD_HSEE);
        if !poll32(self.op_base, XHCI_OP_USBSTS, XHCI_STS_HCH, 0) {
            return -4;
        }

        // Check for HCE
        if (read32(self.op_base, XHCI_OP_USBSTS) & XHCI_STS_HCE) != 0 {
            return -1;
        }

        self.initialized = true;
        0
    }

    // ── Enable Slot command ────────────────────────────────────────────────

    /// Issue an Enable Slot command to allocate a device slot.
    /// Returns 0 on success (slot TRB queued), negative on error.
    pub unsafe fn enable_slot(&mut self) -> I32 {
        if !self.initialized { return -1; }

        let idx = self.cmd_ring.enqueue;
        if idx >= CMD_RING_SIZE - 1 { return -3; } // ring full (minus link TRB)

        let trb = &mut self.cmd_ring.trbs[idx];
        trb.param_lo = 0;
        trb.param_hi = 0;
        trb.status   = 0;
        trb.control  = TRB_TYPE_ENABLE_SLOT | self.cmd_ring.cycle;

        self.cmd_ring.enqueue = idx + 1;

        // Ring HC doorbell 0 (command ring) — value 0
        write32(self.db_base, 0, 0);

        self.device_count += 1;
        0
    }

    /// Return the number of USB devices enumerated.
    pub fn device_count(&self) -> U32 { self.device_count }
}

// ── Global singleton ──────────────────────────────────────────────────────
static mut G_USB: SigmaUsb = SigmaUsb::new();

// ── C bridge ──────────────────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sigma_usb_init(mmio_base: U64) -> I32 {
    G_USB.init(mmio_base)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_usb_enable_slot() -> I32 {
    G_USB.enable_slot()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_usb_device_count() -> U32 {
    G_USB.device_count()
}
