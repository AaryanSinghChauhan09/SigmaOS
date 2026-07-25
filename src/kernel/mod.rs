// SigmaOS Kernel Module
pub mod bus;
pub mod device;
pub mod driver;
pub mod memory;
pub mod object;
pub mod performance;
pub mod roundrobin;
pub mod sched;
pub mod vfs;

pub use boot::firmware::{
    BootLoader, BootParams, FirmwareInterface, Initramfs, KernelCommandLine, SetupHeader,
};
pub use bus::{Bus, PciBus, UsableBus};
pub use container::runtime::{
    Container, ContainerManager, ContainerState, NamespaceConfig, NamespaceSet, OciSpec,
    ResourceConfig, Runtime,
};
pub use device::{Device, DeviceBinding, DeviceManager, DeviceType, DriverError, DriverMetadata};
pub use driver::{Driver, DriverRegistration, DriverRegistry};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_absorb::{
    AbsorbedBuddyAllocator, AbsorbedCfsScheduler, AbsorbedDriverInfo, AbsorbedExt4Driver,
    AbsorbedTcpStack, AbsorbedUsbHidDriver, AbsorptionError, AbsorptionStatus, ConversionRule,
    ConversionRuleType, LinuxAbsorptionEngine, SecurityHardeningLevel, SecurityPolicy,
    SecurityRestriction,
};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use memory::{Page, VmArea, VmSpace, Zone, ZonedPageAllocator};
pub use net::stack::{
    BbrCongestionControl, CongestionControl, NetDevice, Netfilter, NetfilterRule, PfifoFast, Qdisc,
    QdiscManager, RenoCongestionControl, SkBuff, Socket,
};
pub use numa_allocator::{AllocationPolicy, NodeState, NumaAllocator, NumaNode};
pub use object::{KObject, KRef, KernelObject};
pub use package::manager::{
    Generation, PackageMetadata, SigmaPackageManager, SystemConfig, SystemProfile,
};
pub use performance::{
    IpcError as PerfIpcError, ProcessProfile, SchedInstruction, SchedOpcode, UdfSchedVm,
    ZeroCopyQueue,
};
pub use profiler::{KernelProfiler, ProfileEntry, ProfilerStatistics, ScopeTimer, Timer};
pub use roundrobin::{
    RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError,
};
pub use sched::scheduler::{RunQueue, SchedClass, Scheduler};
pub use sched::task::{Cred, ProcessState, SchedPolicy, Task};
pub use scheduler::{Priority, Process, ProcessState, Scheduler as SovereignScheduler};
pub use secure_free::{SanitizationLevel, SecureFreeDetector, SecureFreeStats};
pub use security::lsm::{
    AuditLog, AvcCache, CapabilitySet, Label, LsmHook, MacPolicy, SecurityTask,
};
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
pub use vfs::*;
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
    CachedPage, HugePageManager, HugePageSize, NumaNode as MmNumaNode, NumaTopologyManager,
    OomKiller, PageCache, PageStatus, SlabAllocator as MmSlabAllocator, VmallocManager,
};
pub use power::{
    CpufreqGovernor, CpufreqManager as PowerCpufreqManager, CpufreqPolicy as PowerCpufreqPolicy,
    PowerStateManager, SleepState, ThermalManager, ThermalZone,
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
