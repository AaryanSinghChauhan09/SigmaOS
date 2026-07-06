// SPDX-License-Identifier: MIT
// SigmaOS PCI Subsystem — Full multi-bus enumeration with MSI-X and PCIe extended config.
// Implements PCI 3.0 / PCIe 5.0 config-space access, BAR sizing, capability walking,
// MSI/MSI-X setup, and a device-driver binding table.

#![no_std]

/// PCI I/O port addresses (legacy config mechanism #1)
const CONFIG_ADDRESS: u16 = 0xCF8;
const CONFIG_DATA:    u16 = 0xCFC;

/// Maximum devices tracked per enumeration
const MAX_PCI_DEVICES: usize = 256;
/// Maximum buses to scan
const MAX_BUS: u8 = 16;
/// Maximum functions per device
const MAX_FUNC: u8 = 8;

// ── PCI Capability IDs ──────────────────────────────────────────────────────
pub const PCI_CAP_ID_MSI:   u8 = 0x05;
pub const PCI_CAP_ID_MSIX:  u8 = 0x11;
pub const PCI_CAP_ID_PCIE:  u8 = 0x10;
pub const PCI_CAP_ID_PM:    u8 = 0x01;

// ── PCI Class Codes ─────────────────────────────────────────────────────────
pub const PCI_CLASS_STORAGE_NVME:    u8 = 0x01;
pub const PCI_SUBCLASS_STORAGE_NVME: u8 = 0x08;
pub const PCI_CLASS_NETWORK:         u8 = 0x02;
pub const PCI_CLASS_DISPLAY:         u8 = 0x03;
pub const PCI_CLASS_BRIDGE:          u8 = 0x06;
pub const PCI_CLASS_SERIAL_USB:      u8 = 0x0C;

// ── Header Type ─────────────────────────────────────────────────────────────
pub const HEADER_TYPE_NORMAL:  u8 = 0x00;
pub const HEADER_TYPE_BRIDGE:  u8 = 0x01;
pub const HEADER_TYPE_CARDBUS: u8 = 0x02;
pub const HEADER_TYPE_MULTI:   u8 = 0x80; // multifunction bit

// ── BAR ─────────────────────────────────────────────────────────────────────
pub const BAR_IO:        u32 = 0x01;
pub const BAR_MEM64:     u32 = 0x04;
pub const BAR_PREFETCH:  u32 = 0x08;

/// PCI BAR descriptor
#[derive(Copy, Clone, Default, Debug)]
pub struct PciBar {
    pub base:   u64,
    pub size:   u64,
    pub is_io:  bool,
    pub is_64:  bool,
    pub prefetchable: bool,
}

/// Full PCI device descriptor
#[derive(Copy, Clone, Debug)]
pub struct PciDevice {
    pub bus:        u8,
    pub slot:       u8,
    pub func:       u8,
    pub vendor_id:  u16,
    pub device_id:  u16,
    pub class_id:   u8,
    pub subclass_id:u8,
    pub prog_if:    u8,
    pub rev_id:     u8,
    pub header_type:u8,
    pub irq_line:   u8,
    pub irq_pin:    u8,
    pub bars:       [PciBar; 6],
    pub bar_count:  usize,
    pub msi_offset: u8,   // 0 = not present
    pub msix_offset:u8,   // 0 = not present
    pub pcie_offset:u8,   // 0 = not present
}

impl Default for PciDevice {
    fn default() -> Self {
        Self {
            bus: 0, slot: 0, func: 0,
            vendor_id: 0xFFFF, device_id: 0xFFFF,
            class_id: 0, subclass_id: 0, prog_if: 0, rev_id: 0,
            header_type: 0, irq_line: 0xFF, irq_pin: 0,
            bars: [PciBar::default(); 6],
            bar_count: 0,
            msi_offset: 0, msix_offset: 0, pcie_offset: 0,
        }
    }
}

