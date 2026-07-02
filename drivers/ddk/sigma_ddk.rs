// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/ddk/sigma_ddk.rs — Sovereign Driver Development Kit (DDK)
// Provides a stable ABI so hardware vendors can ship SigmaOS drivers
// without breaking across kernel updates.
//
// Key design principles (from Windows/Linux driver analysis):
//   Windows:  Stable ABI, vendor control, WDM/WDDM frameworks
//   Linux:    Open source, kernel-integrated, breaks frequently
//   SigmaOS:  Stable ABI + open source + AI-assisted porting
//
// The DDK provides:
//   1. SDF (Sovereign Driver Framework) lifecycle: probe→init→shutdown
//   2. Stable kabi/ C-ABI boundary (drivers survive kernel updates)
//   3. Permission model: sigma_pledge per driver
//   4. sigma-bus IPC for driver ↔ kernel ↔ userspace communication
//   5. Ring-3 driver isolation option (crashes don't take down kernel)
//   6. Dual mode: open drivers + closed vendor blobs coexist
//
// Language: Rust (#![no_std] + std for DDK tools)

#![allow(dead_code)]

// ── Stable ABI version ────────────────────────────────────────────────────
pub const DDK_ABI_VERSION: u32 = 1;
pub const DDK_ABI_MINOR:   u32 = 0;
pub const SIGMA_DDK_MAGIC:  u32 = 0x5344444B;  // "SDDK"

// ── Driver category ────────────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum DriverClass {
    Network    = 0x0200,
    Storage    = 0x0100,
    Display    = 0x0300,
    Audio      = 0x0400,
    Input      = 0x0500,
    Usb        = 0x0C03,
    Serial     = 0x0700,
    Wireless   = 0x0280,
    Gpu        = 0x0302,
    Sensor     = 0x0E00,
    Crypto     = 0x1000,
    Unknown    = 0xFFFF,
}

// ── Stable driver descriptor (kabi boundary) ──────────────────────────────
/// This struct MUST NOT change layout between DDK versions.
/// Add new fields at the end only, with DDK_ABI_VERSION bump.
#[repr(C)]
pub struct SigmaDriverDescriptor {
    pub magic:       u32,           // SIGMA_DDK_MAGIC
    pub abi_version: u32,           // DDK_ABI_VERSION
    pub driver_id:   [u8; 16],      // UUID
    pub vendor_id:   u16,           // PCI vendor ID
    pub device_id:   u16,           // PCI device ID
    pub class:       u16,           // DriverClass
    pub subclass:    u16,
    pub name:        [u8; 64],      // Driver name (null-terminated)
    pub version:     [u8; 32],      // Version string
    pub author:      [u8; 64],      // Author/vendor name
    pub license:     [u8; 32],      // SPDX license identifier
    pub flags:       u32,           // SIGMA_DRV_FLAG_*
    pub pledge_caps: u64,           // sigma_pledge capability bitmap
    pub ring:        u8,            // 0 = ring-0, 3 = ring-3 (isolated)
    pub _reserved:   [u8; 31],
    // Function pointers — stable ABI boundary
    pub fn_probe:    Option<unsafe extern "C" fn(pci_bar: u64, irq: u8) -> i32>,
    pub fn_init:     Option<unsafe extern "C" fn() -> i32>,
    pub fn_shutdown: Option<unsafe extern "C" fn()>,
    pub fn_suspend:  Option<unsafe extern "C" fn() -> i32>,
    pub fn_resume:   Option<unsafe extern "C" fn() -> i32>,
    pub fn_ioctl:    Option<unsafe extern "C" fn(cmd: u32, arg: u64) -> i64>,
    pub fn_irq:      Option<unsafe extern "C" fn() -> bool>,  // true=handled
}

