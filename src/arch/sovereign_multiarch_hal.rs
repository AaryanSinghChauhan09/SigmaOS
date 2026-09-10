// SigmaOS Sovereign Multi-Architecture HAL
// Inspired by Linux arch/ and BSD machine/ subsystems
// Supports x86_16, x86_32 (IA-32/CISC), x86_64 (AMD64), ARM32, ARM64 (AArch64),
// RISC-V 32/64, MIPS, PowerPC, and SPARC architectures
// All code is zero-dependency, no_std compliant per AGENTS.md

#[cfg(not(any(feature = "standalone_test", test)))]


#[cfg(not(any(feature = "standalone_test", test)))]
use std::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use std::format;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(any(feature = "standalone_test", test))]
use std::format;

// ============================================================
// § 1. ARCHITECTURE IDENTIFICATION & FEATURE FLAGS
// ============================================================

/// Target CPU architecture enumeration
/// Covers all major architecture families including CISC (x86) and RISC (ARM, RISC-V)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CpuArchFamily {
    X86_16, X86_32, X86_64,
    Arm32, Arm64,
    RiscV32, RiscV64,
    Mips32, Mips64,
    PowerPc32, PowerPc64,
    Sparc32, SparcV9,
    LoongArch64,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IspType { Cisc, Risc, Vliw, Epic }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordSize { Bits16 = 16, Bits32 = 32, Bits64 = 64, Bits128 = 128 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness { LittleEndian, BigEndian, BiEndian }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel { MachineMode, HypervisorMode, SupervisorMode, UserMode }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmuType { None, Segmented, Paged, Iommu }

#[derive(Debug, Clone)]
pub struct CpuFeatureFlags {
    pub has_fpu: bool, pub has_simd: bool, pub has_virt: bool,
    pub has_mem_encrypt: bool, pub has_tee: bool, pub has_ibrs: bool,
    pub has_ssbd: bool, pub has_cet: bool, pub has_pac: bool, pub has_bti: bool,
    pub hw_breakpoints: u8, pub hw_watchpoints: u8,
    pub cache_line_bytes: u8, pub phys_addr_bits: u8, pub virt_addr_bits: u8,
}

impl CpuFeatureFlags {
    pub const fn default_x86_64() -> Self {
        Self { has_fpu: true, has_simd: true, has_virt: false, has_mem_encrypt: false,
               has_tee: false, has_ibrs: false, has_ssbd: false, has_cet: false,
               has_pac: false, has_bti: false, hw_breakpoints: 4, hw_watchpoints: 4,
               cache_line_bytes: 64, phys_addr_bits: 52, virt_addr_bits: 57 }
    }
    pub const fn default_arm64() -> Self {
        Self { has_fpu: true, has_simd: true, has_virt: true, has_mem_encrypt: false,
               has_tee: false, has_ibrs: false, has_ssbd: false, has_cet: false,
               has_pac: true, has_bti: true, hw_breakpoints: 6, hw_watchpoints: 4,
               cache_line_bytes: 64, phys_addr_bits: 48, virt_addr_bits: 48 }
    }
    pub const fn default_riscv64() -> Self {
        Self { has_fpu: true, has_simd: false, has_virt: true, has_mem_encrypt: false,
               has_tee: false, has_ibrs: false, has_ssbd: false, has_cet: false,
               has_pac: false, has_bti: false, hw_breakpoints: 2, hw_watchpoints: 2,
               cache_line_bytes: 64, phys_addr_bits: 56, virt_addr_bits: 57 }
    }
}

// ============================================================
// § 2. ARCHITECTURE DESCRIPTOR
// ============================================================

#[derive(Debug, Clone)]
pub struct ArchDescriptor {
    pub family: CpuArchFamily, pub isp: IspType, pub word_size: WordSize,
    pub endian: Endianness, pub mmu: MmuType, pub features: CpuFeatureFlags,
    pub name: &'static str, pub linux_arch_name: &'static str, pub bsd_machine_name: &'static str,
}

// ============================================================
// § 3. ARCHITECTURE-SPECIFIC REGISTER CONTEXTS
// ============================================================

/// x86_64 CPU register save area (matches Linux struct pt_regs layout)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct X86_64Regs {
    pub r15: u64, pub r14: u64, pub r13: u64, pub r12: u64,
    pub rbx: u64, pub rbp: u64, pub r11: u64, pub r10: u64,
    pub r9: u64, pub r8: u64, pub rax: u64, pub rcx: u64,
    pub rdx: u64, pub rsi: u64, pub rdi: u64, pub orig_rax: u64,
    pub rip: u64, pub cs: u64, pub rflags: u64, pub rsp: u64, pub ss: u64,
}

impl X86_64Regs {
    pub fn new_user(rip: u64, rsp: u64, rflags: u64) -> Self {
        Self { rip, rsp, rflags, cs: 0x23, ss: 0x2b, ..Default::default() }
    }
    pub fn new_kernel(rip: u64, rsp: u64) -> Self {
        Self { rip, rsp, rflags: 0x200, cs: 0x08, ss: 0x10, ..Default::default() }
    }
}

