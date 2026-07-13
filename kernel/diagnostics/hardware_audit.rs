/// SigmaOS: Hardware Audit Module
/// Scans and validates hardware components: CPU features, memory topology,
/// storage devices, PCI devices, and ACPI tables for integrity and compatibility.
///
/// This replaces the legacy C/C++ implementation with memory-safe Rust.

#![no_std]
#![allow(dead_code)]
#![allow(non_snake_case)]

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ──────────────────────────────────────────────────────────────

const MAX_CPU_CORES: usize        = 256;
const MAX_MEMORY_REGIONS: usize   = 32;
const MAX_PCI_DEVICES: usize      = 128;
const MAX_STORAGE_DEVICES: usize  = 16;
const MAX_AUDIT_ENTRIES: usize    = 512;

// ─── CPU Feature Bitmap ─────────────────────────────────────────────────────

/// x86_64 CPU feature flags from CPUID leaves
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CpuFeatures {
    // CPUID[1].ECX
    pub sse3:        SigmaBool,
    pub ssse3:       SigmaBool,
    pub sse4_1:      SigmaBool,
    pub sse4_2:      SigmaBool,
    pub avx:         SigmaBool,
    pub aes_ni:      SigmaBool,
    pub popcnt:      SigmaBool,
    pub rdrand:      SigmaBool,
    // CPUID[7].EBX
    pub avx2:        SigmaBool,
    pub avx512f:     SigmaBool,
    pub avx512bw:    SigmaBool,
    pub avx512vl:    SigmaBool,
    pub bmi1:        SigmaBool,
    pub bmi2:        SigmaBool,
    pub rdseed:      SigmaBool,
    pub sha:         SigmaBool,
    // Virtualization
    pub vmx:         SigmaBool,  // Intel VT-x
    pub svm:         SigmaBool,  // AMD-V
    // Security
    pub smep:        SigmaBool,  // Supervisor Mode Execution Prevention
    pub smap:        SigmaBool,  // Supervisor Mode Access Prevention
    pub umip:        SigmaBool,  // User Mode Instruction Prevention
    pub ibrs:        SigmaBool,  // Spectre mitigation
    pub stibp:       SigmaBool,  // Single Thread Indirect Branch Predictors
}

impl CpuFeatures {
    pub const fn zero() -> Self {
        // All features: false
        unsafe { core::mem::zeroed() }
    }

    /// Check if AVX-512 is fully available for SigmaOS SIMD optimizations
    pub fn has_full_avx512(&self) -> SigmaBool {
        self.avx512f && self.avx512bw && self.avx512vl
    }

    /// Check if all Spectre/Meltdown mitigations are present
    pub fn has_full_spectre_mitigation(&self) -> SigmaBool {
        self.ibrs && self.stibp && self.smep && self.smap
    }
}

// ─── CPU Topology ────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CpuCore {
    pub logical_id:  SigmaU32,
    pub physical_id: SigmaU32,     // Physical core ID (for SMT detection)
    pub socket_id:   SigmaU32,     // NUMA socket
    pub base_freq_mhz: SigmaU32,
    pub max_freq_mhz:  SigmaU32,
    pub online:      SigmaBool,
    pub features:    CpuFeatures,
}

impl CpuCore {
    pub const fn empty() -> Self {
        Self {
            logical_id:    0,
            physical_id:   0,
            socket_id:     0,
            base_freq_mhz: 0,
            max_freq_mhz:  0,
            online:        false,
            features:      CpuFeatures::zero(),
        }
    }
}

// ─── Memory Region ───────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum MemRegionKind {
    Usable       = 0,
    Reserved     = 1,
    AcpiData     = 2,
    AcpiNvs      = 3,
    BadMemory    = 4,
    PersistentMem = 5,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryRegion {
    pub base:  SigmaU64,
    pub size:  SigmaU64,
    pub kind:  MemRegionKind,
    pub numa_node: SigmaU32,
}

impl MemoryRegion {
    pub const fn empty() -> Self {
        Self {
            base: 0, size: 0,
            kind: MemRegionKind::Reserved,
            numa_node: 0,
        }
    }
}

