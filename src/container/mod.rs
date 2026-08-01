// SigmaOS Container & OCI-compatible Pod Subsystem Module

pub mod oci_runtime;
pub mod runtime;

pub use oci_runtime::{
    Container as OciContainer, ContainerError as OciContainerError, ContainerID as OciContainerID,
    ContainerRuntime as OciContainerRuntime, ContainerState as OciContainerState, Namespace,
    Sandbox, SimpleContainer as SimpleOciContainer,
    SimpleContainerRuntime as SimpleOciContainerRuntime, SimpleSandbox,
};
pub use runtime::{
    Container, ContainerCapability, ContainerError, ContainerID, ContainerInfo, ContainerRuntime,
    ContainerState, RuntimeCapability, RuntimeStats, SimpleContainer, SimpleContainerRuntime,
};

// Aliases to bridge standard Container and OCI Container namespaces without collision
pub use oci_runtime::{
    Namespace, Sandbox, SimpleSandbox,
    Container as OciContainer,
    ContainerError as OciContainerError,
    ContainerID as OciContainerID,
    ContainerRuntime as OciContainerRuntime,
    ContainerState as OciContainerState,
    SimpleContainer as SimpleOciContainer,
    SimpleContainerRuntime as SimpleOciContainerRuntime,
};
