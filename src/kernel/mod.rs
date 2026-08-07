// SigmaOS Kernel Module
pub mod breakthroughs;
pub mod ipc;
pub mod memory;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod roundrobin;
pub mod scheduler;
pub mod self_healing;
pub mod udkf;
pub mod breakthrough;

pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
    DeterministicReplayEngine, DynamicKernelPersonalitySwitcher, InterruptRatePredictor,
    KernelPersonalityMode,
};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use meta::{
    ABIManager, KernelGraph, KernelPersona, KernelPlugin, KernelPluginManager, LegacyScheduler,
    MetaKernel, MicroDriver, NetPod,
};
pub use paging::{PageTable, PageTableEntry, PageTableFlags, VirtualMemoryManagerV2};
pub use policy_mechanism::{
    FastPathIpc, InterruptMechanism, PolicyError, PolicyManager, PrivilegeLevel, ProtectionDomain,
    ResourceBroker,
};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use self_healing::{
    SovereignSelfHealingKernel,
};
pub use breakthrough::{
    SovereignKernelModuleSystem, SovereignKernelModule, ModuleState, SigmaSignal, ProcessProvenanceNode, PredictiveScheduler, AdaptiveRoot, ThreatLevel,
};
pub use udkf::{
    UdkfHook, UserDefinedKernelFunctions,
};
