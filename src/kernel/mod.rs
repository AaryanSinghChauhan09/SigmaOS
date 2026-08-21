#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

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
pub mod component;

// Temporarily disabled problematic modules
// pub mod breakthroughs;
// pub mod meta;
// pub mod paging;
// pub mod self_healing;
// pub mod udkf;

// Working exports
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use component::{Component, ComponentTree, ComponentId, ComponentState, CapabilityHandle, CapabilityRights, ComponentError, ResourceType, ResourceAllocation};
pub use architecture::{
    InstructionCyclePhase, ProcessorInitState, CpuRegisters, Irql, HardwareException,
    PoolType, LookasideList, MemoryDescriptorList, ThreadState, Tcb, Pcb, ArchitectureEngine,
};
pub use structures::{
    Apc, ApcMode, ApcQueue, AdvancedAlgorithmsManager, AuditBlock, CircularDoublyLinkedList,
    CpuArchitectureClass, LcgRandom, SequencedSinglyLinkedList,
    SinglyLinkedList, SystemThread, WorkItem, LotteryTask, EdfTask,
};
pub use policy_mechanism::{
    AdaptivePolicy, KernelMechanism, KernelPolicy, PolicyMechanismCoordinator, SovereignMechanism,
    InterruptClass, IoWaitProfile,
};
pub use ipc::{Channel, Message, IpcManager, IpcError};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
