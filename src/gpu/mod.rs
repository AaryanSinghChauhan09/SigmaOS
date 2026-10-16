// SPDX-License-Identifier: MIT
// SigmaOS - GPU Module
// GPU-related components including frequency scaling, drivers, and management

pub mod amdgpu;
pub mod frequency_scaling;
pub mod intel_xe;
pub mod nvidia;

pub use amdgpu::{
    get_amdgpu_manager, init_amdgpu_manager, AmdgpuDevice, AmdgpuManager, AmdgpuMclk,
    AmdgpuOverdrive, AmdgpuPerfLevel, AmdgpuPowerProfile, AmdgpuPowerState, AmdgpuSclk,
    AmdgpuVoltage,
};
pub use frequency_scaling::{
    get_gpu_manager, init_gpu_manager, GpuFrequencyController, GpuFrequencyMode, GpuManager,
    GpuPowerProfile, GpuVendor,
};
pub use intel_xe::{
    get_intel_xe_manager, init_intel_xe_manager, IntelGpuArch, IntelPerfLevel, IntelPowerState,
    IntelXeDevice, IntelXeManager,
};
pub use nvidia::{
    get_nvidia_manager, init_nvidia_manager, NvidiaDevice, NvidiaDriverType, NvidiaManager,
    NvidiaPerfLevel, NvidiaPowerState,
};
