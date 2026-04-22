/// core/orchestrator.rs — Sovereign ShardManager
/// Zero heavy-framework silicon primitives. No HashMap, No PathBuf.

use crate::config::{Config, ProfileConfig};
use std::process::Command;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum ShardState { Active, Inactive, Error(String) }

#[derive(Debug, Clone)]
pub struct ShardInfo {
    pub name:  String,
    pub state: ShardState,
    pub lang:  String,   // "rust", "c", "wasm"
    pub path:  String,
}

pub struct ShardManager {
    shards:    Vec<ShardInfo>,
    root:      String,
    profile:   String,
}

impl ShardManager {
    pub fn with_root(root: &str) -> Self {
        let mut mgr = Self {
            shards:  Vec::new(),
            root:    root.to_string(),
            profile: "default".into(),
        };
        mgr.discover_shards();
        mgr
    }

    /// Character-level path discovery without PathBuf
    fn discover_shards(&mut self) {
        let shard_dir = format!("{}/shards", self.root);
        if let Ok(entries) = fs::read_dir(&shard_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().unwrap().to_string_lossy().to_string();
                    let lang = if fs::metadata(format!("{}/shards/{}/Cargo.toml", self.root, name)).is_ok() { "rust" }
                               else if fs::metadata(format!("{}/shards/{}/Makefile", self.root, name)).is_ok() { "c" }
                               else { "unknown" };
                    self.shards.push(ShardInfo {
                        name, state: ShardState::Active,
                        lang: lang.into(), path: path.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }

    pub fn build_all(&self) -> Result<(), String> {
        eprintln!("Σ [BUILD] Building all active shards...");
        let _ = Command::new("cargo").args(["build", "--workspace", "--release"])
            .current_dir(&self.root).status();
        let _ = Command::new("make").args(["bin"])
            .current_dir(&self.root).status();
        Ok(())
    }

    pub fn build_shard(&self, name: &str) -> Result<(), String> {
        let info = self.shards.iter().find(|s| s.name == name)
            .ok_or_else(|| format!("Shard '{}' not found", name))?;
        let s = if info.lang == "rust" {
            Command::new("cargo").args(["build", "--release"]).current_dir(&info.path).status()
        } else {
            Command::new("make").current_dir(&info.path).status()
        }.map_err(|e| e.to_string())?;
        if s.success() { Ok(()) } else { Err(format!("Build failed for {}", name)) }
    }

    pub fn sync_github(&self) -> Result<String, String> {
        let push = Command::new("git").args(["push"]).current_dir(&self.root).output().map_err(|e| e.to_string())?;
        if push.status.success() { Ok("GitHub sync: OK".into()) } 
        else { Err(String::from_utf8_lossy(&push.stderr).to_string()) }
    }

    pub fn add_shard(&mut self, name: &str) -> Result<(), String> {
        let path = format!("{}/shards/{}", self.root, name);
        fs::create_dir_all(format!("{}/src", path)).map_err(|e| e.to_string())?;
        
        // Manual snake_case (no .replace)
        let mut name_snake = String::new();
        for c in name.chars() {
            if c == '-' { name_snake.push('_'); } else { name_snake.push(c); }
        }

        fs::write(format!("{}/Cargo.toml", path), format!("[package]\nname=\"sigma-{name}\"\nversion=\"1.0.0\"\nedition=\"2021\"\n\n[lib]\nname=\"sigma_{name_snake}\"\ncrate-type=[\"rlib\"]\n")).map_err(|e| e.to_string())?;
        fs::write(format!("{}/src/lib.rs", path), format!("pub fn init() {{ eprintln!(\"Σ [SHARD] {name} initialized.\"); }}\n")).map_err(|e| e.to_string())?;

        self.shards.push(ShardInfo { name: name.into(), state: ShardState::Active, lang: "rust".into(), path });
        Ok(())
    }

    pub fn remove_shard(&mut self, name: &str) -> Result<(), String> {
        if let Some(pos) = self.shards.iter().position(|s| s.name == name) {
            self.shards.remove(pos);
        }
        Ok(())
    }

    pub fn apply_profile(&mut self, name: &str) -> Result<(), String> {
        let path = format!("{}/profiles/{}.json", self.root, name);
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let _config = ProfileConfig::load(&content);
        self.profile = name.to_string();
        Ok(())
    }

    pub fn create_profile(&self, name: &str, theme: &str) -> Result<(), String> {
        let path = format!("{}/profiles/{}.json", self.root, name);
        let content = format!("{{\"name\":\"{}\",\"theme\":\"{}\",\"auto_sync\":true,\"shards\":[\"sync\"]}}", name, theme);
        fs::write(path, content).map_err(|e| e.to_string())
    }

    pub fn install_plugin(&self, name: &str) -> Result<(), String> {
        let dir = format!("{}/plugins/{}", self.root, name);
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        fs::write(format!("{}/plugin.json", dir), format!("{{\"name\":\"{}\",\"enabled\":true}}", name)).map_err(|e| e.to_string())
    }

    pub fn run_wizard(&self) -> Result<(), String> {
        let dirs = ["profiles", "shards", "plugins", "kernel/suites"];
        for d in dirs {
            let _ = fs::create_dir_all(format!("{}/{}", self.root, d));
        }
        self.create_profile("default", "dark")?;
        Ok(())
    }

    pub fn status(&self) -> String {
        format!("Σ SIGMAOS STATUS\nProfile: {}\nShards:  {}\n", self.profile, self.shards.len())
    }

    pub fn list_shards(&self) -> &Vec<ShardInfo> { &self.shards }
}
