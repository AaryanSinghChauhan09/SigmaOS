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
pub mod policy_mechanism;
pub mod roundrobin;
pub mod sched;
pub mod scheduler;
pub mod traits;

pub use architecture::*;
pub use bus::*;
pub use linux_bsd_innovations::*;
pub mod structures;
pub use policy_mechanism::*;
pub use structures::*;
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
pub use roundrobin::{
    RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError,
};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub mod virtual_cpu;
pub mod vmm_paging;
pub use io_uring::{CompletionQueueEntry, IoUringEngine, IoUringOpcode, SubmissionQueueEntry};
pub use virtual_cpu::SovereignVirtualCPU as VirtualCpu;
pub use vmm_paging::{PageTableManager, VirtualMemoryManager};
pub use linux_bsd_innovations::{
    ArchUserRepoManager, BsdPfStateTable, PfFiveTuple, PfStateEntry,
    LinuxFutexEngine, FutexOp, FutexWaiter, FreeBsdVfsNullfs, NullfsLayerNode,
    AlpineHardenedEnv, OpenBsdPledge, BoundedBufferProducerConsumer,
    BottomHalfKernelThread, SoftIrqType, AndroidBroadcastReceiverRegistry, BroadcastReceiver,
    MultikernelMessagePassing, MultikernelMessage, NinePProtocolTranslator, NinePResource,
    MicrokernelTranslatorRegistry, HurdTranslator, NanokernelHardwareBroker, NanokernelIrq,
    SovereignZonesManager, SovereignZone, KmdfDriver, KmdfPnpState, KmdfPowerState, KmdfIoRequest,
    AndroidBinderIpc, BinderNode, GcdDispatchQueue, GcdPriority, GcdTask, EbpfRuntime, EbpfInstruction,
    HammerHistoryFilesystem, HammerBlockTransaction, CarpSecurityRouter, SovereignSwapEngine, SwapPage,
    SovereignNamespaceContainer, NamespaceType, SovereignEventReactor, ReactorRegistration, ReactorEvent,
    HybridKernelManager, NtExecutiveService, MicrokernelCore, ExokernelHardwareMultiplexer, ResourceBinding,
    NetBsdRumpKernel, RumpComponent, DynamicLkmLoader, KernelModule, CapabilityDerivationTree, KernelCapability,
    FreeBsdJail, NixOsDeclarativeManager, GentooUseFlags, VoidRunitInit,
    SovereignCgroupGovernor, CgroupResourceLimits,
    KernelFastPacketEngine, FastPacketFrame, XdpAction,
    KernelAccessController, LandlockPathRule, LandlockAccessRight, PLEDGE_STDIO, PLEDGE_RPATH, PLEDGE_WPATH, PLEDGE_CPATH, PLEDGE_DPATH, PLEDGE_INET, PLEDGE_UNIX, PLEDGE_EXEC,
    InteractiveHybridScheduler, HybridTask,
    CowStorageEngine, CowBlock, Hammer2PfsSnapshot,
    MemoryCompactionSuperpagesAllocator, PhysicalFrameBlock,
};
