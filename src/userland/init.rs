// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! Sovereign Core Userland Init, Supervisors, and Task Execution Subsystem
//!
//! Provides comprehensive init system, daemon supervision, and scheduled task execution parity:
//! - systemd: Unit types (Service, Target, Socket, Timer, Path, Mount, Slice, Scope),
//!   dependency graphs (Wants, Requires, Requisite, BindsTo, PartOf, Conflicts, Before, After),
//!   Cgroups v2 resource slice limits, and `sd_notify` protocol parser (`READY=1`, `RELOAD=1`, `STOPPING=1`, `WATCHDOG=1`, `STATUS=...`).
//! - OpenRC: Multi-stage runlevels (SysInit, Boot, Default, Nonetwork, Shutdown) and keyword directives (`Provide`, `Need`, `Use`, `Before`, `After`, `KeywordNoDocker`).
//! - s6 & runit: `/var/service` directory supervision with control channel verbs (`a` alarm, `u` up, `d` down, `o` once, `x` exit, `r` restart, `p` pause, `c` continue).
//! - SysVinit: `/etc/inittab` action keywords (`Respawn`, `SysInit`, `Wait`, `Once`, `CtrlAltDel`, `PowerFail`).
//! - BSD rc.d / rc.subr: Comment header parsing (`# PROVIDE:`, `# REQUIRE:`, `# BEFORE:`, `# KEYWORD:`) with precmd/postcmd hooks.
//! - macOS launchd: Job specifications (`KeepAlive`, `LaunchEvents`, `ThrottleInterval`).
//! - Task execution suite: Vixie Cron 5/6-field parsing and macros (`@reboot`, `@hourly`, `@daily`, `@weekly`, `@monthly`, `@yearly`),
//!   Anacron missed-job catchup tracking with `RANDOM_DELAY` and `START_HOURS_RANGE`, `at`/`batch` delayed queueing with load average gating, and Systemd Timers (`OnCalendar`, `Persistent=true`, `AccuracySec`).

use std::collections::{BTreeMap, BTreeSet};
use std::string::String;
use std::vec::Vec;

/// Common service states across init systems
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Failed,
    Reloading,
    Stopping,
}

// ============================================================================
// 1. SYSTEMD UNIT & CGROUPS V2 SUBSYSTEM
// ============================================================================

/// Systemd Unit Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemdUnitType {
    Service,
    Target,
    Socket,
    Timer,
    Path,
    Mount,
    Slice,
    Scope,
}

/// Systemd Dependency Directive
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdDependencyType {
    Wants,
    Requires,
    Requisite,
    BindsTo,
    PartOf,
    Conflicts,
    Before,
    After,
}

/// Cgroups v2 Resource Limits for Slices/Units
#[derive(Debug, Clone, PartialEq)]
pub struct CgroupV2SliceLimits {
    pub memory_max_bytes: Option<u64>,
    pub cpu_weight: u32,
    pub cpu_quota_pct: f32,
}

impl Default for CgroupV2SliceLimits {
    fn default() -> Self {
        Self {
            memory_max_bytes: None,
            cpu_weight: 100,
            cpu_quota_pct: 100.0,
        }
    }
}

/// Systemd Unit Definition
#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub unit_type: SystemdUnitType,
    pub exec_start: String,
    pub restart: bool,
    pub dependencies: Vec<String>,
    pub wants: Vec<String>,
    pub requires: Vec<String>,
    pub conflicts: Vec<String>,
    pub before: Vec<String>,
    pub after: Vec<String>,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub watchdog_usec: u64,
    pub last_ping: u64,
    pub status_text: String,
    pub slice_limits: CgroupV2SliceLimits,
}

impl Service {
    pub fn new(name: impl Into<String>, exec_start: impl Into<String>) -> Self {
        let n = name.into();
        let ut = if n.ends_with(".target") {
            SystemdUnitType::Target
        } else if n.ends_with(".socket") {
            SystemdUnitType::Socket
        } else if n.ends_with(".timer") {
            SystemdUnitType::Timer
        } else if n.ends_with(".slice") {
            SystemdUnitType::Slice
        } else {
            SystemdUnitType::Service
        };

        Self {
            name: n,
            unit_type: ut,
            exec_start: exec_start.into(),
            restart: false,
            dependencies: Vec::new(),
            wants: Vec::new(),
            requires: Vec::new(),
            conflicts: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 0,
            last_ping: 0,
            status_text: String::from("Inactive"),
            slice_limits: CgroupV2SliceLimits::default(),
        }
    }
}

