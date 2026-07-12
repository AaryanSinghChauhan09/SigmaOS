//! Legacy Driver Archive
//! Archive for discontinued hardware drivers (floppy drives, PS/2 keyboards, old GPUs)
//! These drivers are "frozen" but still loadable for eternal compatibility

#![no_std]

use crate::drivers::device_driver::{DeviceDriver, DriverInfo, DriverStats, DriverStatus, DriverCapability, BaseDriver};
use crate::drivers::common_types::{SigmaU8, SigmaU16, SigmaU32, SigmaU64, SigmaI32, SigmaI64, SigmaBool, SigmaUsize};

/// Legacy driver metadata
#[repr(C)]
pub struct LegacyDriverMetadata {
    pub original_os: [SigmaU8; 32],  // e.g., "Linux", "Windows 95"
    pub original_version: [SigmaU8; 32],  // Original driver version
    pub port_date: [SigmaU8; 16],  // When it was ported to SigmaOS
    pub last_tested: [SigmaU8; 16],  // Last tested date
    pub compatibility_level: SigmaU32,  // 0-100% compatibility score
    pub requires_emulation: SigmaBool,  // Whether hardware emulation is needed
    pub known_issues: [SigmaU8; 512],  // Known issues/limitations
}

/// Legacy floppy disk driver
pub struct LegacyFloppyDriver {
    base: BaseDriver,
    metadata: LegacyDriverMetadata,
    pub drive_type: SigmaU32,  // 1.44MB, 2.88MB, etc.
    pub tracks: SigmaU32,
    pub sectors_per_track: SigmaU32,
    pub heads: SigmaU32,
}

impl LegacyFloppyDriver {
    pub const fn new() -> Self {
        Self {
            base: BaseDriver::new("Legacy Floppy", "1.0.0", "SigmaOS Legacy"),
            metadata: LegacyDriverMetadata {
                original_os: *b"Linux\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                original_version: *b"2.6.32\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                port_date: *b"2024-01-15\0\0\0\0\0\0\0",
                last_tested: *b"2024-06-01\0\0\0\0\0\0\0",
                compatibility_level: 95,
                requires_emulation: false,
                known_issues: [0; 512],
            },
            drive_type: 0,
            tracks: 80,
            sectors_per_track: 18,
            heads: 2,
        }
    }
}

impl DeviceDriver for LegacyFloppyDriver {
    fn init(&mut self) -> SigmaI32 {
        self.base.init();
        self.base.add_capability(DriverCapability::Read);
        self.base.add_capability(DriverCapability::Write);
        self.base.add_capability(DriverCapability::Ioctl);
        0
    }
    
    fn shutdown(&mut self) -> SigmaI32 {
        self.base.shutdown()
    }
    
    fn read(&mut self, buffer: *mut SigmaU8, size: SigmaUsize) -> SigmaI32 {
        self.base.read(buffer, size)
    }
    
    fn write(&mut self, buffer: *const SigmaU8, size: SigmaUsize) -> SigmaI32 {
        self.base.write(buffer, size)
    }
    
    fn ioctl(&mut self, request: SigmaU32, arg: SigmaU64) -> SigmaI32 {
        self.base.ioctl(request, arg)
    }
    
    fn get_info(&self) -> DriverInfo {
        self.base.get_info()
    }
    
    fn get_stats(&self) -> DriverStats {
        self.base.get_stats()
    }
    
    fn reset(&mut self) -> SigmaI32 {
        self.base.reset()
    }
    
    fn suspend(&mut self) -> SigmaI32 {
        self.base.suspend()
    }
    
    fn resume(&mut self) -> SigmaI32 {
        self.base.resume()
    }
    
    fn has_capability(&self, cap: DriverCapability) -> SigmaBool {
        self.base.has_capability(cap)
    }
    
    fn get_status(&self) -> DriverStatus {
        self.base.get_status()
    }
}

/// Legacy PS/2 keyboard driver
pub struct LegacyPS2KeyboardDriver {
    base: BaseDriver,
    metadata: LegacyDriverMetadata,
    pub scancode_set: SigmaU32,
}

impl LegacyPS2KeyboardDriver {
    pub const fn new() -> Self {
        Self {
            base: BaseDriver::new("Legacy PS/2 Keyboard", "1.0.0", "SigmaOS Legacy"),
            metadata: LegacyDriverMetadata {
                original_os: *b"Linux\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                original_version: *b"2.6.32\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                port_date: *b"2024-01-15\0\0\0\0\0\0\0",
                last_tested: *b"2024-06-01\0\0\0\0\0\0\0",
                compatibility_level: 98,
                requires_emulation: false,
                known_issues: [0; 512],
            },
            scancode_set: 2,
        }
    }
}

