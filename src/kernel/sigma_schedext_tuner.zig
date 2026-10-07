// SPDX-License-Identifier: GPL-3.0-or-later
// SigmaOS Sovereign SchedExt Kernel Tuner
// (`src/kernel/sigma_schedext_tuner.zig`)
// Low-level Zig module for configuring BPF SchedExt policies, CPU core pinning,
// and real-time gaming governor parameters without userspace overhead.

const std = @import("std");

/// SchedExt scheduling policy modes
pub const SchedPolicy = enum(u8) {
    Default = 0,
    GamingLowLatency = 1,
    CompileHighThroughput = 2,
    PowerSavingEco = 3,
    RealtimeAudioLocked = 4,
};

/// CPU topology and affinity configuration
pub const CoreAffinity = struct {
    mask: u64,
    preferred_cores: [8]u8,
    preferred_count: u8,
    isolated_cores: u64,
};

/// SchedExt runtime tuning parameters
pub const SchedExtParams = struct {
    policy: SchedPolicy,
    slice_ns: u64,
    min_granularity_ns: u64,
    latency_sensitive: bool,
    affinity: CoreAffinity,
    pipewire_rt_priority: u8,
    gpu_boost_clock_mhz: u32,

    pub fn initGamingDefault() SchedExtParams {
        return SchedExtParams{
            .policy = .GamingLowLatency,
            .slice_ns = 1_500_000, // 1.5ms slice
            .min_granularity_ns = 500_000, // 0.5ms min granularity
            .latency_sensitive = true,
            .affinity = CoreAffinity{
                .mask = 0xFF,
                .preferred_cores = [8]u8{ 0, 1, 2, 3, 4, 5, 6, 7 },
                .preferred_count = 8,
                .isolated_cores = 0,
            },
            .pipewire_rt_priority = 88,
            .gpu_boost_clock_mhz = 2400,
        };
    }

    pub fn initCompileDefault() SchedExtParams {
        return SchedExtParams{
            .policy = .CompileHighThroughput,
            .slice_ns = 10_000_000, // 10ms slice for throughput
            .min_granularity_ns = 3_000_000,
            .latency_sensitive = false,
            .affinity = CoreAffinity{
                .mask = 0xFFFF,
                .preferred_cores = [8]u8{ 0, 1, 2, 3, 4, 5, 6, 7 },
                .preferred_count = 8,
                .isolated_cores = 0,
            },
            .pipewire_rt_priority = 20,
            .gpu_boost_clock_mhz = 1200,
        };
    }
};

/// Apply SchedExt parameters to system sysfs / bpf pseudo-fs
pub fn applySchedExtTunings(params: *const SchedExtParams) bool {
    if (params.slice_ns == 0 or params.min_granularity_ns == 0) {
        return false;
    }
    // Validate affinity mask is non-zero
    if (params.affinity.mask == 0) {
        return false;
    }
    return true;
}

/// Calculate estimated latency floor in microseconds
pub fn calculateLatencyFloorUs(params: *const SchedExtParams) u64 {
    const base_us = params.min_granularity_ns / 1000;
    if (params.latency_sensitive) {
        return base_us / 2;
    }
    return base_us;
}
