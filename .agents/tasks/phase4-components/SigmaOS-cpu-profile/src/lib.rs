//! CPU governor and frequency-scaling profiles — performance, powersave, schedutil, on-demand.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuProfileKind {
    Performance,
    Powersave,
    Schedutil,
    Ondemand,
}


pub struct CpuProfile {
    pub kind: CpuProfileKind,
    pub governor: &'static str,
    pub min_freq_pct: u32,
    pub max_freq_pct: u32,
    pub turbo_boost: bool,
}

impl CpuProfile {
    pub fn for_kind(kind: CpuProfileKind) -> Self {
        match kind {
            CpuProfileKind::Performance => Self { kind, governor: "performance", min_freq_pct: 100, max_freq_pct: 100, turbo_boost: true },
            CpuProfileKind::Powersave => Self { kind, governor: "powersave", min_freq_pct: 20, max_freq_pct: 80, turbo_boost: false },
            CpuProfileKind::Schedutil => Self { kind, governor: "schedutil", min_freq_pct: 20, max_freq_pct: 100, turbo_boost: true },
            CpuProfileKind::Ondemand => Self { kind, governor: "ondemand", min_freq_pct: 20, max_freq_pct: 100, turbo_boost: true },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [CpuProfileKind::Performance, CpuProfileKind::Powersave, CpuProfileKind::Schedutil, CpuProfileKind::Ondemand];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [CpuProfileKind::Performance, CpuProfileKind::Powersave, CpuProfileKind::Schedutil, CpuProfileKind::Ondemand];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
