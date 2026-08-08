// SigmaOS Kernel Module
pub mod bore;
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
pub mod linux_absorb;
pub mod subsystem;
pub mod pci_scanner;
pub mod signal_dispatcher;

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
pub use policy_mechanism::{
    AdaptivePolicy, InstructionCyclePhase, InterruptClass, IoWaitProfile, KernelMechanism,
    KernelPolicy, PolicyMechanismCoordinator, SovereignMechanism,
};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use virtual_cpu::{CpuError, CpuMode, CpuRing, RegisterSet, SovereignVirtualCPU};
pub use pci_scanner::{PciBusScanner, PciClass, PciDevice, PCI_MAX_BUS, PCI_MAX_DEVICE};
pub use signal_dispatcher::{SignalDispatcher, SovereignSignal};
pub use paging::{PagingController, SimplePageTableEntry, PAGE_SIZE_BYTES, MAX_PHYSICAL_FRAMES};
pub use ipc::{SovereignIpcBus, IpcTransactionMessage, MAX_IPC_MESSAGE_SIZE, IPC_QUEUE_CAPACITY};
