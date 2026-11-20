// SPDX-License-Identifier: MIT
// SigmaOS Cross-Compilation Support
// Multi-architecture build support inspired by Arch Linux's multilib and
// cross-compilation toolchains

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

// ============================================================================
// TARGET ARCHITECTURES
// ============================================================================

/// Supported target architectures (Arch-style naming)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetArch {
    X86_64,
    X86,
    Arm64,
    Armv7,
    Armv6,
    Riscv64,
    Powerpc64le,
}

impl TargetArch {
    /// Get the GNU triple suffix for this architecture
    pub fn triple(&self) -> &'static str {
        match self {
            TargetArch::X86_64 => "x86_64-unknown-linux-gnu",
            TargetArch::X86 => "i686-unknown-linux-gnu",
            TargetArch::Arm64 => "aarch64-unknown-linux-gnu",
            TargetArch::Armv7 => "armv7-unknown-linux-gnueabihf",
            TargetArch::Armv6 => "arm-unknown-linux-gnueabihf",
            TargetArch::Riscv64 => "riscv64-unknown-linux-gnu",
            TargetArch::Powerpc64le => "powerpc64le-unknown-linux-gnu",
        }
    }

    /// Rust target triple
    pub fn rust_target(&self) -> &'static str {
        match self {
            TargetArch::X86_64 => "x86_64-unknown-linux-gnu",
            TargetArch::X86 => "i686-unknown-linux-gnu",
            TargetArch::Arm64 => "aarch64-unknown-linux-gnu",
            TargetArch::Armv7 => "armv7-unknown-linux-gnueabihf",
            TargetArch::Armv6 => "arm-unknown-linux-gnueabihf",
            TargetArch::Riscv64 => "riscv64gc-unknown-linux-gnu",
            TargetArch::Powerpc64le => "powerpc64le-unknown-linux-gnu",
        }
    }

    /// Get bitness
    pub fn bits(&self) -> u32 {
        match self {
            TargetArch::X86_64 | TargetArch::Arm64 | TargetArch::Riscv64 | TargetArch::Powerpc64le => 64,
            _ => 32,
        }
    }

    /// Whether this is a multilib secondary arch (Arch-style)
    pub fn is_multilib(&self) -> bool {
        matches!(self, TargetArch::X86)
    }

    /// Endianness
    pub fn is_little_endian(&self) -> bool {
        true
    }

    /// Pointer size in bytes
    pub fn pointer_size(&self) -> u32 {
        self.bits() / 8
    }
}

impl fmt::Display for TargetArch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rust_target())
    }
}

// ============================================================================
// CROSS-COMPILATION TARGET
// ============================================================================

/// A configured cross-compilation target
#[derive(Debug, Clone)]
pub struct CrossTarget {
    pub name: String,
    pub arch: TargetArch,
    pub sysroot: Option<PathBuf>,
    pub linker: Option<String>,
    pub cflags: Vec<String>,
    pub ldflags: Vec<String>,
    pub rustflags: Vec<String>,
    pub pkg_config_path: Option<PathBuf>,
    pub enabled: bool,
}

impl CrossTarget {
    pub fn new(name: impl Into<String>, arch: TargetArch) -> Self {
        Self {
            name: name.into(),
            arch,
            sysroot: None,
            linker: None,
            cflags: Vec::new(),
            ldflags: Vec::new(),
            rustflags: Vec::new(),
            pkg_config_path: None,
            enabled: true,
        }
    }

    pub fn with_sysroot(mut self, sysroot: PathBuf) -> Self {
        self.sysroot = Some(sysroot);
        self
    }

    pub fn with_linker(mut self, linker: impl Into<String>) -> Self {
        self.linker = Some(linker.into());
        self
    }

    pub fn with_cflag(mut self, flag: impl Into<String>) -> Self {
        self.cflags.push(flag.into());
        self
    }

    pub fn with_ldflag(mut self, flag: impl Into<String>) -> Self {
        self.ldflags.push(flag.into());
        self
    }

    pub fn with_rustflag(mut self, flag: impl Into<String>) -> Self {
        self.rustflags.push(flag.into());
        self
    }

    pub fn with_pkg_config_path(mut self, path: PathBuf) -> Self {
        self.pkg_config_path = Some(path);
        self
    }

    /// Environment variables needed for this target's builds
    pub fn env_vars(&self) -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert("CROSS_COMPILE".to_string(), self.arch.triple().to_string());
        env.insert("HOST".to_string(), std::env::consts::ARCH.to_string());
        env.insert("TARGET".to_string(), self.arch.rust_target().to_string());

