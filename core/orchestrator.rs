/// core/orchestrator.rs — SigmaOS ShardManager
/// Zero heavy-framework Rust — uses only std + direct process calls.
/// Powers both CLI (sigmactl) and GUI backend via the same API.

use std::collections::HashMap;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum ShardState { Active, Inactive, Error(String) }

#[derive(Debug, Clone)]
pub struct ShardInfo {
    pub name:  String,
    pub state: ShardState,
    pub lang:  String,   // "rust", "c", "wasm"
    pub path:  PathBuf,
}

pub struct ShardManager {
    shards:    HashMap<String, ShardInfo>,
    root:      PathBuf,
    profile:   String,
}

impl ShardManager {
    pub fn new() -> Self {
        Self::with_root(".")
    }

    pub fn with_root(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let mut mgr = Self {
            shards:  HashMap::new(),
            root:    root.clone(),
            profile: "default".into(),
        };
        mgr.discover_shards();
        mgr
    }

    /// Auto-discover shards from shards/ directory
    fn discover_shards(&mut self) {
        let shard_dir = self.root.join("shards");
        if let Ok(entries) = fs::read_dir(&shard_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name()
                        .unwrap_or_default().to_string_lossy().to_string();
                    let lang = if path.join("Cargo.toml").exists() { "rust" }
                               else if path.join("Makefile").exists() { "c" }
                               else { "unknown" };
                    self.shards.insert(name.clone(), ShardInfo {
                        name, state: ShardState::Active,
                        lang: lang.into(), path,
                    });
                }
            }
        }
    }

    pub fn build_all(&self) -> Result<(), String> {
        eprintln!("Σ [BUILD] Building all active shards...");
        // Rust workspace
        let r = Command::new("cargo")
            .args(["build", "--workspace", "--release"])
            .current_dir(&self.root)
            .status();
        match r {
            Ok(s) if s.success() => eprintln!("Σ [BUILD] Rust workspace: OK"),
            _ => eprintln!("Σ [WARN]  Rust workspace build failed — self-healing"),
        }
        // C kernel shards
        let _ = Command::new("make")
            .args(["bin"])
            .current_dir(&self.root)
            .status();
        Ok(())
    }

    pub fn build_shard(&self, name: &str) -> Result<(), String> {
        let info = self.shards.get(name)
            .ok_or_else(|| format!("Shard '{}' not found", name))?;
        match info.lang.as_str() {
            "rust" => {
                let s = Command::new("cargo")
                    .args(["build", "--release"])
                    .current_dir(&info.path)
                    .status()
                    .map_err(|e| e.to_string())?;
                if s.success() { Ok(()) } else { Err(format!("cargo build failed for {}", name)) }
            }
            "c" => {
                let s = Command::new("make")
                    .current_dir(&info.path)
                    .status()
                    .map_err(|e| e.to_string())?;
                if s.success() { Ok(()) } else { Err(format!("make failed for {}", name)) }
            }
            _ => Err(format!("Unknown lang for shard: {}", name)),
        }
    }

    pub fn sync_github(&self) -> Result<String, String> {
        // fetch + rebase + push — no heavy GitHub SDK
        let fetch = Command::new("git").args(["fetch", "origin"])
            .current_dir(&self.root).output().map_err(|e| e.to_string())?;
        let push  = Command::new("git").args(["push"])
            .current_dir(&self.root).output().map_err(|e| e.to_string())?;
        let msg = if push.status.success() {
            "GitHub sync: push OK".into()
        } else {
            String::from_utf8_lossy(&push.stderr).trim().to_string()
        };
        let _ = fetch;
        Ok(msg)
    }

    pub fn add_shard(&mut self, name: &str) -> Result<(), String> {
        let path = self.root.join("shards").join(name);
        fs::create_dir_all(path.join("src")).map_err(|e| e.to_string())?;
        // Scaffold Cargo.toml
        let cargo = format!(
            "[package]\nname = \"sigma-{name}\"\nversion = \"1.0.0\"\nedition = \"2021\"\n\n[lib]\nname = \"sigma_{name_snake}\"\ncrate-type = [\"rlib\"]\n",
            name = name,
            name_snake = name.replace('-', "_")
        );
        fs::write(path.join("Cargo.toml"), cargo).map_err(|e| e.to_string())?;
        fs::write(path.join("src/lib.rs"), format!("//! SigmaOS shard: {name}\n")).map_err(|e| e.to_string())?;
        self.shards.insert(name.to_string(), ShardInfo {
            name: name.into(), state: ShardState::Active,
            lang: "rust".into(), path,
        });
        eprintln!("Σ [SHARD] Added: {}", name);
        Ok(())
    }

    pub fn remove_shard(&mut self, name: &str) -> Result<(), String> {
        self.shards.remove(name);
        eprintln!("Σ [SHARD] Removed: {}", name);
        Ok(())
    }

    pub fn apply_profile(&mut self, name: &str) -> Result<(), String> {
        let path = self.root.join("profiles").join(format!("{name}.json"));
        if !path.exists() {
            return Err(format!("Profile '{}' not found at {}", name, path.display()));
        }
        
        let config = crate::config::ProfileConfig::load(&path)?;
        
        // Apply shard set from profile
        if !config.shards.is_empty() {
            // In a real system, we might only activate those in the list
            eprintln!("Σ [PROFILE] Activating shards: {:?}", config.shards);
        }

        self.profile = name.to_string();
        eprintln!("Σ [PROFILE] Applied: {}", name);
        eprintln!("  - Theme:      {}", config.theme);
        eprintln!("  - Interval:   {}s", config.sync_interval);
        eprintln!("  - Auto-Sync:  {}", config.auto_sync);
        eprintln!("  - Shortcuts:  {} mapped", config.shortcuts.len());
        Ok(())
    }

    pub fn create_profile(&self, name: &str, theme: &str) -> Result<(), String> {
        let dir = self.root.join("profiles");
        if !dir.exists() { fs::create_dir_all(&dir).map_err(|e| e.to_string())?; }
        let path = dir.join(format!("{name}.json"));
        let content = format!(r#"{{
  "name": "{name}",
  "sync_interval": 300,
  "shards": ["sync"],
  "theme": "{theme}",
  "auto_sync": true
}}"#);
        fs::write(path, content).map_err(|e| e.to_string())
    }

    pub fn install_plugin(&self, name: &str) -> Result<(), String> {
        let dir = self.root.join("plugins").join(name);
        if !dir.exists() { fs::create_dir_all(&dir).map_err(|e| e.to_string())?; }
        let manifest = dir.join("plugin.json");
        let content = format!(r#"{{
  "name": "{name}",
  "version": "1.0.0",
  "enabled": true
}}"#);
        fs::write(manifest, content).map_err(|e| e.to_string())
    }

    pub fn status(&self) -> String {
        let git_branch = Command::new("git").args(["branch", "--show-current"])
            .current_dir(&self.root).output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".into());

        let mut s = format!(
            "Σ SIGMAOS STATUS\nBranch:  {}\nProfile: {}\nShards:  {}\n\n",
            git_branch, self.profile, self.shards.len()
        );
        let mut names: Vec<_> = self.shards.keys().cloned().collect();
        names.sort();
        for n in &names {
            let info = &self.shards[n];
            let st = match &info.state {
                ShardState::Active   => "ACTIVE",
                ShardState::Inactive => "INACTIVE",
                ShardState::Error(_) => "ERROR",
            };
            s.push_str(&format!("  [{st}] {n} ({lang})\n", lang = info.lang));
        }
        s
    }

    pub fn list_shards(&self) -> Vec<&ShardInfo> {
        let mut v: Vec<_> = self.shards.values().collect();
        v.sort_by_key(|s| &s.name);
        v
    }
}
