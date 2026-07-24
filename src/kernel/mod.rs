// SigmaOS Kernel Module
pub mod device;
pub mod driver;
pub mod bus;
pub mod object;
pub mod vfs;
pub mod sched;
pub mod memory;
pub mod net;
pub mod container;
pub mod package;
pub mod security;
pub mod boot;
pub mod ipc;
pub mod roundrobin;

pub use device::{Device, DeviceType, DeviceManager, DeviceBinding, DriverError, DriverMetadata};
pub use driver::{Driver, DriverRegistration, DriverRegistry};
pub use bus::{Bus, PciBus, UsableBus};
pub use object::{KObject, KRef, KernelObject};
pub use vfs::*;
pub use sched::task::{Task, Cred, ProcessState, SchedPolicy};
pub use sched::scheduler::{Scheduler, RunQueue, SchedClass};
pub use memory::{Page, Zone, ZonedPageAllocator, VmArea, VmSpace};
pub use net::stack::{Socket, NetDevice, SkBuff, CongestionControl, RenoCongestionControl, BbrCongestionControl, Netfilter, NetfilterRule, Qdisc, PfifoFast, QdiscManager};
pub use container::runtime::{Container, ContainerState, Runtime, ContainerManager, NamespaceConfig, NamespaceSet, OciSpec, ResourceConfig};
pub use package::manager::{SigmaPackageManager, Generation, PackageMetadata, SystemConfig, SystemProfile};
pub use security::lsm::{MacPolicy, LsmHook, CapabilitySet, Label, SecurityTask, AvcCache, AuditLog};
pub use boot::firmware::{FirmwareInterface, BootLoader, BootParams, SetupHeader, Initramfs, KernelCommandLine};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_absorb::{
    AbsorbedBuddyAllocator, AbsorbedCfsScheduler, AbsorbedDriverInfo, AbsorbedExt4Driver,
    AbsorbedTcpStack, AbsorbedUsbHidDriver,
    AbsorptionError, AbsorptionStatus, ConversionRule, ConversionRuleType, LinuxAbsorptionEngine,
    SecurityHardeningLevel, SecurityPolicy, SecurityRestriction,
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
