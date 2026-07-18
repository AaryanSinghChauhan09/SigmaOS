//! Software Development Tools and Pipeline engine for SigmaOS developers.
//! Implements local containerized compilation task definitions and target configurations.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildTarget {
    Native,
    CrossARM64,
    CrossRiscV,
}

pub struct DevelopmentPipeline {
    pub target: BuildTarget,
    pub debug_symbols: bool,
}

impl DevelopmentPipeline {
    pub const fn new(target: BuildTarget) -> Self {
        Self {
            target,
            debug_symbols: true,
        }
    }

    pub fn get_rustc_flags(&self) -> &'static str {
        match self.target {
            BuildTarget::Native => "-C opt-level=3",
            BuildTarget::CrossARM64 => "-C opt-level=3 --target=aarch64-unknown-none",
            BuildTarget::CrossRiscV => "-C opt-level=3 --target=riscv64-unknown-none",
        }
    }
}
