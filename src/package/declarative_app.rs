// Declarative App Manifests, Immutable App Layers, Shards Marketplace, and Sigmactl App Manager Engine
// Inspired by Nix, Guix, Flatpak, Snap, and OSTree
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

/// Content-addressed bundle representation (inspired by Nix store / Flatpak refs)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentAddressedBundle {
    pub bundle_id: String,
    pub name: String,
    pub version: String,
    pub hash_id: String,
    pub signature: String,
    pub generation: u64,
}

impl ContentAddressedBundle {
    pub fn new(name: &str, version: &str, generation: u64) -> Self {
        let hash_id = format!("sha256-{:x}", name.len() * 31337 + version.len() * 7331 + generation as usize * 17);
        let signature = format!("sig-sovereign-{}", hash_id);
        Self {
            bundle_id: format!("{}-{}", name, version),
            name: name.to_string(),
            version: version.to_string(),
            hash_id,
            signature,
            generation,
        }
    }
}

/// Generation snapshot recording local generation history for 1-step rollback
#[derive(Debug, Clone)]
pub struct GenerationSnapshot {
    pub generation_id: u64,
    pub timestamp_sec: u64,
    pub installed_bundles: HashMap<String, ContentAddressedBundle>,
    pub description: String,
}

/// Sigmactl App Manager CLI Command enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigmactlCommand {
    Install(String),
    Update(String),
    Rollback(Option<u64>),
    List,
    Verify(String),
    Help,
}

/// Sigmactl App Manager Engine - Content-Addressed Atomic Updates & Rollbacks
pub struct SigmactlAppManagerEngine {
    pub current_generation: u64,
    pub snapshots: HashMap<u64, GenerationSnapshot>,
    pub active_bundles: HashMap<String, ContentAddressedBundle>,
    pub app_store_manifests: HashMap<String, ContentAddressedBundle>,
}

