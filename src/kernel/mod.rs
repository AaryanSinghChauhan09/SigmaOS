// SigmaOS Kernel Module
// Core working components
pub mod memory;
pub mod scheduler;
pub mod cpu_features;

pub use cpu_features::{ApsrFlags, ArmExecutionState};
pub mod object;
pub mod proc;

// Genode-style Component Tree Architecture
pub mod component;
pub mod generation_manager;
pub mod ipc;
pub mod linux_absorb;
pub mod linux_bsd_innovations;
pub mod linux_parity;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod roundrobin;
pub mod sched;
pub mod traits;
pub mod virtual_cpu;
pub mod wdk_core;

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
// pub use policy_mechanism::{
//     AdaptivePolicy, InstructionCyclePhase, InterruptClass, IoWaitProfile, KernelMechanism,
//     KernelPolicy, PolicyMechanismCoordinator, SovereignMechanism,
// };
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use traits::SchedulerError;
pub use virtual_cpu::{CpuError as VirtualCpuError, CpuMode, CpuRing, RegisterSet, SovereignVirtualCPU, Instruction};
pub use wdk_core::{
    Irql, CpuArch, SecurityToken, AddressSpace, ExecutionContext,
    ThreadState, ApcMode, Apc, Dpc, WorkItem, WdkThread,
    EventType, EventObject, SpinLock, MutexObject, FastMutex, GuardedMutex, EResource,
    WdkTimer, TimerTable, Prcb,
    PoolType, PoolAllocation, KernelPoolMemory,
    IoStatusBlock, IoctlControl, IRP, WdkDriverObject, BugCheckData, BugCheckRegistry,
};