// ─── PCI Device ─────────────────────────────────────────────────────────────

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PciDevice {
    pub bus:       SigmaU8,
    pub slot:      SigmaU8,
    pub func:      SigmaU8,
    pub vendor_id: SigmaU16,
    pub device_id: SigmaU16,
    pub class:     SigmaU8,
    pub subclass:  SigmaU8,
    pub rev_id:    SigmaU8,
    pub irq:       SigmaU8,
    pub driver:    [SigmaU8; 32],   // bound driver name
}

impl PciDevice {
    pub const fn empty() -> Self {
        unsafe { core::mem::zeroed() }
    }

    pub fn class_name(&self) -> &'static str {
        match self.class {
            0x01 => "Storage Controller",
            0x02 => "Network Controller",
            0x03 => "Display Controller",
            0x04 => "Multimedia Controller",
            0x06 => "Bridge",
            0x0C => "Serial Bus Controller",
            0x0D => "Wireless Controller",
            _    => "Unknown",
        }
    }
}

// ─── Storage Device ──────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum StorageKind { NVMe = 0, SATA = 1, UFS = 2, MMC = 3, USB = 4 }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StorageDevice {
    pub kind:         StorageKind,
    pub capacity_mb:  SigmaU64,
    pub model:        [SigmaU8; 40],
    pub serial:       [SigmaU8; 20],
    pub smart_ok:     SigmaBool,    // S.M.A.R.T. status
    pub wear_level:   SigmaU8,      // 0-100 (SSD wear level)
    pub block_size:   SigmaU32,
}

impl StorageDevice {
    pub const fn empty() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

// ─── Audit Entry ─────────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AuditSeverity { Info = 0, Warning = 1, Error = 2, Critical = 3 }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AuditEntry {
    pub severity:    AuditSeverity,
    pub component:   [SigmaU8; 32],
    pub message:     [SigmaU8; 96],
    pub timestamp_ms: SigmaU64,
}

impl AuditEntry {
    pub const fn empty() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

// ─── Hardware Audit Engine ───────────────────────────────────────────────────

pub struct HardwareAudit {
    pub initialized:     SigmaBool,

    // Detected hardware
    pub cpu_count:       SigmaUsize,
    pub cpus:            [CpuCore; MAX_CPU_CORES],
    pub vendor_name:     [SigmaU8; 13],  // "GenuineIntel" or "AuthenticAMD"
    pub brand_string:    [SigmaU8; 48],

    pub mem_region_count: SigmaUsize,
    pub mem_regions:      [MemoryRegion; MAX_MEMORY_REGIONS],
    pub total_memory_mb:  SigmaU64,

    pub pci_count:       SigmaUsize,
    pub pci_devices:     [PciDevice; MAX_PCI_DEVICES],

    pub storage_count:   SigmaUsize,
    pub storage_devices: [StorageDevice; MAX_STORAGE_DEVICES],

    // Audit log
    pub audit_count:     SigmaUsize,
    pub audit_log:       [AuditEntry; MAX_AUDIT_ENTRIES],

    // Overall health
    pub health_ok:       SigmaBool,
    pub warning_count:   SigmaU32,
    pub error_count:     SigmaU32,
}

impl HardwareAudit {
    pub const fn new() -> Self {
        unsafe { core::mem::zeroed() }
    }

    /// Run a full hardware audit — enumerate all hardware, validate capabilities.
    pub unsafe fn run_audit(&mut self) {
        self.initialized = false;

        // 1. CPU Enumeration
        self.enumerate_cpus();

        // 2. Memory Map
        self.enumerate_memory();

        // 3. PCI Bus Scan
        self.scan_pci_bus();

        // 4. Storage Device Scan
        self.scan_storage();

        // 5. Validate SigmaOS requirements
        self.validate_requirements();

        self.health_ok    = self.error_count == 0;
        self.initialized  = true;
    }

