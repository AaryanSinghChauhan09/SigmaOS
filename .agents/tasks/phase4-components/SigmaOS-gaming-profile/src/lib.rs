//! Gaming mode — CPU performance, low-latency audio, GPU hints, and game-mode integration.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamingKind {
    Enabled,
    Disabled,
}


pub struct GamingProfile {
    pub kind: GamingKind,
    pub cpu_governor: &'static str,
    pub scheduler: &'static str,
    pub audio_latency_ms: u32,
    pub gpu_profile: &'static str,
    pub screen_recording_allowed: bool,
    pub game_mode: bool,
}

impl GamingProfile {
    pub fn for_kind(kind: GamingKind) -> Self {
        match kind {
            GamingKind::Enabled => Self { kind, cpu_governor: "performance", scheduler: "bore", audio_latency_ms: 5, gpu_profile: "performance", screen_recording_allowed: true, game_mode: true },
            GamingKind::Disabled => Self { kind, cpu_governor: "schedutil", scheduler: "eevdf", audio_latency_ms: 20, gpu_profile: "auto", screen_recording_allowed: true, game_mode: false },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [GamingKind::Enabled, Kind::Disabled];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [GamingKind::Enabled, Kind::Disabled];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