        if let Some(linker) = &self.linker {
            env.insert("CARGO_TARGET_{}_LINKER".to_string(),
                format!("{}{}", self.target_env_prefix(), linker));
        }

        if let Some(sysroot) = &self.sysroot {
            env.insert("CFLAGS".to_string(), format!("--sysroot={} {}",
                sysroot.display(), self.cflags.join(" ")));
            env.insert("LDFLAGS".to_string(), format!("--sysroot={} {}",
                sysroot.display(), self.ldflags.join(" ")));
        } else if !self.cflags.is_empty() || !self.ldflags.is_empty() {
            env.insert("CFLAGS".to_string(), self.cflags.join(" "));
            env.insert("LDFLAGS".to_string(), self.ldflags.join(" "));
        }

        if let Some(pc) = &self.pkg_config_path {
            env.insert("PKG_CONFIG_PATH".to_string(), pc.display().to_string());
            env.insert("PKG_CONFIG_SYSROOT_DIR".to_string(),
                self.sysroot.as_ref().map(|s| s.display().to_string()).unwrap_or_default());
        }

        if !self.rustflags.is_empty() {
            env.insert("RUSTFLAGS".to_string(), self.rustflags.join(" "));
        }

        env
    }

    fn target_env_prefix(&self) -> String {
        // cargo target env key uses uppercase target triple with - -> _
        self.arch.rust_target().to_uppercase().replace('-', "_") + "_"
    }

    /// Check whether the toolchain for this target appears installed
    pub fn check_toolchain(&self) -> ToolchainStatus {
        let prefix = format!("{}-", self.arch.triple().split('-').next().unwrap_or(""));
        // Simple heuristic: check if a cross gcc exists on PATH
        let gcc = format!("{}gcc", self.arch.triple());
        let status = Command::new("which").arg(&gcc).output();
        match status {
            Ok(out) if out.status.success() => ToolchainStatus::Installed,
            _ => ToolchainStatus::Missing(format!("{} (looked for {})", self.name, gcc)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ToolchainStatus {
    Installed,
    Missing(String),
}

// ============================================================================
// CROSS-COMPILATION MANAGER
// ============================================================================

/// Manages cross-compilation targets
#[derive(Debug)]
pub struct CrossCompileManager {
    targets: HashMap<String, CrossTarget>,
    host_arch: TargetArch,
}

impl CrossCompileManager {
    pub fn new() -> Self {
        let host_arch = match std::env::consts::ARCH {
            "x86_64" => TargetArch::X86_64,
            "x86" => TargetArch::X86,
            "aarch64" => TargetArch::Arm64,
            "arm" => TargetArch::Armv7,
            "riscv64" => TargetArch::Riscv64,
            "powerpc64" => TargetArch::Powerpc64le,
            _ => TargetArch::X86_64,
        };
        Self {
            targets: HashMap::new(),
            host_arch,
        }
    }

    pub fn host_arch(&self) -> TargetArch {
        self.host_arch
    }

    pub fn add_target(&mut self, target: CrossTarget) {
        self.targets.insert(target.name.clone(), target);
    }

    pub fn get_target(&self, name: &str) -> Option<&CrossTarget> {
        self.targets.get(name)
    }

    pub fn remove_target(&mut self, name: &str) -> Option<CrossTarget> {
        self.targets.remove(name)
    }

    pub fn list_targets(&self) -> Vec<&CrossTarget> {
        self.targets.values().collect()
    }

    pub fn enabled_targets(&self) -> Vec<&CrossTarget> {
        self.targets.values().filter(|t| t.enabled).collect()
    }

    /// Generate a cargo config snippet (.cargo/config.toml) for all targets
    pub fn generate_cargo_config(&self) -> String {
        let mut out = String::new();
        for target in self.targets.values().filter(|t| t.enabled) {
            out.push_str(&format!("\n[target.{}]\n", target.arch.rust_target()));
            if let Some(linker) = &target.linker {
                out.push_str(&format!("linker = \"{}\"\n", linker));
            }
            if !target.rustflags.is_empty() {
                out.push_str(&format!("rustflags = [{}]\n",
                    target.rustflags.iter()
                        .map(|f| format!("\"{}\"", f))
                        .collect::<Vec<_>>().join(", ")));
            }
        }
        out
    }

    /// Generate makepkg-style cross build environment script
    pub fn generate_build_env_script(&self) -> String {
        let mut out = String::from("#!/bin/sh\n# SigmaOS cross-compilation environment\n\n");
        for target in self.targets.values().filter(|t| t.enabled) {
            out.push_str(&format!("# Target: {} ({})\n", target.name, target.arch));
            for (k, v) in target.env_vars() {
                out.push_str(&format!("export {}=\"{}\"\n", k, v));
            }
            out.push_str("\n");
        }
        out
    }

    /// Check all toolchains and report status
    pub fn check_all_toolchains(&self) -> HashMap<String, ToolchainStatus> {
        self.targets
            .iter()
            .map(|(name, t)| (name.clone(), t.check_toolchain()))
            .collect()
    }

    /// Build command for a target using cargo
    pub fn cargo_build_cmd(&self, target_name: &str, release: bool) -> Option<Command> {
        let target = self.targets.get(target_name)?;
        if !target.enabled {
            return None;
        }
        let mut cmd = Command::new("cargo");
        cmd.arg("build")
            .arg("--target").arg(target.arch.rust_target());
        if release {
            cmd.arg("--release");
        }
        for (k, v) in target.env_vars() {
            cmd.env(k, v);
        }
        Some(cmd)
    }

    /// List multilib packages needed for a secondary arch (Arch-style)
    pub fn multilib_packages(&self, arch: TargetArch) -> Vec<String> {
        match arch {
            TargetArch::X86 => vec![
                "lib32-glibc".to_string(),
                "lib32-gcc-libs".to_string(),
                "lib32-libstdc++".to_string(),
                "lib32-ncurses".to_string(),
                "lib32-zlib".to_string(),
            ],
            _ => Vec::new(),
        }
    }
}

impl Default for CrossCompileManager {
    fn default() -> Self { Self::new() }
}

// ============================================================================
// CROSS-COMPILATION ERRORS
// ============================================================================

#[derive(Debug)]
pub enum CrossCompileError {
    UnknownTarget(String),
    ToolchainMissing(String),
    TargetDisabled(String),
    BuildFailed(String),
}

impl fmt::Display for CrossCompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrossCompileError::UnknownTarget(t) => write!(f, "Unknown target: {}", t),
            CrossCompileError::ToolchainMissing(t) => write!(f, "Toolchain missing for: {}", t),
            CrossCompileError::TargetDisabled(t) => write!(f, "Target disabled: {}", t),
            CrossCompileError::BuildFailed(m) => write!(f, "Build failed: {}", m),
        }
    }
}

