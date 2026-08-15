// SigmaOS Kernel Module
pub mod architecture;
pub mod breakthroughs;
pub mod ipc;
pub mod linux_bsd_innovations;
pub mod memory;
pub mod policy_mechanism;
pub mod roundrobin;
pub mod scheduler;
pub mod structures;

pub use architecture::{
    ArchitectureEngine, CpuRegisters, HardwareException,
    InstructionCyclePhase as ArchInstructionCyclePhase, Irql, LookasideList, MemoryDescriptorList,
    Pcb, PoolType, ProcessorInitState, Tcb, ThreadState,
};
pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_bsd_innovations::{
    BsdPfStateTable, FreeBsdVfsNullfs, FutexOp, FutexWaiter, LinuxFutexEngine,
    PfFiveTuple, PfStateEntry, SovereignCgroupGovernor, CgroupResourceLimits,
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