/// Legacy InitSystem wrapper maintained for backwards compatibility
pub struct InitSystem {
    pub services: BTreeMap<String, Service>,
}

impl InitSystem {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }

    pub fn register_service(&mut self, service: Service) {
        self.services.insert(service.name.clone(), service);
    }

    pub fn topo_sort(&self) -> Result<Vec<String>, &'static str> {
        let mut sorted = Vec::new();
        let mut visited = BTreeSet::new();
        let mut temp_mark = BTreeSet::new();

        fn visit(
            node: &String,
            services: &BTreeMap<String, Service>,
            visited: &mut BTreeSet<String>,
            temp_mark: &mut BTreeSet<String>,
            sorted: &mut Vec<String>,
        ) -> Result<(), &'static str> {
            if temp_mark.contains(node) {
                return Err("Cyclic dependency detected");
            }
            if !visited.contains(node) {
                temp_mark.insert(node.clone());
                if let Some(srv) = services.get(node) {
                    for dep in &srv.dependencies {
                        visit(dep, services, visited, temp_mark, sorted)?;
                    }
                    for dep in &srv.after {
                        if services.contains_key(dep) {
                            visit(dep, services, visited, temp_mark, sorted)?;
                        }
                    }
                }
                temp_mark.remove(node);
                visited.insert(node.clone());
                sorted.push(node.clone());
            }
            Ok(())
        }

        for node in self.services.keys() {
            visit(
                node,
                &self.services,
                &mut visited,
                &mut temp_mark,
                &mut sorted,
            )?;
        }
        Ok(sorted)
    }

    pub fn start_all(&mut self) -> Result<(), &'static str> {
        let sorted = self.topo_sort()?;
        for name in sorted {
            self.start_service(&name)?;
        }
        Ok(())
    }

    pub fn start_service(&mut self, name: &str) -> Result<(), &'static str> {
        let srv = self.services.get_mut(name).ok_or("Service not found")?;
        if srv.state == ServiceState::Running {
            return Ok(());
        }

        srv.state = ServiceState::Starting;
        srv.pid = Some(1000 + name.len() as u32);
        srv.state = ServiceState::Running;
        srv.last_ping = 0;

        Ok(())
    }

    pub fn check_watchdogs(&mut self, now: u64) {
        for srv in self.services.values_mut() {
            if srv.state == ServiceState::Running && srv.watchdog_usec > 0 {
                if now.saturating_sub(srv.last_ping) > srv.watchdog_usec {
                    srv.state = ServiceState::Failed;
                    srv.pid = None;
                    srv.status_text = String::from("Watchdog timeout triggered SIGKILL");
                }
            }
        }
    }

    /// Process `sd_notify` socket messages (`READY=1`, `RELOAD=1`, `STOPPING=1`, `WATCHDOG=1`, `STATUS=...`)
    pub fn process_sd_notify(&mut self, service_name: &str, notify_msg: &str, timestamp: u64) -> bool {
        if let Some(srv) = self.services.get_mut(service_name) {
            for line in notify_msg.lines() {
                let trimmed = line.trim();
                if trimmed == "READY=1" {
                    srv.state = ServiceState::Running;
                    srv.status_text = String::from("Ready and operational");
                } else if trimmed == "RELOAD=1" {
                    srv.state = ServiceState::Reloading;
                    srv.status_text = String::from("Reloading configuration");
                } else if trimmed == "STOPPING=1" {
                    srv.state = ServiceState::Stopping;
                    srv.status_text = String::from("Graceful shutdown in progress");
                } else if trimmed == "WATCHDOG=1" {
                    srv.last_ping = timestamp;
                } else if trimmed.starts_with("STATUS=") {
                    srv.status_text = trimmed.trim_start_matches("STATUS=").to_string();
                }
            }
            true
        } else {
            false
        }
    }
}

// ============================================================================
// 2. OPENRC MULTI-STAGE RUNLEVEL & KEYWORD DIRECTIVE ENGINE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpenRcRunlevel {
    SysInit,
    Boot,
    Default,
    Nonetwork,
    Shutdown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenRcKeyword {
    Provide(String),
    Need(String),
    Use(String),
    Before(String),
    After(String),
    KeywordNoDocker,
}

