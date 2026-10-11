// SPDX-License-Identifier: MIT
// SigmaOS - GPU Module
// GPU-related components including frequency scaling, drivers, and management

pub mod frequency_scaling;
pub mod amdgpu;
pub mod nvidia;
pub mod intel_xe;

pub use frequency_scaling::{
    GpuVendor, GpuFrequencyMode, GpuPowerProfile,
    GpuFrequencyController, GpuManager,
    init_gpu_manager, get_gpu_manager,
};
pub use amdgpu::{
    AmdgpuDevice, AmdgpuManager, AmdgpuPowerProfile, AmdgpuPerfLevel,
    AmdgpuSclk, AmdgpuMclk, AmdgpuVoltage, AmdgpuPowerState, AmdgpuOverdrive,
    init_amdgpu_manager, get_amdgpu_manager,
};
pub use nvidia::{
    NvidiaDevice, NvidiaManager, NvidiaDriverType, NvidiaPerfLevel, NvidiaPowerState,
    init_nvidia_manager, get_nvidia_manager,
};
pub use intel_xe::{
    IntelXeDevice, IntelXeManager, IntelGpuArch, IntelPerfLevel, IntelPowerState,
    init_intel_xe_manager, get_intel_xe_manager,
};
