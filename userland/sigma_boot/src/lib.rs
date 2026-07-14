pub mod initramfs;

pub use initramfs::InitramfsBuilder;

/// SigmaBoot: Boot image and virtualization packager.
/// Packages the Kernel, sigma_init, and userland into a bootable QEMU target.
pub struct SigmaBoot {
    pub builder: InitramfsBuilder,
}

impl Default for SigmaBoot {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaBoot {
    pub fn new() -> Self {
        Self {
            builder: InitramfsBuilder::new(),
        }
    }
}