impl std::error::Error for CrossCompileError {}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_arch_properties() {
        assert_eq!(TargetArch::X86_64.bits(), 64);
        assert_eq!(TargetArch::X86.bits(), 32);
        assert_eq!(TargetArch::Arm64.bits(), 64);
        assert_eq!(TargetArch::X86.pointer_size(), 4);
        assert!(TargetArch::X86.is_multilib());
        assert!(!TargetArch::X86_64.is_multilib());
        assert!(TargetArch::X86_64.is_little_endian());
    }

    #[test]
    fn test_cross_target_env_vars() {
        let target = CrossTarget::new("arm64", TargetArch::Arm64)
            .with_linker("aarch64-linux-gnu-gcc")
            .with_cflag("-O2")
            .with_rustflag("-C target-cpu=cortex-a72");

        let env = target.env_vars();
        assert_eq!(env.get("TARGET").unwrap(), "aarch64-unknown-linux-gnu");
        assert!(env.contains_key("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER"));
        assert!(env.get("RUSTFLAGS").unwrap().contains("target-cpu"));
    }

    #[test]
    fn test_cross_compile_manager() {
        let mut mgr = CrossCompileManager::new();
        mgr.add_target(CrossTarget::new("arm64", TargetArch::Arm64));
        mgr.add_target(CrossTarget::new("riscv64", TargetArch::Riscv64));

        assert_eq!(mgr.list_targets().len(), 2);
        assert!(mgr.get_target("arm64").is_some());
        assert!(mgr.get_target("nonexistent").is_none());

        let config = mgr.generate_cargo_config();
        assert!(config.contains("aarch64-unknown-linux-gnu"));
        assert!(config.contains("riscv64gc-unknown-linux-gnu"));
    }

    #[test]
    fn test_multilib_packages() {
        let mgr = CrossCompileManager::new();
        let pkgs = mgr.multilib_packages(TargetArch::X86);
        assert!(!pkgs.is_empty());
        assert!(pkgs.contains(&"lib32-glibc".to_string()));
        assert!(mgr.multilib_packages(TargetArch::X86_64).is_empty());
    }

    #[test]
    fn test_build_env_script() {
        let mut mgr = CrossCompileManager::new();
        mgr.add_target(CrossTarget::new("arm64", TargetArch::Arm64)
            .with_linker("aarch64-linux-gnu-gcc"));
        let script = mgr.generate_build_env_script();
        assert!(script.contains("#!/bin/sh"));
        assert!(script.contains("export TARGET="));
    }
}
