// SigmaOS Kernel Module
pub mod architecture;
pub mod atomic_extended;
pub mod block_dev;
pub mod bore;
pub mod breakthrough;
pub mod breakthroughs;
pub mod breakthroughs_v2;
pub mod bus;
pub mod classic_os;
pub mod component;
pub mod console;
pub mod cpu_features;
pub mod cpufreq;
pub mod device;
pub mod driver;
pub mod dtrace_compat;
pub mod ebpf;
pub mod exports;
pub mod gap_closing;
pub mod gap_filling;
pub mod generation_manager;
pub mod io_uring;
pub mod ipc;
pub mod kqueue;
pub mod linux_absorb;
pub mod linux_bsd_innovations;
pub mod linux_parity;
pub mod memory;
pub mod meta;
pub mod net;
pub mod numa_allocator;
pub mod numa_scheduler;
pub mod object;
pub mod os_innovations;
pub mod paging;
pub mod pci_scanner;
pub mod perf_mm;
pub mod performance;
pub mod policy_mechanism;
pub mod process;
pub mod processor_management;
pub mod profiler;
pub mod roundrobin;
pub mod scheduler;
pub mod secure_free;
pub mod self_healing;
pub mod sigma_kernel_autotuner;
pub mod signal_dispatcher;
pub mod slab_alloc;
pub mod slab_allocator;
pub mod structures;
pub mod subsystem;
pub mod sysctl;
pub mod traits;
pub mod tty;
pub mod udkf;
pub mod unix_primitives;
pub mod user_defined;
pub mod virtual_cpu;
pub mod vmm_paging;
pub mod watchdog;
pub mod wdk_core;

pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
};
pub use gap_closing::{
    AcpiInterruptManager, GapError, IrqRoutingTable, JournalBlock, JournalState, MetadataJournal,
    Pml4PageTableEntry, VirtualMemoryPagingManager,
};
pub use generation_manager::{Generation, GenerationManager};
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
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use virtual_cpu::SovereignVirtualCPU as VirtualCpu;
pub use vmm_paging::{PageTableManager, VirtualMemoryManager};
