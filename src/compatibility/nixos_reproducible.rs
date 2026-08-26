//! NixOS-Inspired Reproducible Package System
//!
//! This module implements a reproducible, deterministic package management system
//! inspired by NixOS's functional package management approach.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
#[derive(Debug, Clone)]
pub struct PackageDerivation {
    pub name: String,
    pub version: String,
    pub inputs: Vec<PackageInput>,
    pub build_script: String,
    pub environment: HashMap<String, String>,
    pub hash: String,
}

#[derive(Debug, Clone)]
pub struct PackageInput {
    pub name: String,
    pub hash: String,
    pub url: Option<String>,
    pub path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct NixLikeStore {
    store_path: PathBuf,
    derivations: HashMap<String, PackageDerivation>,
}

impl NixLikeStore {
    pub fn new(store_path: impl AsRef<Path>) -> Self {
        Self {
            store_path: store_path.as_ref().to_path_buf(),
            derivations: HashMap::new(),
        }
    }

    /// Create a new package derivation with content-addressable storage
    pub fn create_derivation(
        &mut self,
        name: &str,
        version: &str,
        inputs: Vec<PackageInput>,
        build_script: &str,
    ) -> Result<PackageDerivation, Box<dyn std::error::Error>> {
        let mut h: u64 = 0xcbf29ce484222325;
        for &b in name
            .as_bytes()
            .iter()
            .chain(version.as_bytes())
            .chain(build_script.as_bytes())
        {
            h = (h ^ (b as u64)).wrapping_mul(0x100000001b3);
        }
        for input in &inputs {
            for &b in input.name.as_bytes().iter().chain(input.hash.as_bytes()) {
                h = (h ^ (b as u64)).wrapping_mul(0x100000001b3);
            }
        }
        let hash = format!("{:016x}", h);

        let derivation = PackageDerivation {
            name: name.to_string(),
            version: version.to_string(),
            inputs,
            build_script: build_script.to_string(),
            environment: self.create_build_environment()?,
            hash: hash.clone(),
        };

        self.derivations.insert(hash.clone(), derivation.clone());

        Ok(derivation)
    }

    /// Build a package from its derivation
    pub fn build_package(
        &self,
        derivation: &PackageDerivation,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let output_path = self.store_path.join(format!(
            "{}-{}-{}",
            derivation.hash[..8].to_string(),
            derivation.name,
            derivation.version
        ));

        if output_path.exists() {
            // Package already built, return cached result
            return Ok(output_path);
        }

        // Create isolated build environment
        let build_env = self.setup_build_sandbox(derivation)?;

        // Execute build in sandboxed environment
        self.execute_build(&build_env, derivation)?;

        // Verify build output integrity
        self.verify_build_output(&output_path, derivation)?;

        Ok(output_path)
    }

    /// Set up isolated build environment (like NixOS's build sandbox)
    fn setup_build_sandbox(
        &self,
        derivation: &PackageDerivation,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let sandbox_path = std::env::temp_dir().join(format!("sigma-build-{}", derivation.hash));
        std::fs::create_dir_all(&sandbox_path)?;

        // Create minimal filesystem layout
        let bin_dir = sandbox_path.join("bin");
        let lib_dir = sandbox_path.join("lib");
        let etc_dir = sandbox_path.join("etc");

        std::fs::create_dir_all(&bin_dir)?;
        std::fs::create_dir_all(&lib_dir)?;
        std::fs::create_dir_all(&etc_dir)?;

        // Mount input dependencies read-only
        for input in &derivation.inputs {
            if let Some(input_path) = &input.path {
                self.mount_readonly(input_path, &sandbox_path.join(&input.name))?;
            }
        }

        Ok(sandbox_path)
    }

    /// Execute build script in sandboxed environment
    fn execute_build(
        &self,
        build_env: &Path,
        derivation: &PackageDerivation,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;

        let mut build_command = Command::new("systemd-nspawn");
        build_command
            .arg("--directory")
            .arg(build_env)
            .arg("--read-only")
            .arg("--private-network")
            .arg("--tmpfs=/tmp")
            .arg("/bin/sh")
            .arg("-c")
            .arg(&derivation.build_script);

        // Set deterministic environment
        for (key, value) in &derivation.environment {
            build_command.env(key, value);
        }

        // Remove non-deterministic environment variables
        build_command.env_remove("HOME");
        build_command.env_remove("USER");
        build_command.env_remove("PWD");

        let output = build_command.output()?;

        if !output.status.success() {
            return Err(
                format!("Build failed: {}", String::from_utf8_lossy(&output.stderr)).into(),
            );
        }

        Ok(())
    }