    unsafe fn enumerate_cpus(&mut self) {
        let cpu_count = kernel_cpuid_cpu_count();
        self.cpu_count = (cpu_count as SigmaUsize).min(MAX_CPU_CORES);

        // Read vendor name from CPUID[0]
        kernel_cpuid_vendor(&mut self.vendor_name);
        // Read brand string from CPUID[0x80000002-4]
        kernel_cpuid_brand_string(&mut self.brand_string);

        for i in 0..self.cpu_count {
            let mut core = CpuCore::empty();
            core.logical_id = i as SigmaU32;
            kernel_cpuid_core_info(i as SigmaU32, &mut core);
            self.cpus[i] = core;
        }

        // Validate: SigmaOS requires at least AVX2 for SIMD optimizations
        if self.cpu_count > 0 && !self.cpus[0].features.avx2 {
            self.log_audit(AuditSeverity::Warning, b"CPU",
                           b"AVX2 not available: SIMD optimizations disabled");
        }

        // Validate: check for Spectre/Meltdown mitigations
        if self.cpu_count > 0 && !self.cpus[0].features.has_full_spectre_mitigation() {
            self.log_audit(AuditSeverity::Warning, b"CPU",
                           b"Incomplete Spectre/Meltdown mitigations detected");
        }
    }

    unsafe fn enumerate_memory(&mut self) {
        let region_count = kernel_e820_get_count();
        self.mem_region_count = (region_count as SigmaUsize).min(MAX_MEMORY_REGIONS);
        self.total_memory_mb = 0;

        for i in 0..self.mem_region_count {
            let mut region = MemoryRegion::empty();
            kernel_e820_get_region(i as SigmaU32, &mut region);
            if region.kind == MemRegionKind::Usable {
                self.total_memory_mb += region.size / (1024 * 1024);
            }
            if region.kind == MemRegionKind::BadMemory {
                self.log_audit(AuditSeverity::Error, b"Memory",
                               b"Bad memory region detected in E820 map");
            }
            self.mem_regions[i] = region;
        }

        // Validate: SigmaOS minimal requires 512MB
        if self.total_memory_mb < 512 {
            self.log_audit(AuditSeverity::Critical, b"Memory",
                           b"Insufficient RAM: SigmaOS requires minimum 512MB");
            self.error_count += 1;
        }
    }

    unsafe fn scan_pci_bus(&mut self) {
        self.pci_count = 0;

        // Iterate PCI bus 0-255, device 0-31, function 0-7
        'bus: for bus in 0u8..=255 {
            for slot in 0u8..31 {
                for func in 0u8..7 {
                    if self.pci_count >= MAX_PCI_DEVICES { break 'bus; }

                    let vendor = kernel_pci_read_u16(bus, slot, func, 0x00);
                    if vendor == 0xFFFF { continue; } // No device

                    let mut dev = PciDevice::empty();
                    dev.bus       = bus;
                    dev.slot      = slot;
                    dev.func      = func;
                    dev.vendor_id = vendor;
                    dev.device_id = kernel_pci_read_u16(bus, slot, func, 0x02);
                    dev.class     = kernel_pci_read_u8(bus, slot, func, 0x0B);
                    dev.subclass  = kernel_pci_read_u8(bus, slot, func, 0x0A);
                    dev.rev_id    = kernel_pci_read_u8(bus, slot, func, 0x08);

                    // Lookup bound driver name
                    kernel_pci_get_driver_name(bus, slot, func, &mut dev.driver);

                    self.pci_devices[self.pci_count] = dev;
                    self.pci_count += 1;

                    // Only function 0 for non-multi-function devices
                    if func == 0 {
                        let header = kernel_pci_read_u8(bus, slot, func, 0x0E);
                        if (header & 0x80) == 0 { break; } // single function
                    }
                }
            }
        }
    }

    unsafe fn scan_storage(&mut self) {
        self.storage_count = 0;
        let count = kernel_storage_enumerate(
            &mut self.storage_devices as *mut _ as *mut u8,
            MAX_STORAGE_DEVICES as SigmaU32,
        );
        self.storage_count = (count as SigmaUsize).min(MAX_STORAGE_DEVICES);

        // Check S.M.A.R.T. status for all drives
        for i in 0..self.storage_count {
            if !self.storage_devices[i].smart_ok {
                self.log_audit(AuditSeverity::Error, b"Storage",
                               b"S.M.A.R.T. failure detected on storage device");
                self.error_count += 1;
            }
            if self.storage_devices[i].wear_level > 90 {
                self.log_audit(AuditSeverity::Warning, b"Storage",
                               b"SSD wear level critical (>90%)");
                self.warning_count += 1;
            }
        }
    }

