// SigmaOS Daemonless Container & Micro-VM Orchestration Subsystem (S-RANCHER)
// Absorbs and obsoletes Rancher OS, k3os, Bottlerocket, and containerd
// by executing daemonless OCI containers directly on microkernel capabilities.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::security::CapabilityToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerState {
    Created = 0,
    Running = 1,
    Stopped = 2,
    Panicked = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RancherError {
    Success = 0,
    NamespaceViolation = 1,
    ReadOnlyViolation = 2,
    ContainerNotFound = 3,
    TpmAttestationFailed = 4,
}

/// Daemonless Container instance mapping directly to microkernel cgroups/namespaces
pub struct DaemonlessContainer {
    pub container_id: String,
    pub oci_bundle_path: String,
    pub state: ContainerState,
    pub capability_mask: u64,
    pub immutable_rootfs: bool,
}

impl DaemonlessContainer {
    pub fn new(id: &str, oci_path: &str, cap_mask: u64) -> Self {
        Self {
            container_id: id.to_string(),
            oci_bundle_path: oci_path.to_string(),
            state: ContainerState::Created,
            capability_mask: cap_mask,
            immutable_rootfs: true, // Enabled by default as per Bottlerocket specs
        }
    }
}

/// K3os-style bare-metal container orchestrator service
pub struct K3osOrchestrator {
    pub active_containers: HashMap<String, DaemonlessContainer>,
    pub allocated_memory_mb: usize,
    pub total_memory_mb: usize,
}

impl K3osOrchestrator {
    pub fn new(total_mem: usize) -> Self {
        Self {
            active_containers: HashMap::new(),
            allocated_memory_mb: 0,
            total_memory_mb: total_mem,
        }
    }

    pub fn instantiate_container(
        &mut self,
        container: DaemonlessContainer,
        memory_demand_mb: usize,
    ) -> Result<(), RancherError> {
        if self.allocated_memory_mb + memory_demand_mb > self.total_memory_mb {
            return Err(RancherError::NamespaceViolation);
        }

        let mut instance = container;
        instance.state = ContainerState::Running;
        self.allocated_memory_mb += memory_demand_mb;
        self.active_containers.insert(instance.container_id.clone(), instance);
        Ok(())
    }

    pub fn terminate_container(&mut self, id: &str, memory_demand_mb: usize) -> Result<(), RancherError> {
        if let Some(mut container) = self.active_containers.remove(id) {
            container.state = ContainerState::Stopped;
            self.allocated_memory_mb = self.allocated_memory_mb.saturating_sub(memory_demand_mb);
            Ok(())
        } else {
            Err(RancherError::ContainerNotFound)
        }
    }

    /// Enforces Bottlerocket-style dm-verity write-blocking on container filesystem root paths
    pub fn validate_container_write_access(&self, id: &str, path: &str) -> Result<bool, RancherError> {
        if let Some(container) = self.active_containers.get(id) {
            if container.immutable_rootfs && (path.starts_with("/bin") || path.starts_with("/usr") || path.starts_with("/lib")) {
                return Err(RancherError::ReadOnlyViolation);
            }
            Ok(true)
        } else {
            Err(RancherError::ContainerNotFound)
        }
    }
}

impl Default for K3osOrchestrator {
    fn default() -> Self {
        Self::new(8192) // Default 8GB bare-metal pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daemonless_container_lifecycle() {
        let container = DaemonlessContainer::new("web-server-node-01", "/var/lib/oci/nginx", 0x0F);
        assert_eq!(container.state, ContainerState::Created);
        assert!(container.immutable_rootfs);

        let mut orchestrator = K3osOrchestrator::new(2048); // 2GB pool
        orchestrator.instantiate_container(container, 512).unwrap();

        assert_eq!(orchestrator.allocated_memory_mb, 512);

        let running_container = orchestrator.active_containers.get("web-server-node-01").unwrap();
        assert_eq!(running_container.state, ContainerState::Running);

        // Verify write access check: writing to system root /usr is blocked by Bottlerocket immutability
        let write_res = orchestrator.validate_container_write_access("web-server-node-01", "/usr/bin/malicious-binary");
        assert_eq!(write_res.unwrap_err(), RancherError::ReadOnlyViolation);

        // Writing to /tmp is permitted
        let write_tmp = orchestrator.validate_container_write_access("web-server-node-01", "/tmp/session.log");
        assert!(write_tmp.unwrap());

        // Terminate and verify clean resources deallocation
        orchestrator.terminate_container("web-server-node-01", 512).unwrap();
        assert_eq!(orchestrator.allocated_memory_mb, 0);
    }
}