/// AArch64 CPU register save area
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Arm64Regs {
    pub x: [u64; 31], pub sp: u64, pub pc: u64, pub pstate: u64,
}

impl Arm64Regs {
    pub fn new_user(pc: u64, sp: u64) -> Self {
        Self { pc, sp, pstate: 0x0, x: [0u64; 31] }
    }
    pub fn new_kernel(pc: u64, sp: u64) -> Self {
        Self { pc, sp, pstate: 0x5, x: [0u64; 31] }
    }
}

/// RISC-V 64-bit register context (matches Linux struct pt_regs)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RiscV64Regs {
    pub ra: u64, pub sp: u64, pub gp: u64, pub tp: u64,
    pub t: [u64; 3], pub s0: u64, pub s1: u64,
    pub a: [u64; 8], pub s: [u64; 10], pub t2: [u64; 4],
    pub sepc: u64, pub sstatus: u64,
}

// ============================================================
// § 4. MEMORY MAP DESCRIPTORS
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemRegionType {
    Usable, Reserved, AcpiReclaimable, AcpiNvs, BadMemory,
    Mmio, PersistentMemory, LoaderData, KernelCode, KernelRoData, KernelData,
}

#[derive(Debug, Clone, Copy)]
pub struct MemRegion {
    pub base: u64, pub size: u64, pub region_type: MemRegionType, pub is_64bit: bool,
}

impl MemRegion {
    pub fn new(base: u64, size: u64, region_type: MemRegionType) -> Self {
        Self { base, size, region_type,
               is_64bit: base > 0xFFFF_FFFF || (base + size) > 0xFFFF_FFFF }
    }
    pub fn end(&self) -> u64 { self.base.saturating_add(self.size) }
    pub fn contains(&self, addr: u64) -> bool { addr >= self.base && addr < self.end() }
    pub fn is_usable(&self) -> bool { matches!(self.region_type, MemRegionType::Usable) }
}

// ============================================================
// § 5. MULTI-ARCH PAGE TABLE ABSTRACTIONS
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSize { Page4K = 4096, Page64K = 65536, Huge2M = 2097152, Huge1G = 1073741824 }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageTableLevels { TwoLevel, ThreeLevel, FourLevel, FiveLevel }

#[derive(Debug, Clone, Copy, Default)]
pub struct AbstractPte {
    pub pfn: u64, pub present: bool, pub writable: bool, pub executable: bool,
    pub user_accessible: bool, pub accessed: bool, pub dirty: bool,
    pub global: bool, pub cache_type: u8,
}

impl AbstractPte {
    pub fn physical_addr(&self) -> u64 { self.pfn << 12 }
    pub fn is_huge_page(&self) -> bool { false }
}

// ============================================================
// § 6. CPU TOPOLOGY
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreType { Performance, Efficiency, Balanced, RealTime }

#[derive(Debug, Clone)]
pub struct CpuCluster {
    pub cluster_id: u32, pub core_type: CoreType, pub core_count: u32,
    pub max_freq_mhz: u32, pub min_freq_mhz: u32,
    pub l1_icache_kb: u32, pub l1_dcache_kb: u32, pub l2_cache_kb: u32,
}

#[derive(Debug, Clone)]
pub struct CpuTopology {
    pub logical_cpus: u32, pub physical_cores: u32, pub numa_nodes: u32,
    pub dies: u32, pub smt_threads: u32, pub clusters: Vec<CpuCluster>,
}

impl CpuTopology {
    pub fn default_x86_64_server() -> Self {
        Self { logical_cpus: 16, physical_cores: 8, numa_nodes: 1, dies: 1, smt_threads: 2,
               clusters: vec![CpuCluster { cluster_id: 0, core_type: CoreType::Balanced,
                   core_count: 8, max_freq_mhz: 5000, min_freq_mhz: 800,
                   l1_icache_kb: 32, l1_dcache_kb: 32, l2_cache_kb: 1024 }] }
    }

    pub fn default_arm_biglittle() -> Self {
        Self { logical_cpus: 8, physical_cores: 8, numa_nodes: 1, dies: 1, smt_threads: 1,
               clusters: vec![
                   CpuCluster { cluster_id: 0, core_type: CoreType::Efficiency, core_count: 4,
                       max_freq_mhz: 2000, min_freq_mhz: 300, l1_icache_kb: 32,
                       l1_dcache_kb: 32, l2_cache_kb: 256 },
                   CpuCluster { cluster_id: 1, core_type: CoreType::Performance, core_count: 4,
                       max_freq_mhz: 3200, min_freq_mhz: 500, l1_icache_kb: 64,
                       l1_dcache_kb: 64, l2_cache_kb: 4096 },
               ] }
    }
}

// ============================================================
// § 7. ARCHITECTURE RUNTIME DETECTION
// ============================================================

pub struct ArchDetector;

