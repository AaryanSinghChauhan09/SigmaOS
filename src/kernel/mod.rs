// SigmaOS Kernel Module — Phase K: IPv4 stack, TCP, Block I/O, Page Cache, Crypto
#![allow(dead_code, unused_imports, clippy::all)]

// ── Core kernel primitives ─────────────────────────────────────────────────
pub mod cpu_features;
pub mod cpufreq;
pub mod gap_filling;
pub mod ipc;
pub mod linux_absorb;
pub mod memory;
pub mod numa_allocator;
pub mod performance;
pub mod profiler;
pub mod roundrobin;
pub mod scheduler;
pub mod secure_free;
pub mod slab_allocator;
pub mod subsystem;
pub mod traits;
pub mod watchdog;

// ── Phase J: subsystem registry & legacy device drivers ───────────────────
pub mod drivers;
pub mod subsystems;

// ── Phase J: process management (fork/exec/signals/namespaces/cgroups) ────
pub mod proc;

// ── Phase J: advanced memory management (SLAB/vmalloc/HugePages/OOM/NUMA) ─
pub mod mm;

// ── Phase J: kernel filesystems (proc_fs / sysfs / devtmpfs) ─────────────
pub mod fs;

// ── Phase J: interrupt infrastructure (IRQ domain / softirq / workqueue) ──
pub mod irq;

// ── Phase J: power management (CPUfreq / thermal / suspend-resume) ─────────
pub mod power;

// ── Phase J: network socket layer / netfilter / traffic control ────────────
pub mod net;

// ── Phase K: block device I/O layer (bio, elevator, blk-mq) ───────────────
pub mod block_dev;

// ── Phase K: crypto subsystem (SHA-256, HMAC, AES, PBKDF2, CSPRNG) ────────
pub mod crypto;

// ── Phase K: syscall table (POSIX-300 + SigmaOS extensions) ─────────────
pub mod syscall;

// ── Core re-exports ────────────────────────────────────────────────────────
pub use cpu_features::{CpuInstructionExtension, SovereignCompilerOptimizer};
pub use cpufreq::{CpufreqManager, CpufreqPolicy, CpufreqStats, GovernorType};
pub use gap_filling::{
    IpcMessage, PageDirectoryController, PageDirectoryEntry, SignalDispatcher, SovereignIpcBus,
    SovereignSignal,
};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_absorb::{
    AbsorbedBuddyAllocator, AbsorbedCfsScheduler, AbsorbedDriverInfo, AbsorbedExt4Driver,
    AbsorbedTcpStack, AbsorbedUsbHidDriver, AbsorptionError, AbsorptionStatus, ConversionRule,
    ConversionRuleType, LinuxAbsorptionEngine, SecurityHardeningLevel, SecurityPolicy,
    SecurityRestriction,
};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use numa_allocator::{AllocationPolicy, NodeState, NumaAllocator, NumaNode};
pub use performance::{
    IpcError as PerfIpcError, ProcessProfile, SchedInstruction, SchedOpcode, UdfSchedVm,
    ZeroCopyQueue,
};
pub use profiler::{KernelProfiler, ProfileEntry, ProfilerStatistics, ScopeTimer, Timer};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler as SovereignScheduler};
pub use secure_free::{SanitizationLevel, SecureFreeDetector, SecureFreeStats};
pub use slab_allocator::{SlabAllocator, SlabCache, SlabCacheStats, SlabState};
pub use subsystem::{
    DeviceDriver, DriverError, DriverMetadata, DriverRegistry, DriverType, FileFlags, FileHandle,
    FileSystem, FsError, IoOperation, IoResult, LinuxHeritage, MapFlags, MemoryError,
    MemoryManager, NetworkError, NetworkStack, Scheduler, SchedulerError, SecureDriverWrapper,
    SocketDomain, SocketHandle, SocketProtocol, SocketType,
};
pub use traits::{
    DeviceDriver as TraitsDeviceDriver, DriverError as TraitsDriverError,
    DriverMetadata as TraitsDriverMetadata, FileSystem as TraitsFileSystem,
    FilesystemMetadata as TraitsFilesystemMetadata, FsError as TraitsFsError,
    MemoryError as TraitsMemoryError, MemoryManager as TraitsMemoryManager,
    MemoryManagerMetadata as TraitsMemoryManagerMetadata, NetworkError as TraitsNetworkError,
    NetworkStack as TraitsNetworkStack, NetworkStackMetadata as TraitsNetworkStackMetadata,
    Scheduler as TraitsScheduler, SchedulerError as TraitsSchedulerError,
    SchedulerMetadata as TraitsSchedulerMetadata,
    DriverRegistry, DriverType, FileFlags, FileHandle, IoOperation, IoResult, LinuxHeritage,
    MapFlags, SecureDriverWrapper, SocketDomain, SocketHandle, SocketProtocol,
};
pub use watchdog::{
    HardwareMonitor, MonitorThreshold, WatchdogAction, WatchdogDevice, WatchdogManager,
    WatchdogState,
};

// ── Phase J + K consolidated re-exports ────────────────────────────────────
pub use proc::{
    CgroupManager, Namespace, NamespaceManager, NamespaceType, ProcessLifecycleManager,
    ResourceLimits, Signal, SignalHandler, SignalManager,
};
// mm: single export covering both Phase J and Phase K additions
pub use fs::{DevTmpFs, DeviceClass, ProcFileSystem, SysfsTree};
pub use irq::{
    ControllerType, IRQController, IRQHandler, IrqDomain, SoftirqEngine, SoftirqType, Work,
    Workqueue,
};
pub use mm::{
    CachedPage, HugePageManager, HugePageSize, NumaNode as MmNumaNode, NumaTopologyManager, OomKiller, PageCache,
    PageStatus, SlabAllocator as MmSlabAllocator, VmallocManager,
};
pub use power::{
    CpufreqGovernor, CpufreqManager as PowerCpufreqManager, CpufreqPolicy as PowerCpufreqPolicy, PowerStateManager, SleepState, ThermalManager,
    ThermalZone,
};
// net: single export covering Phase J (socket/netfilter/tc) + Phase K (IPv4/TCP)
pub use block_dev::{
    Bio, BioOp, BlockDeviceManager, DeadlineScheduler, RamDisk, BLOCK_SIZE, SECTOR_SIZE,
};
pub use crypto::{
    hmac_sha256, pbkdf2_hmac_sha256, sha256, Aes128, CryptoAlgorithm, CryptoEngine, SigmaCsprng,
};
pub use net::{
    AddressFamily, ArpTable, CongestionAlgorithm, Ipv4Header, Ipv4Stack, NetfilterTable,
    NfHookpoint, NfRule, NfVerdict, Pfifo, PfifoFast, Protocol, QPacket, Route, RoutingTable, Sfq,
    SockAddrIn, SocketLayer, SocketType as NetSocketType, Tbf, TcpConnection, TcpSegment, TcpState,
};
pub use syscall::{SyscallArgs, SyscallError, SyscallNr, SyscallResult, SyscallTable};
