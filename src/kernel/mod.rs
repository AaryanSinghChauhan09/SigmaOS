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
pub mod architecture;
pub mod breakthroughs;
pub mod component;
pub mod ipc;
pub mod linux_bsd_innovations;
pub mod memory;
pub mod meta;
pub mod object;
pub mod paging;
pub mod policy_mechanism;
pub mod proc;
pub mod roundrobin;
pub mod scheduler;
pub mod self_healing;
pub mod structures;
pub mod udkf;

pub use architecture::{
    ArchitectureEngine, CpuArchitectureClass, CpuRegisters, HardwareException, Irql,
    LookasideList, MemoryDescriptorList, Pcb, PoolType, ProcessorInitState, SystemServiceDescriptorTable,
    SyscallHandler, Tcb, ThreadState as ArchThreadState,
};
pub use breakthroughs::*;
pub use component::{CapabilityHandle, CapabilityRights, Component, ComponentError, ComponentId, ComponentState, ComponentTree, ResourceAllocation, ResourceType};
pub use ipc::*;
pub use linux_bsd_innovations::*;
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use meta::{
    ABIManager, KernelGraph, KernelPersona, KernelPlugin, KernelPluginManager, LegacyScheduler,
    MetaKernel, MicroDriver, NetPod,
};
pub use paging::{PageTable, PageTableEntry, PageTableFlags, VirtualMemoryManagerV2};
pub use policy_mechanism::*;
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler, SchedulerError};
pub use structures::*;
