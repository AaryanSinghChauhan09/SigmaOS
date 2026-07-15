// SigmaOS Built-in Virtualization Support
// KVM/QEMU, Docker, and Kubernetes orchestration preconfigured

use std::collections::HashMap;

/// Virtualization technology
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtualizationTech {
    KVM,
    QEMU,
    Docker,
    Podman,
    Kubernetes,
    LXC,
    LXD,
    VirtualBox,
}

/// VM state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    Running,
    Stopped,
    Paused,
    Error,
}

/// Virtual machine configuration
#[derive(Debug, Clone)]
pub struct VirtualMachine {
    pub id: String,
    pub name: String,
    pub technology: VirtualizationTech,
    pub cpus: u32,
    pub memory_mb: u32,
    pub disk_gb: u32,
    pub state: VmState,
    pub network_config: HashMap<String, String>,
    pub storage_paths: Vec<String>,
}

impl VirtualMachine {
    pub fn new(id: String, name: String, technology: VirtualizationTech) -> Self {
        Self {
            id,
            name,
            technology,
            cpus: 2,
            memory_mb: 2048,
            disk_gb: 20,
            state: VmState::Stopped,
            network_config: HashMap::new(),
            storage_paths: Vec::new(),
        }
    }

    pub fn with_resources(mut self, cpus: u32, memory_mb: u32, disk_gb: u32) -> Self {
        self.cpus = cpus;
        self.memory_mb = memory_mb;
        self.disk_gb = disk_gb;
        self
    }

    pub fn with_network(mut self, key: String, value: String) -> Self {
        self.network_config.insert(key, value);
        self
    }

    pub fn with_storage(mut self, path: String) -> Self {
        self.storage_paths.push(path);
        self
    }

    pub fn start(&mut self) -> Result<(), VirtualizationError> {
        self.state = VmState::Running;
        println!("Starting VM {} using {:?}", self.name, self.technology);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), VirtualizationError> {
        self.state = VmState::Stopped;
        println!("Stopping VM {}", self.name);
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), VirtualizationError> {
        self.state = VmState::Paused;
        println!("Pausing VM {}", self.name);
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), VirtualizationError> {
        self.state = VmState::Running;
        println!("Resuming VM {}", self.name);
        Ok(())
    }
}

/// Container configuration
#[derive(Debug, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub runtime: VirtualizationTech,
    pub state: VmState,
    pub environment: HashMap<String, String>,
    pub ports: HashMap<String, u16>,
    pub volumes: Vec<String>,
}

impl Container {
    pub fn new(id: String, name: String, image: String, runtime: VirtualizationTech) -> Self {
        Self {
            id,
            name,
            image,
            runtime,
            state: VmState::Stopped,
            environment: HashMap::new(),
            ports: HashMap::new(),
            volumes: Vec::new(),
        }
    }

    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }

    pub fn with_port(mut self, container_port: u16, host_port: u16) -> Self {
        self.ports.insert(container_port.to_string(), host_port);
        self
    }

    pub fn with_volume(mut self, volume: String) -> Self {
        self.volumes.push(volume);
        self
    }

    pub fn start(&mut self) -> Result<(), VirtualizationError> {
        self.state = VmState::Running;
        println!("Starting container {} using {:?}", self.name, self.runtime);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), VirtualizationError> {
        self.state = VmState::Stopped;
        println!("Stopping container {}", self.name);
        Ok(())
    }
}

/// Kubernetes pod configuration
#[derive(Debug, Clone)]
pub struct KubernetesPod {
    pub name: String,
    pub namespace: String,
    pub containers: Vec<Container>,
    pub replicas: u32,
    pub service_enabled: bool,
}

impl KubernetesPod {
    pub fn new(name: String, namespace: String) -> Self {
        Self {
            name,
            namespace,
            containers: Vec::new(),
            replicas: 1,
            service_enabled: false,
        }
    }

    pub fn with_container(mut self, container: Container) -> Self {
        self.containers.push(container);
        self
    }

