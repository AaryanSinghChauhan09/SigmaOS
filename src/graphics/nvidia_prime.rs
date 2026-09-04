use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

/// NVIDIA PRIME Hybrid Graphics Operating Profile inspired by Ubuntu/Debian nvidia-prime,
/// Arch Linux __NV_PRIME_RENDER_OFFLOAD, and Fedora optimus-manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimeProfile {
    /// Render offload on demand via __NV_PRIME_RENDER_OFFLOAD=1 (Nvidia dGPU offload)
    Offload,
    /// Dynamic switching between iGPU and dGPU based on workload (On Demand)
    OnDemand,
    /// Discrete GPU only for max performance (NVIDIA Discrete)
    DiscreteNvidia,
    /// Integrated GPU only for power saving (Intel / AMD Integrated)
    IntegratedOnly,
    /// Reverse PRIME: Discrete GPU drives secondary displays routed to integrated display
    ReversePrime,
}

/// Dynamic GPU Power State inspired by Linux kernel D3cold / D3hot & FreeBSD bbswitch
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuPowerState {
    /// Fully active and powered on (D0)
    D0Active,
    /// Low power sleep state (D3hot)
    D3hot,
    /// Dynamic runtime power-off state for discrete GPU (D3cold / bbswitch OFF)
    D3coldDynamicOff,
}

/// Offload environment variables generator for GL/Vulkan/EGL render offloading
#[derive(Debug, Clone)]
pub struct NvidiaPrimeOffloadConfig {
    pub prime_render_offload: bool,      // __NV_PRIME_RENDER_OFFLOAD=1
    pub glx_vendor_library_name: String, // __GLX_VENDOR_LIBRARY_NAME=nvidia
    pub vk_layer_nv_optimus: String,     // __VK_LAYER_NV_optimus=NVIDIA_only
    pub egl_vendor_library_filenames: String, // __EGL_VENDOR_LIBRARY_FILENAMES
}

impl Default for NvidiaPrimeOffloadConfig {
    fn default() -> Self {
        Self {
            prime_render_offload: true,
            glx_vendor_library_name: String::from("nvidia"),
            vk_layer_nv_optimus: String::from("NVIDIA_only"),
            egl_vendor_library_filenames: String::from(
                "/usr/share/glvnd/egl_vendor.d/10_nvidia.json",
            ),
        }
    }
}

impl NvidiaPrimeOffloadConfig {
    /// Generates key-value environment pairs for process offloading
    pub fn build_env_vars(&self) -> Vec<(String, String)> {
        if !self.prime_render_offload {
            return Vec::new();
        }

        vec![
            (String::from("__NV_PRIME_RENDER_OFFLOAD"), String::from("1")),
            (
                String::from("__GLX_VENDOR_LIBRARY_NAME"),
                self.glx_vendor_library_name.clone(),
            ),
            (
                String::from("__VK_LAYER_NV_optimus"),
                self.vk_layer_nv_optimus.clone(),
            ),
            (
                String::from("__EGL_VENDOR_LIBRARY_FILENAMES"),
                self.egl_vendor_library_filenames.clone(),
            ),
        ]
    }
}

/// DRM PRIME DMA-BUF buffer descriptor for cross-GPU buffer sharing (iGPU display <-> dGPU render)
#[derive(Debug, Clone)]
pub struct PrimeDmaBufShare {
    pub buffer_id: u32,
    pub dma_buf_fd: i32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub size_bytes: usize,
    pub exported_from_dgpu: bool,
}

/// NVIDIA PRIME Hybrid Graphics Controller Engine for SigmaOS
pub struct NvidiaPrimeEngine {
    pub active_profile: PrimeProfile,
    pub power_state: GpuPowerState,
    pub offload_config: NvidiaPrimeOffloadConfig,
    pub shared_dma_buffers: Vec<PrimeDmaBufShare>,
    pub gpu_temperature_c: u32,
    pub pcie_link_gen: u8,
    pub active_offloaded_processes: BTreeMap<u32, String>,
}

impl NvidiaPrimeEngine {
    pub fn new() -> Self {
        Self {
            active_profile: PrimeProfile::Offload,
            power_state: GpuPowerState::D0Active,
            offload_config: NvidiaPrimeOffloadConfig::default(),
            shared_dma_buffers: Vec::new(),
            gpu_temperature_c: 42,
            pcie_link_gen: 4,
            active_offloaded_processes: BTreeMap::new(),
        }
    }

    /// Switch PRIME operating profile (optimus-manager / nvidia-prime parity)
    pub fn set_profile(&mut self, profile: PrimeProfile) -> Result<(), &'static str> {
        self.active_profile = profile;

