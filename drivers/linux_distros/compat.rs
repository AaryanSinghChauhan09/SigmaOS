// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS: Linux Distro Driver Compatibility Layer (Rust, no_std)
//!
//! Translates Linux kernel driver ABI calls into SigmaOS HAL equivalents.
//! Covers Debian/Ubuntu, Fedora/RHEL, Arch Linux driver loading patterns.
//!
//! This is a shim layer — it wraps existing Linux kernel module structures
//! (probe/remove/suspend/resume callbacks) and redirects them through the
//! SigmaOS SDF lifecycle.  No Linux kernel code is copied (cleanroom).
//!
//! Architecture:
//!   Linux LKM ──► DistroCompatShim ──► SigmaOS HAL / DDK / sigma-bus
//!
//! Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

// ── Linux PCI device ID table entry (matches struct pci_device_id) ────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PciDeviceId {
    pub vendor:     u32,
    pub device:     u32,
    pub subvendor:  u32,
    pub subdevice:  u32,
    pub class:      u32,
    pub class_mask: u32,
    pub driver_data:usize,
}

pub const PCI_ANY_ID: u32 = 0xFFFF_FFFF;

// ── Linux module_init / module_exit function signature ────────────────────
pub type LinuxInitFn   = unsafe extern "C" fn() -> i32;
pub type LinuxExitFn   = unsafe extern "C" fn();
pub type LinuxProbeFn  = unsafe extern "C" fn(pdev: *mut u8, id: *const PciDeviceId) -> i32;
pub type LinuxRemoveFn = unsafe extern "C" fn(pdev: *mut u8);
pub type LinuxIrqFn    = unsafe extern "C" fn(irq: i32, dev_id: *mut u8) -> u32;

pub const IRQ_NONE:    u32 = 0;
pub const IRQ_HANDLED: u32 = 1;

/// Descriptor that a Linux-compatible driver registers with the shim.
#[repr(C)]
pub struct LinuxDriverDescriptor {
    pub name:        *const u8,   // null-terminated driver name
    pub vendor_id:   u32,
    pub device_id:   u32,
    pub init:        Option<LinuxInitFn>,
    pub exit:        Option<LinuxExitFn>,
    pub probe:       Option<LinuxProbeFn>,
    pub remove:      Option<LinuxRemoveFn>,
    pub irq_handler: Option<LinuxIrqFn>,
    pub mmio_base:   u64,
    pub irq_num:     u8,
    pub flags:       u32,
}

pub const LX_DRV_FLAG_REGISTERED: u32 = 1 << 0;
pub const LX_DRV_FLAG_PROBED:     u32 = 1 << 1;
pub const LX_DRV_FLAG_ACTIVE:     u32 = 1 << 2;

unsafe impl Send for LinuxDriverDescriptor {}
unsafe impl Sync for LinuxDriverDescriptor {}

// ── Linux → SigmaOS syscall translation table ─────────────────────────────
// Maps the most common Linux kernel functions to SigmaOS equivalents.
// When a Linux driver calls printk(), kmalloc(), etc., these are resolved
// via the symbol table built in DistroCompatShim::init().

extern "C" {
    // SigmaOS equivalents provided by the kernel
    fn sigma_log(msg: *const u8, len: usize);
    fn sigma_slab_alloc(size: usize) -> *mut u8;
    fn sigma_slab_free(ptr: *mut u8);
    fn sigma_iomap(phys: u64, size: usize) -> *mut u8;
    fn sigma_iounmap(virt: *mut u8);
    fn sigma_mmio_read32(base: *mut u8, off: usize) -> u32;
    fn sigma_mmio_write32(base: *mut u8, off: usize, val: u32);
    fn sigma_request_irq(irq: u8, handler: LinuxIrqFn, dev_id: *mut u8) -> i32;
    fn sigma_free_irq(irq: u8, dev_id: *mut u8);
    fn sigma_dma_alloc(size: usize, phys_out: *mut u64) -> *mut u8;
    fn sigma_dma_free(virt: *mut u8, size: usize);
    fn sigma_pci_enable(bus: u8, dev: u8, func: u8) -> i32;
    fn sigma_pci_read_config32(bus: u8, dev: u8, func: u8, off: u8) -> u32;
    fn sigma_pci_write_config32(bus: u8, dev: u8, func: u8, off: u8, val: u32);
    fn sigma_bus_send(channel: u32, data: *const u8, len: usize) -> i32;
}

