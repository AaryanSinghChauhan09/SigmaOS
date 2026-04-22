/// core/orchestrator.rs — Sovereign ShardManager
/// Zero high-level abstractions. Raw FFI to host silicon.

use crate::config::{Config, ProfileConfig};
use std::fs;

// ── Raw Windows FFI (Zero Dependency) ──────────────────────────────────────
#[cfg(windows)]
extern "C" {
    fn CreateProcessA(
        lpApplicationName: *const u8,
        lpCommandLine: *mut u8,
        lpProcessAttributes: *mut u8,
        lpThreadAttributes: *mut u8,
        bInheritHandles: i32,
        dwCreationFlags: u32,
        lpEnvironment: *mut u8,
        lpCurrentDirectory: *const u8,
        lpStartupInfo: *mut u8,
        lpProcessInformation: *mut u8,
    ) -> i32;
    fn WaitForSingleObject(hHandle: *mut u8, dwMilliseconds: u32) -> u32;
    fn CloseHandle(hHandle: *mut u8) -> i32;
}

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
    pub shards:    Vec<ShardInfo>,
    pub root:      String,
    pub profile:   String,
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
        self.spawn_raw("cargo build --workspace --release");
        self.spawn_raw("make bin");
        Ok(())
    }

    pub fn build_shard(&self, name: &str) -> Result<(), String> {
        let info = self.shards.iter().find(|s| s.name == name)
            .ok_or_else(|| format!("Shard '{}' not found", name))?;
        let cmd = if info.lang == "rust" { "cargo build --release" } else { "make" };
        self.spawn_raw_in_dir(cmd, &info.path);
        Ok(())
    }

    pub fn sync_github(&self) -> Result<String, String> {
        self.spawn_raw("git push");
        Ok("GitHub sync initiated".into())
    }

    pub fn add_shard(&mut self, name: &str) -> Result<(), String> {
        let path = format!("{}/shards/{}", self.root, name);
        let _ = fs::create_dir_all(format!("{}/src", path));
        
        let mut name_snake = String::new();
        for c in name.chars() {
            if c == '-' { name_snake.push('_'); } else { name_snake.push(c); }
        }

        let _ = fs::write(format!("{}/Cargo.toml", path), format!("[package]\nname=\"sigma-{name}\"\nversion=\"1.0.0\"\nedition=\"2021\"\n\n[lib]\nname=\"sigma_{name_snake}\"\ncrate-type=[\"rlib\"]\n"));
        let _ = fs::write(format!("{}/src/lib.rs", path), format!("pub fn init() {{ }}\n"));

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
        if let Ok(content) = fs::read_to_string(path) {
            let _config = ProfileConfig::load(&content);
            self.profile = name.to_string();
        }
        Ok(())
    }

    pub fn create_profile(&self, name: &str, theme: &str) -> Result<(), String> {
        let path = format!("{}/profiles/{}.json", self.root, name);
        let content = format!("{{\"name\":\"{}\",\"theme\":\"{}\",\"auto_sync\":true}}", name, theme);
        let _ = fs::write(path, content);
        Ok(())
    }

    pub fn install_plugin(&self, name: &str) -> Result<(), String> {
        let dir = format!("{}/plugins/{}", self.root, name);
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(format!("{}/plugin.json", dir), format!("{{\"name\":\"{}\",\"enabled\":true}}", name));
        Ok(())
    }

    pub fn status(&self) -> String {
        format!("Σ SIGMAOS STATUS\nProfile: {}\nShards:  {}\n", self.profile, self.shards.len())
    }

    pub fn list_shards(&self) -> &Vec<ShardInfo> { &self.shards }

    // ── Low-Level Silicon Spawning ──────────────────────────────────────────
    fn spawn_raw(&self, cmd: &str) { self.spawn_raw_in_dir(cmd, &self.root); }

    fn spawn_raw_in_dir(&self, cmd: &str, dir: &str) {
        #[cfg(windows)]
        unsafe {
            let mut si = [0u8; 128]; // Large enough for STARTUPINFOA
            let mut pi = [0u8; 32];  // Large enough for PROCESS_INFORMATION
            let mut cmd_buf = cmd.to_string().into_bytes();
            cmd_buf.push(0);
            let mut dir_buf = dir.to_string().into_bytes();
            dir_buf.push(0);
            
            CreateProcessA(
                core::ptr::null(),
                cmd_buf.as_mut_ptr(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                1, 0,
                core::ptr::null_mut(),
                dir_buf.as_ptr(),
                si.as_mut_ptr(),
                pi.as_mut_ptr(),
            );
        }
    }
}