impl DeviceDriver for LegacyPS2KeyboardDriver {
    fn init(&mut self) -> SigmaI32 {
        self.base.init();
        self.base.add_capability(DriverCapability::Read);
        self.base.add_capability(DriverCapability::Interrupt);
        0
    }
    
    fn shutdown(&mut self) -> SigmaI32 {
        self.base.shutdown()
    }
    
    fn read(&mut self, buffer: *mut SigmaU8, size: SigmaUsize) -> SigmaI32 {
        self.base.read(buffer, size)
    }
    
    fn write(&mut self, buffer: *const SigmaU8, size: SigmaUsize) -> SigmaI32 {
        self.base.write(buffer, size)
    }
    
    fn ioctl(&mut self, request: SigmaU32, arg: SigmaU64) -> SigmaI32 {
        self.base.ioctl(request, arg)
    }
    
    fn get_info(&self) -> DriverInfo {
        self.base.get_info()
    }
    
    fn get_stats(&self) -> DriverStats {
        self.base.get_stats()
    }
    
    fn reset(&mut self) -> SigmaI32 {
        self.base.reset()
    }
    
    fn suspend(&mut self) -> SigmaI32 {
        self.base.suspend()
    }
    
    fn resume(&mut self) -> SigmaI32 {
        self.base.resume()
    }
    
    fn has_capability(&self, cap: DriverCapability) -> SigmaBool {
        self.base.has_capability(cap)
    }
    
    fn get_status(&self) -> DriverStatus {
        self.base.get_status()
    }
}

/// Legacy PS/2 mouse driver
pub struct LegacyPS2MouseDriver {
    base: BaseDriver,
    metadata: LegacyDriverMetadata,
    pub resolution: SigmaU32,
}

impl LegacyPS2MouseDriver {
    pub const fn new() -> Self {
        Self {
            base: BaseDriver::new("Legacy PS/2 Mouse", "1.0.0", "SigmaOS Legacy"),
            metadata: LegacyDriverMetadata {
                original_os: *b"Linux\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                original_version: *b"2.6.32\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                port_date: *b"2024-01-15\0\0\0\0\0\0\0",
                last_tested: *b"2024-06-01\0\0\0\0\0\0\0",
                compatibility_level: 98,
                requires_emulation: false,
                known_issues: [0; 512],
            },
            resolution: 400,
        }
    }
}

impl DeviceDriver for LegacyPS2MouseDriver {
    fn init(&mut self) -> SigmaI32 {
        self.base.init();
        self.base.add_capability(DriverCapability::Read);
        self.base.add_capability(DriverCapability::Interrupt);
        0
    }
    
    fn shutdown(&mut self) -> SigmaI32 {
        self.base.shutdown()
    }
    
    fn read(&mut self, buffer: *mut SigmaU8, size: SigmaUsize) -> SigmaI32 {
        self.base.read(buffer, size)
    }
    
    fn write(&mut self, buffer: *const SigmaU8, size: SigmaUsize) -> SigmaI32 {
        self.base.write(buffer, size)
    }
    
    fn ioctl(&mut self, request: SigmaU32, arg: SigmaU64) -> SigmaI32 {
        self.base.ioctl(request, arg)
    }
    
    fn get_info(&self) -> DriverInfo {
        self.base.get_info()
    }
    
    fn get_stats(&self) -> DriverStats {
        self.base.get_stats()
    }
    
    fn reset(&mut self) -> SigmaI32 {
        self.base.reset()
    }
    
    fn suspend(&mut self) -> SigmaI32 {
        self.base.suspend()
    }
    
    fn resume(&mut self) -> SigmaI32 {
        self.base.resume()
    }
    
    fn has_capability(&self, cap: DriverCapability) -> SigmaBool {
        self.base.has_capability(cap)
    }
    
    fn get_status(&self) -> DriverStatus {
        self.base.get_status()
    }
}

/// Legacy VGA driver
pub struct LegacyVGADriver {
    base: BaseDriver,
    metadata: LegacyDriverMetadata,
    pub mode: SigmaU32,
}