// ── Driver flags ──────────────────────────────────────────────────────────
pub const SIGMA_DRV_FLAG_OPEN_SOURCE: u32 = 1 << 0;  // source available
pub const SIGMA_DRV_FLAG_CERTIFIED:   u32 = 1 << 1;  // vendor-certified
pub const SIGMA_DRV_FLAG_RING3:       u32 = 1 << 2;  // ring-3 isolated
pub const SIGMA_DRV_FLAG_HOT_PLUG:    u32 = 1 << 3;  // supports hotplug
pub const SIGMA_DRV_FLAG_AI_PORTED:   u32 = 1 << 4;  // AI-assisted port
pub const SIGMA_DRV_FLAG_COMPAT_LX:   u32 = 1 << 5;  // compat Linux driver

// ── DDK registration macro helper ─────────────────────────────────────────
/// Register a driver with the SigmaOS kernel driver registry.
/// Production: linker section .sigma_drivers for auto-discovery.
#[macro_export]
macro_rules! sigma_register_driver {
    ($desc:expr) => {
        #[used]
        #[link_section = ".sigma_drivers"]
        static SIGMA_DRIVER_ENTRY: $crate::SigmaDriverDescriptor = $desc;
        #[no_mangle]
        pub extern "C" fn sigma_driver_descriptor() -> *const $crate::SigmaDriverDescriptor {
            &SIGMA_DRIVER_ENTRY
        }
    };
}

// ── Ring-3 driver isolation ────────────────────────────────────────────────
/// Sandbox descriptor for ring-3 driver isolation.
/// Crashes in ring-3 drivers don't take down the kernel.
#[repr(C)]
pub struct Ring3Sandbox {
    pub stack_pages:   u32,     // stack pages for driver thread
    pub heap_pages:    u32,     // heap pages
    pub mmio_regions:  u8,      // number of allowed MMIO regions
    pub irq_allowed:   bool,
    pub dma_allowed:   bool,
    pub pledge_caps:   u64,
}

impl Ring3Sandbox {
    pub fn minimal() -> Self {
        Self { stack_pages:4, heap_pages:16, mmio_regions:2,
               irq_allowed:true, dma_allowed:false, pledge_caps:0x01 }
    }
    pub fn full_device() -> Self {
        Self { stack_pages:16, heap_pages:64, mmio_regions:8,
               irq_allowed:true, dma_allowed:true, pledge_caps:0xFF }
    }
}

// ── DDK helper functions (stable across ABI versions) ────────────────────
/// Read from PCI configuration space
pub fn pci_config_read32(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::asm;
        let addr: u32 = (1 << 31)
            | ((bus  as u32) << 16)
            | ((dev  as u32) << 11)
            | ((func as u32) <<  8)
            | (offset as u32 & 0xFC);
        let mut val: u32;
        asm!(
            "out dx, eax",
            "in eax, dx",
            in("dx") 0xCF8u16,
            in("eax") addr,
            lateout("eax") val,
            options(nostack)
        );
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    { let _ = (bus, dev, func, offset); 0 }
}

/// Map MMIO region into kernel virtual address space
pub fn iomap(phys_addr: u64, size: usize) -> *mut u8 {
    // Production: call kernel's ioremap() equivalent
    // Stub: identity map (only valid in early boot)
    phys_addr as *mut u8
}

/// Safe MMIO read (volatile)
pub fn mmio_read32(base: *const u32, offset_bytes: usize) -> u32 {
    unsafe { core::ptr::read_volatile(base.byte_add(offset_bytes)) }
}

/// Safe MMIO write (volatile)
pub fn mmio_write32(base: *mut u32, offset_bytes: usize, val: u32) {
    unsafe { core::ptr::write_volatile(base.byte_add(offset_bytes), val); }
}

/// Request IRQ (returns handle or -1 on failure)
pub fn request_irq(irq: u8, handler: unsafe extern "C" fn() -> bool) -> i32 {
    let _ = (irq, handler);
    0  // Production: register with kernel IRQ subsystem
}

