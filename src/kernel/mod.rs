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
pub mod subsystem;
pub mod vmm_paging;
pub mod processor_management;
pub mod cpufreq;
pub mod structures;
pub mod object;
pub mod performance;

pub use cpu_features::{ApsrFlags, ArmExecutionState};
pub mod linux_bsd_innovations;
pub use linux_bsd_innovations::{
    BoundedBufferProducerConsumer, SoftIrqType, BottomHalfKernelThread, BroadcastReceiver,
    AndroidBroadcastReceiverRegistry,
};
pub use architecture::{SovereignSystemBus, IoModuleController};
pub mod object;
pub mod proc;

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
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState};
pub use structures::{
    AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, AuditBlock, CircularDoublyLinkedList,
    CpuContext, EdfTask, IrqlLevel, IrqlState, LcgRandom, LotteryTask,
    SequencedSinglyLinkedList, SinglyLinkedList, SystemThread, WorkItem,
};
pub use component::{Component, ComponentTree, ComponentId, ComponentState, CapabilityHandle, CapabilityRights, ComponentError, ResourceType, ResourceAllocation};
