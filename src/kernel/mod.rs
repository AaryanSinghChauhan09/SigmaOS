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
pub mod paging;
pub mod policy_mechanism;
pub mod roundrobin;
pub mod sched;
pub mod scheduler;
pub mod subsystem;
pub mod vmm_paging;
pub mod processor_management;
pub mod cpufreq;
pub mod structures;
pub mod object;

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
    AdaptivePolicy, InstructionCyclePhase, InterruptClass, IoWaitProfile, KernelMechanism,
    KernelPolicy, PolicyMechanismCoordinator, SovereignMechanism,
};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use structures::{
    AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, AuditBlock, CircularDoublyLinkedList,
    CpuArchitectureClass, EdfTask, LcgRandom, LotteryTask, SequencedSinglyLinkedList,
    SinglyLinkedList, SystemThread, WorkItem,
};
pub use component::{Component, ComponentTree, ComponentId, ComponentState, CapabilityHandle, CapabilityRights, ComponentError, ResourceType, ResourceAllocation};