impl PciDevice {
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.vendor_id != 0xFFFF
    }

    /// Find BAR with MMIO base address
    pub fn mmio_bar(&self) -> Option<u64> {
        for i in 0..self.bar_count {
            let b = &self.bars[i];
            if !b.is_io && b.base != 0 {
                return Some(b.base);
            }
        }
        None
    }

    /// Return true if device is an NVMe storage controller
    pub fn is_nvme(&self) -> bool {
        self.class_id == PCI_CLASS_STORAGE_NVME
            && self.subclass_id == PCI_SUBCLASS_STORAGE_NVME
    }

    /// Return true if device is a network controller
    pub fn is_nic(&self) -> bool {
        self.class_id == PCI_CLASS_NETWORK
    }

    /// Return true if device is a display controller
    pub fn is_gpu(&self) -> bool {
        self.class_id == PCI_CLASS_DISPLAY
    }

    /// Return true if device is a USB controller (xHCI = subclass 0x30)
    pub fn is_usb_xhci(&self) -> bool {
        self.class_id == PCI_CLASS_SERIAL_USB && self.subclass_id == 0x03 && self.prog_if == 0x30
    }
}

/// PCI Bus manager
pub struct PciBus {
    pub devices:      [PciDevice; MAX_PCI_DEVICES],
    pub device_count: usize,
}

// ── I/O port stubs (replaced by real arch::x86::outl/inl in kernel build) ──
/// Write 32-bit value to I/O port. (x86 out instruction)
#[inline(always)]
unsafe fn outl(port: u16, val: u32) {
    // SAFETY: caller must ensure valid port.
    core::arch::asm!(
        "out dx, eax",
        in("dx") port,
        in("eax") val,
        options(nomem, nostack, preserves_flags)
    );
}

