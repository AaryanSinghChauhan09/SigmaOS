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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimeProfile {
    Integrated,
    Nvidia,
    HybridOnDemand,
    ComputeOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DynamicPowerState {
    D0Active,
    D3hot,
    D3coldPowerOff,
}

#[derive(Debug, Clone)]
pub struct PrimeOffloadEnv {
    pub nv_prime_render_offload: u8,
    pub glx_vendor_library_name: String,
    pub vk_layer_nv_optimus: String,
}

impl PrimeOffloadEnv {
    pub fn for_nvidia_offload() -> Self {
        Self {
            nv_prime_render_offload: 1,
            glx_vendor_library_name: "nvidia".to_string(),
            vk_layer_nv_optimus: "NVIDIA_only".to_string(),
        }
    }

    pub fn for_integrated() -> Self {
        Self {
            nv_prime_render_offload: 0,
            glx_vendor_library_name: "mesa".to_string(),
            vk_layer_nv_optimus: "non_nvidia".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DmaBufHandle {
    pub fd: i32,
    pub size_bytes: usize,
    pub stride_bytes: usize,
    pub modifier: u64,
    pub width: u32,
    pub height: u32,
}

pub struct DmaBufSyncEngine {
    pub exported_buffers: Vec<DmaBufHandle>,
    pub imported_buffers: Vec<DmaBufHandle>,
    pub sync_fences_count: usize,
}

impl DmaBufSyncEngine {
    pub fn new() -> Self {
        Self {
            exported_buffers: Vec::new(),
            imported_buffers: Vec::new(),
            sync_fences_count: 0,
        }
    }

    pub fn export_dma_buf(&mut self, size: usize, stride: usize, width: u32, height: u32) -> DmaBufHandle {
        let fd = (self.exported_buffers.len() + 10) as i32;
        let handle = DmaBufHandle {
            fd,
            size_bytes: size,
            stride_bytes: stride,
            modifier: 0x0010_0000_0000_0001, // DRM_FORMAT_MOD_NVIDIA_16BX2_BLOCK_LINEAR
            width,
            height,
        };
        self.exported_buffers.push(handle.clone());
        handle
    }

    pub fn import_dma_buf(&mut self, handle: DmaBufHandle) -> Result<(), &'static str> {
        if handle.fd < 0 {
            return Err("Invalid file descriptor for DMA-BUF handle");
        }
        self.imported_buffers.push(handle);
        self.sync_fences_count += 1;
        Ok(())
    }
}

impl Default for DmaBufSyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NvidiaPrimeEngine {
    pub active_profile: PrimeProfile,
    pub power_state: DynamicPowerState,
    pub offload_env: PrimeOffloadEnv,
    pub sync_engine: DmaBufSyncEngine,
    pub discrete_gpu_id: Option<usize>,
    pub integrated_gpu_id: Option<usize>,
}

impl NvidiaPrimeEngine {
    pub fn new() -> Self {
        Self {
            active_profile: PrimeProfile::HybridOnDemand,
            power_state: DynamicPowerState::D3coldPowerOff,
            offload_env: PrimeOffloadEnv::for_nvidia_offload(),
            sync_engine: DmaBufSyncEngine::new(),
            discrete_gpu_id: None,
            integrated_gpu_id: None,
        }
    }

    pub fn set_profile(&mut self, profile: PrimeProfile) {
        self.active_profile = profile;
        match profile {
            PrimeProfile::Integrated => {
                self.offload_env = PrimeOffloadEnv::for_integrated();
                self.power_state = DynamicPowerState::D3coldPowerOff;
            }
            PrimeProfile::Nvidia => {
                self.offload_env = PrimeOffloadEnv::for_nvidia_offload();
                self.power_state = DynamicPowerState::D0Active;
            }
            PrimeProfile::HybridOnDemand => {
                self.offload_env = PrimeOffloadEnv::for_nvidia_offload();
                self.power_state = DynamicPowerState::D3coldPowerOff;
            }
            PrimeProfile::ComputeOnly => {
                self.offload_env = PrimeOffloadEnv::for_nvidia_offload();
                self.power_state = DynamicPowerState::D0Active;
            }
        }
    }

    pub fn request_power_state(&mut self, state: DynamicPowerState) {
        self.power_state = state;
    }

    pub fn offload_render_buffer(&mut self, width: u32, height: u32, stride: usize) -> Result<DmaBufHandle, &'static str> {
        if self.power_state == DynamicPowerState::D3coldPowerOff {
            self.power_state = DynamicPowerState::D0Active;
        }

        let size = (stride * height as usize) as usize;
        let buf = self.sync_engine.export_dma_buf(size, stride, width, height);
        self.sync_engine.import_dma_buf(buf.clone())?;

        Ok(buf)
    }
}

impl Default for NvidiaPrimeEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GraphicsManager {
    pub gpus: Vec<GpuDevice>,
    pub active_pipelines: Vec<RenderPipeline>,
    pub is_prime_hybrid_graphics_enabled: bool,
    pub prime_engine: SovereignNvidiaPrimeEngine,
    pub prime_engine: NvidiaPrimeEngine,
}

impl GraphicsManager {
    pub fn new() -> Self {
        Self {
            gpus: Vec::new(),
            active_pipelines: Vec::new(),
            is_prime_hybrid_graphics_enabled: true,
            prime_engine: SovereignNvidiaPrimeEngine::new(),
            prime_engine: NvidiaPrimeEngine::new(),
        }
    }

    pub fn register_gpu(&mut self, gpu: GpuDevice) {
        if gpu.is_discrete {
            self.prime_engine.discrete_gpu_id = Some(gpu.gpu_id);
        } else {
            self.prime_engine.integrated_gpu_id = Some(gpu.gpu_id);
        }
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

        if is_offloaded {
            self.prime_engine.request_power_state(DynamicPowerState::D0Active);
        }

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

            gpu_id: 2,
            name: "NVIDIA RTX 4080".to_string(),
    fn test_nvidia_prime_profile_switching() {
        let mut engine = NvidiaPrimeEngine::new();
        assert_eq!(engine.active_profile, PrimeProfile::HybridOnDemand);
        assert_eq!(engine.power_state, DynamicPowerState::D3coldPowerOff);

        engine.set_profile(PrimeProfile::Integrated);
        assert_eq!(engine.active_profile, PrimeProfile::Integrated);
        assert_eq!(engine.offload_env.nv_prime_render_offload, 0);
        assert_eq!(engine.offload_env.glx_vendor_library_name, "mesa");

        engine.set_profile(PrimeProfile::Nvidia);
        assert_eq!(engine.active_profile, PrimeProfile::Nvidia);
        assert_eq!(engine.offload_env.nv_prime_render_offload, 1);
        assert_eq!(engine.offload_env.glx_vendor_library_name, "nvidia");
        assert_eq!(engine.power_state, DynamicPowerState::D0Active);
    }

    #[test]
    fn test_rtd3_power_state_transitions() {
            name: "Intel Iris Xe".to_string(),
            vram_capacity_bytes: 4096 * 1024 * 1024,
            name: "Nvidia RTX 4090 Mobile".to_string(),
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
        assert_eq!(mgr.prime_engine.power_state, DynamicPowerState::D3coldPowerOff);
        mgr.create_pipeline(GraphicsBackendApi::Vulkan, true).unwrap();
        assert_eq!(mgr.prime_engine.power_state, DynamicPowerState::D0Active);
    }

    #[test]
    fn test_dma_buf_sync_export_import() {
        let mut engine = NvidiaPrimeEngine::new();
        assert_eq!(engine.sync_engine.exported_buffers.len(), 0);

        let buf = engine.offload_render_buffer(1920, 1080, 1920 * 4).unwrap();
        assert_eq!(buf.width, 1920);
        assert_eq!(buf.height, 1080);
        assert!(buf.fd >= 10);
        assert_eq!(engine.sync_engine.exported_buffers.len(), 1);
        assert_eq!(engine.sync_engine.imported_buffers.len(), 1);
        assert_eq!(engine.sync_engine.sync_fences_count, 1);
        assert_eq!(engine.power_state, DynamicPowerState::D0Active);
    }
}
