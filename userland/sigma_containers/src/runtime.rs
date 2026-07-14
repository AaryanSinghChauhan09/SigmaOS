use crate::image::Image;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_CID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
pub struct ContainerConfig {
    pub memory_limit_mb: u32,
    pub capabilities: Vec<String>,
}

#[derive(Debug)]
pub struct Container {
    pub id: u64,
    pub image_ref: String,
    pub config: ContainerConfig,
}

pub struct ContainerRuntime {}

impl Default for ContainerRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(&self, image: Image, config: ContainerConfig) -> Result<Container, String> {
        Ok(Container {
            id: NEXT_CID.fetch_add(1, Ordering::SeqCst),
            image_ref: image.digest,
            config,
        })
    }

    pub fn start(&self, _container: &Container) -> Result<(), String> {
        // Here we would interface with the OS kernel to spawn the isolated process.
        Ok(())
    }
}
