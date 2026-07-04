// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// runtime/containers/sigma_container.rs — sigma-pod: Lightweight Container Runtime
// Language: Rust (std) — OOP via SigmaPod struct + ContainerSpec

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

// ── Container Spec (OCI-compatible) ──────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ContainerSpec {
    pub id:         String,
    pub image:      String,
    pub command:    Vec<String>,
    pub env:        BTreeMap<String, String>,
    pub mounts:     Vec<Mount>,
    pub network:    NetworkConfig,
    pub resources:  ResourceLimits,
    pub readonly:   bool,
    pub hostname:   String,
}

#[derive(Clone, Debug)]
pub struct Mount {
    pub host_path:      PathBuf,
    pub container_path: PathBuf,
    pub readonly:       bool,
}

#[derive(Clone, Debug, Default)]
pub struct NetworkConfig {
    pub mode:     NetworkMode,
    pub hostname: String,
    pub ports:    Vec<PortMapping>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum NetworkMode { #[default] Bridge, Host, None }

#[derive(Clone, Debug)]
pub struct PortMapping { pub host: u16, pub container: u16, pub proto: &'static str }

#[derive(Clone, Debug, Default)]
pub struct ResourceLimits {
    pub mem_bytes:  Option<u64>,
    pub cpu_shares: Option<u32>,
    pub pids_max:   Option<u32>,
}

// ── Container State ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerState { Created, Running, Stopped, Failed }

pub struct Container {
    pub spec:      ContainerSpec,
    pub state:     ContainerState,
    pub pid:       Option<u32>,
    pub exit_code: Option<i32>,
    pub rootfs:    PathBuf,
    pub created:   std::time::SystemTime,
}

// ── Image Layer Manager ───────────────────────────────────────────────────────

pub struct ImageManager {
    storage: PathBuf,
}

impl ImageManager {
    pub fn new(base: &str) -> Self {
        let p = PathBuf::from(base);
        let _ = fs::create_dir_all(&p);
        Self { storage: p }
    }

    /// Pull/extract an OCI image into the storage directory
    pub fn pull(&self, image: &str) -> Result<PathBuf, String> {
        let safe = image.replace(['/', ':'], "_");
        let dest = self.storage.join(&safe);
        if dest.exists() { return Ok(dest); }
        // In production: fetch layers from OCI registry via sigma-curl
        // For now: create a minimal rootfs skeleton
        for dir in &["bin", "lib", "etc", "proc", "sys", "dev", "tmp", "home"] {
            let _ = fs::create_dir_all(dest.join(dir));
        }
        let _ = fs::write(dest.join("etc/hostname"), image);
        Ok(dest)
    }

    pub fn rootfs_path(&self, image: &str) -> PathBuf {
        let safe = image.replace(['/', ':'], "_");
        self.storage.join(safe)
    }
}

// ── Namespace Setup (Linux) ───────────────────────────────────────────────────

#[cfg(target_os = "linux")]
fn setup_namespaces(spec: &ContainerSpec) -> bool {
    use std::os::unix::process::CommandExt;
    // Real impl: unshare(CLONE_NEWPID | CLONE_NEWNS | CLONE_NEWNET | CLONE_NEWUTS)
    // Placeholder: returns true (actual namespaces require kernel support)
    let _ = spec;
    true
}

// ── sigma-pod Runtime ─────────────────────────────────────────────────────────

pub struct SigmaPod {
    containers: BTreeMap<String, Container>,
    image_mgr:  ImageManager,
    state_dir:  PathBuf,
}

impl SigmaPod {
    pub fn new(state_dir: &str) -> Self {
        let p = PathBuf::from(state_dir);
        let _ = fs::create_dir_all(&p);
        Self {
            containers: BTreeMap::new(),
            image_mgr:  ImageManager::new(&format!("{}/images", state_dir)),
            state_dir:  p,
        }
    }

    /// Create container (OCI create phase)
    pub fn create(&mut self, spec: ContainerSpec) -> Result<String, String> {
        let id = spec.id.clone();
        if self.containers.contains_key(&id) {
            return Err(format!("container {} already exists", id));
        }
        let rootfs = self.image_mgr.pull(&spec.image)?;
        let c = Container {
            spec, state: ContainerState::Created,
            pid: None, exit_code: None, rootfs,
            created: std::time::SystemTime::now(),
        };
        self.containers.insert(id.clone(), c);
        // Write state to disk
        let _ = fs::write(self.state_dir.join(format!("{}.state", id)), "created");
        Ok(id)
    }

    /// Start a created container
    pub fn start(&mut self, id: &str) -> Result<u32, String> {
        let c = self.containers.get_mut(id)
            .ok_or_else(|| format!("container {} not found", id))?;
        if c.state == ContainerState::Running {
            return Err(format!("{} already running", id));
        }

        // Apply sigma_pledge restrictions (placeholder)
        // sigma_pledge(caps) — called via sigma syscall in production

        let mut cmd = Command::new(c.spec.command.first().unwrap_or(&"/bin/sh".to_owned()));
        if c.spec.command.len() > 1 { cmd.args(&c.spec.command[1..]); }
        for (k, v) in &c.spec.env { cmd.env(k, v); }
        cmd.current_dir(&c.rootfs)
           .stdout(Stdio::inherit())
           .stderr(Stdio::inherit());

        let child = cmd.spawn().map_err(|e| format!("spawn failed: {}", e))?;
        let pid = child.id();
        c.pid   = Some(pid);
        c.state = ContainerState::Running;
        let _ = fs::write(self.state_dir.join(format!("{}.state", id)), "running");
        Ok(pid)
    }

    /// Stop a running container
    pub fn stop(&mut self, id: &str, signal: i32) -> Result<(), String> {
        let c = self.containers.get_mut(id)
            .ok_or_else(|| format!("container {} not found", id))?;
        if let Some(pid) = c.pid {
            #[cfg(unix)]
            unsafe { libc::kill(pid as i32, signal); }
        }
        c.state    = ContainerState::Stopped;
        c.exit_code = Some(0);
        c.pid      = None;
        let _ = fs::write(self.state_dir.join(format!("{}.state", id)), "stopped");
        Ok(())
    }

    /// Remove a stopped container
    pub fn remove(&mut self, id: &str) -> Result<(), String> {
        let c = self.containers.get(id)
            .ok_or_else(|| format!("container {} not found", id))?;
        if c.state == ContainerState::Running {
            return Err(format!("{} is still running; stop first", id));
        }
        self.containers.remove(id);
        let _ = fs::remove_file(self.state_dir.join(format!("{}.state", id)));
        Ok(())
    }

    /// List all containers
    pub fn list(&self) -> Vec<(&str, ContainerState, Option<u32>)> {
        self.containers.iter()
            .map(|(id, c)| (id.as_str(), c.state, c.pid))
            .collect()
    }

    pub fn get(&self, id: &str) -> Option<&Container> { self.containers.get(id) }
    pub fn container_count(&self) -> usize { self.containers.len() }
}
