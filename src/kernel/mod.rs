#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Kernel Module
pub mod breakthroughs;
pub mod ipc;
pub mod memory;
pub mod object;
pub mod performance;
pub mod sched;
pub mod vfs;
pub mod roundrobin;
pub mod scheduler;
pub mod meta;
pub mod paging;
pub mod policy_mechanism;
pub mod breakthroughs;
pub mod ipc;
pub mod linux_absorb;
pub mod subsystem;

pub use crate::boot::firmware::{
    BootLoader, BootParams, FirmwareInterface, Initramfs, KernelCommandLine, SetupHeader,
};
pub use bus::{Bus, PciBus, UsableBus};
pub use crate::container::runtime::oci::{
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
    SecurityRestriction, ModuleLoadError, KernelModule, LkmLoader, KpatchPatch, KpatchManager,
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
    AiNativeRuntime, EnergyAwareScheduler, PrivacyFirstSandbox, SelfHealingKernel, SigmaFsPlusPlus,
    UniversalAbiTranslator, UserDefinedKernelFunctions,
    DeterministicReplayEngine, DynamicKernelPersonalitySwitcher, InterruptRatePredictor,
    KernelPersonalityMode,
};
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
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError as RoundRobinSchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use numa_scheduler::{NumaTask, LockFreeTaskQueue, NumaNode, NumaScheduler};