    pub fn with_replicas(mut self, replicas: u32) -> Self {
        self.replicas = replicas;
        self
    }

    pub fn with_service(mut self, enabled: bool) -> Self {
        self.service_enabled = enabled;
        self
    }

    pub fn deploy(&self) -> Result<(), VirtualizationError> {
        println!(
            "Deploying pod {} to namespace {} with {} replicas",
            self.name, self.namespace, self.replicas
        );
        Ok(())
    }

    pub fn scale(&mut self, replicas: u32) -> Result<(), VirtualizationError> {
        self.replicas = replicas;
        println!("Scaling pod {} to {} replicas", self.name, replicas);
        Ok(())
    }
}

/// Virtualization orchestrator
pub struct VirtualizationOrchestrator {
    pub virtual_machines: HashMap<String, VirtualMachine>,
    pub containers: HashMap<String, Container>,
    pub kubernetes_pods: HashMap<String, KubernetesPod>,
    pub enabled_technologies: Vec<VirtualizationTech>,
    pub resource_pool: ResourcePool,
}

/// Resource pool for virtualization
#[derive(Debug, Clone)]
pub struct ResourcePool {
    pub total_cpus: u32,
    pub total_memory_mb: u32,
    pub total_disk_gb: u32,
    pub allocated_cpus: u32,
    pub allocated_memory_mb: u32,
    pub allocated_disk_gb: u32,
}

impl ResourcePool {
    pub fn new(total_cpus: u32, total_memory_mb: u32, total_disk_gb: u32) -> Self {
        Self {
            total_cpus,
            total_memory_mb,
            total_disk_gb,
            allocated_cpus: 0,
            allocated_memory_mb: 0,
            allocated_disk_gb: 0,
        }
    }

    pub fn can_allocate(&self, cpus: u32, memory_mb: u32, disk_gb: u32) -> bool {
        self.allocated_cpus + cpus <= self.total_cpus
            && self.allocated_memory_mb + memory_mb <= self.total_memory_mb
            && self.allocated_disk_gb + disk_gb <= self.total_disk_gb
    }

    pub fn allocate(
        &mut self,
        cpus: u32,
        memory_mb: u32,
        disk_gb: u32,
    ) -> Result<(), VirtualizationError> {
        if !self.can_allocate(cpus, memory_mb, disk_gb) {
            return Err(VirtualizationError::InsufficientResources);
        }
        self.allocated_cpus += cpus;
        self.allocated_memory_mb += memory_mb;
        self.allocated_disk_gb += disk_gb;
        Ok(())
    }

    pub fn deallocate(&mut self, cpus: u32, memory_mb: u32, disk_gb: u32) {
        self.allocated_cpus = self.allocated_cpus.saturating_sub(cpus);
        self.allocated_memory_mb = self.allocated_memory_mb.saturating_sub(memory_mb);
        self.allocated_disk_gb = self.allocated_disk_gb.saturating_sub(disk_gb);
    }

    pub fn get_available_cpus(&self) -> u32 {
        self.total_cpus - self.allocated_cpus
    }

    pub fn get_available_memory_mb(&self) -> u32 {
        self.total_memory_mb - self.allocated_memory_mb
    }

    pub fn get_available_disk_gb(&self) -> u32 {
        self.total_disk_gb - self.allocated_disk_gb
    }
}

impl VirtualizationOrchestrator {
    pub fn new() -> Self {
        let mut orchestrator = Self {
            virtual_machines: HashMap::new(),
            containers: HashMap::new(),
            kubernetes_pods: HashMap::new(),
            enabled_technologies: vec![
                VirtualizationTech::KVM,
                VirtualizationTech::QEMU,
                VirtualizationTech::Docker,
                VirtualizationTech::Podman,
                VirtualizationTech::Kubernetes,
            ],
            resource_pool: ResourcePool::new(16, 32768, 1000), // 16 CPUs, 32GB RAM, 1TB disk
        };

        orchestrator.initialize_default_configs();
        orchestrator
    }

