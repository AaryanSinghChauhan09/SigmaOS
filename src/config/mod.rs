pub mod declarative;
pub mod loader;
pub mod manager;

pub use declarative::{
    SystemConfig, NetworkConfig, DesktopConfig, SecurityConfig, KernelConfig, PerformanceConfig,
    SigmaOsConfig,
};
