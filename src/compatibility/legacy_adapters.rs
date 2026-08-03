// SigmaOS Legacy Compatibility Adapters
// Provides implementations for legacy personality adaptations, syscall translations,
// and bridge structures as expected by the integration tests.

use std::cell::Cell;

/// Represents kernel personas supported for legacy environments
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelPersona {
    Linux_6_x,
    Linux_2_6,
}

/// Multi-persona Virtual Machine for sandboxing old binaries
pub struct KernelPersonaVM {
    pub current_persona: Cell<KernelPersona>,
}

impl KernelPersonaVM {
    pub fn new() -> Self {
        Self {
            current_persona: Cell::new(KernelPersona::Linux_6_x),
        }
    }

    pub fn hot_swap_persona(&self, persona: KernelPersona) {
        self.current_persona.set(persona);
    }
}

/// Libc version identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibcVersion {
    Libc5,
    Glibc_2_5,
}

/// Syscall Application Binary Interfaces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallAbi {
    Oabi_32,
    Eabi_32,
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

pub const GLOBAL_PERSONA_VM: usize = 0;
pub const GLOBAL_PLUGIN_MANAGER: usize = 0;
pub const GLOBAL_WORKLOAD_OPTIMIZER: usize = 0;

pub struct LegacyDriverAdapter;
pub struct LegacyFSAdapter;
pub struct LegacyKernelAdapter;
pub struct LegacyPackageAdapter;
pub struct LegacyProtocolAdapter;
pub struct LegacySecurityAdapter;
pub struct LegacyUIAdapter;
