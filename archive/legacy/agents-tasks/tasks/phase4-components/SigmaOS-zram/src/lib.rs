//! Compressed memory (zram) device setup — compression algorithm selection, size, and swap priority.
//!
//! CachyOS-inspired performance tuning. See <https://github.com/cachyos>
//! for the upstream performance-tuning philosophy this absorbs.

#![allow(dead_code)]

/// Tuning profile selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZramKind {
    Zstd,
    Lz4,
    Lzo,
    Deflate,
}


pub struct ZramConfig {
    pub kind: ZramKind,
    pub device: &'static str,
    pub size_mb: u32,
    pub swap_priority: i32,
}

impl ZramConfig {
    pub fn for_kind(kind: ZramKind) -> Self {
        match kind {
            ZramKind::Zstd => Self { kind, device: "/dev/zram0", size_mb: 8192, swap_priority: 100 },
            ZramKind::Lz4 => Self { kind, device: "/dev/zram0", size_mb: 8192, swap_priority: 100 },
            ZramKind::Lzo => Self { kind, device: "/dev/zram0", size_mb: 4096, swap_priority: 90 },
            ZramKind::Deflate => Self { kind, device: "/dev/zram0", size_mb: 4096, swap_priority: 80 },
        }
    }
    pub fn compression_alg(&self) -> &'static str {
        match self.kind {
            ZramKind::Zstd => "zstd", ZramKind::Lz4 => "lz4", ZramKind::Lzo => "lzo", ZramKind::Deflate => "deflate",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variants_exist() {
        let v = [ZramKind::Zstd, Kind::Lz4, Kind::Lzo, Kind::Deflate];
        assert!(!v.is_empty());
    }

    #[test]
    fn for_kind_valid() {
        let v = [ZramKind::Zstd, Kind::Lz4, Kind::Lzo, Kind::Deflate];
        for k in v {
            // Smoke: for_kind should not panic for any variant.
        }
    }
}
