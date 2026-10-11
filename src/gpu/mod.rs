// SPDX-License-Identifier: MIT
// SigmaOS - GPU Module
// GPU-related components including frequency scaling, drivers, and management

pub mod frequency_scaling;

pub use frequency_scaling::{
    GpuVendor, GpuFrequencyMode, GpuPowerProfile,
    GpuFrequencyController, GpuManager,
    init_gpu_manager, get_gpu_manager,
};
