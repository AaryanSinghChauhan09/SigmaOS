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

pub use distro::cachy_linux_missing_components_parity_pr::*;
pub use distro::cachy_linux_missing_components_v65_pr::*;
pub use distro::cachy_linux_missing_components_v66_pr::*;
pub use distro::cachy_linux_missing_components_v67_pr::*;
pub use distro::debian_gap_closure_advancements_v26::*;
pub use distro::debian_github_repo_missing_components_v68_pr::*;
pub use distro::debian_github_repo_unimplemented_ideas_v27_pr::*;
pub use distro::debian_linux_gap_closure_v40_pr::*;
pub use distro::open_source_os_missing_components_v54_pr::*;
pub use distro::open_source_os_missing_components_v57_pr::*;
pub use distro::open_source_os_missing_components_v60_pr::*;
pub use distro::open_source_os_missing_components_v61_pr::*;
pub use distro::open_source_os_unimplemented_ideas_v64_pr::*;
pub use distro::open_source_os_unimplemented_ideas_v65_pr::*;
pub use distro::open_source_os_unimplemented_ideas_v68_pr::*;
pub use distro::open_source_os_unimplemented_ideas_pr::*;
pub use distro::open_source_os_unimplemented_ideas_pr_gateway::*;
pub use distro::open_source_os_unimplemented_ideas_pr_suite::*;
pub use distro::open_source_os_unimplemented_ideas_v63_pr::*;
