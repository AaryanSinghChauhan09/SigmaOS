//! KARL — Kernel Address Layout Randomization
//! Inspired by OpenBSD's KARL: kernel is relinked at every boot with a fresh
//! random layout, making ROP/JOP exploitation orders of magnitude harder.
//!
//! Reference: OpenBSD src/sys/arch/amd64/amd64/kaslr.S
//! SigmaOS adaptation: software-level KASLR with compile-time + runtime mixing.

use crate::crypto::entropy;
use core::sync::atomic::{AtomicU64, Ordering};

/// Base virtual address of the kernel image.
/// In a real implementation, the bootloader passes this at startup.
static KERNEL_BASE: AtomicU64 = AtomicU64::new(0xFFFF_FFFF_8000_0000);

/// Per-boot random slide applied to the kernel base.
static BOOT_SLIDE: AtomicU64 = AtomicU64::new(0);

/// KARL configuration
#[derive(Debug, Clone, Copy)]
pub struct KarlConfig {
    /// Whether KARL is active for this boot
    pub enabled: bool,
    /// Alignment granularity for the slide (2MB page-aligned)
    pub alignment: u64,
    /// Maximum slide size (256MB)
    pub max_slide: u64,
}

impl Default for KarlConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            alignment: 0x20_0000,
            max_slide: 0x1000_0000,
        }
    }
}

/// Initialize KARL — generate random boot slide.
/// Called once during early boot before any kernel symbols are used.
///
/// # Safety
/// Must be called exactly once, before the kernel's virtual address space is active.
pub unsafe fn init(config: &KarlConfig) {
    if !config.enabled {
        return;
    }
    let mut seed = [0u8; 8];
    entropy::get_entropy_bytes(&mut seed);
    let raw = u64::from_le_bytes(seed);
    // Align to page boundary and limit to max_slide
    let slide = (raw % (config.max_slide / config.alignment)) * config.alignment;
    BOOT_SLIDE.store(slide, Ordering::SeqCst);
    KERNEL_BASE.fetch_add(slide, Ordering::SeqCst);
}

/// Get the current kernel base virtual address (after KARL slide).
pub fn kernel_base() -> u64 {
    KERNEL_BASE.load(Ordering::Relaxed)
}

/// Get the KARL boot slide applied this boot.
pub fn boot_slide() -> u64 {
    BOOT_SLIDE.load(Ordering::Relaxed)
}

/// Translate a compile-time kernel symbol address to the runtime address.
/// Use this whenever you embed a raw kernel address.
#[inline(always)]
pub fn relocate(compile_time_addr: u64) -> u64 {
    compile_time_addr.wrapping_add(boot_slide())
}

/// Verify KARL is active and the slide is nonzero (sanity check during boot).
pub fn verify() -> bool {
    BOOT_SLIDE.load(Ordering::Relaxed) != 0
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relocate_adds_slide() {
        BOOT_SLIDE.store(0x200000, Ordering::SeqCst);
        KERNEL_BASE.store(0xFFFF_FFFF_8020_0000, Ordering::SeqCst);
        assert_eq!(relocate(0xFFFF_FFFF_8000_0000), 0xFFFF_FFFF_8020_0000);
    }

    #[test]
    fn test_default_config() {
        let cfg = KarlConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.alignment, 0x200000);
    }
}