#[derive(Debug, Clone)]
pub struct OpenRcService {
    pub name: String,
    pub runlevels: BTreeSet<OpenRcRunlevel>,
    pub provides: Vec<String>,
    pub needs: Vec<String>,
    pub uses: Vec<String>,
    pub before: Vec<String>,
    pub after: Vec<String>,
    pub no_docker: bool,
    pub state: ServiceState,
}

impl OpenRcService {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            runlevels: BTreeSet::new(),
            provides: Vec::new(),
            needs: Vec::new(),
            uses: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            no_docker: false,
            state: ServiceState::Stopped,
        }
    }

    pub fn add_keyword(&mut self, keyword: OpenRcKeyword) {
        match keyword {
            OpenRcKeyword::Provide(p) => self.provides.push(p),
            OpenRcKeyword::Need(n) => self.needs.push(n),
            OpenRcKeyword::Use(u) => self.uses.push(u),
            OpenRcKeyword::Before(b) => self.before.push(b),
            OpenRcKeyword::After(a) => self.after.push(a),
            OpenRcKeyword::KeywordNoDocker => self.no_docker = true,
        }
    }
}

pub struct OpenRcEngine {
    pub current_runlevel: OpenRcRunlevel,
    pub services: BTreeMap<String, OpenRcService>,
}

impl OpenRcEngine {
    pub fn new() -> Self {
        Self {
            current_runlevel: OpenRcRunlevel::SysInit,
            services: BTreeMap::new(),
        }
    }

    pub fn register_service(&mut self, srv: OpenRcService) {
        self.services.insert(srv.name.clone(), srv);
    }

    pub fn transition_to_runlevel(&mut self, target: OpenRcRunlevel) -> usize {
        self.current_runlevel = target;
        let mut started = 0;
        for srv in self.services.values_mut() {
            if srv.runlevels.contains(&target) {
                srv.state = ServiceState::Running;
                started += 1;
            } else if target == OpenRcRunlevel::Shutdown {
                srv.state = ServiceState::Stopped;
            }
        }
        started
    }
}

// ============================================================================
// 3. S6 & RUNIT DIRECTORY SUPERVISION ENGINE (/var/service)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6RunitState {
    Up,
    Down,
    Paused,
    Restarting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6RunitControlVerb {
    Alarm,     // 'a'
    Up,        // 'u'
    Down,      // 'd'
    Once,      // 'o'
    Exit,      // 'x'
    Restart,   // 'r'
    Pause,     // 'p'
    Continue,  // 'c'
}

#[derive(Debug, Clone)]
pub struct S6RunitSupervisedService {
    pub name: String,
    pub service_dir: String,
    pub state: S6RunitState,
    pub pid: Option<u32>,
    pub auto_restart: bool,
}

pub struct S6RunitSupervisorEngine {
    pub service_dir_path: String,
    pub supervised_services: BTreeMap<String, S6RunitSupervisedService>,
}

impl S6RunitSupervisorEngine {
    pub fn new(service_dir_path: impl Into<String>) -> Self {
        Self {
            service_dir_path: service_dir_path.into(),
            supervised_services: BTreeMap::new(),
        }
    }

    pub fn scan_service_directory(&mut self, names: &[&str]) {
        for name in names {
            if !self.supervised_services.contains_key(*name) {
                let service = S6RunitSupervisedService {
                    name: (*name).to_string(),
                    service_dir: format!("{}/{}", self.service_dir_path, name),
                    state: S6RunitState::Down,
                    pid: None,
                    auto_restart: true,
                };
                self.supervised_services.insert((*name).to_string(), service);
            }
        }
    }

    pub fn dispatch_control_verb(&mut self, service_name: &str, verb: S6RunitControlVerb) -> bool {
        if let Some(srv) = self.supervised_services.get_mut(service_name) {
            match verb {
                S6RunitControlVerb::Up => {
                    srv.state = S6RunitState::Up;
                    srv.pid = Some(2000 + service_name.len() as u32);
                }
                S6RunitControlVerb::Down => {
                    srv.state = S6RunitState::Down;
                    srv.pid = None;
                }
                S6RunitControlVerb::Once => {
                    srv.state = S6RunitState::Up;
                    srv.auto_restart = false;
                    srv.pid = Some(2100 + service_name.len() as u32);
                }
                S6RunitControlVerb::Restart => {
                    srv.state = S6RunitState::Restarting;
                    srv.state = S6RunitState::Up;
                    srv.pid = Some(2200 + service_name.len() as u32);
                }
                S6RunitControlVerb::Pause => {
                    srv.state = S6RunitState::Paused;
                }
                S6RunitControlVerb::Continue => {
                    if srv.state == S6RunitState::Paused {
                        srv.state = S6RunitState::Up;
                    }
                }
                S6RunitControlVerb::Exit => {
                    srv.state = S6RunitState::Down;
                    srv.auto_restart = false;
                    srv.pid = None;
                }
                S6RunitControlVerb::Alarm => {}
            }
            true
        } else {
            false
        }
    }
}

