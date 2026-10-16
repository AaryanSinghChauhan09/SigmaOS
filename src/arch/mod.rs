pub mod hal;
pub mod x86_64;

pub use x86_64::sovereign_rust_osdev_synthesis;
pub use x86_64::sovereign_rust_osdev_synthesis::*;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;
