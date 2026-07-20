// SigmaOS Kernel Module
pub mod ipc;
pub mod linux_absorb;
pub mod memory;
pub mod roundrobin;
pub mod scheduler;
pub mod subsystem;

pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_absorb::{
    AbsorbedBuddyAllocator, AbsorbedCfsScheduler, AbsorbedDriverInfo, AbsorbedExt4Driver,
    AbsorbedTcpStack, AbsorbedUsbHidDriver, AbsorptionError, AbsorptionStatus,
    AbsorptionEngine as LinuxAbsorptionEngine, ConversionRule, ConversionRuleType,
    LinuxAbsorptionEngine, SecurityHardeningLevel, SecurityPolicy, SecurityRestriction,
};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use subsystem::{
    DeviceDriver, DriverError, DriverMetadata, DriverRegistry, DriverType, FileSystem,
    FileFlags, FileHandle, FsError, IoOperation, IoResult, LinuxHeritage, MapFlags,
    MemoryError, MemoryManager, NetworkError, NetworkStack, Scheduler, SchedulerError,
    SecureDriverWrapper, SocketDomain, SocketHandle, SocketProtocol, SocketType,
};
