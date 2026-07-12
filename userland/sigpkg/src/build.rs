//! SigmaOS Package Building System
//! Builds packages from source with reproducible builds
//! Supports multiple build backends (Cargo, CMake, Make, custom)

use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;
use std::env;

/// Build configuration
#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub source_dir: PathBuf,
    pub build_dir: PathBuf,
    pub install_dir: PathBuf,
    pub build_type: BuildType,
    pub parallel_jobs: usize,
    pub source_date_epoch: Option<i64>,
}

/// Build type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuildType {
    Debug,
    Release,
    Profile,
}

/// Build result
#[derive(Debug, Clone)]
pub struct BuildResult {
    pub success: bool,
    pub output_path: PathBuf,
    pub build_log: String,
    pub duration_secs: u64,
    pub reproducible: bool,
}

/// Build backend
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuildBackend {
    Cargo,
    CMake,
    Make,
    Autotools,
    Custom,
}

/// Package manifest
#[derive(Debug, Clone)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub build_backend: BuildBackend,
    pub dependencies: Vec<String>,
    pub build_script: Option<String>,
}

impl BuildConfig {
    pub fn new(source_dir: PathBuf) -> Self {
        Self {
            source_dir: source_dir.clone(),
            build_dir: source_dir.join("build"),
            install_dir: source_dir.join("install"),
            build_type: BuildType::Release,
            parallel_jobs: num_cpus::get(),
            source_date_epoch: env::var("SOURCE_DATE_EPOCH")
                .ok()
                .and_then(|s| s.parse().ok()),
        }
    }
    
    pub fn with_build_type(mut self, build_type: BuildType) -> Self {
        self.build_type = build_type;
        self
    }
    
    pub fn with_parallel_jobs(mut self, jobs: usize) -> Self {
        self.parallel_jobs = jobs;
        self
    }
    
    pub fn with_source_date_epoch(mut self, epoch: i64) -> Self {
        self.source_date_epoch = Some(epoch);
        self
    }
}

impl BuildResult {
    pub fn success(output_path: PathBuf, build_log: String, duration_secs: u64) -> Self {
        Self {
            success: true,
            output_path,
            build_log,
            duration_secs,
            reproducible: true,
        }
    }
    
    pub fn failure(build_log: String, duration_secs: u64) -> Self {
        Self {
            success: false,
            output_path: PathBuf::new(),
            build_log,
            duration_secs,
            reproducible: false,
        }
    }
}

/// Build package from source
pub fn build_package(config: &BuildConfig, manifest: &PackageManifest) -> BuildResult {
    let start = std::time::Instant::now();
    
    // Set environment variables for reproducible builds
    let mut env = std::collections::HashMap::new();
    env.insert("CARGO_BUILD_JOBS".to_string(), config.parallel_jobs.to_string());
    
    if let Some(epoch) = config.source_date_epoch {
        env.insert("SOURCE_DATE_EPOCH".to_string(), epoch.to_string());
        env.insert("BUILD_DATE".to_string(), epoch.to_string());
    }
    
    // Create build directory
    fs::create_dir_all(&config.build_dir).expect("Failed to create build directory");
    
    // Run build based on backend
    let result = match manifest.build_backend {
        BuildBackend::Cargo => build_cargo(config, manifest, &env),
        BuildBackend::CMake => build_cmake(config, manifest, &env),
        BuildBackend::Make => build_make(config, manifest, &env),
        BuildBackend::Autotools => build_autotools(config, manifest, &env),
        BuildBackend::Custom => build_custom(config, manifest, &env),
    };
    
    let duration = start.elapsed().as_secs();
    
    match result {
        Ok(log) => BuildResult::success(config.build_dir.clone(), log, duration),
        Err(log) => BuildResult::failure(log, duration),
    }
}

