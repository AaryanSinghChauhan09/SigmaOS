// SigmaOS Virtualization Module
pub mod orchestration;
pub mod oci_pod;
pub mod advanced_virt;

pub use orchestration::{
    Container, KubernetesPod, LegacyVirtualizationStrategy, ModernVirtualizationStrategy,
    ResourcePool, VirtualMachine, VirtualizationError, VirtualizationOrchestrator,
    VirtualizationStrategy, VirtualizationStrategyFactory, VirtualizationTech, VmState,
};

pub use oci_pod::{
    ContainerConfig, OciPod, OciPodManager, PodState,
};

pub use advanced_virt::{VirtualMachine as AdvancedVirtualMachine, VirtualizationManager as AdvancedVirtualizationManager, VmState as AdvancedVmState};