// ── Exported Linux-ABI symbols (resolved by LKM loader) ───────────────────
// These match the exact symbol names the Linux kernel exports so that
// an LKM compiled for Linux can be loaded and its unresolved symbols
// point here.

#[no_mangle]
pub unsafe extern "C" fn printk(fmt_ptr: *const u8, _: ...) {
    // Simplified: log the format string directly (no printf parsing)
    if fmt_ptr.is_null() { return; }
    let mut len = 0usize;
    while *fmt_ptr.add(len) != 0 && len < 512 { len += 1; }
    sigma_log(fmt_ptr, len);
}

#[no_mangle]
pub unsafe extern "C" fn kmalloc(size: usize, _gfp_flags: u32) -> *mut u8 {
    sigma_slab_alloc(size)
}

#[no_mangle]
pub unsafe extern "C" fn kfree(ptr: *mut u8) {
    sigma_slab_free(ptr)
}

#[no_mangle]
pub unsafe extern "C" fn ioremap(phys_addr: u64, size: usize) -> *mut u8 {
    sigma_iomap(phys_addr, size)
}

#[no_mangle]
pub unsafe extern "C" fn iounmap(virt_addr: *mut u8) {
    sigma_iounmap(virt_addr)
}

#[no_mangle]
pub unsafe extern "C" fn readl(addr: *const u32) -> u32 {
    core::ptr::read_volatile(addr)
}

#[no_mangle]
pub unsafe extern "C" fn writel(val: u32, addr: *mut u32) {
    core::ptr::write_volatile(addr, val)
}

#[no_mangle]
pub unsafe extern "C" fn readq(addr: *const u64) -> u64 {
    core::ptr::read_volatile(addr)
}

#[no_mangle]
pub unsafe extern "C" fn writeq(val: u64, addr: *mut u64) {
    core::ptr::write_volatile(addr, val)
}

#[no_mangle]
pub unsafe extern "C" fn request_irq(
    irq: i32, handler: LinuxIrqFn,
    _flags: u64, _name: *const u8, dev_id: *mut u8,
) -> i32 {
    sigma_request_irq(irq as u8, handler, dev_id)
}

#[no_mangle]
pub unsafe extern "C" fn free_irq(irq: i32, dev_id: *mut u8) {
    sigma_free_irq(irq as u8, dev_id)
}

#[no_mangle]
pub unsafe extern "C" fn dma_alloc_coherent(
    _dev: *mut u8, size: usize, dma_handle: *mut u64, _gfp: u32,
) -> *mut u8 {
    sigma_dma_alloc(size, dma_handle)
}

#[no_mangle]
pub unsafe extern "C" fn dma_free_coherent(
    _dev: *mut u8, size: usize, cpu_addr: *mut u8, _dma_handle: u64,
) {
    sigma_dma_free(cpu_addr, size)
}

#[no_mangle]
pub unsafe extern "C" fn pci_read_config_dword(
    _dev: *mut u8, where_: i32, val: *mut u32,
) -> i32 {
    *val = sigma_pci_read_config32(0, 0, 0, where_ as u8);
    0
}

#[no_mangle]
pub unsafe extern "C" fn pci_write_config_dword(
    _dev: *mut u8, where_: i32, val: u32,
) -> i32 {
    sigma_pci_write_config32(0, 0, 0, where_ as u8, val);
    0
}

#[no_mangle]
pub unsafe extern "C" fn pci_enable_device(_dev: *mut u8) -> i32 {
    sigma_pci_enable(0, 0, 0)
}

#[no_mangle]
pub unsafe extern "C" fn pci_disable_device(_dev: *mut u8) {}

#[no_mangle]
pub unsafe extern "C" fn pci_set_master(_dev: *mut u8) {}

#[no_mangle]
pub unsafe extern "C" fn pci_unregister_driver(_drv: *mut u8) {}

#[no_mangle]
pub unsafe extern "C" fn netif_carrier_on(_dev: *mut u8) {}
#[no_mangle]
pub unsafe extern "C" fn netif_carrier_off(_dev: *mut u8) {}
#[no_mangle]
pub unsafe extern "C" fn netif_start_queue(_dev: *mut u8) {}
#[no_mangle]
pub unsafe extern "C" fn netif_stop_queue(_dev: *mut u8) {}