        match profile {
            PrimeProfile::IntegratedOnly => {
                // Transition discrete GPU to D3cold power off state (bbswitch OFF)
                self.power_state = GpuPowerState::D3coldDynamicOff;
                self.offload_config.prime_render_offload = false;
            }
            PrimeProfile::DiscreteNvidia => {
                // Keep GPU in high performance D0 state
                self.power_state = GpuPowerState::D0Active;
                self.offload_config.prime_render_offload = false;
            }
            PrimeProfile::Offload | PrimeProfile::OnDemand | PrimeProfile::ReversePrime => {
                self.power_state = GpuPowerState::D0Active;
                self.offload_config.prime_render_offload = true;
            }
        }

        Ok(())
    }

    /// Export DMA-BUF buffer from dGPU to iGPU display server
    pub fn export_prime_dma_buf(
        &mut self,
        buffer_id: u32,
        fd: i32,
        width: u32,
        height: u32,
        pitch: u32,
        size_bytes: usize,
    ) -> Result<(), &'static str> {
        if self.power_state == GpuPowerState::D3coldDynamicOff {
            return Err(
                "NVIDIA PRIME: Cannot export DMA-BUF while dGPU is in D3cold power off state",
            );
        }

        self.shared_dma_buffers.push(PrimeDmaBufShare {
            buffer_id,
            dma_buf_fd: fd,
            width,
            height,
            pitch,
            size_bytes,
            exported_from_dgpu: true,
        });

        Ok(())
    }

    /// Register a process to run with GPU offload environment variables
    pub fn register_offload_process(
        &mut self,
        pid: u32,
        process_name: &str,
    ) -> Result<Vec<(String, String)>, &'static str> {
        if self.active_profile == PrimeProfile::IntegratedOnly {
            return Err("NVIDIA PRIME: Offload requested but system is in IntegratedOnly power saving profile");
        }

        self.active_offloaded_processes
            .insert(pid, process_name.to_string());

        // Ensure dGPU is awake
        self.power_state = GpuPowerState::D0Active;

        Ok(self.offload_config.build_env_vars())
    }

    /// Unregister process and update power state if no offloaded applications are active
    pub fn unregister_offload_process(&mut self, pid: u32) {
        self.active_offloaded_processes.remove(&pid);

        if self.active_offloaded_processes.is_empty()
            && self.active_profile == PrimeProfile::OnDemand
        {
            // Auto-suspend dGPU to D3cold when idle on demand
            self.power_state = GpuPowerState::D3coldDynamicOff;
        }
    }

    /// Get current power and thermal telemetry status
    pub fn query_telemetry(&self) -> (PrimeProfile, GpuPowerState, u32, u8, usize) {
        (
            self.active_profile,
            self.power_state,
            self.gpu_temperature_c,
            self.pcie_link_gen,
            self.active_offloaded_processes.len(),
        )
    }
}

impl Default for NvidiaPrimeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvidia_prime_profile_switching() {
        let mut prime = NvidiaPrimeEngine::new();
        assert_eq!(prime.active_profile, PrimeProfile::Offload);
        assert_eq!(prime.power_state, GpuPowerState::D0Active);

        // Switch to IntegratedOnly power saver
        assert!(prime.set_profile(PrimeProfile::IntegratedOnly).is_ok());
        assert_eq!(prime.power_state, GpuPowerState::D3coldDynamicOff);
        assert!(!prime.offload_config.prime_render_offload);

        // Attempt process offload during IntegratedOnly
        let offload_err = prime.register_offload_process(101, "blender");
        assert!(offload_err.is_err());
    }

    #[test]
    fn test_nvidia_prime_offload_env_generation() {
        let mut prime = NvidiaPrimeEngine::new();
        let envs = prime.register_offload_process(202, "vkcube").unwrap();

        assert_eq!(envs.len(), 4);
        assert!(envs.contains(&(String::from("__NV_PRIME_RENDER_OFFLOAD"), String::from("1"))));
        assert!(envs.contains(&(
            String::from("__GLX_VENDOR_LIBRARY_NAME"),
            String::from("nvidia")
        )));
    }

    #[test]
    fn test_nvidia_prime_dma_buf_sharing() {
        let mut prime = NvidiaPrimeEngine::new();
        let res = prime.export_prime_dma_buf(1, 10, 1920, 1080, 1920 * 4, 1920 * 1080 * 4);
        assert!(res.is_ok());
        assert_eq!(prime.shared_dma_buffers.len(), 1);
        assert!(prime.shared_dma_buffers[0].exported_from_dgpu);
    }
}