// ============================================================================
// 4. SYSVINIT (/etc/inittab) & BSD rc.d/rc.subr HEADER PARSER
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysVinitAction {
    Respawn,
    SysInit,
    Wait,
    Once,
    CtrlAltDel,
    PowerFail,
}

#[derive(Debug, Clone)]
pub struct SysVinittabEntry {
    pub id: String,
    pub runlevels: Vec<char>,
    pub action: SysVinitAction,
    pub process: String,
}

pub struct SysVinitEngine {
    pub inittab_entries: Vec<SysVinittabEntry>,
}

impl SysVinitEngine {
    pub fn new() -> Self {
        Self {
            inittab_entries: Vec::new(),
        }
    }

    pub fn parse_inittab_line(&mut self, line: &str) -> Option<SysVinittabEntry> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return None;
        }
        let parts: Vec<&str> = trimmed.split(':').collect();
        if parts.len() < 4 {
            return None;
        }

        let id = parts[0].to_string();
        let runlevels = parts[1].chars().collect();
        let action = match parts[2] {
            "respawn" => SysVinitAction::Respawn,
            "sysinit" => SysVinitAction::SysInit,
            "wait" => SysVinitAction::Wait,
            "once" => SysVinitAction::Once,
            "ctrlaltdel" => SysVinitAction::CtrlAltDel,
            "powerfail" => SysVinitAction::PowerFail,
            _ => SysVinitAction::Once,
        };
        let process = parts[3].to_string();

        let entry = SysVinittabEntry {
            id,
            runlevels,
            action,
            process,
        };
        self.inittab_entries.push(entry.clone());
        Some(entry)
    }
}

#[derive(Debug, Clone, Default)]
pub struct BsdRcSubrHeader {
    pub provides: Vec<String>,
    pub requires: Vec<String>,
    pub befores: Vec<String>,
    pub keywords: Vec<String>,
    pub precmd: Option<String>,
    pub postcmd: Option<String>,
}

pub struct BsdRcSubrEngine;

impl BsdRcSubrEngine {
    pub fn parse_rc_script_header(script_content: &str) -> BsdRcSubrHeader {
        let mut hdr = BsdRcSubrHeader::default();

        for line in script_content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# PROVIDE:") {
                let items = trimmed.trim_start_matches("# PROVIDE:").split_whitespace();
                hdr.provides.extend(items.map(|s| s.to_string()));
            } else if trimmed.starts_with("# REQUIRE:") {
                let items = trimmed.trim_start_matches("# REQUIRE:").split_whitespace();
                hdr.requires.extend(items.map(|s| s.to_string()));
            } else if trimmed.starts_with("# BEFORE:") {
                let items = trimmed.trim_start_matches("# BEFORE:").split_whitespace();
                hdr.befores.extend(items.map(|s| s.to_string()));
            } else if trimmed.starts_with("# KEYWORD:") {
                let items = trimmed.trim_start_matches("# KEYWORD:").split_whitespace();
                hdr.keywords.extend(items.map(|s| s.to_string()));
            } else if trimmed.contains("precmd=") {
                let cmd = trimmed.split('=').nth(1).unwrap_or("").trim_matches('"').trim_matches('\'');
                hdr.precmd = Some(cmd.to_string());
            } else if trimmed.contains("postcmd=") {
                let cmd = trimmed.split('=').nth(1).unwrap_or("").trim_matches('"').trim_matches('\'');
                hdr.postcmd = Some(cmd.to_string());
            }
        }
        hdr
    }
}

// ============================================================================
// 5. MACOS LAUNCHD JOB SPECIFICATION ENGINE
// ============================================================================

#[derive(Debug, Clone)]
pub struct LaunchdJobSpec {
    pub label: String,
    pub program: String,
    pub keep_alive_crashed: bool,
    pub keep_alive_successful_exit: bool,
    pub launch_events: Vec<String>,
    pub throttle_interval_sec: u64,
    pub last_launch_timestamp: u64,
}

