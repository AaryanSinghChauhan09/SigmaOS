// SigmaOS Virtualization Module
pub mod orchestration;

pub use orchestration::{
    Container, KubernetesPod, ResourcePool, VirtualMachine, VirtualizationError,
    VirtualizationOrchestrator, VirtualizationTech, VmState,
    VirtualizationStrategy, VirtualizationStrategyFactory, LegacyVirtualizationStrategy,
    ModernVirtualizationStrategy,
};