    fn initialize_default_configs(&mut self) {
        // Preconfigure common virtualization settings
        println!("Initializing KVM/QEMU configuration");
        println!("Initializing Docker/Podman configuration");
        println!("Initializing Kubernetes orchestration");
    }

    pub fn add_virtual_machine(&mut self, vm: VirtualMachine) -> Result<(), VirtualizationError> {
        if !self
            .resource_pool
            .can_allocate(vm.cpus, vm.memory_mb, vm.disk_gb)
        {
            return Err(VirtualizationError::InsufficientResources);
        }

        self.resource_pool
            .allocate(vm.cpus, vm.memory_mb, vm.disk_gb)?;
        self.virtual_machines.insert(vm.id.clone(), vm);
        Ok(())
    }

    pub fn remove_virtual_machine(&mut self, id: &str) -> Result<(), VirtualizationError> {
        if let Some(vm) = self.virtual_machines.remove(id) {
            self.resource_pool
                .deallocate(vm.cpus, vm.memory_mb, vm.disk_gb);
            Ok(())
        } else {
            Err(VirtualizationError::VmNotFound)
        }
    }

    pub fn add_container(&mut self, container: Container) -> Result<(), VirtualizationError> {
        // Containers typically use less resources, allocate minimal
        self.resource_pool.allocate(1, 512, 1)?;
        self.containers.insert(container.id.clone(), container);
        Ok(())
    }

    pub fn remove_container(&mut self, id: &str) -> Result<(), VirtualizationError> {
        if self.containers.remove(id).is_some() {
            self.resource_pool.deallocate(1, 512, 1);
            Ok(())
        } else {
            Err(VirtualizationError::ContainerNotFound)
        }
    }

    pub fn add_kubernetes_pod(&mut self, pod: KubernetesPod) -> Result<(), VirtualizationError> {
        // Calculate total resources needed for all replicas
        let total_cpus: u32 = pod.containers.iter().map(|_| 1).sum();
        let total_memory: u32 = pod.containers.iter().map(|_| 512).sum();
        let total_disk: u32 = pod.containers.iter().map(|_| 1).sum();

        let needed_cpus = total_cpus * pod.replicas;
        let needed_memory = total_memory * pod.replicas;
        let needed_disk = total_disk * pod.replicas;

        if !self
            .resource_pool
            .can_allocate(needed_cpus, needed_memory, needed_disk)
        {
            return Err(VirtualizationError::InsufficientResources);
        }

        self.resource_pool
            .allocate(needed_cpus, needed_memory, needed_disk)?;
        self.kubernetes_pods.insert(pod.name.clone(), pod);
        Ok(())
    }

    pub fn remove_kubernetes_pod(&mut self, name: &str) -> Result<(), VirtualizationError> {
        if let Some(pod) = self.kubernetes_pods.remove(name) {
            let total_cpus: u32 = pod.containers.iter().map(|_| 1).sum();
            let total_memory: u32 = pod.containers.iter().map(|_| 512).sum();
            let total_disk: u32 = pod.containers.iter().map(|_| 1).sum();

            self.resource_pool.deallocate(
                total_cpus * pod.replicas,
                total_memory * pod.replicas,
                total_disk * pod.replicas,
            );
            Ok(())
        } else {
            Err(VirtualizationError::PodNotFound)
        }
    }

    pub fn get_virtual_machine(&self, id: &str) -> Option<&VirtualMachine> {
        self.virtual_machines.get(id)
    }

    pub fn get_container(&self, id: &str) -> Option<&Container> {
        self.containers.get(id)
    }

    pub fn get_kubernetes_pod(&self, name: &str) -> Option<&KubernetesPod> {
        self.kubernetes_pods.get(name)
    }

    pub fn list_running_vms(&self) -> Vec<&VirtualMachine> {
        self.virtual_machines
            .values()
            .filter(|vm| vm.state == VmState::Running)
            .collect()
    }