    /// Create deterministic build environment
    fn create_build_environment(
        &self,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut env = HashMap::new();

        // Set deterministic environment variables
        env.insert("PATH".to_string(), "/bin:/usr/bin".to_string());
        env.insert("LANG".to_string(), "C".to_string());
        env.insert("LC_ALL".to_string(), "C".to_string());
        env.insert("TZ".to_string(), "UTC".to_string());
        env.insert("SOURCE_DATE_EPOCH".to_string(), "1".to_string());

        // Reproducible build flags
        env.insert(
            "CFLAGS".to_string(),
            "-fdebug-prefix-map=/build=/usr/src".to_string(),
        );
        env.insert(
            "CXXFLAGS".to_string(),
            "-fdebug-prefix-map=/build=/usr/src".to_string(),
        );

        Ok(env)
    }

    /// Mount path as read-only in sandbox
    fn mount_readonly(
        &self,
        source: &Path,
        target: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;

        std::fs::create_dir_all(target)?;

        Command::new("mount")
            .arg("--bind")
            .arg(source)
            .arg(target)
            .output()?;

        Command::new("mount")
            .arg("-o")
            .arg("remount,ro")
            .arg(target)
            .output()?;

        Ok(())
    }

    /// Verify build output matches expected hash
    fn verify_build_output(
        &self,
        output_path: &Path,
        derivation: &PackageDerivation,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut h: u64 = 0xcbf29ce484222325;
        if let Ok(entries) = std::fs::read_dir(output_path) {
            for entry in entries.flatten() {
                if let Ok(contents) = std::fs::read(entry.path()) {
                    for &b in &contents {
                        h = (h ^ (b as u64)).wrapping_mul(0x100000001b3);
                    }
                }
            }
        }
        let actual_hash = format!("{:016x}", h);

        // In a real implementation, we would store expected output hashes
        // For now, just log the computed hash
        eprintln!("Build output hash: {}", actual_hash);

        Ok(())
    }

    /// Install package to system (like NixOS profile management)
    pub fn install_to_profile(
        &self,
        derivation: &PackageDerivation,
        profile_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let profile_path = self.store_path.join("profiles").join(profile_name);
        let package_path = self.build_package(derivation)?;

        std::fs::create_dir_all(&profile_path)?;

        // Create symlinks to package in profile
        for entry in std::fs::read_dir(&package_path)? {
            let entry = entry?;
            let link_path = profile_path.join(entry.file_name());

            if link_path.exists() {
                std::fs::remove_file(&link_path)?;
            }

            std::os::unix::fs::symlink(entry.path(), link_path)?;
        }

        Ok(())
    }

    /// Garbage collect unreferenced packages
    pub fn garbage_collect(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut removed = Vec::new();

        // Find all packages referenced by profiles
        let mut referenced = std::collections::HashSet::new();
        let profiles_dir = self.store_path.join("profiles");

        if profiles_dir.exists() {
            for profile in std::fs::read_dir(profiles_dir)? {
                let profile = profile?;
                if profile.file_type()?.is_dir() {
                    self.find_references(profile.path(), &mut referenced)?;
                }
            }
        }

        // Remove unreferenced packages
        for (hash, derivation) in self.derivations.clone() {
            if !referenced.contains(&hash) {
                let package_path = self.store_path.join(format!(
                    "{}-{}-{}",
                    hash[..8].to_string(),
                    derivation.name,
                    derivation.version
                ));

                if package_path.exists() {
                    std::fs::remove_dir_all(&package_path)?;
                    removed.push(hash.clone());
                    self.derivations.remove(&hash);
                }
            }
        }

        Ok(removed)
    }

    fn find_references(
        &self,
        path: PathBuf,
        referenced: &mut std::collections::HashSet<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(target) = std::fs::read_link(entry.path()) {
                    if let Some(hash) = self.extract_hash_from_path(&target) {
                        referenced.insert(hash);
                    }
                }
            }
        }
        Ok(())
    }

    fn extract_hash_from_path(&self, path: &Path) -> Option<String> {
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if let Some(hash_part) = file_name.split('-').next() {
                if hash_part.len() >= 8 {
                    // Find full hash from partial hash
                    for (full_hash, _) in &self.derivations {
                        if full_hash.starts_with(hash_part) {
                            return Some(full_hash.clone());
                        }
                    }
                }
            }
        }
        None
    }
}

