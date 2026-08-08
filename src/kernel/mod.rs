// SigmaOS Kernel Module
<<<<<<< HEAD
pub mod breakthroughs;
||||||| 23ef22a4a
pub mod breakthroughs;
pub mod breakthroughs_v2;
pub mod exports;
=======
pub mod bore;
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
pub mod ipc;
pub mod memory;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
<<<<<<< HEAD
pub mod roundrobin;
pub mod scheduler;
pub mod traits;
pub mod virtual_cpu;
pub mod wdk_core;
||||||| 23ef22a4a
pub mod roundrobin;
pub mod scheduler;
pub mod object;
=======
pub mod breakthroughs;
pub mod linux_absorb;
pub mod subsystem;
pub mod pci_scanner;
pub mod signal_dispatcher;
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e

pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
};
pub use device::{Device, DeviceBinding, DeviceManager, DeviceType, DriverError, DriverMetadata};
pub use driver::{Driver, DriverRegistration, DriverRegistry};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use policy_mechanism::{
    AdaptivePolicy, InstructionCyclePhase, InterruptClass, IoWaitProfile, KernelMechanism,
    KernelPolicy, PolicyMechanismCoordinator, SovereignMechanism,
};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
<<<<<<< HEAD
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
||||||| 23ef22a4a
pub use structures::{
    Apc, ApcMode, ApcQueue, CircularDoublyLinkedList, CpuContext,
    SequencedSinglyLinkedList, SinglyLinkedList, SystemThread, ThreadState, WorkItem,
};
=======
pub use virtual_cpu::{CpuError, CpuMode, CpuRing, RegisterSet, SovereignVirtualCPU};
pub use pci_scanner::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use signal_dispatcher::{SignalDispatcher, SovereignSignal};
pub use paging::{PagingController, SimplePageTableEntry, PAGE_SIZE_BYTES, MAX_PHYSICAL_FRAMES};
pub use ipc::{SovereignIpcBus, IpcTransactionMessage, MAX_IPC_MESSAGE_SIZE, IPC_QUEUE_CAPACITY};
>>>>>>> origin/jules-14967948003256892231-7e7b3d2e
