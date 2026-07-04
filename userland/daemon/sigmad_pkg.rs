// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/daemon/sigmad_pkg.rs — sigmad-pkg: Background Package Manager Daemon
// Language: Rust (std) — OOP via PkgDaemon

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::fs;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PkgDaemonState { Idle, Checking, Downloading, Installing, Error(String) }

#[derive(Clone, Debug)]
pub struct PkgJob {
    pub id:        u64,
    pub pkg_name:  String,
    pub version:   String,
    pub action:    PkgAction,
    pub state:     PkgJobState,
    pub progress:  u8,    // 0-100
    pub error:     Option<String>,
    pub created:   Instant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PkgAction { Install, Remove, Update, Verify }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PkgJobState { Queued, Running, Done, Failed }

pub struct PkgDaemon {
    state:       PkgDaemonState,
    jobs:        BTreeMap<u64, PkgJob>,
    next_job_id: u64,
    cache_dir:   PathBuf,
    db_path:     PathBuf,
    registry_url: String,
}

impl PkgDaemon {
    pub fn new(cache_dir: &str, db_path: &str) -> Self {
        let _ = fs::create_dir_all(cache_dir);
        Self {
            state:       PkgDaemonState::Idle,
            jobs:        BTreeMap::new(),
            next_job_id: 1,
            cache_dir:   PathBuf::from(cache_dir),
            db_path:     PathBuf::from(db_path),
            registry_url: "https://pkg.sigmaos.app/v1".to_owned(),
        }
    }

    pub fn enqueue(&mut self, pkg: &str, version: &str, action: PkgAction) -> u64 {
        let id = self.next_job_id; self.next_job_id += 1;
        self.jobs.insert(id, PkgJob {
            id, pkg_name: pkg.to_owned(), version: version.to_owned(),
            action, state: PkgJobState::Queued, progress: 0,
            error: None, created: Instant::now(),
        });
        eprintln!("[sigmad-pkg] enqueued job {} {:?} {}", id, action, pkg);
        id
    }

    pub fn process_next(&mut self) -> bool {
        let next_id = self.jobs.values()
            .find(|j| j.state == PkgJobState::Queued)
            .map(|j| j.id);
        let id = match next_id { Some(i) => i, None => return false };
        if let Some(job) = self.jobs.get_mut(&id) {
            job.state = PkgJobState::Running;
            let pkg = job.pkg_name.clone();
            let action = job.action;
            self.state = match action {
                PkgAction::Install => PkgDaemonState::Downloading,
                PkgAction::Remove  => PkgDaemonState::Installing,
                PkgAction::Update  => PkgDaemonState::Checking,
                PkgAction::Verify  => PkgDaemonState::Checking,
            };
            eprintln!("[sigmad-pkg] processing {:?} {}", action, pkg);
        }
        // Simulate work (real: delegate to sigma-pkg binary)
        std::thread::sleep(Duration::from_millis(100));
        if let Some(job) = self.jobs.get_mut(&id) {
            job.progress = 100;
            job.state    = PkgJobState::Done;
        }
        self.state = PkgDaemonState::Idle;
        true
    }

    pub fn job_status(&self, id: u64) -> Option<&PkgJob> { self.jobs.get(&id) }

    pub fn pending_count(&self) -> usize {
        self.jobs.values().filter(|j| j.state == PkgJobState::Queued).count()
    }

    pub fn daemon_state(&self) -> &PkgDaemonState { &self.state }

    pub fn run(&mut self) {
        eprintln!("[sigmad-pkg] package manager daemon started");
        loop {
            while self.process_next() {}
            std::thread::sleep(Duration::from_millis(500));
        }
    }
}