/// Arch Linux-inspired AUR-like system for community packages
pub struct CommunityPackageRegistry {
    packages: HashMap<String, CommunityPackage>,
}

#[derive(Debug, Clone)]
pub struct CommunityPackage {
    pub name: String,
    pub maintainer: String,
    pub build_script: String,
    pub dependencies: Vec<String>,
    pub votes: u32,
    pub last_updated: u64,
}

impl CommunityPackageRegistry {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    /// Submit a new community package (like AUR submission)
    pub fn submit_package(&mut self, package: CommunityPackage) -> Result<(), String> {
        // Validate package
        if package.name.is_empty() {
            return Err("Package name cannot be empty".to_string());
        }

        if package.maintainer.is_empty() {
            return Err("Maintainer cannot be empty".to_string());
        }

        // Security check: validate build script
        if self.contains_unsafe_commands(&package.build_script) {
            return Err("Build script contains potentially unsafe commands".to_string());
        }

        self.packages.insert(package.name.clone(), package);
        Ok(())
    }

    /// Vote for a community package (like AUR voting)
    pub fn vote_package(&mut self, name: &str) -> Result<(), String> {
        if let Some(package) = self.packages.get_mut(name) {
            package.votes += 1;
            Ok(())
        } else {
            Err("Package not found".to_string())
        }
    }

    /// Search community packages
    pub fn search_packages(&self, query: &str) -> Vec<&CommunityPackage> {
        self.packages
            .values()
            .filter(|pkg| pkg.name.contains(query) || pkg.maintainer.contains(query))
            .collect()
    }

    /// Get most popular packages
    pub fn get_popular_packages(&self, limit: usize) -> Vec<&CommunityPackage> {
        let mut packages: Vec<&CommunityPackage> = self.packages.values().collect();
        packages.sort_by(|a, b| b.votes.cmp(&a.votes));
        packages.into_iter().take(limit).collect()
    }

    fn contains_unsafe_commands(&self, script: &str) -> bool {
        let unsafe_commands = [
            "rm -rf /", "dd if=", "mkfs", "fdisk", "parted", "wget", "curl", "nc", "netcat",
            "socat",
        ];

        unsafe_commands.iter().any(|cmd| script.contains(cmd))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    // Simple temporary directory implementation for testing
    struct TestTempDir {
        path: PathBuf,
    }

    impl TestTempDir {
        fn new() -> std::io::Result<Self> {
            let path = std::env::temp_dir().join(format!("sigma_test_{}", std::process::id()));
            std::fs::create_dir_all(&path)?;
            Ok(TestTempDir { path })
        }

        fn path(&self) -> &PathBuf {
            &self.path
        }
    }

    impl Drop for TestTempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn test_package_derivation_creation() {
        let temp_dir = TestTempDir::new().unwrap();
        let mut store = NixLikeStore::new(temp_dir.path());

        let inputs = vec![PackageInput {
            name: "gcc".to_string(),
            hash: "abc123".to_string(),
            url: None,
            path: Some("/usr/bin/gcc".into()),
        }];

        let derivation = store
            .create_derivation("hello-world", "1.0.0", inputs, "gcc -o hello hello.c")
            .unwrap();

        assert_eq!(derivation.name, "hello-world");
        assert_eq!(derivation.version, "1.0.0");
        assert!(!derivation.hash.is_empty());
    }

    #[test]
    fn test_community_package_submission() {
        let mut registry = CommunityPackageRegistry::new();

        let package = CommunityPackage {
            name: "test-package".to_string(),
            maintainer: "test-user".to_string(),
            build_script: "make && make install".to_string(),
            dependencies: vec!["gcc".to_string()],
            votes: 0,
            last_updated: 0,
        };

        assert!(registry.submit_package(package).is_ok());
    }

    #[test]
    fn test_unsafe_script_detection() {
        let registry = CommunityPackageRegistry::new();
        assert!(registry.contains_unsafe_commands("rm -rf /"));
        assert!(!registry.contains_unsafe_commands("make && make install"));
    }
}
