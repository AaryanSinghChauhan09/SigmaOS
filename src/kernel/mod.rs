// SigmaOS Kernel Module
pub mod breakthroughs;
pub mod bus;
pub mod device;
pub mod driver;
pub mod ipc;
pub mod linux_absorb;
pub mod memory;
pub mod meta;
pub mod object;
pub mod paging;
pub mod performance;
pub mod policy_mechanism;
pub mod roundrobin;
pub mod scheduler;
pub mod subsystem;

pub use crate::boot::firmware::{
    BootLoader, BootParams, FirmwareInterface, Initramfs, KernelCommandLine, SetupHeader,
};
pub use crate::container::runtime::oci::{
    Container, ContainerManager, ContainerState, NamespaceConfig, NamespaceSet, OciSpec,
    ResourceConfig, Runtime,
};
pub use breakthroughs::{
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
};
pub use bus::{Bus, PciBus, UsableBus};
pub use device::{Device, DeviceBinding, DeviceManager, DeviceType, DriverError, DriverMetadata};
pub use driver::{Driver, DriverRegistration, DriverRegistry};
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use linux_absorb::{
    AbsorbedBuddyAllocator, AbsorbedCfsScheduler, AbsorbedDriverInfo, AbsorbedExt4Driver,
    AbsorbedTcpStack, AbsorbedUsbHidDriver, AbsorptionError, AbsorptionStatus, ConversionRule,
    ConversionRuleType, KernelModule, KpatchManager, KpatchPatch, LinuxAbsorptionEngine, LkmLoader,
    ModuleLoadError, SecurityHardeningLevel, SecurityPolicy, SecurityRestriction,
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
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
