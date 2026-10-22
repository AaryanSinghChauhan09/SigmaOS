#![allow(unused_extern_crates)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod mod_primitives;
mod mod_arch;
mod mod_backup;
mod mod_driver;
mod mod_kernel;
mod mod_memory;
mod mod_process;
mod mod_scheduler;
mod mod_vfs;

pub use mod_primitives::*;
pub use mod_arch::*;
pub use mod_backup::*;
pub use mod_driver::*;
pub use mod_kernel::*;
pub use mod_memory::*;
pub use mod_process::*;
pub use mod_scheduler::*;
pub use mod_vfs::*;

pub mod distro;
pub use distro::*;

mod kernel;
mod system;
mod creative;
mod shell;

#[cfg(test)]
mod tests {
    mod backup_manager_tests;
    mod kernel_tests;
}
