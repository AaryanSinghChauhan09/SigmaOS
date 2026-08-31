//! Graphics Acceleration Support inspired by Mesa, Vulkan, and DRI
//! Vulkan 1.3 / OpenGL 4.6 APIs, PRIME GPU offloading, compute shaders, and ray tracing pipelines.
extern crate alloc;


use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::productivity::mint_competitor::{NvidiaPrimeProfile, SovereignNvidiaPrimeEngine};

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
    pub prime_engine: SovereignNvidiaPrimeEngine,
}

impl GraphicsManager {
    pub fn new() -> Self {
        Self {
            gpus: Vec::new(),
            active_pipelines: Vec::new(),
            is_prime_hybrid_graphics_enabled: true,
            prime_engine: SovereignNvidiaPrimeEngine::new(),
        }
    }

    pub fn register_gpu(&mut self, gpu: GpuDevice) {
        self.gpus.push(gpu);
    }

    pub fn set_prime_profile(&mut self, profile: NvidiaPrimeProfile) -> Result<bool, &'static str> {
        self.prime_engine.set_profile(profile)
    }

    pub fn create_pipeline(&mut self, api: GraphicsBackendApi, force_discrete_offload: bool) -> Result<usize, &'static str> {
        let pipeline_id = self.active_pipelines.len() + 1;
        let has_dgpu = self.gpus.iter().any(|g| g.is_discrete);
        let profile_offload = match self.prime_engine.active_profile {
            NvidiaPrimeProfile::NvidiaPerformance => true,
            NvidiaPrimeProfile::IntegratedIntelRadeon => false,
            NvidiaPrimeProfile::NvidiaOnDemand | NvidiaPrimeProfile::OffloadCompute => force_discrete_offload,
        };
        let is_offloaded = (force_discrete_offload || profile_offload) && has_dgpu;

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

    #[test]
    fn test_graphics_manager_nvidia_prime_profiles() {
        let mut mgr = GraphicsManager::new();

        mgr.register_gpu(GpuDevice {
            gpu_id: 1,
            name: "Intel Graphics".to_string(),
            vendor_id: 0x8086,
            is_discrete: false,
            vram_capacity_bytes: 1024 * 1024 * 1024,
            supports_ray_tracing: false,
            supports_compute_shaders: true,
        });

        mgr.register_gpu(GpuDevice {
            gpu_id: 2,
            name: "NVIDIA RTX 4080".to_string(),
            vendor_id: 0x10DE,
            is_discrete: true,
            vram_capacity_bytes: 16384 * 1024 * 1024,
            supports_ray_tracing: true,
            supports_compute_shaders: true,
        });

        // 1. Default On-Demand mode: without force offload -> not offloaded
        let pipe1 = mgr.create_pipeline(GraphicsBackendApi::ModernOpenGl, false).unwrap();
        assert!(!mgr.active_pipelines[pipe1 - 1].is_prime_offloaded);

        // 2. Default On-Demand mode: with force offload -> offloaded
        let pipe2 = mgr.create_pipeline(GraphicsBackendApi::Vulkan, true).unwrap();
        assert!(mgr.active_pipelines[pipe2 - 1].is_prime_offloaded);

        // 3. Switch profile to NvidiaPerformance (runtime pending relogin)
        mgr.set_prime_profile(NvidiaPrimeProfile::NvidiaPerformance).unwrap();
        let _ = mgr.prime_engine.apply_pending_profile().unwrap();

        let pipe3 = mgr.create_pipeline(GraphicsBackendApi::DirectRenderingDri3, false).unwrap();
        assert!(mgr.active_pipelines[pipe3 - 1].is_prime_offloaded);
    }
}
