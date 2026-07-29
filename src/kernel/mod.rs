// SigmaOS Kernel Module
pub mod breakthroughs;
pub mod gap_closing;
pub mod generation_manager;
pub mod ipc;
pub mod memory;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod linux_absorb;
pub mod subsystem;
pub mod bus;
pub mod roundrobin;
pub mod scheduler;
pub mod object;

pub use crate::boot::firmware::{
    BootLoader, BootParams, FirmwareInterface, Initramfs, KernelCommandLine, SetupHeader,
};
pub use bus::{Bus, PciBus, UsableBus};
pub use crate::container::runtime::oci::{
    Container, ContainerManager, ContainerState, NamespaceConfig, NamespaceSet, OciSpec,
    ResourceConfig, Runtime,
};
pub use generation_manager::{Generation, GenerationManager};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use gap_closing::{
    GapError, Pml4PageTableEntry, VirtualMemoryPagingManager, IrqRoutingTable,
    AcpiInterruptManager, JournalState, JournalBlock, MetadataJournal,
};
pub use crate::unimplemented_features::{
    UniversalAbiTranslator, SelfHealingKernel, AiNativeRuntime, EnergyAwareScheduler,
    UserDefinedKernelFunctions, PrivacyFirstSandbox, SigmaFsPlus as SigmaFsPlusPlus,
};
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
