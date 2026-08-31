// SigmaOS Kernel Module
#![allow(ambiguous_glob_reexports)]

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
pub mod crashdump_netconsole;
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
pub mod module_loader;
pub mod net;
pub mod numa_allocator;
pub mod numa_scheduler;
pub mod object;
pub mod os_innovations;
pub mod paging;
pub mod performance;
pub mod policy_mechanism;
pub mod roundrobin;
pub mod sched;
pub mod scheduler;
pub mod structures;
pub mod subsystem;
pub mod virtual_cpu;
pub mod vmm_paging;

pub use architecture::*;
pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
};
pub use bus::*;
pub use crashdump_netconsole::{
    CpuRegisterState, KdumpConfig, KdumpHeader, NetconsoleConfig, PersistentCrashDump,
    SovereignKdumpManager, SovereignNetconsole,
};
pub use gap_closing::{
    AcpiInterruptManager, GapError, IrqRoutingTable, JournalBlock, JournalState, MetadataJournal,
    Pml4PageTableEntry, VirtualMemoryPagingManager,
};
pub use generation_manager::{Generation, GenerationManager};
pub use io_uring::{CompletionQueueEntry, IoUringEngine, IoUringOpcode, SubmissionQueueEntry};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_bsd_innovations::*;
pub use linux_bsd_innovations::{
    AlpineHardenedEnv, AndroidBinderIpc, AndroidBroadcastReceiverRegistry, ArchUserRepoManager,
    BinderNode, BottomHalfKernelThread, BoundedBufferProducerConsumer, BroadcastReceiver,
    BsdPfStateTable, CapabilityDerivationTree, CarpSecurityRouter, CgroupResourceLimits, CowBlock,
    CowStorageEngine, CpuIsaMicroarch, DevlinkHealthReporter, DynamicLkmLoader, EbpfInstruction,
    EbpfRuntime, ExokernelHardwareMultiplexer, FastPacketFrame, FreeBsdCapsicumEngine,
    FreeBsdGeomTopology, FreeBsdJail, FreeBsdVfsNullfs, FreeBsdVnetManager, FutexOp, FutexWaiter,
    GcdDispatchQueue, GcdPriority, GcdTask, GentooUseFlags, GeomClass, GeomProvider,
    Hammer2PfsSnapshot, HammerBlockTransaction, HammerHistoryFilesystem, HurdTranslator,
    HybridKernelManager, HybridTask, IntelClearLinuxStatelessEngine, InteractiveHybridScheduler,
    KernelAccessController, KernelCapability, KernelFastPacketEngine, KernelModule, KmdfDriver,
    KmdfIoRequest, KmdfPnpState, KmdfPowerState, LandlockAccessRight, LandlockPathRule,
    LinuxDevlinkHealthMonitor, LinuxFutexEngine, LinuxLandlockLsmRuleEngine,
    MemoryCompactionSuperpagesAllocator, MicrokernelCore, MicrokernelTranslatorRegistry,
    MultikernelMessage, MultikernelMessagePassing, NamespaceType, NanokernelHardwareBroker,
    NanokernelIrq, NetBsdRumpKernel, NinePProtocolTranslator, NinePResource,
    NixOsDeclarativeManager, NtExecutiveService, NullfsLayerNode, OpenBsdPledge,
    OpenBsdUnveilEngine, OpenSuseSnapperEngine, PfFiveTuple, PfStateEntry, PhysicalFrameBlock,
    ReactorEvent, ReactorRegistration, ResourceBinding, RumpComponent, SnapperSnapshot,
    SoftIrqType, SovereignCgroupGovernor, SovereignEventReactor, SovereignNamespaceContainer,
    SovereignSwapEngine, SovereignZone, SovereignZonesManager, SwapDeviceConfig, SwapPage,
    UnveilPathRule, VnetNetworkStack, VoidLinuxRunitSupervisor, VoidRunitInit, VoidRunitService,
    VoidRunitStage, XdpAction, ZramCompressedPage, CAP_MMAP_FLAG, CAP_READ_FLAG, CAP_SEEK_FLAG,
    CAP_WRITE_FLAG, PLEDGE_CPATH, PLEDGE_DPATH, PLEDGE_EXEC, PLEDGE_INET, PLEDGE_RPATH,
    PLEDGE_STDIO, PLEDGE_UNIX, PLEDGE_WPATH,
};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use meta::{
    ABIManager, KernelGraph, KernelPersona, KernelPlugin, KernelPluginManager, LegacyScheduler,
    MetaKernel, MicroDriver, NetPod,
};
pub use paging::{PageTable, PageTableEntry, PageTableFlags, VirtualMemoryManagerV2};
pub use policy_mechanism::*;
pub use policy_mechanism::{
    FastPathIpc, InterruptMechanism, PolicyError, PolicyManager, PrivilegeLevel, ProtectionDomain,
    ResourceBroker,
};
pub use roundrobin::{
    RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError,
};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use structures::*;
pub use virtual_cpu::SovereignVirtualCPU as VirtualCpu;
pub use vmm_paging::{PageTableManager, VirtualMemoryManager};