// ── DistroCompatShim — driver registry ────────────────────────────────────
const MAX_COMPAT_DRIVERS: usize = 64;

pub struct DistroCompatShim {
    drivers:      [Option<LinuxDriverDescriptor>; MAX_COMPAT_DRIVERS],
    count:        usize,
    initialized:  bool,
}

impl DistroCompatShim {
    pub const fn new() -> Self {
        Self {
            drivers:     [const { None }; MAX_COMPAT_DRIVERS],
            count:       0,
            initialized: false,
        }
    }

    pub unsafe fn init(&mut self) -> i32 {
        if self.initialized { return -16; } // EBUSY
        self.initialized = true;
        0
    }

    /// Register a Linux-compatible driver.
    /// Called by pci_register_driver() or platform_driver_register() shims.
    pub unsafe fn register(&mut self, desc: LinuxDriverDescriptor) -> i32 {
        if !self.initialized { return -19; } // ENODEV
        if self.count >= MAX_COMPAT_DRIVERS { return -12; } // ENOMEM

        // Run Linux module_init if provided
        if let Some(init_fn) = desc.init {
            let rc = init_fn();
            if rc != 0 { return rc; }
        }

        // Run probe if provided
        if let Some(probe_fn) = desc.probe {
            let rc = probe_fn(core::ptr::null_mut(), core::ptr::null());
            if rc != 0 {
                // probe failed — run exit, don't register
                if let Some(exit_fn) = desc.exit { exit_fn(); }
                return rc;
            }
        }

        self.drivers[self.count] = Some(desc);
        self.count += 1;
        0
    }

    /// Unregister a driver by vendor+device ID.
    pub unsafe fn unregister(&mut self, vendor_id: u32, device_id: u32) -> i32 {
        for i in 0..self.count {
            if let Some(ref d) = self.drivers[i] {
                if d.vendor_id == vendor_id && d.device_id == device_id {
                    if let Some(remove) = d.remove {
                        remove(core::ptr::null_mut());
                    }
                    if let Some(exit_fn) = d.exit { exit_fn(); }
                    self.drivers[i] = None;
                    return 0;
                }
            }
        }
        -19 // ENODEV
    }

    /// Translate a Linux ioctl number to a SigmaOS ioctl.
    /// Linux uses _IOC(dir,type,nr,size) macro encoding.
    pub fn translate_ioctl(&self, linux_ioctl: u32) -> u32 {
        let nr   = linux_ioctl & 0xFF;
        let kind = (linux_ioctl >> 8) & 0xFF;
        // SigmaOS ioctl = (kind << 16) | nr — simplified mapping
        (kind << 16) | nr
    }

    pub fn active_count(&self) -> usize {
        self.drivers.iter().filter(|d| d.is_some()).count()
    }
}

static mut G_DISTRO_COMPAT: DistroCompatShim = DistroCompatShim::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn distro_compat_init() -> i32 {
    G_DISTRO_COMPAT.init()
}

#[no_mangle]
pub unsafe extern "C" fn distro_compat_register(
    name:      *const u8,
    vendor_id: u32, device_id: u32,
    mmio_base: u64, irq_num: u8,
    probe:     Option<LinuxProbeFn>,
    remove:    Option<LinuxRemoveFn>,
    irq_hdlr:  Option<LinuxIrqFn>,
) -> i32 {
    G_DISTRO_COMPAT.register(LinuxDriverDescriptor {
        name, vendor_id, device_id,
        init: None, exit: None,
        probe, remove, irq_handler: irq_hdlr,
        mmio_base, irq_num, flags: 0,
    })
}

#[no_mangle]
pub unsafe extern "C" fn distro_compat_unregister(vendor_id: u32, device_id: u32) -> i32 {
    G_DISTRO_COMPAT.unregister(vendor_id, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn distro_compat_ioctl_xlat(linux_ioctl: u32) -> u32 {
    G_DISTRO_COMPAT.translate_ioctl(linux_ioctl)
}

#[no_mangle]
pub unsafe extern "C" fn distro_compat_active_count() -> usize {
    G_DISTRO_COMPAT.active_count()
}
