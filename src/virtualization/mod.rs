#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Virtualization Module
pub mod kvm_vcpu;
pub mod oci_pod;
pub mod orchestration;
pub mod rancher;

pub use kvm_vcpu::{
    KvmExitCode, KvmMemoryRegion, KvmVcpu, KvmVcpuRegisters, KvmVcpuSregs, VirtioDeviceBackend,
    VirtioDeviceType,
};

pub use orchestration::{
    Container, KubernetesPod, LegacyVirtualizationStrategy, ModernVirtualizationStrategy,
    ResourcePool, VirtualMachine, VirtualizationError, VirtualizationOrchestrator,
    VirtualizationStrategy, VirtualizationStrategyFactory, VirtualizationTech, VmState,
};

pub use oci_pod::{ContainerConfig, OciPod, OciPodManager, PodState};
pub use rancher::{
    DaemonlessContainer, K3osOrchestrator, RancherError, RancherHarvesterVirtualMachineGovernor,
    RancherK3sEmbeddedClusterController, RancherSystemDockerEngine,
};
