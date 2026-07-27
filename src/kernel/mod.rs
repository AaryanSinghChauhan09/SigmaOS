// SigmaOS Kernel Module
pub mod bus;
pub mod device;
pub mod driver;
pub mod memory;
pub mod object;
pub mod performance;
pub mod roundrobin;
pub mod scheduler;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod breakthroughs;
pub mod sigma_kernel_autotuner;

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
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use meta::{
    MetaKernel, KernelPersona, KernelPlugin, KernelPluginManager, MicroDriver,
    ABIManager, NetPod, KernelGraph, LegacyScheduler,
};
pub use paging::{
    PageTable, PageTableEntry, PageTableFlags, VirtualMemoryManagerV2,
};
pub use policy_mechanism::{
    ResourceBroker, PolicyManager, ProtectionDomain, InterruptMechanism, FastPathIpc,
    PrivilegeLevel, PolicyError,
};
pub use breakthroughs::{
    UniversalAbiTranslator, SigmaFsPlusPlus, SelfHealingKernel, AiNativeRuntime,
    EnergyAwareScheduler, UserDefinedKernelFunctions, PrivacyFirstSandbox,
};
