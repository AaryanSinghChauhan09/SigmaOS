// SigmaOS Kernel Module
// Core working components
pub mod memory;
pub mod scheduler;
pub mod object;
pub mod proc;
pub mod architecture;
pub mod structures;
pub mod policy_mechanism;
pub mod ipc;
pub mod roundrobin;

// Genode-style Component Tree Architecture
pub mod breakthroughs;
pub mod bus;
pub mod component;
pub mod generation_manager;
pub mod linux_absorb;
pub mod linux_bsd_innovations;
pub mod linux_parity;
pub mod meta;
pub mod paging;
pub mod sched;
pub mod subsystem;
pub mod vmm_paging;
pub mod processor_management;
pub mod cpufreq;

pub use vmm_paging::{PageTableFlags as VmmPageFlags, PageTableManager as VmmPageTableManager, VirtualMemoryManager as VmmManager, VmArea, VmProtection};

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
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use policy_mechanism::{
    AdaptivePolicy, FastPathIpc, InterruptClass, InterruptMechanism, IoWaitProfile, KernelMechanism,
    KernelPolicy, PolicyError, PolicyManager, PolicyMechanismCoordinator, PrivilegeLevel,
    ProtectionDomain, ResourceBroker, SovereignMechanism,
};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState};
pub use structures::{
    AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, AuditBlock, CpuArchitectureClass,
    CircularDoublyLinkedList, CpuContext, EdfTask, IrqlLevel, IrqlState, LcgRandom, LotteryTask,
    SequencedSinglyLinkedList, SinglyLinkedList, SystemThread, ThreadState, WorkItem,
};
pub use component::{Component, ComponentTree, ComponentId, ComponentState, CapabilityHandle, CapabilityRights, ComponentError, ResourceType, ResourceAllocation};
