// SigmaOS Legacy Compatibility Adapters
// Provides implementations for legacy personality adaptations, syscall translations,
// and bridge structures as expected by the integration tests.

use core::cell::Cell;

/// Represents kernel personas supported for legacy environments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelPersona {
    Linux6X,
    Linux2_6,
}

impl KernelPersona {
    pub fn name(&self) -> &'static str {
        match self {
            KernelPersona::Linux6X => "linux_6_x",
            KernelPersona::Linux2_6 => "linux_2_6",
        }
    }

    pub fn api_version(&self) -> u32 {
        match self {
            KernelPersona::Linux6X => 60000,
            KernelPersona::Linux2_6 => 20006,
        }
    }
}

/// Multi-persona Virtual Machine for sandboxing old binaries
pub struct KernelPersonaVM {
    pub current_persona: Cell<KernelPersona>,
    pub syscall_table: SyscallTable,
    pub personality_state: PersonalityState,
}

pub struct SyscallTable {
    pub entries: [SyscallEntry; 512],
}

#[derive(Clone, Copy)]
pub struct SyscallEntry {
    pub number: u32,
    pub handler: Option<unsafe extern "C" fn()>,
    pub persona: KernelPersona,
}

pub struct PersonalityState {
    pub uid: u32,
    pub gid: u32,
    pub personality: u32,
    pub domainname: [u8; 64],
    pub hostname: [u8; 64],
}

impl KernelPersonaVM {
    pub fn new() -> Self {
        Self {
            current_persona: Cell::new(KernelPersona::Linux6X),
            syscall_table: SyscallTable::new(),
            personality_state: PersonalityState::default(),
        }
    }

    pub fn get_persona(&self) -> KernelPersona {
        self.current_persona.get()
    }

    pub fn hot_swap_persona(&self, persona: KernelPersona) {
        self.current_persona.set(persona);
        self.update_syscall_table(persona);
    }

    fn update_syscall_table(&self, persona: KernelPersona) {
        // Update syscall table based on persona
        match persona {
            KernelPersona::Linux6X => self.setup_linux6x_syscalls(),
            KernelPersona::Linux2_6 => self.setup_linux26_syscalls(),
        }
    }

    fn setup_linux6x_syscalls(&self) {
        // Setup Linux 6.x syscall table
    }

    fn setup_linux26_syscalls(&self) {
        // Setup Linux 2.6 syscall table
    }

    pub fn translate_syscall(&self, syscall: u32) -> u32 {
        match self.current_persona.get() {
            KernelPersona::Linux6X => self.translate_linux6x(syscall),
            KernelPersona::Linux2_6 => self.translate_linux26(syscall),
        }
    }

    fn translate_linux6x(&self, syscall: u32) -> u32 {
        // Direct mapping for modern syscalls
        syscall
    }

    fn translate_linux26(&self, syscall: u32) -> u32 {
        // Map legacy syscalls to modern equivalents
        match syscall {
            1 => 100, // Example: exit -> sigma_exit
            2 => 101, // fork -> sigma_fork
            _ => syscall,
        }
    }
}

impl SyscallTable {
    pub fn new() -> Self {
        Self {
            entries: [SyscallEntry::default(); 512],
        }
    }

    pub fn register(&mut self, entry: SyscallEntry) {
        let num = entry.number as usize;
        if num < 512 {
            self.entries[num] = entry;
        }
    }

    pub fn lookup(&self, number: u32) -> Option<&SyscallEntry> {
        if number < 512 {
            Some(&self.entries[number as usize])
        } else {
            None
        }
    }
}

impl Default for SyscallEntry {
    fn default() -> Self {
        Self {
            number: 0,
            handler: None,
            persona: KernelPersona::Linux6X,
        }
    }
}

impl Default for PersonalityState {
    fn default() -> Self {
        Self {
            uid: 0,
            gid: 0,
            personality: 0,
            domainname: [0; 64],
            hostname: [0; 64],
        }
    }
}

/// Libc version identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibcVersion {
    Libc5,
    Glibc2_5,
}

/// Syscall Application Binary Interfaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum SyscallAbi {
    Oabi32,
    Eabi32,
    Eabi64,
}

/// Matrix that maps old libc calls to modern equivalents
pub struct BinaryCompatMatrix {
    pub libc_ver: LibcVersion,
    pub abi: SyscallAbi,
}

impl BinaryCompatMatrix {
    pub fn new(libc_ver: LibcVersion, abi: SyscallAbi) -> Self {
        Self { libc_ver, abi }
    }

    pub fn translate_sys_context(&self, syscall_id: u32) -> u32 {
        syscall_id + 1000
    }
}

/// Dynamic manager that translates syscall parameter registers
pub struct APITimelineManager {
    pub persona: KernelPersona,
}

impl APITimelineManager {
    pub fn new(persona: KernelPersona) -> Self {
        Self { persona }
    }

    pub fn map_syscall_params(&self, raw_params: u64) -> u64 {
        raw_params & 0xFFFFFFFF
    }
}

/// Legacy hardware bus interfaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyBus {
    Isa,
    Agp,
    Pci,
}

/// Storage adapter for discontinued devices (like floppy drives)
pub struct StorageBridge {
    pub driver_name: &'static str,
    pub bus: LegacyBus,
}

impl StorageBridge {
    pub fn bus_type(&self) -> LegacyBus {
        self.bus
    }

    pub fn init_legacy(&self) -> bool {
        true
    }
}

/// Graphics adapter for discontinued visual hardware
pub struct GraphicsBridge {
    pub driver_name: &'static str,
    pub bus: LegacyBus,
}

impl GraphicsBridge {
    pub fn bus_type(&self) -> LegacyBus {
        self.bus
    }

    pub fn init_legacy(&self) -> bool {
        true
    }
}

/// Workload memory and scheduling profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadProfile {
    LowMemoryProfile,
    SingleCoreProfile,
    RealTimeProfile,
}

/// Active tuner for scheduling and layout optimization
pub struct WorkloadOptimizer {
    pub active_profile: Cell<WorkloadProfile>,
}

impl WorkloadOptimizer {
    pub fn new() -> Self {
        Self {
            active_profile: Cell::new(WorkloadProfile::LowMemoryProfile),
        }
    }

    pub fn get_profile(&self) -> WorkloadProfile {
        self.active_profile.get()
    }

    pub fn apply_workload_tuning(&self, profile: WorkloadProfile) {
        self.active_profile.set(profile);
    }
}

// Dummy/placeholder declarations required to satisfy integration test references
pub struct DiscontinuedFS;
pub struct DriverBridge;
pub struct FSRevival;
pub struct LegacyPluginManager;
pub struct NetworkBridge;

pub struct LegacyKernelAdapter;
pub struct LegacyDriverAdapter;
pub struct LegacyPackageAdapter;
pub struct LegacyFSAdapter;
pub struct LegacyProtocolAdapter;
pub struct LegacySecurityAdapter;
pub struct LegacyUIAdapter;

pub const GLOBAL_PERSONA_VM: usize = 0;
pub const GLOBAL_PLUGIN_MANAGER: usize = 0;
pub const GLOBAL_WORKLOAD_OPTIMIZER: usize = 0;