impl LegacyVGADriver {
    pub const fn new() -> Self {
        Self {
            base: BaseDriver::new("Legacy VGA", "1.0.0", "SigmaOS Legacy"),
            metadata: LegacyDriverMetadata {
                original_os: *b"Linux\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                original_version: *b"2.6.32\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                port_date: *b"2024-01-15\0\0\0\0\0\0\0",
                last_tested: *b"2024-06-01\0\0\0\0\0\0\0",
                compatibility_level: 90,
                requires_emulation: false,
                known_issues: [0; 512],
            },
            mode: 3,  // Standard VGA text mode
        }
    }
}

impl DeviceDriver for LegacyVGADriver {
    fn init(&mut self) -> SigmaI32 {
        self.base.init();
        self.base.add_capability(DriverCapability::Read);
        self.base.add_capability(DriverCapability::Write);
        self.base.add_capability(DriverCapability::Mmap);
        self.base.add_capability(DriverCapability::Ioctl);
        0
    }
    
    fn shutdown(&mut self) -> SigmaI32 {
        self.base.shutdown()
    }
    
    fn read(&mut self, buffer: *mut SigmaU8, size: SigmaUsize) -> SigmaI32 {
        self.base.read(buffer, size)
    }
    
    fn write(&mut self, buffer: *const SigmaU8, size: SigmaUsize) -> SigmaI32 {
        self.base.write(buffer, size)
    }
    
    fn ioctl(&mut self, request: SigmaU32, arg: SigmaU64) -> SigmaI32 {
        self.base.ioctl(request, arg)
    }
    
    fn get_info(&self) -> DriverInfo {
        self.base.get_info()
    }
    
    fn get_stats(&self) -> DriverStats {
        self.base.get_stats()
    }
    
    fn reset(&mut self) -> SigmaI32 {
        self.base.reset()
    }
    
    fn suspend(&mut self) -> SigmaI32 {
        self.base.suspend()
    }
    
    fn resume(&mut self) -> SigmaI32 {
        self.base.resume()
    }
    
    fn has_capability(&self, cap: DriverCapability) -> SigmaBool {
        self.base.has_capability(cap)
    }
    
    fn get_status(&self) -> DriverStatus {
        self.base.get_status()
    }
}

/// Legacy driver registry
pub struct LegacyDriverRegistry {
    pub floppy: Option<LegacyFloppyDriver>,
    pub ps2_keyboard: Option<LegacyPS2KeyboardDriver>,
    pub ps2_mouse: Option<LegacyPS2MouseDriver>,
    pub vga: Option<LegacyVGADriver>,
}

impl LegacyDriverRegistry {
    pub const fn new() -> Self {
        Self {
            floppy: None,
            ps2_keyboard: None,
            ps2_mouse: None,
            vga: None,
        }
    }
    
    pub fn init_floppy(&mut self) -> SigmaI32 {
        self.floppy = Some(LegacyFloppyDriver::new());
        if let Some(driver) = &mut self.floppy {
            driver.init()
        } else {
            -1
        }
    }
    
    pub fn init_ps2_keyboard(&mut self) -> SigmaI32 {
        self.ps2_keyboard = Some(LegacyPS2KeyboardDriver::new());
        if let Some(driver) = &mut self.ps2_keyboard {
            driver.init()
        } else {
            -1
        }
    }
    
    pub fn init_ps2_mouse(&mut self) -> SigmaI32 {
        self.ps2_mouse = Some(LegacyPS2MouseDriver::new());
        if let Some(driver) = &mut self.ps2_mouse {
            driver.init()
        } else {
            -1
        }
    }
    
    pub fn init_vga(&mut self) -> SigmaI32 {
        self.vga = Some(LegacyVGADriver::new());
        if let Some(driver) = &mut self.vga {
            driver.init()
        } else {
            -1
        }
    }
}

/// Global legacy driver registry
static mut LEGACY_REGISTRY: Option<LegacyDriverRegistry> = None;

/// Initialize legacy driver registry
#[no_mangle]
pub unsafe extern "C" fn legacy_driver_registry_init() -> SigmaI32 {
    LEGACY_REGISTRY = Some(LegacyDriverRegistry::new());
    0
}

/// Get global legacy driver registry
#[no_mangle]
pub unsafe extern "C" fn legacy_driver_registry_get() -> *mut LegacyDriverRegistry {
    match &mut LEGACY_REGISTRY {
        Some(registry) => registry as *mut LegacyDriverRegistry,
        None => core::ptr::null_mut(),
    }
}
