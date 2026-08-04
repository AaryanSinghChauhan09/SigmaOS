// SigmaOS Kernel Module
pub mod breakthroughs;
pub mod bus;
pub mod device;
pub mod driver;
||||||| 43be3a7e8
pub mod ipc;
pub mod bore;
pub mod ipc;
pub mod memory;
pub mod object;
pub mod performance;
pub mod roundrobin;
pub mod scheduler;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod breakthroughs;
||||||| 165ded71c
pub mod memory;
pub mod object;
pub mod performance;
pub mod roundrobin;
pub mod scheduler;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod breakthroughs;
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
||||||| 43be3a7e8
pub mod virtual_cpu;
||||||| 0ddf2eac7
pub mod ipc;
pub mod linux_absorb;
pub mod subsystem;
||||||| 2139cb2f8
pub mod traits;

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
||||||| 43be3a7e8
pub use bore::{BoreScheduler, BoreTask};
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
||||||| 43be3a7e8
pub use virtual_cpu::{CpuError, CpuMode, CpuRing, RegisterSet, SovereignVirtualCPU};
||||||| 165ded71c
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
||||||| 2139cb2f8
pub use traits::SchedulerError;
