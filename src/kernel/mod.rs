// SigmaOS Kernel Module
pub mod breakthroughs;
pub mod ipc;
pub mod memory;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod perf_mm;
pub mod roundrobin;
pub mod scheduler;
pub mod traits;
pub mod gap_closing;
pub mod generation_manager;

pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
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
pub use scheduler::{Priority, Process, ProcessState};
pub use traits::{Scheduler, SchedulerError};
pub use gap_closing::{
    GapError, Pml4PageTableEntry, VirtualMemoryPagingManager, IrqRoutingTable,
    AcpiInterruptManager, JournalState, JournalBlock, MetadataJournal,
};
pub use generation_manager::{Generation, GenerationManager};