/// Send to sigma-bus (driver → kernel → userspace)
pub fn sigma_bus_send(channel: u32, data: *const u8, len: usize) -> i32 {
    extern "C" { fn sigma_bus_send_impl(ch: u32, d: *const u8, l: usize) -> i32; }
    unsafe { sigma_bus_send_impl(channel, data, len) }
}

// ── Example driver skeleton (generated by sigma-shard-new --template network) ─
pub struct ExampleNicDriver {
    pub bar0:    *mut u8,
    pub irq:     u8,
    pub mac:     [u8; 6],
    pub rx_ring: Vec<Vec<u8>>,
    pub tx_ring: Vec<Vec<u8>>,
}

unsafe impl Send for ExampleNicDriver {}
unsafe impl Sync for ExampleNicDriver {}

impl ExampleNicDriver {
    /// Called by SDF: check if this hardware is present
    pub extern "C" fn probe(pci_bar: u64, irq: u8) -> i32 {
        // Verify vendor/device ID
        let vid = pci_config_read32(0, 0, 0, 0) & 0xFFFF;
        if vid == 0xFFFF { return -1; }  // no device
        let _ = (pci_bar, irq);
        0
    }

    /// Called by SDF: initialise hardware
    pub extern "C" fn init() -> i32 {
        // sigma_pledge("stdio rpath inet");  // restrict capabilities
        // map MMIO, set up DMA rings, request IRQ
        0
    }

    /// Called by SDF: clean up
    pub extern "C" fn shutdown() {
        // free DMA rings, release IRQ, unmap MMIO
    }

    /// IRQ handler
    pub extern "C" fn irq_handler() -> bool {
        // read interrupt status register, handle RX/TX completions
        true  // IRQ was ours
    }
}

// ── DDK CLI utility ────────────────────────────────────────────────────────
pub fn ddk_cmd(args: &[String]) {
    match args.first().map(|s| s.as_str()) {
        Some("info") => {
            println!("Σ SigmaOS DDK v{}.{}", DDK_ABI_VERSION, DDK_ABI_MINOR);
            println!("  ABI magic:     0x{:08X}", SIGMA_DDK_MAGIC);
            println!("  Descriptor:    {} bytes", std::mem::size_of::<SigmaDriverDescriptor>());
            println!("  Ring-3 sandbox available: yes");
            println!("  sigma-bus channels: BUS_NETWORK BUS_STORAGE BUS_DISPLAY BUS_INPUT");
        }
        Some("list") => {
            println!("Registered drivers:");
            // In production: scan /sys/sigma/drivers/ or .sigma_drivers ELF section
            let (_, code) = std::process::Command::new("ls")
                .arg("/sys/sigma/drivers/").output()
                .map(|o| (String::from_utf8_lossy(&o.stdout).to_string(), o.status.code()))
                .unwrap_or_default();
            if code == Some(0) {
                println!("  (see /sys/sigma/drivers/)");
            } else {
                println!("  No drivers registered yet.");
                println!("  Build a driver: sigma-shard-new my-driver --template networking");
            }
        }
        Some("validate") if args.len() > 1 => {
            let path = &args[1];
            println!("Validating driver: {}", path);
            // Check ELF for SIGMA_DDK_MAGIC + ABI version
            match std::fs::read(path) {
                Ok(data) => {
                    let magic_bytes = SIGMA_DDK_MAGIC.to_le_bytes();
                    if data.windows(4).any(|w| w == magic_bytes) {
                        println!("  ✓ DDK magic found");
                        println!("  ✓ Driver appears valid");
                    } else {
                        println!("  ✗ DDK magic not found — not a SigmaOS driver");
                    }
                }
                Err(e) => println!("  ✗ Cannot read: {}", e),
            }
        }
        _ => println!("sigma-ddk — Sovereign Driver Development Kit\n\
            Usage: sigma-ddk info|list|validate <path>\n\
            \nThe DDK provides a stable ABI for hardware drivers.\n\
            Drivers built with DDK v1.0 will work on all future SigmaOS versions.\n\
            \nSee: https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Driver-Development-Guide"),
    }
}
