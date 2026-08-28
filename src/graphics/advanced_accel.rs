//! Graphics Acceleration Support inspired by Mesa, Vulkan, and DRI
//! Vulkan 1.3 / OpenGL 4.6 APIs, PRIME GPU offloading, compute shaders, and ray tracing pipelines.
extern crate alloc;


use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsBackendApi {
    Vulkan,
    ModernOpenGl,
    DirectRenderingDri3,
}

#[derive(Debug, Clone)]
pub struct GpuDevice {
    pub gpu_id: usize,
    pub name: String,
    pub vendor_id: u16,
    pub is_discrete: bool,
    pub vram_capacity_bytes: usize,
    pub supports_ray_tracing: bool,
    pub supports_compute_shaders: bool,
}

#[derive(Debug, Clone)]
pub struct RenderPipeline {
    pub pipeline_id: usize,
    pub api: GraphicsBackendApi,
    pub is_prime_offloaded: bool,
}

pub struct GraphicsManager {
    pub gpus: Vec<GpuDevice>,
    pub active_pipelines: Vec<RenderPipeline>,
    pub is_prime_hybrid_graphics_enabled: bool,
}

impl GraphicsManager {
    pub fn new() -> Self {
        Self {
            gpus: Vec::new(),
            active_pipelines: Vec::new(),
            is_prime_hybrid_graphics_enabled: true,
        }
    }

    pub fn register_gpu(&mut self, gpu: GpuDevice) {
        self.gpus.push(gpu);
    }

    pub fn create_pipeline(&mut self, api: GraphicsBackendApi, force_discrete_offload: bool) -> Result<usize, &'static str> {
        let pipeline_id = self.active_pipelines.len() + 1;
        let is_offloaded = force_discrete_offload && self.gpus.iter().any(|g| g.is_discrete);

        self.active_pipelines.push(RenderPipeline {
            pipeline_id,
            api,
            is_prime_offloaded: is_offloaded,
        });

        Ok(pipeline_id)
    }
}

impl Default for GraphicsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesa_vulkan_dri_graphics() {
        let mut mgr = GraphicsManager::new();

        mgr.register_gpu(GpuDevice {
            gpu_id: 1,
            name: "Intel Xe Graphics (iGPU)".to_string(),
            vendor_id: 0x8086,
            is_discrete: false,
            vram_capacity_bytes: 2048 * 1024 * 1024,
            supports_ray_tracing: false,
            supports_compute_shaders: true,
        });

        mgr.register_gpu(GpuDevice {
            gpu_id: 2,
            name: "Nvidia RDNA3 / Ampere (dGPU)".to_string(),
            vendor_id: 0x10DE,
            is_discrete: true,
            vram_capacity_bytes: 8192 * 1024 * 1024,
            supports_ray_tracing: true,
            supports_compute_shaders: true,
        });

        let pipe_id = mgr.create_pipeline(GraphicsBackendApi::Vulkan, true).unwrap();
        assert_eq!(pipe_id, 1);
        assert!(mgr.active_pipelines[0].is_prime_offloaded);
    }
}
