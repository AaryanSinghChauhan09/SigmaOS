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
pub mod block_dev;
pub mod core;
pub mod cpu_features;
pub mod cpufreq;
pub mod device;
pub mod driver;
pub mod ebpf;
pub mod gap_filling;
pub mod numa_allocator;
pub mod object;
pub mod performance;
pub mod profiler;
pub mod secure_free;
pub mod self_healing;
pub mod sigma_kernel_autotuner;
pub mod slab_allocator;
pub mod traits;
pub mod udkf;
pub mod user_defined;
pub mod watchdog;

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

pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel,
    SigmaFsPlusPlus, UniversalAbiTranslator, UserDefinedKernelFunctions,
};
pub use gap_closing::{
    GapError, Pml4PageTableEntry, VirtualMemoryPagingManager, IrqRoutingTable,
    AcpiInterruptManager, JournalState, JournalBlock, MetadataJournal,
};
