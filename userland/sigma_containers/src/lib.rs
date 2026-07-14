pub mod runtime;
pub mod image;
pub mod network;
pub mod volume;
pub mod microvm;

pub use runtime::{ContainerRuntime, Container, ContainerConfig};
pub use image::{ImageManager, Image};
pub use network::NetworkManager;
pub use volume::VolumeManager;
pub use microvm::MicroVMEngine;

/// SigmaContainer: The native OS-level container and micro-VM engine.
/// Displaces Docker, Podman, LXC, and Firecracker with a unified, capability-based runtime.
pub struct SigmaContainer {
    pub runtime: ContainerRuntime,
    pub image_manager: ImageManager,
    pub network_manager: NetworkManager,
    pub volume_manager: VolumeManager,
    pub microvm_engine: MicroVMEngine,
}

impl Default for SigmaContainer {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaContainer {
    pub fn new() -> Self {
        Self {
            runtime: ContainerRuntime::new(),
            image_manager: ImageManager::new(),
            network_manager: NetworkManager::new(),
            volume_manager: VolumeManager::new(),
            microvm_engine: MicroVMEngine::new(),
        }
    }

    pub fn create_container(&mut self, image_ref: &str, config: ContainerConfig) -> Result<Container, String> {
        let image = self.image_manager.pull(image_ref)?;
        let container = self.runtime.create(image, config)?;
        Ok(container)
    }

    pub fn run_container(&self, container: &Container) -> Result<(), String> {
        self.runtime.start(container)
    }
}
