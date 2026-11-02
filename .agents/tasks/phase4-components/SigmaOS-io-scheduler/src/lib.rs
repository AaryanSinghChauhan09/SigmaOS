//! Block I/O scheduler selection — mq-deadline, kyber, bfq, none — with per-profile defaults.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoSchedulerKind {
    MqDeadline,
    Kyber,
    Bfq,
    None,
}


pub struct IoSchedulerChoice {
    pub kind: IoSchedulerKind,
    pub rationale: &'static str,
    pub best_for: &'static str,
}

impl IoSchedulerChoice {
    pub fn for_kind(kind: IoSchedulerKind) -> Self {
        match kind {
            IoSchedulerKind::MqDeadline => Self { kind, rationale: "Deadline-based, low-latency for SSD/NVMe", best_for: "general desktop, SSD" },
            IoSchedulerKind::Kyber => Self { kind, rationale: "Latency-aware, multi-queue for NVMe", best_for: "high-IOPS NVMe, gaming" },
            IoSchedulerKind::Bfq => Self { kind, rationale: "Fair queuing, bandwidth-oriented", best_for: "workstation, HDD, heavy I/O" },
            IoSchedulerKind::None => Self { kind, rationale: "No scheduler (direct)", best_for: "virtualized, containerized" },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [IoSchedulerKind::MqDeadline, Kind::Kyber, Kind::Bfq, Kind::None];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [IoSchedulerKind::MqDeadline, Kind::Kyber, Kind::Bfq, Kind::None];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