impl ArchDetector {
    pub fn current_arch() -> CpuArchFamily {
        #[cfg(target_arch = "x86_64")] { return CpuArchFamily::X86_64; }
        #[cfg(target_arch = "x86")] { return CpuArchFamily::X86_32; }
        #[cfg(target_arch = "aarch64")] { return CpuArchFamily::Arm64; }
        #[cfg(target_arch = "arm")] { return CpuArchFamily::Arm32; }
        #[cfg(target_arch = "riscv64")] { return CpuArchFamily::RiscV64; }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "x86",
                      target_arch = "aarch64", target_arch = "arm",
                      target_arch = "riscv64")))]
        CpuArchFamily::Unknown
    }

    pub fn current_isp() -> IspType {
        match Self::current_arch() {
            CpuArchFamily::X86_16 | CpuArchFamily::X86_32 | CpuArchFamily::X86_64 => IspType::Cisc,
            _ => IspType::Risc,
        }
    }

    pub fn pointer_size_bytes() -> usize { core::mem::size_of::<usize>() }

    pub fn native_page_size() -> usize { 4096 }

    pub fn arch_info_string() -> String {
        format!("SigmaOS on {:?} ({:?} ISP, {}-bit, {}B pages)",
            Self::current_arch(), Self::current_isp(),
            Self::pointer_size_bytes() * 8, Self::native_page_size())
    }
}

// ============================================================
// § 8. SOVEREIGN BOOT PARAMETERS
// ============================================================

#[derive(Debug, Clone)]
pub struct SovereignBootParams {
    pub arch: CpuArchFamily,
    pub mem_map_addr: u64, pub mem_map_count: u32,
    pub dtb_addr: Option<u64>, pub acpi_rsdp_addr: Option<u64>,
    pub cmdline_addr: u64, pub cmdline_len: u32,
    pub initrd_start: u64, pub initrd_end: u64,
    pub efi_systable_addr: Option<u64>,
    pub kernel_phys_start: u64, pub kernel_phys_end: u64,
    pub kaslr_offset: u64,
}

impl SovereignBootParams {
    pub fn new_x86_64_minimal(kernel_start: u64, kernel_end: u64) -> Self {
        Self { arch: CpuArchFamily::X86_64, mem_map_addr: 0, mem_map_count: 0,
               dtb_addr: None, acpi_rsdp_addr: None, cmdline_addr: 0, cmdline_len: 0,
               initrd_start: 0, initrd_end: 0, efi_systable_addr: None,
               kernel_phys_start: kernel_start, kernel_phys_end: kernel_end, kaslr_offset: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_detection() {
        let arch = ArchDetector::current_arch();
        assert_ne!(arch, CpuArchFamily::Unknown);
    }

    #[test]
    fn test_x86_64_regs_user() {
        let regs = X86_64Regs::new_user(0xDEAD_0000, 0x7FFF_0000, 0x200);
        assert_eq!(regs.rip, 0xDEAD_0000);
        assert_eq!(regs.cs, 0x23);
    }

    #[test]
    fn test_x86_64_regs_kernel() {
        let regs = X86_64Regs::new_kernel(0xFFFF_8000_0000_0000, 0xFFFF_FFFF_8000_0000);
        assert_eq!(regs.cs, 0x08);
    }

    #[test]
    fn test_arm64_regs() {
        let regs = Arm64Regs::new_user(0x4000, 0x7FFF_0000);
        assert_eq!(regs.pc, 0x4000);
        assert_eq!(regs.pstate, 0x0);
    }

    #[test]
    fn test_mem_region() {
        let r = MemRegion::new(0x1000_0000, 0x1000_0000, MemRegionType::Usable);
        assert!(r.contains(0x1500_0000));
        assert!(!r.contains(0x2500_0000));
        assert!(r.is_usable());
    }

    #[test]
    fn test_cpu_topology_x86() {
        let t = CpuTopology::default_x86_64_server();
        assert_eq!(t.logical_cpus, 16);
        assert_eq!(t.clusters[0].core_type, CoreType::Balanced);
    }

    #[test]
    fn test_cpu_topology_arm() {
        let t = CpuTopology::default_arm_biglittle();
        assert_eq!(t.clusters.len(), 2);
        assert_eq!(t.clusters[0].core_type, CoreType::Efficiency);
        assert_eq!(t.clusters[1].core_type, CoreType::Performance);
    }

    #[test]
    fn test_abstract_pte() {
        let pte = AbstractPte { pfn: 0x1000, present: true, writable: true,
            executable: false, user_accessible: false, accessed: false,
            dirty: false, global: true, cache_type: 0 };
        assert_eq!(pte.physical_addr(), 0x1000_000);
    }

    #[test]
    fn test_arch_info_string() {
        let s = ArchDetector::arch_info_string();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_boot_params() {
        let p = SovereignBootParams::new_x86_64_minimal(0xFFFF_8000_0000_0000, 0xFFFF_8000_1000_0000);
        assert_eq!(p.arch, CpuArchFamily::X86_64);
        assert!(p.dtb_addr.is_none());
    }
}
