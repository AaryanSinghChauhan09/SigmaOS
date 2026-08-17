// SigmaOS Kernel Module
pub mod architecture;
pub mod breakthroughs;
pub mod bus;
pub mod component;
pub mod generation_manager;
pub mod ipc;
pub mod linux_absorb;
pub mod linux_bsd_innovations;
pub mod linux_parity;
pub mod memory;
pub mod meta;
pub mod object;
pub mod paging;
pub mod policy_mechanism;
pub mod roundrobin;
pub mod sched;
pub mod scheduler;
pub mod subsystem;

pub use architecture::{
    ArchitectureEngine, CpuRegisters, HardwareException,
    InstructionCyclePhase as ArchInstructionCyclePhase, Irql, LookasideList, MemoryDescriptorList,
    Pcb, PoolType, ProcessorInitState, Tcb, ThreadState,
};
pub use crate::container::runtime::oci::{
    Container, ContainerManager, ContainerState, NamespaceConfig, NamespaceSet, OciSpec,
    ResourceConfig, Runtime,
};
pub use bus::{Bus, PciBus, UsableBus};
pub use generation_manager::{Generation, GenerationManager};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_bsd_innovations::{
    BsdPfStateTable, FreeBsdVfsNullfs, FutexOp, FutexWaiter, LinuxFutexEngine,
    PfFiveTuple, PfStateEntry, SovereignCgroupGovernor, CgroupResourceLimits,
};
pub use linux_parity::{
    BpfLsmPolicyGovernor, CompletionQueueEntry, KernelIoUringEngine, LsmHookType,
    MemfdSecretGuard, PageFolio, PageFolioCacheManager, SubmissionQueueEntry,
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
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use component::{Component, ComponentTree, ComponentId, ComponentState, CapabilityHandle, CapabilityRights, ComponentError, ResourceType, ResourceAllocation};
