// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/tools/sigma_update_daemon.rs — sigma-update background daemon
// Language: Rust (std) — OOP via UpdateDaemon

use std::time::{Duration, Instant};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpdatePolicy { Manual, CheckOnly, AutoInstall, AutoReboot }

#[derive(Clone, Debug)]
pub struct UpdateInfo {
    pub version:     String,
    pub description: String,
    pub size_bytes:  u64,
    pub critical:    bool,
    pub url:         String,
    pub sha256:      [u8; 32],
}

pub struct UpdateDaemon {
    policy:       UpdatePolicy,
    check_interval: Duration,
    last_check:   Option<Instant>,
    pending:      Option<UpdateInfo>,
    state_dir:    PathBuf,
    registry_url: String,
}

impl UpdateDaemon {
    pub fn new(state_dir: &str) -> Self {
        let _ = fs::create_dir_all(state_dir);
        Self {
            policy: UpdatePolicy::CheckOnly,
            check_interval: Duration::from_secs(3600 * 6), // 6 hours
            last_check: None,
            pending: None,
            state_dir: PathBuf::from(state_dir),
            registry_url: "https://updates.sigmaos.app/v1".to_owned(),
        }
    }

    pub fn set_policy(&mut self, p: UpdatePolicy) { self.policy = p; }

    pub fn should_check(&self) -> bool {
        match self.last_check {
            None => true,
            Some(t) => t.elapsed() >= self.check_interval,
        }
    }

    /// Simulated update check (real: fetch version.json from registry)
    pub fn check(&mut self) -> Option<&UpdateInfo> {
        self.last_check = Some(Instant::now());
        // Write last-check timestamp
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default().as_secs();
        let _ = fs::write(self.state_dir.join("last_check"), ts.to_string());
        // In production: HTTP GET {registry_url}/latest → parse JSON
        // For now: no pending update (placeholder)
        self.pending = None;
        self.pending.as_ref()
    }

    pub fn apply(&mut self) -> bool {
        if let Some(ref info) = self.pending.clone() {
            eprintln!("[sigma-update] applying update to {}", info.version);
            // Delegate to sigma_update.rs A/B updater
            let _ = std::process::Command::new("sigma-update")
                .args(["apply", &info.url]).status();
            self.pending = None;
            return true;
        }
        false
    }

    pub fn mark_boot_ok(&self) {
        let _ = fs::write(self.state_dir.join("boot_ok"), "1");
    }

    pub fn run_daemon(&mut self) {
        eprintln!("[sigma-update] daemon started (policy: {:?})", self.policy);
        self.mark_boot_ok();
        loop {
            if self.should_check() {
                self.check();
                if self.policy == UpdatePolicy::AutoInstall {
                    if self.pending.is_some() { self.apply(); }
                }
            }
            std::thread::sleep(Duration::from_secs(60));
        }
    }
}
