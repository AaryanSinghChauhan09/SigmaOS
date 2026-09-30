// Build System Helper for SigmaOS
// Implements build system utilities per Wiki 10-Development.md
// Supports Rust, Zig, and Nim build processes

use std::process::Command;
use std::string::{String, ToString};
use std::vec::Vec;

/// Build target architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildTarget {
    X86_64,
    Aarch64,
    Riscv64,
}

impl BuildTarget {
    pub fn to_rust_target(&self) -> &'static str {
        match self {
            BuildTarget::X86_64 => "x86_64-unknown-linux-gnu",
            BuildTarget::Aarch64 => "aarch64-unknown-linux-gnu",
            BuildTarget::Riscv64 => "riscv64gc-unknown-linux-gnu",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "x86_64" | "x86_64-unknown-linux-gnu" => Some(BuildTarget::X86_64),
            "aarch64" | "aarch64-unknown-linux-gnu" => Some(BuildTarget::Aarch64),
            "riscv64" | "riscv64gc-unknown-linux-gnu" => Some(BuildTarget::Riscv64),
            _ => None,
        }
    }
}

/// Build configuration
#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub target: BuildTarget,
    pub release: bool,
    pub features: Vec<String>,
}

impl BuildConfig {
    pub fn new(target: BuildTarget) -> Self {
        BuildConfig {
            target,
            release: false,
            features: Vec::new(),
        }
    }

    pub fn set_release(&mut self, release: bool) {
        self.release = release;
    }

    pub fn add_feature(&mut self, feature: String) {
        self.features.push(feature);
    }

    pub fn get_cargo_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        args.push(String::from("build"));

        if self.release {
            args.push(String::from("--release"));
        }

        args.push(String::from("--target"));
        args.push(String::from(self.target.to_rust_target()));

        for feature in &self.features {
            args.push(String::from("--features"));
            args.push(feature.clone());
        }

        args
    }
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self::new(BuildTarget::X86_64)
    }
}

/// Build system manager
pub struct BuildSystemManager {
    pub config: BuildConfig,
}

impl BuildSystemManager {
    pub fn new(config: BuildConfig) -> Self {
        BuildSystemManager { config }
    }

    pub fn build_rust(&self) -> Result<String, String> {
        let args = self.config.get_cargo_args();
        let output = Command::new("cargo").args(&args).output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(String::from_utf8_lossy(&result.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&result.stderr).to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute cargo: {}", e)),
        }
    }

    pub fn build_rust_release(&self) -> Result<String, String> {
        let mut config = self.config.clone();
        config.set_release(true);
        let manager = BuildSystemManager::new(config);
        manager.build_rust()
    }

    pub fn test_rust(&self) -> Result<String, String> {
        let output = Command::new("cargo").args(&["test"]).output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(String::from_utf8_lossy(&result.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&result.stderr).to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute cargo test: {}", e)),
        }
    }

    pub fn format_rust(&self) -> Result<String, String> {
        let output = Command::new("cargo").args(&["fmt"]).output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(String::from_utf8_lossy(&result.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&result.stderr).to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute cargo fmt: {}", e)),
        }
    }

    pub fn check_rust(&self) -> Result<String, String> {
        let output = Command::new("cargo").args(&["check", "--lib"]).output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(String::from_utf8_lossy(&result.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&result.stderr).to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute cargo check: {}", e)),
        }
    }

    pub fn doc_rust(&self) -> Result<String, String> {
        let output = Command::new("cargo").args(&["doc", "--open"]).output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(String::from_utf8_lossy(&result.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&result.stderr).to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute cargo doc: {}", e)),
        }
    }

    pub fn clippy_rust(&self) -> Result<String, String> {
        let output = Command::new("cargo")
            .args(&[
                "clippy",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ])
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(String::from_utf8_lossy(&result.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&result.stderr).to_string())
                }
            }
            Err(e) => Err(format!("Failed to execute cargo clippy: {}", e)),
        }
    }
}

impl Default for BuildSystemManager {
    fn default() -> Self {
        Self::new(BuildConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_target_from_str() {
        assert_eq!(BuildTarget::from_str("x86_64"), Some(BuildTarget::X86_64));
        assert_eq!(BuildTarget::from_str("aarch64"), Some(BuildTarget::Aarch64));
        assert_eq!(BuildTarget::from_str("riscv64"), Some(BuildTarget::Riscv64));
        assert_eq!(BuildTarget::from_str("invalid"), None);
    }

    #[test]
    fn test_build_target_to_rust_target() {
        assert_eq!(
            BuildTarget::X86_64.to_rust_target(),
            "x86_64-unknown-linux-gnu"
        );
        assert_eq!(
            BuildTarget::Aarch64.to_rust_target(),
            "aarch64-unknown-linux-gnu"
        );
        assert_eq!(
            BuildTarget::Riscv64.to_rust_target(),
            "riscv64gc-unknown-linux-gnu"
        );
    }

    #[test]
    fn test_build_config_creation() {
        let config = BuildConfig::new(BuildTarget::Aarch64);
        assert_eq!(config.target, BuildTarget::Aarch64);
        assert!(!config.release);
    }

    #[test]
    fn test_build_config_release() {
        let mut config = BuildConfig::new(BuildTarget::X86_64);
        config.set_release(true);
        assert!(config.release);
    }

    #[test]
    fn test_build_config_features() {
        let mut config = BuildConfig::new(BuildTarget::X86_64);
        config.add_feature(String::from("feature1"));
        config.add_feature(String::from("feature2"));
        assert_eq!(config.features.len(), 2);
    }

    #[test]
    fn test_build_config_get_cargo_args() {
        let mut config = BuildConfig::new(BuildTarget::X86_64);
        config.set_release(true);
        config.add_feature(String::from("test-feature"));

        let args = config.get_cargo_args();
        assert!(args.contains(&String::from("build")));
        assert!(args.contains(&String::from("--release")));
        assert!(args.contains(&String::from("--target")));
        assert!(args.contains(&String::from("x86_64-unknown-linux-gnu")));
        assert!(args.contains(&String::from("--features")));
        assert!(args.contains(&String::from("test-feature")));
    }

    #[test]
    fn test_build_system_manager_creation() {
        let config = BuildConfig::new(BuildTarget::X86_64);
        let manager = BuildSystemManager::new(config);
        assert_eq!(manager.config.target, BuildTarget::X86_64);
    }

    #[test]
    fn test_build_system_manager_default() {
        let manager = BuildSystemManager::default();
        assert_eq!(manager.config.target, BuildTarget::X86_64);
    }
}
