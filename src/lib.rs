// SigmaOS Library
// Core library for SigmaOS operating system

extern crate alloc;

// Core working modules
pub mod kernel;
pub mod klib;
pub mod drivers;

// Minimal security module for capability tokens
pub mod security {
    pub mod capability;
}

// Temporarily disabled problematic modules
// pub mod accessibility;
// pub mod automation;
// pub mod compatibility;
// pub mod container;
// pub mod customization;
// pub mod dashboard;
// pub mod desktop;
// pub mod device;
// pub mod driver;
// pub mod filesystem;
// pub mod ml;
// pub mod network;
// pub mod observability;
// pub mod orchestration;
pub mod distro;
// pub mod package;
// pub mod performance;
// pub mod productivity;
// pub mod remote;
// pub mod resilience;
// pub mod shell;
// pub mod sigpkg;
// pub mod virtualization;
// pub mod graphics {
//     pub mod compositor;
//     pub mod paint;
//     pub mod video;
// }
// pub mod hardware {
//     pub mod compatibility;
//     pub mod win32;
// }
// pub mod power {
//     pub mod governor;
// }
// pub mod ai {
//     pub mod agent;
//     pub mod orchestrator;
// }
// pub mod boot;
// pub mod system;
// pub mod installer;
