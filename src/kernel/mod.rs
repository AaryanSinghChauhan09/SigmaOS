// SigmaOS Kernel Module
pub mod breakthroughs;
pub mod breakthroughs_v2;
pub mod exports;
pub mod ipc;
pub mod memory;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod roundrobin;
pub mod scheduler;
pub mod structures;

pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
    DeterministicReplayEngine, DynamicKernelPersonalitySwitcher, InterruptRatePredictor,
    KernelPersonalityMode,
};
pub use breakthroughs_v2::{
    AdaptiveInitSystem, AdaptiveInitTarget, AiDrivenDaemon, CrossModeMultiplexer,
    DiagnosticSeverity, MultiplexerDisplayMode, NeuroSymbolicEngine, PersonaManager,
    ProgrammableRootPersona, ProvenanceBlock, ProvenanceChainVerifier,
    SelfOptimizingShellBuiltin, TemporalFileSystem,
};
pub use exports::{
    KernelSymbol, KernelSymbolType, KldModule, SymbolRegistry, SysInitItem, SysInitOrchestrator,
    SysInitPriority,
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
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use structures::{
    Apc, ApcMode, ApcQueue, CircularDoublyLinkedList, CpuContext,
    SequencedSinglyLinkedList, SinglyLinkedList, SystemThread, ThreadState, WorkItem,
};
