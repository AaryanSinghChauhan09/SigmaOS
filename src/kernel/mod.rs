// SigmaOS Kernel Module
pub mod breakthroughs;
pub mod gap_closing;
pub mod generation_manager;
pub mod ipc;
pub mod memory;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
#[cfg(any())]
pub mod breakthroughs;
#[cfg(any())]
pub mod ipc;
pub mod linux_absorb;
pub mod subsystem;
// pub mod traits;
pub mod watchdog;

pub use crate::boot::firmware::{
    BootLoader, BootParams, FirmwareInterface, Initramfs, KernelCommandLine, SetupHeader,
};
pub use bus::{Bus, PciBus, UsableBus};
pub use crate::container::runtime::oci::{
    Container, ContainerManager, ContainerState, NamespaceConfig, NamespaceSet, OciSpec,
    ResourceConfig, Runtime,
};
pub use generation_manager::{Generation, GenerationManager};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_absorb::{
    AbsorbedBuddyAllocator, AbsorbedCfsScheduler, AbsorbedDriverInfo, AbsorbedExt4Driver,
    AbsorbedTcpStack, AbsorbedUsbHidDriver, AbsorptionEngine as LinuxAbsorptionEngine,
    AbsorptionError, AbsorptionStatus, ConversionRule, ConversionRuleType,
    SecurityHardeningLevel, SecurityPolicy, SecurityRestriction,
};
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
pub use profiler::{KernelProfiler, ProfileEntry, ProfilerStatistics, ScopeTimer, Timer};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use secure_free::{SanitizationLevel, SecureFreeDetector, SecureFreeStats};
pub use slab_allocator::{SlabAllocator, SlabCache, SlabCacheStats, SlabState};
pub use subsystem::{
    DeviceDriver, DriverError, DriverMetadata, DriverRegistry, DriverType, FileFlags, FileHandle,
    FileSystem, FilesystemMetadata, FsError, IoOperation, IoResult, LinuxHeritage, MapFlags, MemoryError,
    MemoryManager, MemoryManagerMetadata, NetworkError, NetworkStack, NetworkStackMetadata, SchedulerError, SchedulerMetadata, SecureDriverWrapper,
    SocketDomain, SocketHandle, SocketProtocol, SocketType,
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
