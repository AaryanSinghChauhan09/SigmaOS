// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// virtualization/ocirunner/sigma_oci.rs — OCI Container Runtime
// Language: Rust (std)
// Pattern: OOP via OciRuntime struct implementing ContainerRuntime trait

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::fs;

// ── OCI Spec Types ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct OciMount {
    pub destination: String,
    pub source:      String,
    pub fs_type:     String,
    pub options:     Vec<String>,
}

#[derive(Clone, Debug)]
pub struct OciProcess {
    pub args: Vec<String>,
    pub env:  Vec<String>,
    pub cwd:  String,
}

#[derive(Clone, Debug)]
pub struct OciSpec {
    pub root_path:  PathBuf,
    pub process:    OciProcess,
    pub hostname:   String,
    pub mounts:     Vec<OciMount>,
    pub resources:  OciResources,
}

#[derive(Clone, Debug, Default)]
pub struct OciResources {
    pub memory_limit_bytes: Option<u64>,
    pub cpu_shares:         Option<u32>,
}

// ── Container State ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ContainerState { Created, Running, Stopped, Deleted }

pub struct Container {
    pub id:       String,
    pub state:    ContainerState,
    pub spec:     OciSpec,
    pub pid:      Option<u32>,
    pub exit_code: Option<i32>,
}

// ── ContainerRuntime Trait ────────────────────────────────────────────────────

pub trait ContainerRuntime: Send + Sync {
    fn create(&mut self, id: &str, spec: OciSpec) -> Result<(), RuntimeError>;
    fn start (&mut self, id: &str)                -> Result<(), RuntimeError>;
    fn stop  (&mut self, id: &str, signal: i32)   -> Result<(), RuntimeError>;
    fn delete(&mut self, id: &str)                -> Result<(), RuntimeError>;
    fn state (&self,     id: &str)                -> Option<ContainerState>;
    fn list  (&self)                              -> Vec<String>;
}

#[derive(Debug)]
pub enum RuntimeError {
    NotFound(String),
    AlreadyExists(String),
    NotRunning(String),
    SpawnFailed(String),
    IoError(String),
}

// ── SigmaOS OCI Runtime ───────────────────────────────────────────────────────

pub struct OciRuntime {
    containers: BTreeMap<String, Container>,
    state_dir:  PathBuf,
}

impl OciRuntime {
    pub fn new(state_dir: &str) -> Self {
        let _ = fs::create_dir_all(state_dir);
        Self {
            containers: BTreeMap::new(),
            state_dir:  PathBuf::from(state_dir),
        }
    }

    fn get_mut(&mut self, id: &str) -> Result<&mut Container, RuntimeError> {
        self.containers.get_mut(id)
            .ok_or_else(|| RuntimeError::NotFound(id.to_owned()))
    }
}

impl ContainerRuntime for OciRuntime {
    fn create(&mut self, id: &str, spec: OciSpec) -> Result<(), RuntimeError> {
        if self.containers.contains_key(id) {
            return Err(RuntimeError::AlreadyExists(id.to_owned()));
        }
        // Validate root path exists
        if !spec.root_path.exists() {
            return Err(RuntimeError::IoError(
                format!("root path {:?} not found", spec.root_path)));
        }
        // Write container state to disk
        let state_file = self.state_dir.join(format!("{}.json", id));
        let _ = fs::write(&state_file, format!(
            r#"{{"id":"{}","state":"Created","hostname":"{}"}}"#,
            id, spec.hostname));

        self.containers.insert(id.to_owned(), Container {
            id:        id.to_owned(),
            state:     ContainerState::Created,
            spec,
            pid:       None,
            exit_code: None,
        });
        Ok(())
    }

    fn start(&mut self, id: &str) -> Result<(), RuntimeError> {
        let container = self.get_mut(id)?;
        if container.state == ContainerState::Running {
            return Ok(());
        }
        // Build sigma_pledge-restricted process
        // On real SigmaOS: use sigma_pledge + sigma_unveil syscalls
        // Here: launch via Command with restricted environment
        let mut cmd = Command::new(&container.spec.process.args[0]);
        cmd.args(&container.spec.process.args[1..])
           .current_dir(&container.spec.process.cwd)
           .stdout(Stdio::inherit())
           .stderr(Stdio::inherit());

        // Set environment
        for env_str in &container.spec.process.env {
            if let Some((k, v)) = env_str.split_once('=') {
                cmd.env(k, v);
            }
        }

        let child = cmd.spawn()
            .map_err(|e| RuntimeError::SpawnFailed(e.to_string()))?;

        container.pid   = Some(child.id());
        container.state = ContainerState::Running;
        Ok(())
    }

    fn stop(&mut self, id: &str, _signal: i32) -> Result<(), RuntimeError> {
        let container = self.get_mut(id)?;
        if container.state != ContainerState::Running {
            return Err(RuntimeError::NotRunning(id.to_owned()));
        }
        #[cfg(unix)]
        if let Some(pid) = container.pid {
            unsafe { libc::kill(pid as i32, _signal); }
        }
        container.state    = ContainerState::Stopped;
        container.exit_code = Some(0);
        container.pid      = None;
        Ok(())
    }

    fn delete(&mut self, id: &str) -> Result<(), RuntimeError> {
        {
            let container = self.get_mut(id)?;
            if container.state == ContainerState::Running {
                return Err(RuntimeError::NotRunning(
                    format!("{} is still running; stop first", id)));
            }
        }
        let state_file = self.state_dir.join(format!("{}.json", id));
        let _ = fs::remove_file(state_file);
        self.containers.remove(id);
        Ok(())
    }

    fn state(&self, id: &str) -> Option<ContainerState> {
        self.containers.get(id).map(|c| c.state)
    }

    fn list(&self) -> Vec<String> {
        self.containers.keys().cloned().collect()
    }
}