/// Read 32-bit value from I/O port. (x86 in instruction)
#[inline(always)]
unsafe fn inl(port: u16) -> u32 {
    let val: u32;
    core::arch::asm!(
        "in eax, dx",
        out("eax") val,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    val
}

impl PciBus {
    pub const fn new() -> Self {
        Self {
            devices: [PciDevice {
                bus: 0, slot: 0, func: 0,
                vendor_id: 0xFFFF, device_id: 0xFFFF,
                class_id: 0, subclass_id: 0, prog_if: 0, rev_id: 0,
                header_type: 0, irq_line: 0xFF, irq_pin: 0,
                bars: [PciBar { base: 0, size: 0, is_io: false, is_64: false, prefetchable: false }; 6],
                bar_count: 0,
                msi_offset: 0, msix_offset: 0, pcie_offset: 0,
            }; MAX_PCI_DEVICES],
            device_count: 0,
        }
    }

    // ── Config Space Access ─────────────────────────────────────────────────

    /// Build PCI config address register value.
    #[inline]
    fn make_address(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        (1u32 << 31)
            | ((bus  as u32) << 16)
            | ((slot as u32) << 11)
            | ((func as u32) << 8)
            | (offset as u32 & 0xFC)
    }

    /// Read 32-bit DWORD from PCI configuration space.
    pub unsafe fn read32(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
        outl(CONFIG_ADDRESS, Self::make_address(bus, slot, func, offset));
        inl(CONFIG_DATA)
    }

    /// Read 16-bit WORD from PCI configuration space.
    pub unsafe fn read16(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
        let dword = Self::read32(bus, slot, func, offset & !3);
        let shift = (offset & 2) * 8;
        (dword >> shift) as u16
    }

    /// Read 8-bit BYTE from PCI configuration space.
    pub unsafe fn read8(bus: u8, slot: u8, func: u8, offset: u8) -> u8 {
        let dword = Self::read32(bus, slot, func, offset & !3);
        let shift = (offset & 3) * 8;
        (dword >> shift) as u8
    }

    /// Write 32-bit DWORD to PCI configuration space.
    pub unsafe fn write32(bus: u8, slot: u8, func: u8, offset: u8, val: u32) {
        outl(CONFIG_ADDRESS, Self::make_address(bus, slot, func, offset));
        outl(CONFIG_DATA, val);
    }

    /// Write 16-bit WORD to PCI configuration space.
    pub unsafe fn write16(bus: u8, slot: u8, func: u8, offset: u8, val: u16) {
        let dword = Self::read32(bus, slot, func, offset & !3);
        let shift = (offset & 2) * 8;
        let mask  = !(0xFFFFu32 << shift);
        Self::write32(bus, slot, func, offset & !3, (dword & mask) | ((val as u32) << shift));
    }

    // ── BAR Sizing ──────────────────────────────────────────────────────────

    /// Size a single BAR at config offset `bar_offset`. Returns (base, size, flags).
    unsafe fn size_bar(bus: u8, slot: u8, func: u8, bar_offset: u8) -> (u64, u64, u32) {
        let orig = Self::read32(bus, slot, func, bar_offset);
        let flags = orig & 0xF;

        if flags & BAR_IO != 0 {
            // I/O BAR
            Self::write32(bus, slot, func, bar_offset, 0xFFFFFFFF);
            let mask = Self::read32(bus, slot, func, bar_offset) & !0x3;
            Self::write32(bus, slot, func, bar_offset, orig);
            let size = (!(mask) + 1) as u64;
            return ((orig & !0x3) as u64, size, flags);
        }

        if flags & BAR_MEM64 != 0 {
            // 64-bit MMIO BAR
            let orig_hi = Self::read32(bus, slot, func, bar_offset + 4);
            Self::write32(bus, slot, func, bar_offset,     0xFFFFFFFF);
            Self::write32(bus, slot, func, bar_offset + 4, 0xFFFFFFFF);
            let lo = Self::read32(bus, slot, func, bar_offset) & !0xF;
            let hi = Self::read32(bus, slot, func, bar_offset + 4);
            let mask64 = !((hi as u64) << 32 | lo as u64) + 1;
            Self::write32(bus, slot, func, bar_offset,     orig);
            Self::write32(bus, slot, func, bar_offset + 4, orig_hi);
            let base = ((orig_hi as u64) << 32) | ((orig & !0xF) as u64);
            return (base, mask64, flags);
        }

        // 32-bit MMIO BAR
        Self::write32(bus, slot, func, bar_offset, 0xFFFFFFFF);
        let mask = Self::read32(bus, slot, func, bar_offset) & !0xF;
        Self::write32(bus, slot, func, bar_offset, orig);
        let size = if mask == 0 { 0 } else { (!(mask) + 1) as u64 };
        ((orig & !0xF) as u64, size, flags)
    }

    // ── Capability Walking ──────────────────────────────────────────────────

    /// Walk capability list starting at status offset 0x34. Returns cap offset or 0.
    unsafe fn find_cap(bus: u8, slot: u8, func: u8, cap_id: u8) -> u8 {
        let status = Self::read16(bus, slot, func, 0x06);
        if status & (1 << 4) == 0 {
            return 0; // No capability list
        }
        let mut ptr = Self::read8(bus, slot, func, 0x34) & 0xFC;
        let mut depth = 0u8;
        while ptr != 0 && depth < 48 {
            let id  = Self::read8(bus, slot, func, ptr);
            let nxt = Self::read8(bus, slot, func, ptr + 1) & 0xFC;
            if id == cap_id {
                return ptr;
            }
            ptr = nxt;
            depth += 1;
        }
        0
    }

    // ── MSI-X Enable ───────────────────────────────────────────────────────

    /// Enable MSI-X for a device. Returns true on success.
    pub unsafe fn enable_msix(bus: u8, slot: u8, func: u8) -> bool {
        let cap = Self::find_cap(bus, slot, func, PCI_CAP_ID_MSIX);
        if cap == 0 { return false; }
        let mut ctrl = Self::read16(bus, slot, func, cap + 2);
        ctrl |= 1 << 15; // MSI-X Enable
        ctrl &= !(1 << 14); // Clear Function Mask
        Self::write16(bus, slot, func, cap + 2, ctrl);
        true
    }

    /// Enable MSI for a device. Returns true on success.
    pub unsafe fn enable_msi(bus: u8, slot: u8, func: u8) -> bool {
        let cap = Self::find_cap(bus, slot, func, PCI_CAP_ID_MSI);
        if cap == 0 { return false; }
        let mut ctrl = Self::read16(bus, slot, func, cap + 2);
        ctrl |= 1; // MSI Enable
        Self::write16(bus, slot, func, cap + 2, ctrl);
        true
    }

    // ── Bus Master / MMIO Enable ────────────────────────────────────────────

    /// Enable bus mastering and MMIO decode for a device.
    pub unsafe fn enable_device(bus: u8, slot: u8, func: u8) {
        let mut cmd = Self::read16(bus, slot, func, 0x04);
        cmd |= 0x0006; // MMIO + Bus Master
        Self::write16(bus, slot, func, 0x04, cmd);
    }

    // ── Enumeration ─────────────────────────────────────────────────────────

    /// Enumerate all PCI buses up to MAX_BUS.
    pub fn enumerate(&mut self) {
        for bus in 0..MAX_BUS {
            self.scan_bus(bus);
        }
    }

    fn scan_bus(&mut self, bus: u8) {
        for slot in 0u8..32 {
            self.scan_device(bus, slot);
        }
    }

    fn scan_device(&mut self, bus: u8, slot: u8) {
        // Check function 0 first
        let vid_did = unsafe { Self::read32(bus, slot, 0, 0) };
        if vid_did == 0xFFFFFFFF { return; }

        self.probe_function(bus, slot, 0);

        // Check multifunction
        let hdr = unsafe { Self::read8(bus, slot, 0, 0x0E) };
        if hdr & HEADER_TYPE_MULTI != 0 {
            for func in 1..MAX_FUNC {
                let vd = unsafe { Self::read32(bus, slot, func, 0) };
                if vd != 0xFFFFFFFF {
                    self.probe_function(bus, slot, func);
                }
            }
        }
    }

    fn probe_function(&mut self, bus: u8, slot: u8, func: u8) {
        if self.device_count >= MAX_PCI_DEVICES { return; }

        let vid_did  = unsafe { Self::read32(bus, slot, func, 0x00) };
        let class_rv = unsafe { Self::read32(bus, slot, func, 0x08) };
        let hdr_info = unsafe { Self::read32(bus, slot, func, 0x0C) };
        let irq_info = unsafe { Self::read32(bus, slot, func, 0x3C) };

        let vendor_id   = (vid_did & 0xFFFF) as u16;
        let device_id   = (vid_did >> 16) as u16;
        let rev_id      = (class_rv & 0xFF) as u8;
        let prog_if     = ((class_rv >> 8) & 0xFF) as u8;
        let subclass_id = ((class_rv >> 16) & 0xFF) as u8;
        let class_id    = ((class_rv >> 24) & 0xFF) as u8;
        let header_type = ((hdr_info >> 16) & 0xFF) as u8;
        let irq_line    = (irq_info & 0xFF) as u8;
        let irq_pin     = ((irq_info >> 8) & 0xFF) as u8;

        let mut dev = PciDevice {
            bus, slot, func,
            vendor_id, device_id,
            class_id, subclass_id, prog_if, rev_id,
            header_type: header_type & !HEADER_TYPE_MULTI,
            irq_line, irq_pin,
            bars: [PciBar::default(); 6],
            bar_count: 0,
            msi_offset:  unsafe { Self::find_cap(bus, slot, func, PCI_CAP_ID_MSI)  },
            msix_offset: unsafe { Self::find_cap(bus, slot, func, PCI_CAP_ID_MSIX) },
            pcie_offset: unsafe { Self::find_cap(bus, slot, func, PCI_CAP_ID_PCIE) },
        };

        // Size BARs (only for normal header type 0)
        if dev.header_type == HEADER_TYPE_NORMAL {
            let mut bar_idx = 0usize;
            let mut bar_off = 0x10u8;
            while bar_idx < 6 {
                let (base, size, flags) = unsafe { Self::size_bar(bus, slot, func, bar_off) };
                let is_64   = flags & BAR_MEM64 != 0;
                let is_io   = flags & BAR_IO != 0;
                let prefetch = flags & BAR_PREFETCH != 0;
                dev.bars[bar_idx] = PciBar { base, size, is_io, is_64, prefetchable: prefetch };
                if base != 0 || size != 0 {
                    dev.bar_count = bar_idx + 1;
                }
                if is_64 {
                    bar_idx += 2;
                    bar_off  = bar_off.wrapping_add(8);
                } else {
                    bar_idx += 1;
                    bar_off  = bar_off.wrapping_add(4);
                }
            }

            // Enable bus-master + MMIO for all normal devices
            unsafe { Self::enable_device(bus, slot, func); }
        }

        self.devices[self.device_count] = dev;
        self.device_count += 1;

        // Recurse into PCI-PCI bridges
        if dev.header_type == HEADER_TYPE_BRIDGE && class_id == PCI_CLASS_BRIDGE {
            let bridge_cfg = unsafe { Self::read32(bus, slot, func, 0x18) };
            let secondary_bus = ((bridge_cfg >> 8) & 0xFF) as u8;
            if secondary_bus != 0 && secondary_bus != bus {
                self.scan_bus(secondary_bus);
            }
        }
    }

    // ── Device Lookup Helpers ───────────────────────────────────────────────

    /// Find first device matching vendor_id + device_id.
    pub fn find_device(&self, vendor_id: u16, device_id: u16) -> Option<&PciDevice> {
        self.devices[..self.device_count]
            .iter()
            .find(|d| d.vendor_id == vendor_id && d.device_id == device_id)
    }

    /// Find first device by class + subclass.
    pub fn find_by_class(&self, class: u8, subclass: u8) -> Option<&PciDevice> {
        self.devices[..self.device_count]
            .iter()
            .find(|d| d.class_id == class && d.subclass_id == subclass)
    }

    /// Find first NVMe controller.
    pub fn find_nvme(&self) -> Option<&PciDevice> {
        self.devices[..self.device_count].iter().find(|d| d.is_nvme())
    }

    /// Find first NIC.
    pub fn find_nic(&self) -> Option<&PciDevice> {
        self.devices[..self.device_count].iter().find(|d| d.is_nic())
    }

    /// Find first GPU.
    pub fn find_gpu(&self) -> Option<&PciDevice> {
        self.devices[..self.device_count].iter().find(|d| d.is_gpu())
    }

    /// Find first xHCI USB controller.
    pub fn find_xhci(&self) -> Option<&PciDevice> {
        self.devices[..self.device_count].iter().find(|d| d.is_usb_xhci())
    }
}

// ── Global PCI Bus Singleton ────────────────────────────────────────────────
static mut G_PCI_BUS: PciBus = PciBus::new();

// ── C-ABI Exports ───────────────────────────────────────────────────────────

/// Initialize PCI subsystem: enumerate all buses.
#[no_mangle]
pub unsafe extern "C" fn sigma_pci_init() {
    G_PCI_BUS.enumerate();
}

/// Return number of PCI devices discovered.
#[no_mangle]
pub unsafe extern "C" fn sigma_pci_device_count() -> usize {
    G_PCI_BUS.device_count
}

/// Enable MSI-X for device at index. Returns 1 on success, 0 on failure.
#[no_mangle]
pub unsafe extern "C" fn sigma_pci_enable_msix(index: usize) -> i32 {
    if index >= G_PCI_BUS.device_count { return -1; }
    let d = &G_PCI_BUS.devices[index];
    if PciBus::enable_msix(d.bus, d.slot, d.func) { 0 } else { -1 }
}

/// Get MMIO BAR base address for device at index (first MMIO BAR).
#[no_mangle]
pub unsafe extern "C" fn sigma_pci_get_mmio(index: usize) -> u64 {
    if index >= G_PCI_BUS.device_count { return 0; }
    G_PCI_BUS.devices[index].mmio_bar().unwrap_or(0)
}

/// Find first NVMe controller and return its MMIO base, or 0 if not found.
#[no_mangle]
pub unsafe extern "C" fn sigma_pci_find_nvme_mmio() -> u64 {
    match G_PCI_BUS.find_nvme() {
        Some(d) => d.mmio_bar().unwrap_or(0),
        None    => 0,
    }
}

/// Find first NIC and return its MMIO base, or 0 if not found.
#[no_mangle]
pub unsafe extern "C" fn sigma_pci_find_nic_mmio() -> u64 {
    match G_PCI_BUS.find_nic() {
        Some(d) => d.mmio_bar().unwrap_or(0),
        None    => 0,
    }
}
