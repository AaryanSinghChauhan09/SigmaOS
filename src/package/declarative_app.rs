// Declarative App Manifests, Immutable App Layers, and Shards Marketplace Engine
// Conforms to SigmaOS Zero-Dependency, Sovereign Package Architecture

use std::collections::HashMap;

/// Hardware access permissions requested by a declarative application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HardwareAccessPermissions {
    pub allow_gpu_compute: bool,
    pub allow_audio_playback: bool,
    pub allow_camera: bool,
    pub allow_network_access: bool,
    pub allow_usb_devices: bool,
}

impl HardwareAccessPermissions {
    pub fn new() -> Self {
        Self {
            allow_gpu_compute: false,
            allow_audio_playback: false,
            allow_camera: false,
            allow_network_access: false,
            allow_usb_devices: false,
        }
    }

    pub fn full_access() -> Self {
        Self {
            allow_gpu_compute: true,
            allow_audio_playback: true,
            allow_camera: true,
            allow_network_access: true,
            allow_usb_devices: true,
        }
    }
}

impl Default for HardwareAccessPermissions {
    fn default() -> Self {
        Self::new()
    }
}

/// Declarative application manifest (single config file specification)
#[derive(Debug, Clone)]
pub struct DeclarativeAppManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub publisher: String,
    pub entrypoint: String,
    pub dependencies: Vec<String>,
    pub environment: HashMap<String, String>,
    pub hardware_permissions: HardwareAccessPermissions,
    pub memory_limit_mb: u64,
    pub cpu_cores_limit: u32,
}

impl DeclarativeAppManifest {
    pub fn new(name: &str, version: &str, entrypoint: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: String::new(),
            publisher: "SigmaOS Developer Community".to_string(),
            entrypoint: entrypoint.to_string(),
            dependencies: Vec::new(),
            environment: HashMap::new(),
            hardware_permissions: HardwareAccessPermissions::new(),
            memory_limit_mb: 512,
            cpu_cores_limit: 2,
        }
    }

    /// Parses a TOML/YAML-style declarative manifest configuration string
    pub fn parse_manifest_spec(content: &str) -> Result<Self, &'static str> {
        let mut name = String::new();
        let mut version = String::new();
        let mut entrypoint = String::new();
        let mut description = String::new();
        let mut publisher = String::new();
        let mut dependencies = Vec::new();
        let mut env = HashMap::new();
        let mut perms = HardwareAccessPermissions::new();

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some((key, val)) = line.split_once('=') {
                let k = key.trim();
                let v = val.trim().trim_matches('"').trim_matches('\'');
                match k {
                    "name" => name = v.to_string(),
                    "version" => version = v.to_string(),
                    "entrypoint" => entrypoint = v.to_string(),
                    "description" => description = v.to_string(),
                    "publisher" => publisher = v.to_string(),
                    "allow_gpu" => perms.allow_gpu_compute = v.parse().unwrap_or(false),
                    "allow_audio" => perms.allow_audio_playback = v.parse().unwrap_or(false),
                    "allow_network" => perms.allow_network_access = v.parse().unwrap_or(false),
                    "allow_usb" => perms.allow_usb_devices = v.parse().unwrap_or(false),
                    "depends" => {
                        for dep in v.split(',') {
                            let clean_dep = dep.trim();
                            if !clean_dep.is_empty() {
                                dependencies.push(clean_dep.to_string());
                            }
                        }
                    }
                    _ => {
                        if k.starts_with("env.") {
                            let env_var = k.trim_start_matches("env.");
                            env.insert(env_var.to_string(), v.to_string());
                        }
                    }
                }
            }
        }

        if name.is_empty() || version.is_empty() || entrypoint.is_empty() {
            return Err("Declarative manifest requires 'name', 'version', and 'entrypoint'");
        }

        Ok(DeclarativeAppManifest {
            name,
            version,
            description,
            publisher,
            entrypoint,
            dependencies,
            environment: env,
            hardware_permissions: perms,
            memory_limit_mb: 512,
            cpu_cores_limit: 2,
        })
    }
}

/// Immutable application layer (read-only Squashed overlay layer)
#[derive(Debug, Clone)]
pub struct ImmutableAppLayer {
    pub manifest: DeclarativeAppManifest,
    pub layer_hash: String,
    pub mount_path: String,
    pub is_read_only: bool,
    pub active_slot: char, // 'A' or 'B' for zero-downtime atomic hot reboots
}