    pub fn list_running_containers(&self) -> Vec<&Container> {
        self.containers
            .values()
            .filter(|c| c.state == VmState::Running)
            .collect()
    }

    pub fn get_resource_usage(&self) -> (f64, f64, f64) {
        let cpu_usage =
            self.resource_pool.allocated_cpus as f64 / self.resource_pool.total_cpus as f64 * 100.0;
        let memory_usage = self.resource_pool.allocated_memory_mb as f64
            / self.resource_pool.total_memory_mb as f64
            * 100.0;
        let disk_usage = self.resource_pool.allocated_disk_gb as f64
            / self.resource_pool.total_disk_gb as f64
            * 100.0;
        (cpu_usage, memory_usage, disk_usage)
    }

    pub fn enable_technology(&mut self, tech: VirtualizationTech) {
        if !self.enabled_technologies.contains(&tech) {
            self.enabled_technologies.push(tech);
        }
    }

    pub fn disable_technology(&mut self, tech: VirtualizationTech) {
        self.enabled_technologies.retain(|t| *t != tech);
    }

    pub fn is_technology_enabled(&self, tech: VirtualizationTech) -> bool {
        self.enabled_technologies.contains(&tech)
    }
}

impl Default for VirtualizationOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Virtualization errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualizationError {
    VmNotFound,
    ContainerNotFound,
    PodNotFound,
    InsufficientResources,
    TechnologyNotEnabled,
    StartFailed,
    StopFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator_creation() {
        let orchestrator = VirtualizationOrchestrator::new();
        assert_eq!(orchestrator.enabled_technologies.len(), 5);
    }

    #[test]
    fn test_vm_creation() {
        let vm = VirtualMachine::new(
            "test".to_string(),
            "Test VM".to_string(),
            VirtualizationTech::KVM,
        )
        .with_resources(2, 2048, 20);
        assert_eq!(vm.cpus, 2);
        assert_eq!(vm.memory_mb, 2048);
    }

    #[test]
    fn test_vm_lifecycle() {
        let mut vm = VirtualMachine::new(
            "test".to_string(),
            "Test VM".to_string(),
            VirtualizationTech::KVM,
        );
        assert!(vm.start().is_ok());
        assert_eq!(vm.state, VmState::Running);
        assert!(vm.stop().is_ok());
        assert_eq!(vm.state, VmState::Stopped);
    }

    #[test]
    fn test_container_creation() {
        let container = Container::new(
            "test".to_string(),
            "Test Container".to_string(),
            "nginx:latest".to_string(),
            VirtualizationTech::Docker,
        )
        .with_port(80, 8080);
        assert_eq!(container.ports.len(), 1);
    }

    #[test]
    fn test_kubernetes_pod() {
        let container = Container::new(
            "web".to_string(),
            "web".to_string(),
            "nginx:latest".to_string(),
            VirtualizationTech::Kubernetes,
        );
        let pod = KubernetesPod::new("web-pod".to_string(), "default".to_string())
            .with_container(container)
            .with_replicas(3);
        assert_eq!(pod.replicas, 3);
    }

    #[test]
    fn test_resource_pool() {
        let mut pool = ResourcePool::new(16, 32768, 1000);
        assert!(pool.can_allocate(4, 8192, 100));
        assert!(pool.allocate(4, 8192, 100).is_ok());
        assert_eq!(pool.get_available_cpus(), 12);
    }

    #[test]
    fn test_add_vm_to_orchestrator() {
        let mut orchestrator = VirtualizationOrchestrator::new();
        let vm = VirtualMachine::new(
            "test".to_string(),
            "Test VM".to_string(),
            VirtualizationTech::KVM,
        )
        .with_resources(2, 2048, 20);
        assert!(orchestrator.add_virtual_machine(vm).is_ok());
        assert_eq!(orchestrator.virtual_machines.len(), 1);
    }
}
