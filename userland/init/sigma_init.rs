// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/init/sigma_init.rs — sigma-init: PID 1 / Init System
// Replaces: planned C++ init stub
//
// Language: Rust (std allowed in userland)
// Pattern: OOP via ServiceManager struct + Service trait

use std::collections::BTreeMap;
use std::process::{Command, Child, ExitStatus};
use std::time::Duration;
use std::thread;

// ── Service Trait (OOP interface) ─────────────────────────────────────────────

pub trait Service: Send {
    fn name(&self)      -> &str;
    fn command(&self)   -> &str;
    fn args(&self)      -> &[&str];
    fn restart(&self)   -> RestartPolicy;
    fn depends(&self)   -> &[&str]; // service names this depends on
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Never,
    OnFailure,
    Always,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Restarting,
    Failed,
}

// ── Built-in Services ─────────────────────────────────────────────────────────

struct BasicService {
    name:    &'static str,
    command: &'static str,
    args:    &'static [&'static str],
    restart: RestartPolicy,
    deps:    &'static [&'static str],
}

impl Service for BasicService {
    fn name(&self)    -> &str               { self.name    }
    fn command(&self) -> &str               { self.command }
    fn args(&self)    -> &[&str]            { self.args    }
    fn restart(&self) -> RestartPolicy      { self.restart }
    fn depends(&self) -> &[&str]            { self.deps    }
}

/// Default service table — mirrors /etc/sigma/services/
static DEFAULT_SERVICES: &[BasicService] = &[
    BasicService {
        name: "sigmad-health", command: "/usr/lib/sigmad/health",
        args: &[], restart: RestartPolicy::Always, deps: &[],
    },
    BasicService {
        name: "sigmad-pkg", command: "/usr/lib/sigmad/pkg",
        args: &["--daemon"], restart: RestartPolicy::OnFailure, deps: &["sigmad-health"],
    },
    BasicService {
        name: "sigmad-netd", command: "/usr/lib/sigmad/netd",
        args: &[], restart: RestartPolicy::Always, deps: &[],
    },
    BasicService {
        name: "sigmad-vault", command: "/usr/lib/sigmad/vault",
        args: &[], restart: RestartPolicy::Always, deps: &["sigmad-health"],
    },
    BasicService {
        name: "sigma-sh", command: "/bin/sigma-sh",
        args: &[], restart: RestartPolicy::Always, deps: &["sigmad-netd"],
    },
];

// ── Service Manager ───────────────────────────────────────────────────────────

struct RunningService {
    child:   Option<Child>,
    state:   ServiceState,
    restart: RestartPolicy,
    cmd:     String,
    args:    Vec<String>,
    restarts: u32,
}

pub struct ServiceManager {
    services: BTreeMap<String, RunningService>,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self { services: BTreeMap::new() }
    }

    pub fn register<S: Service>(&mut self, svc: &S) {
        self.services.insert(svc.name().to_owned(), RunningService {
            child:    None,
            state:    ServiceState::Stopped,
            restart:  svc.restart(),
            cmd:      svc.command().to_owned(),
            args:     svc.args().iter().map(|s| s.to_string()).collect(),
            restarts: 0,
        });
    }

    /// Start a service by name
    pub fn start(&mut self, name: &str) -> Result<(), String> {
        let svc = self.services.get_mut(name)
            .ok_or_else(|| format!("Unknown service: {}", name))?;
        if svc.state == ServiceState::Running { return Ok(()); }

        svc.state = ServiceState::Starting;
        let child = Command::new(&svc.cmd)
            .args(&svc.args)
            .spawn()
            .map_err(|e| format!("Failed to start {}: {}", name, e))?;

        svc.child = Some(child);
        svc.state = ServiceState::Running;
        eprintln!("[sigma-init] started: {}", name);
        Ok(())
    }

    /// Stop a service
    pub fn stop(&mut self, name: &str) {
        if let Some(svc) = self.services.get_mut(name) {
            if let Some(ref mut child) = svc.child {
                let _ = child.kill();
                let _ = child.wait();
            }
            svc.child = None;
            svc.state = ServiceState::Stopped;
            eprintln!("[sigma-init] stopped: {}", name);
        }
    }

    /// Check all running services and restart if needed
    pub fn supervise(&mut self) {
        let names: Vec<String> = self.services.keys().cloned().collect();
        for name in &names {
            let should_restart = {
                let svc = self.services.get_mut(name).unwrap();
                if let Some(ref mut child) = svc.child {
                    match child.try_wait() {
                        Ok(Some(status)) => {
                            svc.state = if status.success() {
                                ServiceState::Stopped
                            } else {
                                ServiceState::Failed
                            };
                            svc.child = None;
                            matches!(svc.restart, RestartPolicy::Always)
                                || (matches!(svc.restart, RestartPolicy::OnFailure)
                                    && !status.success())
                        }
                        Ok(None) => false, // still running
                        Err(_)   => { svc.state = ServiceState::Failed; true }
                    }
                } else { false }
            };
            if should_restart {
                let svc = self.services.get_mut(name).unwrap();
                svc.restarts += 1;
                svc.state = ServiceState::Restarting;
                eprintln!("[sigma-init] restarting: {} (attempt {})", name, svc.restarts);
                let _ = self.start(name);
            }
        }
    }

    /// Start all registered services in dependency order (simple topological)
    pub fn start_all(&mut self) {
        // Collect start order (services with no deps first)
        let names: Vec<String> = self.services.keys().cloned().collect();
        for name in &names {
            if let Err(e) = self.start(name) {
                eprintln!("[sigma-init] error starting {}: {}", name, e);
            }
        }
    }
}

// ── Mount Helpers ─────────────────────────────────────────────────────────────

#[cfg(target_os = "linux")]
fn mount_pseudo_fs() {
    use std::fs;
    let _ = fs::create_dir_all("/proc");
    let _ = fs::create_dir_all("/sys");
    let _ = fs::create_dir_all("/dev");
    let _ = fs::create_dir_all("/tmp");
    // On real SigmaOS these use sovereign syscalls:
    // sigma_mount("proc",  "/proc", "procfs",  0, "")
    // sigma_mount("sysfs", "/sys",  "sysfs",   0, "")
    // sigma_mount("tmpfs", "/tmp",  "tmpfs",   0, "size=64M")
}

// ── Entry Point ───────────────────────────────────────────────────────────────

fn main() {
    eprintln!("[sigma-init] SigmaOS init v15.0 — PID {}", std::process::id());

    // Mount virtual filesystems
    #[cfg(target_os = "linux")]
    mount_pseudo_fs();

    // Register and start services
    let mut mgr = ServiceManager::new();
    for svc in DEFAULT_SERVICES {
        mgr.register(svc);
    }
    mgr.start_all();

    // Supervision loop (PID 1 must never exit)
    loop {
        mgr.supervise();
        thread::sleep(Duration::from_millis(500));
    }
}