/// Build Cargo project
fn build_cargo(config: &BuildConfig, manifest: &PackageManifest, env: &std::collections::HashMap<String, String>) -> Result<String, String> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&config.source_dir);
    cmd.arg("build");
    
    match config.build_type {
        BuildType::Debug => cmd.arg("--debug"),
        BuildType::Release => {
            cmd.arg("--release");
            cmd.arg("--profile").arg("release");
        }
        BuildType::Profile => {
            cmd.arg("--release");
            cmd.arg("--profile").arg("profile");
        }
    }
    
    // Set environment variables
    for (key, value) in env {
        cmd.env(key, value);
    }
    
    let output = cmd.output().map_err(|e| format!("Failed to run cargo: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Build CMake project
fn build_cmake(config: &BuildConfig, manifest: &PackageManifest, env: &std::collections::HashMap<String, String>) -> Result<String, String> {
    // Configure
    let mut configure = Command::new("cmake");
    configure.current_dir(&config.build_dir);
    configure.arg("-S").arg(&config.source_dir);
    configure.arg("-B").arg(&config.build_dir);
    
    match config.build_type {
        BuildType::Debug => configure.arg("-DCMAKE_BUILD_TYPE=Debug"),
        BuildType::Release => configure.arg("-DCMAKE_BUILD_TYPE=Release"),
        BuildType::Profile => configure.arg("-DCMAKE_BUILD_TYPE=RelWithDebInfo"),
    }
    
    configure.arg("-GNinja");
    
    for (key, value) in env {
        configure.env(key, value);
    }
    
    let output = configure.output().map_err(|e| format!("Failed to run cmake: {}", e))?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    
    // Build
    let mut build = Command::new("ninja");
    build.current_dir(&config.build_dir);
    build.arg("-j").arg(config.parallel_jobs.to_string());
    
    let output = build.output().map_err(|e| format!("Failed to run ninja: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Build Make project
fn build_make(config: &BuildConfig, manifest: &PackageManifest, env: &std::collections::HashMap<String, String>) -> Result<String, String> {
    let mut cmd = Command::new("make");
    cmd.current_dir(&config.source_dir);
    cmd.arg("-j").arg(config.parallel_jobs.to_string());
    
    match config.build_type {
        BuildType::Debug => cmd.env("CFLAGS", "-g -O0"),
        BuildType::Release => cmd.env("CFLAGS", "-O3"),
        BuildType::Profile => cmd.env("CFLAGS", "-O2 -g"),
    }
    
    for (key, value) in env {
        cmd.env(key, value);
    }
    
    let output = cmd.output().map_err(|e| format!("Failed to run make: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Build Autotools project
fn build_autotools(config: &BuildConfig, manifest: &PackageManifest, env: &std::collections::HashMap<String, String>) -> Result<String, String> {
    // Configure
    let mut configure = Command::new("./configure");
    configure.current_dir(&config.source_dir);
    
    match config.build_type {
        BuildType::Debug => configure.arg("--enable-debug"),
        BuildType::Release => configure.arg("--disable-debug"),
        BuildType::Profile => configure.arg("--enable-profile"),
    }
    
    for (key, value) in env {
        configure.env(key, value);
    }
    
    let output = configure.output().map_err(|e| format!("Failed to run configure: {}", e))?;
    
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    
    // Build
    let mut build = Command::new("make");
    build.current_dir(&config.source_dir);
    build.arg("-j").arg(config.parallel_jobs.to_string());
    
    let output = build.output().map_err(|e| format!("Failed to run make: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Build with custom script
fn build_custom(config: &BuildConfig, manifest: &PackageManifest, env: &std::collections::HashMap<String, String>) -> Result<String, String> {
    let script = manifest.build_script.as_ref().ok_or("No build script specified")?;
    
    let mut cmd = Command::new(script);
    cmd.current_dir(&config.source_dir);
    
    for (key, value) in env {
        cmd.env(key, value);
    }
    
    let output = cmd.output().map_err(|e| format!("Failed to run custom build script: {}", e))?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Verify build reproducibility
pub fn verify_reproducibility(config1: &BuildConfig, config2: &BuildConfig, manifest: &PackageManifest) -> bool {
    let result1 = build_package(config1, manifest);
    let result2 = build_package(config2, manifest);
    
    if !result1.success || !result2.success {
        return false;
    }
    
    // Compare file hashes
    let hash1 = hash_file(&result1.output_path);
    let hash2 = hash_file(&result2.output_path);
    
    hash1 == hash2
}

/// Calculate file hash
fn hash_file(path: &Path) -> String {
    use std::io::Read;
    
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return String::new(),
    };
    
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let n = match file.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => break,
        };
        
        if n == 0 {
            break;
        }
        
        hasher.update(&buffer[..n]);
    }
    
    format!("{:x}", hasher.finalize())
}

/// Parse package manifest
pub fn parse_manifest(path: &Path) -> Result<PackageManifest, String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read manifest: {}", e))?;
    
    // Parse TOML manifest
    let manifest: toml::Value = toml::from_str(&content).map_err(|e| format!("Failed to parse TOML: {}", e))?;
    
    let name = manifest.get("name")
        .and_then(|v| v.as_str())
        .ok_or("Missing name")?
        .to_string();
    
    let version = manifest.get("version")
        .and_then(|v| v.as_str())
        .ok_or("Missing version")?
        .to_string();
    
    let description = manifest.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    let build_backend_str = manifest.get("build_backend")
        .and_then(|v| v.as_str())
        .unwrap_or("cargo");
    
    let build_backend = match build_backend_str {
        "cargo" => BuildBackend::Cargo,
        "cmake" => BuildBackend::CMake,
        "make" => BuildBackend::Make,
        "autotools" => BuildBackend::Autotools,
        "custom" => BuildBackend::Custom,
        _ => BuildBackend::Cargo,
    };
    
    let dependencies = manifest.get("dependencies")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();
    
    let build_script = manifest.get("build_script")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    Ok(PackageManifest {
        name,
        version,
        description,
        build_backend,
        dependencies,
        build_script,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_config() {
        let config = BuildConfig::new(PathBuf::from("/tmp/test"));
        assert_eq!(config.build_type, BuildType::Release);
    }
    
    #[test]
    fn test_build_config_with_type() {
        let config = BuildConfig::new(PathBuf::from("/tmp/test"))
            .with_build_type(BuildType::Debug);
        assert_eq!(config.build_type, BuildType::Debug);
    }
}
