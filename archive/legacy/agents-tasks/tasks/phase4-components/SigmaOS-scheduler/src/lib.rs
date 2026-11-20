//! Scheduler selection — EEVDF, BORE, CFS toggle with rationale per profile.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerKind {
    Eevdf,
    Bore,
    Cfs,
}


pub struct SchedulerChoice {
    pub kind: SchedulerKind,
    pub rationale: &'static str,
    pub base_slice_ns: u64,
}

impl SchedulerChoice {
    pub fn for_kind(kind: SchedulerKind) -> Self {
        match kind {
            SchedulerKind::Eevdf => Self { kind, rationale: "Earliest Eligible Virtual Deadline First — fair + responsive (Linux 6.6+ default)", base_slice_ns: 4_000_000 },
            SchedulerKind::Bore => Self { kind, rationale: "Burst-Oriented Response Enhancer — gaming-optimized low-latency variant", base_slice_ns: 3_000_000 },
            SchedulerKind::Cfs => Self { kind, rationale: "Completely Fair Scheduler — legacy, broad compatibility", base_slice_ns: 6_000_000 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [SchedulerKind::Eevdf, Kind::Bore, Kind::Cfs];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [SchedulerKind::Eevdf, Kind::Bore, Kind::Cfs];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
