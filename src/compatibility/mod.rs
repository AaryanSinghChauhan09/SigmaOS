// SigmaOS Compatibility Module
pub mod constellation;
pub mod cross_platform;
pub mod historic_linux;
pub mod mint_linux;
pub mod chimera_linux;
pub mod relay_nexus;
pub mod solid_kernel;
pub mod india_stack_localization;

pub use cross_platform::{
    ApplicationBinary, BinaryFormat, CompatibilityError, CompatibilityManager, CompatibilityMode,
    ContainerRuntime, HtmlRendererCapability, MediaDecoderCapability,
    SupersetApplicationCapability, TargetPlatform, TranslationLayer,
};
pub use india_stack_localization::{IndianLanguage, LocalizationManager, LocalizationProvider};

pub use historic_linux::{
    Era0_11SyscallEmulator, Era1_0SyscallEmulator, Era2_4SyscallEmulator, HistoricError,
    HistoricSyscallEmulator, HistoricalCpuState, LinuxEra, VintageDriverTranslator,
    VintagePackageConverter, VintageVirtualizationSandbox,
};

pub use mint_linux::{
    MintUpdateLevel, MintUpdatePackage, MintUpdateManager, MintBackupTool,
    MintAppMetadata, MintSoftwareManager, MintReportAlertSeverity, MintReportAlert,
    MintReportSystem,
};

pub use chimera_linux::{
    DinitServiceState, DinitService, DinitServiceManager, BsdUserlandCompat,
    ApkPackageMetadata, ApkPackageStore,
};

pub use relay_nexus::{
    PersonaType, KernelRelay, SyscallEntry, SyscallEncyclopediaEntry, FileEntry,
    NetworkEntry, ProcessEntry, SyscallEncyclopedia, LegacyDriver, DriverVaultV2,
    StorageVaultV2, NetworkVaultV2, GraphicsVaultV2, DriverVaultV2Manager, FirmwareType,
    FirmwareNexus, BIOSNexus, UEFINexus, CorebootNexus, FirmwareNexusManager,
    BuildChronicle, LegacyCChronicle, LegacyCppChronicle, LegacyAsmChronicle,
    BuildChronicleManager, SecurityModelType, SecurityNexus, DACNexus, SELinuxNexus,
    ZeroTrustNexus, SecurityNexusManager, PeripheralArchiveV2, FloppyArchiveV2,
    TapeArchiveV2, CRTArchiveV2, DotMatrixArchiveV2, PeripheralArchiveV2Manager,
};

pub use solid_kernel::{
    IScheduler, RoundRobinSchedulerPort, PrioritySchedulerPort, SolidKernelCore,
    ComplianceScheduler, AuditBlock, SigmaFSPlusPlus,
};


// ----------------------------------------------------
// Core Integration Test Types and Compatibility APIs
// ----------------------------------------------------
use core::cell::Cell;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelPersona {
    Linux_6_x,
    Linux_2_6,
}

pub struct KernelPersonaVM {
    pub current_persona: Cell<KernelPersona>,
}

impl KernelPersonaVM {
    pub fn new() -> Self {
        KernelPersonaVM {
            current_persona: Cell::new(KernelPersona::Linux_6_x),
        }
    }
    pub fn hot_swap_persona(&self, persona: KernelPersona) {
        self.current_persona.set(persona);
    }
}

unsafe impl Send for KernelPersonaVM {}
unsafe impl Sync for KernelPersonaVM {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibcVersion {
    Libc5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallAbi {
    Oabi_32,
}

pub struct BinaryCompatMatrix {
    pub libc: LibcVersion,
    pub abi: SyscallAbi,
}

impl BinaryCompatMatrix {
    pub fn new(libc: LibcVersion, abi: SyscallAbi) -> Self {
        BinaryCompatMatrix { libc, abi }
    }
    pub fn translate_sys_context(&self, sys_num: usize) -> usize {
        sys_num + 1000
    }
}

pub struct APITimelineManager {
    pub persona: KernelPersona,
}

impl APITimelineManager {
    pub fn new(persona: KernelPersona) -> Self {
        APITimelineManager { persona }
    }
    pub fn map_syscall_params(&self, param: u64) -> u64 {
        param & 0x00000000FFFFFFFF
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyBus {
    Isa,
    Agp,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkloadProfile {
    LowMemoryProfile,
    SingleCoreProfile,
}

pub struct WorkloadOptimizer {
    pub active_profile: Cell<WorkloadProfile>,
}

impl WorkloadOptimizer {
    pub fn new() -> Self {
        WorkloadOptimizer {
            active_profile: Cell::new(WorkloadProfile::LowMemoryProfile),
        }
    }
    pub fn apply_workload_tuning(&self, profile: WorkloadProfile) {
        self.active_profile.set(profile);
    }
}

unsafe impl Send for WorkloadOptimizer {}
unsafe impl Sync for WorkloadOptimizer {}

// Dummies for other required unresolved imports
pub struct DiscontinuedFS;
pub struct DriverBridge;
pub struct FSRevival;
pub struct LegacyPluginManager;
pub struct NetworkBridge;

pub static GLOBAL_PERSONA_VM: Option<KernelPersonaVM> = None;
pub static GLOBAL_PLUGIN_MANAGER: Option<LegacyPluginManager> = None;
pub static GLOBAL_WORKLOAD_OPTIMIZER: Option<WorkloadOptimizer> = None;
