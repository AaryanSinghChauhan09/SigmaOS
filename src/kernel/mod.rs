// SigmaOS Kernel Module
pub mod ipc;
pub mod memory;
pub mod roundrobin;
pub mod scheduler;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod breakthroughs;
pub mod generation_manager;
pub mod gap_closing;

pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use meta::{
    MetaKernel, KernelPersona, KernelPlugin, KernelPluginManager, MicroDriver,
    ABIManager, NetPod, KernelGraph, LegacyScheduler,
};
pub use paging::{
    PageTable, PageTableEntry, PageTableFlags, VirtualMemoryManagerV2,
};
pub use policy_mechanism::{
    ResourceBroker, PolicyManager, ProtectionDomain, InterruptMechanism, FastPathIpc,
    PrivilegeLevel, PolicyError,
};
pub use breakthroughs::{
    UniversalAbiTranslator, SigmaFsPlusPlus, SelfHealingKernel, AiNativeRuntime,
    EnergyAwareScheduler, UserDefinedKernelFunctions, PrivacyFirstSandbox,
};
pub use generation_manager::{Generation, GenerationManager};
pub use gap_closing::{
    GapError, Pml4PageTableEntry, VirtualMemoryPagingManager,
    IrqRoutingTable, AcpiInterruptManager,
    JournalState, JournalBlock, MetadataJournal,
};