    fn validate_requirements(&mut self) {
        // Validate minimum CPU requirement (x86_64 baseline)
        if self.cpu_count == 0 {
            unsafe {
                self.log_audit(AuditSeverity::Critical, b"CPU",
                               b"No CPUs detected");
            }
            self.error_count += 1;
        }

        // Validate storage is present
        if self.storage_count == 0 {
            unsafe {
                self.log_audit(AuditSeverity::Critical, b"Storage",
                               b"No bootable storage devices detected");
            }
            self.error_count += 1;
        }
    }

    unsafe fn log_audit(&mut self, severity: AuditSeverity, component: &[u8], message: &[u8]) {
        if self.audit_count >= MAX_AUDIT_ENTRIES { return; }

        let mut entry = AuditEntry::empty();
        entry.severity    = severity;
        entry.timestamp_ms = kernel_monotonic_ms();

        let comp_len = component.len().min(31);
        entry.component[..comp_len].copy_from_slice(&component[..comp_len]);

        let msg_len = message.len().min(95);
        entry.message[..msg_len].copy_from_slice(&message[..msg_len]);

        match severity {
            AuditSeverity::Warning  => self.warning_count += 1,
            AuditSeverity::Error    => self.error_count += 1,
            AuditSeverity::Critical => self.error_count += 1,
            _ => {}
        }

        self.audit_log[self.audit_count] = entry;
        self.audit_count += 1;
    }
}

// ─── Global Singleton ────────────────────────────────────────────────────────

static mut AUDIT: HardwareAudit = HardwareAudit::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn hardware_audit_run() -> SigmaBool {
    AUDIT.run_audit();
    AUDIT.health_ok
}

#[no_mangle]
pub unsafe extern "C" fn hardware_audit_cpu_count() -> SigmaU32 {
    AUDIT.cpu_count as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn hardware_audit_memory_mb() -> SigmaU64 {
    AUDIT.total_memory_mb
}

#[no_mangle]
pub unsafe extern "C" fn hardware_audit_pci_count() -> SigmaU32 {
    AUDIT.pci_count as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn hardware_audit_storage_count() -> SigmaU32 {
    AUDIT.storage_count as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn hardware_audit_error_count() -> SigmaU32 {
    AUDIT.error_count
}

#[no_mangle]
pub unsafe extern "C" fn hardware_audit_has_avx512() -> SigmaBool {
    AUDIT.cpu_count > 0 && AUDIT.cpus[0].features.has_full_avx512()
}

#[no_mangle]
pub unsafe extern "C" fn hardware_audit_has_aes_ni() -> SigmaBool {
    AUDIT.cpu_count > 0 && AUDIT.cpus[0].features.aes_ni
}

// ─── Kernel HAL Externs ──────────────────────────────────────────────────────

extern "C" {
    fn kernel_cpuid_cpu_count() -> SigmaU32;
    fn kernel_cpuid_vendor(buf: *mut SigmaU8);
    fn kernel_cpuid_brand_string(buf: *mut SigmaU8);
    fn kernel_cpuid_core_info(logical_id: SigmaU32, out: *mut CpuCore);
    fn kernel_e820_get_count() -> SigmaU32;
    fn kernel_e820_get_region(idx: SigmaU32, out: *mut MemoryRegion);
    fn kernel_pci_read_u8(bus: SigmaU8, slot: SigmaU8, func: SigmaU8, offset: SigmaU8) -> SigmaU8;
    fn kernel_pci_read_u16(bus: SigmaU8, slot: SigmaU8, func: SigmaU8, offset: SigmaU8) -> SigmaU16;
    fn kernel_pci_get_driver_name(bus: SigmaU8, slot: SigmaU8, func: SigmaU8, out: *mut [SigmaU8; 32]);
    fn kernel_storage_enumerate(out: *mut SigmaU8, max: SigmaU32) -> SigmaU32;
    fn kernel_monotonic_ms() -> SigmaU64;
}