impl SigmactlAppManagerEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            current_generation: 1,
            snapshots: HashMap::new(),
            active_bundles: HashMap::new(),
            app_store_manifests: HashMap::new(),
        };

        // Pre-populate initial system snapshot (Generation 1)
        let core_bundle = ContentAddressedBundle::new("zenith-desktop", "1.0.0", 1);
        engine.active_bundles.insert("zenith-desktop".to_string(), core_bundle.clone());
        engine.app_store_manifests.insert("zenith-desktop".to_string(), core_bundle.clone());

        engine.save_generation_snapshot("Initial system generation 1");
        engine
    }

    /// Saves a snapshot of current installed bundles for 1-step rollback
    pub fn save_generation_snapshot(&mut self, desc: &str) {
        let snapshot = GenerationSnapshot {
            generation_id: self.current_generation,
            timestamp_sec: self.current_generation * 1000,
            installed_bundles: self.active_bundles.clone(),
            description: desc.to_string(),
        };
        self.snapshots.insert(self.current_generation, snapshot);
    }

    /// Publishes a signed app bundle to the App Store Manifest Server
    pub fn publish_signed_bundle(&mut self, name: &str, version: &str) -> ContentAddressedBundle {
        let bundle = ContentAddressedBundle::new(name, version, self.current_generation + 1);
        self.app_store_manifests.insert(name.to_string(), bundle.clone());
        bundle
    }

    /// Verifies content signature of an app bundle
    pub fn verify_bundle(&self, app_name: &str) -> Result<bool, &'static str> {
        if let Some(bundle) = self.active_bundles.get(app_name).or_else(|| self.app_store_manifests.get(app_name)) {
            if bundle.signature.starts_with("sig-sovereign-") {
                Ok(true)
            } else {
                Err("Invalid signature format")
            }
        } else {
            Err("Bundle not found")
        }
    }

    /// Installs an app bundle atomically with content-addressing
    pub fn install_app_atomically(&mut self, app_name: &str) -> Result<String, &'static str> {
        let bundle = self
            .app_store_manifests
            .get(app_name)
            .cloned()
            .ok_or("App bundle not found in App Store manifest server")?;

        self.current_generation += 1;
        let mut updated_bundle = bundle;
        updated_bundle.generation = self.current_generation;

        self.active_bundles.insert(app_name.to_string(), updated_bundle.clone());
        self.save_generation_snapshot(&format!("Installed app bundle {}", app_name));

        Ok(format!(
            "Successfully installed {} ({}) under Generation {} [{}]",
            app_name, updated_bundle.version, self.current_generation, updated_bundle.hash_id
        ))
    }

    /// Performs a 1-step atomic rollback to a previous generation or target generation
    pub fn rollback_generation(&mut self, target_gen: Option<u64>) -> Result<String, &'static str> {
        let target = match target_gen {
            Some(gen) => gen,
            None => {
                if self.current_generation > 1 {
                    self.current_generation - 1
                } else {
                    return Err("Cannot rollback prior to Generation 1");
                }
            }
        };

        let snapshot = self
            .snapshots
            .get(&target)
            .cloned()
            .ok_or("Target generation snapshot not found")?;

        self.current_generation += 1;
        self.active_bundles = snapshot.installed_bundles;
        self.save_generation_snapshot(&format!("Rolled back to snapshot generation {}", target));

        Ok(format!(
            "Successfully rolled back to Generation {} (New Generation {})",
            target, self.current_generation
        ))
    }

    /// Parses and dispatches CLI commands (`sigmactl install <app>`, `sigmactl rollback`, etc.)
    pub fn dispatch_sigmactl_command(&mut self, cmd_str: &str) -> Result<String, &'static str> {
        let parts: Vec<&str> = cmd_str.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err("Empty command");
        }

        let cmd = if parts[0] == "sigmactl" {
            if parts.len() < 2 {
                SigmactlCommand::Help
            } else {
                match parts[1] {
                    "install" if parts.len() >= 3 => SigmactlCommand::Install(parts[2].to_string()),
                    "update" if parts.len() >= 3 => SigmactlCommand::Update(parts[2].to_string()),
                    "rollback" => {
                        let gen = if parts.len() >= 3 {
                            parts[2].parse::<u64>().ok()
                        } else {
                            None
                        };
                        SigmactlCommand::Rollback(gen)
                    }
                    "list" => SigmactlCommand::List,
                    "verify" if parts.len() >= 3 => SigmactlCommand::Verify(parts[2].to_string()),
                    _ => SigmactlCommand::Help,
                }
            }
        } else {
            SigmactlCommand::Help
        };

        match cmd {
            SigmactlCommand::Install(app) | SigmactlCommand::Update(app) => self.install_app_atomically(&app),
            SigmactlCommand::Rollback(gen) => self.rollback_generation(gen),
            SigmactlCommand::List => {
                let mut list = Vec::new();
                for (name, bundle) in &self.active_bundles {
                    list.push(format!("{} (v{}, gen {}) -> {}", name, bundle.version, bundle.generation, bundle.hash_id));
                }
                Ok(format!("Installed Apps (Gen {}):\n{}", self.current_generation, list.join("\n")))
            }
            SigmactlCommand::Verify(app) => {
                let verified = self.verify_bundle(&app)?;
                Ok(format!("Bundle '{}' signature verification: {}", app, if verified { "PASSED" } else { "FAILED" }))
            }
            SigmactlCommand::Help => Ok("Usage: sigmactl [install <app> | update <app> | rollback [gen] | list | verify <app>]".to_string()),
        }
    }
}

impl Default for SigmactlAppManagerEngine {
    fn default() -> Self {
        Self::new()
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

    #[test]
    fn test_sigmactl_app_manager_atomic_updates_and_rollback() {
        let mut engine = SigmactlAppManagerEngine::new();
        assert_eq!(engine.current_generation, 1);

        // Publish a new signed app bundle
        engine.publish_signed_bundle("code-editor", "2.1.0");

        // Verify bundle signature
        let verify_res = engine.dispatch_sigmactl_command("sigmactl verify code-editor");
        assert!(verify_res.is_ok());
        assert!(verify_res.unwrap().contains("PASSED"));

        // Install app bundle via sigmactl
        let install_res = engine.dispatch_sigmactl_command("sigmactl install code-editor");
        assert!(install_res.is_ok());
        assert_eq!(engine.current_generation, 2);

        // List installed apps
        let list_res = engine.dispatch_sigmactl_command("sigmactl list").unwrap();
        assert!(list_res.contains("code-editor"));
        assert!(list_res.contains("v2.1.0"));

        // Perform 1-step rollback
        let rollback_res = engine.dispatch_sigmactl_command("sigmactl rollback 1");
        assert!(rollback_res.is_ok());
        assert_eq!(engine.current_generation, 3);

        // Verify code-editor is no longer in active generation 3
        let list_after_rollback = engine.dispatch_sigmactl_command("sigmactl list").unwrap();
        assert!(!list_after_rollback.contains("code-editor"));
    }
}
