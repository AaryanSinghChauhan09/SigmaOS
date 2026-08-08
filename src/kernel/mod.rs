// SigmaOS Kernel Module
pub mod bore;
pub mod ipc;
pub mod memory;
pub mod roundrobin;
pub mod scheduler;
pub mod virtual_cpu;
||||||| 43be3a7e8
pub mod self_healing;
pub mod udkf;
pub mod breakthrough;

pub use bore::{BoreScheduler, BoreTask};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use virtual_cpu::{CpuError, CpuMode, CpuRing, RegisterSet, SovereignVirtualCPU};
||||||| 43be3a7e8
pub use self_healing::{
    SovereignSelfHealingKernel,
};
pub use breakthrough::{
    SovereignKernelModuleSystem, SovereignKernelModule, ModuleState, SigmaSignal, ProcessProvenanceNode, PredictiveScheduler, AdaptiveRoot, ThreatLevel,
};
pub use udkf::{
    UdkfHook, UserDefinedKernelFunctions,
};