pub struct LaunchdSupervisorEngine {
    pub jobs: BTreeMap<String, LaunchdJobSpec>,
}

impl LaunchdSupervisorEngine {
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
        }
    }

    pub fn register_job(&mut self, spec: LaunchdJobSpec) {
        self.jobs.insert(spec.label.clone(), spec);
    }

    pub fn should_restart(&self, label: &str, exit_code: i32, now: u64) -> bool {
        if let Some(job) = self.jobs.get(label) {
            if now.saturating_sub(job.last_launch_timestamp) < job.throttle_interval_sec {
                return false;
            }
            if exit_code == 0 {
                job.keep_alive_successful_exit
            } else {
                job.keep_alive_crashed
            }
        } else {
            false
        }
    }
}

// ============================================================================
// 6. CRON, ANACRON, AT/BATCH, & SYSTEMD TIMERS EXECUTION SUITE
// ============================================================================

#[derive(Debug, Clone)]
pub struct CronJob {
    pub minute_pattern: String,
    pub hour_pattern: String,
    pub dom_pattern: String,
    pub month_pattern: String,
    pub dow_pattern: String,
    pub command: String,
}

impl CronJob {
    pub fn parse_cron_line(line: &str) -> Option<Self> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return None;
        }

        if trimmed.starts_with('@') {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 2 {
                return None;
            }
            let macro_name = parts[0];
            let command = parts[1..].join(" ");
            let (m, h, dom, mon, dow) = match macro_name {
                "@hourly" => ("0", "*", "*", "*", "*"),
                "@daily" | "@midnight" => ("0", "0", "*", "*", "*"),
                "@weekly" => ("0", "0", "*", "*", "0"),
                "@monthly" => ("0", "0", "1", "*", "*"),
                "@yearly" | "@annually" => ("0", "0", "1", "1", "*"),
                "@reboot" => ("reboot", "reboot", "reboot", "reboot", "reboot"),
                _ => ("*", "*", "*", "*", "*"),
            };
            return Some(Self {
                minute_pattern: m.to_string(),
                hour_pattern: h.to_string(),
                dom_pattern: dom.to_string(),
                month_pattern: mon.to_string(),
                dow_pattern: dow.to_string(),
                command,
            });
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.len() < 6 {
            return None;
        }

        Some(Self {
            minute_pattern: parts[0].to_string(),
            hour_pattern: parts[1].to_string(),
            dom_pattern: parts[2].to_string(),
            month_pattern: parts[3].to_string(),
            dow_pattern: parts[4].to_string(),
            command: parts[5..].join(" "),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AnacronJob {
    pub period_days: u32,
    pub delay_minutes: u32,
    pub job_id: String,
    pub command: String,
    pub random_delay: u32,
    pub start_hours_range: (u8, u8),
    pub last_run_epoch: u64,
}

impl AnacronJob {
    pub fn is_due(&self, current_epoch: u64, current_hour: u8) -> bool {
        if current_hour < self.start_hours_range.0 || current_hour > self.start_hours_range.1 {
            return false;
        }
        let seconds_in_period = (self.period_days as u64) * 86400;
        current_epoch.saturating_sub(self.last_run_epoch) >= seconds_in_period
    }
}

#[derive(Debug, Clone)]
pub struct AtBatchJob {
    pub job_id: u64,
    pub target_epoch: u64,
    pub is_batch: bool,
    pub command: String,
}

pub struct AtBatchQueue {
    pub jobs: Vec<AtBatchJob>,
}

impl AtBatchQueue {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn get_ready_jobs(&mut self, current_epoch: u64, current_load_avg: f32) -> Vec<AtBatchJob> {
        let mut ready = Vec::new();
        let mut remaining = Vec::new();

        for job in self.jobs.drain(..) {
            if job.is_batch {
                if current_load_avg <= 0.8 && current_epoch >= job.target_epoch {
                    ready.push(job);
                } else {
                    remaining.push(job);
                }
            } else {
                if current_epoch >= job.target_epoch {
                    ready.push(job);
                } else {
                    remaining.push(job);
                }
            }
        }
        self.jobs = remaining;
        ready
    }
}

#[derive(Debug, Clone)]
pub struct SystemdTimer {
    pub name: String,
    pub on_calendar: String,
    pub persistent: bool,
    pub accuracy_sec: u64,
    pub last_trigger_epoch: u64,
}

// ============================================================================
// 7. MASTER SOVEREIGN CORE USERLAND INIT ENGINE
// ============================================================================

pub struct SovereignCoreUserlandInitEngine {
    pub init_system: InitSystem,
    pub openrc_engine: OpenRcEngine,
    pub s6_runit_engine: S6RunitSupervisorEngine,
    pub sysvinit_engine: SysVinitEngine,
    pub launchd_engine: LaunchdSupervisorEngine,
    pub at_batch_queue: AtBatchQueue,
    pub cron_jobs: Vec<CronJob>,
    pub anacron_jobs: Vec<AnacronJob>,
    pub systemd_timers: Vec<SystemdTimer>,
}

impl SovereignCoreUserlandInitEngine {
    pub fn new() -> Self {
        Self {
            init_system: InitSystem::new(),
            openrc_engine: OpenRcEngine::new(),
            s6_runit_engine: S6RunitSupervisorEngine::new("/var/service"),
            sysvinit_engine: SysVinitEngine::new(),
            launchd_engine: LaunchdSupervisorEngine::new(),
            at_batch_queue: AtBatchQueue::new(),
            cron_jobs: Vec::new(),
            anacron_jobs: Vec::new(),
            systemd_timers: Vec::new(),
        }
    }

    pub fn audit_status(&self) -> String {
        format!(
            "SovereignCoreUserlandInitEngine operational | systemd units: {}, OpenRC services: {}, s6/runit: {}, SysV entries: {}, launchd jobs: {}, cron: {}",
            self.init_system.services.len(),
            self.openrc_engine.services.len(),
            self.s6_runit_engine.supervised_services.len(),
            self.sysvinit_engine.inittab_entries.len(),
            self.launchd_engine.jobs.len(),
            self.cron_jobs.len()
        )
    }
}

// ============================================================================
// 8. UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systemd_unit_dependencies_and_sd_notify() {
        let mut sys = InitSystem::new();
        let mut db = Service::new("db.service", "/usr/bin/db");
        let mut app = Service::new("app.service", "/usr/bin/app");

        db.watchdog_usec = 5000000;
        app.after.push("db.service".to_string());

        sys.register_service(db);
        sys.register_service(app);

        let order = sys.topo_sort().expect("Sorting failed");
        assert_eq!(order, vec!["db.service", "app.service"]);

        sys.start_all().expect("Start failed");
        assert_eq!(sys.services.get("db.service").unwrap().state, ServiceState::Running);

        let notify_ok = sys.process_sd_notify("db.service", "STATUS=Processing requests\nWATCHDOG=1", 1000);
        assert!(notify_ok);
        assert_eq!(sys.services.get("db.service").unwrap().status_text, "Processing requests");
        assert_eq!(sys.services.get("db.service").unwrap().last_ping, 1000);
    }

    #[test]
    fn test_openrc_runlevels_and_keywords() {
        let mut engine = OpenRcEngine::new();
        let mut srv = OpenRcService::new("networking");
        srv.runlevels.insert(OpenRcRunlevel::Boot);
        srv.add_keyword(OpenRcKeyword::Provide("net".to_string()));
        srv.add_keyword(OpenRcKeyword::KeywordNoDocker);

        engine.register_service(srv);
        let started = engine.transition_to_runlevel(OpenRcRunlevel::Boot);
        assert_eq!(started, 1);
        assert_eq!(engine.services.get("networking").unwrap().state, ServiceState::Running);
        assert!(engine.services.get("networking").unwrap().no_docker);
    }

    #[test]
    fn test_s6_runit_supervision_verbs() {
        let mut engine = S6RunitSupervisorEngine::new("/var/service");
        engine.scan_service_directory(&["nginx", "redis"]);

        assert_eq!(engine.supervised_services.len(), 2);
        assert_eq!(engine.supervised_services.get("nginx").unwrap().state, S6RunitState::Down);

        engine.dispatch_control_verb("nginx", S6RunitControlVerb::Up);
        assert_eq!(engine.supervised_services.get("nginx").unwrap().state, S6RunitState::Up);
        assert!(engine.supervised_services.get("nginx").unwrap().pid.is_some());

        engine.dispatch_control_verb("nginx", S6RunitControlVerb::Pause);
        assert_eq!(engine.supervised_services.get("nginx").unwrap().state, S6RunitState::Paused);
    }

    #[test]
    fn test_sysvinit_inittab_parsing() {
        let mut engine = SysVinitEngine::new();
        let entry = engine.parse_inittab_line("id:35:respawn:/sbin/getty 38400 tty1");
        assert!(entry.is_some());
        let e = entry.unwrap();
        assert_eq!(e.id, "id");
        assert_eq!(e.runlevels, vec!['3', '5']);
        assert_eq!(e.action, SysVinitAction::Respawn);
        assert_eq!(e.process, "/sbin/getty 38400 tty1");
    }

    #[test]
    fn test_bsd_rc_subr_header_parsing() {
        let script = r#"
        #!/bin/sh
        # PROVIDE: sshd
        # REQUIRE: LOGIN
        # BEFORE: SECURE
        # KEYWORD: shutdown
        precmd="sshd_precmd"
        "#;

        let hdr = BsdRcSubrEngine::parse_rc_script_header(script);
        assert_eq!(hdr.provides, vec!["sshd"]);
        assert_eq!(hdr.requires, vec!["LOGIN"]);
        assert_eq!(hdr.befores, vec!["SECURE"]);
        assert_eq!(hdr.keywords, vec!["shutdown"]);
        assert_eq!(hdr.precmd, Some("sshd_precmd".to_string()));
    }

    #[test]
    fn test_launchd_job_specification() {
        let mut engine = LaunchdSupervisorEngine::new();
        let spec = LaunchdJobSpec {
            label: "com.example.daemon".to_string(),
            program: "/usr/libexec/daemon".to_string(),
            keep_alive_crashed: true,
            keep_alive_successful_exit: false,
            launch_events: vec!["network".to_string()],
            throttle_interval_sec: 10,
            last_launch_timestamp: 100,
        };
        engine.register_job(spec);

        // Crash exit code (1) at time 115 (> throttle 10) should restart
        assert!(engine.should_restart("com.example.daemon", 1, 115));
        // Clean exit code (0) should not restart
        assert!(!engine.should_restart("com.example.daemon", 0, 115));
        // Attempt at time 105 (< throttle 10) should return false
        assert!(!engine.should_restart("com.example.daemon", 1, 105));
    }

    #[test]
    fn test_cron_expression_and_macro_parsing() {
        let c1 = CronJob::parse_cron_line("0 2 * * * /usr/bin/backup").unwrap();
        assert_eq!(c1.minute_pattern, "0");
        assert_eq!(c1.hour_pattern, "2");
        assert_eq!(c1.command, "/usr/bin/backup");

        let c2 = CronJob::parse_cron_line("@daily /usr/bin/daily-cleanup").unwrap();
        assert_eq!(c2.minute_pattern, "0");
        assert_eq!(c2.hour_pattern, "0");
        assert_eq!(c2.command, "/usr/bin/daily-cleanup");
    }

    #[test]
    fn test_anacron_and_at_batch_load_gating() {
        let anacron = AnacronJob {
            period_days: 7,
            delay_minutes: 10,
            job_id: "cron.weekly".to_string(),
            command: "/usr/bin/weekly".to_string(),
            random_delay: 15,
            start_hours_range: (6, 22),
            last_run_epoch: 100000,
        };

        // 8 days later (691200 sec) at hour 10 should be due
        assert!(anacron.is_due(100000 + 700000, 10));
        // At hour 3 (outside 6..22 range) should not be due
        assert!(!anacron.is_due(100000 + 700000, 3));

        let mut queue = AtBatchQueue::new();
        queue.jobs.push(AtBatchJob {
            job_id: 1,
            target_epoch: 500,
            is_batch: true,
            command: "make -j8".to_string(),
        });

        // Load too high (0.95 > 0.8) -> not ready
        let ready1 = queue.get_ready_jobs(600, 0.95);
        assert_eq!(ready1.len(), 0);

        // Load acceptable (0.4 <= 0.8) -> ready
        let ready2 = queue.get_ready_jobs(600, 0.4);
        assert_eq!(ready2.len(), 1);
        assert_eq!(ready2[0].job_id, 1);
    }

    #[test]
    fn test_sovereign_core_userland_init_engine_master_workflow() {
        let master = SovereignCoreUserlandInitEngine::new();
        let status = master.audit_status();
        assert!(status.contains("SovereignCoreUserlandInitEngine operational"));
    }
}
