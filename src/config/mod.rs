pub mod loader;
pub mod manager;
pub mod declarative;

pub use declarative::{
    SystemConfig, NetworkConfig, DesktopConfig, SecurityConfig, KernelConfig, PerformanceConfig,
    SigmaOsConfig,
};
