// SigmaOS Kernel Module
// Core working components
pub mod memory;
pub mod scheduler;
pub mod object;
pub mod proc;
pub mod component;
pub mod architecture;
pub mod structures;
pub mod policy_mechanism;
pub mod ipc;
pub mod roundrobin;

// Working exports
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use component::{Component, ComponentTree, ComponentId, ComponentState, CapabilityHandle, CapabilityRights, ComponentError, ResourceType, ResourceAllocation};
pub use architecture::{
    ArchitectureEngine, CpuArchitectureClass, CpuRegisters, HardwareException, InstructionCyclePhase,
    InterruptClass, IoWaitProfile, Irql, LookasideList, MemoryDescriptorList, Pcb, PoolType, ProcessorInitState,
    Tcb, ThreadState,
};
pub use structures::{
    AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, AuditBlock, CircularDoublyLinkedList, EdfTask,
    LcgRandom, LotteryTask, SequencedSinglyLinkedList, SinglyLinkedList, SystemThread, WorkItem,
};
pub use policy_mechanism::{
    AdaptivePolicy, KernelMechanism, KernelPolicy, PolicyMechanismCoordinator,
    SovereignMechanism,
};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
