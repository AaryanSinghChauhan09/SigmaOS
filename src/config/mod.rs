pub mod declarative;
pub mod loader;
pub mod manager;

pub use declarative::{
    DesktopConfig, KernelConfig, NetworkConfig, PerformanceConfig, SecurityConfig, SigmaOsConfig,
    SystemConfig,
};
