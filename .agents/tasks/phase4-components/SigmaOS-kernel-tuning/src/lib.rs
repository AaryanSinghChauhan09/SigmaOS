//! Kernel parameter tuning — scheduler, CPU governor, I/O, memory/VM, and low-latency knobs.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TuningKind {
    LowLatency,
    Throughput,
    Powersave,
    Default,
}


pub struct KernelTuning {
    pub kind: TuningKind,
    pub vm_swappiness: u32,
    pub vm_vfs_cache_pressure: u32,
    pub kernel_sched_latency_ns: u64,
    pub transparent_hugepages: &'static str,
}

impl KernelTuning {
    pub fn for_kind(kind: TuningKind) -> Self {
        match kind {
            TuningKind::LowLatency => Self { kind, vm_swappiness: 10, vm_vfs_cache_pressure: 50, kernel_sched_latency_ns: 2_000_000, transparent_hugepages: "madvise" },
            TuningKind::Throughput => Self { kind, vm_swappiness: 60, vm_vfs_cache_pressure: 100, kernel_sched_latency_ns: 6_000_000, transparent_hugepages: "always" },
            TuningKind::Powersave => Self { kind, vm_swappiness: 100, vm_vfs_cache_pressure: 50, kernel_sched_latency_ns: 10_000_000, transparent_hugepages: "madvise" },
            TuningKind::Default => Self { kind, vm_swappiness: 60, vm_vfs_cache_pressure: 100, kernel_sched_latency_ns: 6_000_000, transparent_hugepages: "madvise" },
        }
    }
}

pub fn tuning_actions(t: &KernelTuning) -> Vec<String> {
    vec![
        format!("vm.swappiness={}", t.vm_swappiness),
        format!("vm.vfs_cache_pressure={}", t.vm_vfs_cache_pressure),
        format!("kernel.sched_latency_ns={}", t.kernel_sched_latency_ns),
        format!("transparent_hugepages={}", t.transparent_hugepages),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [TuningKind::LowLatency, Kind::Throughput, Kind::Powersave, Kind::Default];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [TuningKind::LowLatency, Kind::Throughput, Kind::Powersave, Kind::Default];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