impl ImmutableAppLayer {
    pub fn new(manifest: DeclarativeAppManifest, hash: &str) -> Self {
        let mount_path = format!("/shards/layers/{}-{}", manifest.name, manifest.version);
        Self {
            manifest,
            layer_hash: hash.to_string(),
            mount_path,
            is_read_only: true,
            active_slot: 'A',
        }
    }

    /// Performs zero-downtime atomic A/B slot switch for immutable app update
    pub fn switch_active_slot(&mut self, new_version: &str, new_hash: &str) {
        self.manifest.version = new_version.to_string();
        self.layer_hash = new_hash.to_string();
        self.active_slot = if self.active_slot == 'A' { 'B' } else { 'A' };
        self.mount_path = format!(
            "/shards/layers/{}-{}-slot_{}",
            self.manifest.name, self.manifest.version, self.active_slot
        );
    }
}

/// Curated Shards Marketplace for modular SigmaOS applications
pub struct ShardsMarketplace {
    pub marketplace_name: String,
    pub published_apps: HashMap<String, DeclarativeAppManifest>,
    pub installed_layers: HashMap<String, ImmutableAppLayer>,
}

impl ShardsMarketplace {
    pub fn new() -> Self {
        Self {
            marketplace_name: "SigmaOS Sovereign Shards Hub".to_string(),
            published_apps: HashMap::new(),
            installed_layers: HashMap::new(),
        }
    }

    /// Publishes a modular app manifest to the Shards Marketplace
    pub fn publish_app(&mut self, manifest: DeclarativeAppManifest) {
        self.published_apps.insert(manifest.name.clone(), manifest);
    }

    /// Search published Shard apps by keyword or name
    pub fn search_apps(&self, query: &str) -> Vec<&DeclarativeAppManifest> {
        let query_lower = query.to_lowercase();
        self.published_apps
            .values()
            .filter(|app| {
                app.name.to_lowercase().contains(&query_lower)
                    || app.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Installs a published Shard app into an immutable read-only layer
    pub fn install_shard(&mut self, app_name: &str) -> Result<&ImmutableAppLayer, &'static str> {
        let manifest = self
            .published_apps
            .get(app_name)
            .cloned()
            .ok_or("App shard not found in marketplace")?;

        // Simulate hash generation for immutable SquashFS overlay layer
        let hash = format!("{:x}", manifest.name.len() * 1234567);
        let layer = ImmutableAppLayer::new(manifest, &hash);

        self.installed_layers.insert(app_name.to_string(), layer);
        Ok(self.installed_layers.get(app_name).unwrap())
    }
}

impl Default for ShardsMarketplace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declarative_manifest_parsing() {
        let spec = r#"
            name = "zenith-editor"
            version = "1.2.0"
            entrypoint = "/usr/bin/zenith"
            description = "High-fidelity Sovereign Text Editor"
            allow_gpu = "true"
            allow_audio = "false"
            allow_network = "true"
            depends = "sigma-libc, zenith-gui"
            env.THEME = "dark"
        "#;

        let manifest = DeclarativeAppManifest::parse_manifest_spec(spec).unwrap();
        assert_eq!(manifest.name, "zenith-editor");
        assert_eq!(manifest.version, "1.2.0");
        assert_eq!(manifest.entrypoint, "/usr/bin/zenith");
        assert!(manifest.hardware_permissions.allow_gpu_compute);
        assert!(!manifest.hardware_permissions.allow_audio_playback);
        assert!(manifest.hardware_permissions.allow_network_access);
        assert_eq!(manifest.dependencies, vec!["sigma-libc", "zenith-gui"]);
        assert_eq!(manifest.environment.get("THEME").unwrap(), "dark");
    }

    #[test]
    fn test_immutable_layer_slot_switching() {
        let manifest = DeclarativeAppManifest::new("sigma-terminal", "0.9.0", "/bin/sigterm");
        let mut layer = ImmutableAppLayer::new(manifest, "hash1234");

        assert_eq!(layer.active_slot, 'A');
        assert!(layer.is_read_only);

        layer.switch_active_slot("1.0.0", "hash5678");
        assert_eq!(layer.active_slot, 'B');
        assert_eq!(layer.manifest.version, "1.0.0");
        assert!(layer.mount_path.contains("slot_B"));
    }

    #[test]
    fn test_shards_marketplace_workflow() {
        let mut marketplace = ShardsMarketplace::new();
        let app = DeclarativeAppManifest::new("calculator-shard", "2.0.0", "/bin/calc");
        marketplace.publish_app(app);

        let results = marketplace.search_apps("calc");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "calculator-shard");

        let layer = marketplace.install_shard("calculator-shard").unwrap();
        assert!(layer.is_read_only);
        assert_eq!(layer.manifest.name, "calculator-shard");
    }
}
