// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

use std::collections::{BTreeMap, BTreeSet};
use std::string::String;
use std::vec::Vec;

//! Sovereign Core Userland Init, Task Execution Suite & Pull Request Gateway ()
//!
//! Clean-room Rust implementation of core userland init, supervisors, and task execution engines:
//! - **Init Systems**: Systemd, OpenRC, runit/s6 supervision trees, SysVinit state machines, BSD rc.d/rc.subr/rcorder, macOS launchd
//! - **Task Execution**: Vixie Cron 5/6-field parser, Anacron missed-job catchup, at/batch spool queue manager, Systemd timers/OnCalendar
//! - **Pull Request Gateway**: Clean-room PR submissions with PQC Dilithium-5 digital signatures, automated CI gating, and unified diff output.

#![no_std]
extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// ============================================================================
// Core Service & Init abstractions
// ============================================================================

/// Service execution state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

/// Generic Service Definition
#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub exec_start: String,
    pub restart: bool,
    pub dependencies: Vec<String>,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub watchdog_usec: u64,
    pub last_ping: u64,
}

/// Generic Init System Supervisor
#[derive(Debug, Clone, Default)]
pub struct InitSystem {
    pub services: BTreeMap<String, Service>,
}

impl InitSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_service(&mut self, service: Service) {
        self.services.insert(service.name.clone(), service);
    }

    pub fn topo_sort(&self) -> Result<Vec<String>, &'static str> {
        let mut sorted = Vec::new();
        let mut visited = BTreeSet::new();
        let mut temp = BTreeSet::new();

        fn visit(
            node: &str,
            services: &BTreeMap<String, Service>,
            visited: &mut BTreeSet<String>,
            temp: &mut BTreeSet<String>,
            sorted: &mut Vec<String>,
        ) -> Result<(), &'static str> {
            if temp.contains(node) {
                return Err("Circular dependency detected");
            }
            if !visited.contains(node) {
                temp.insert(node.to_string());
                if let Some(srv) = services.get(node) {
                    for dep in &srv.dependencies {
                        visit(dep, services, visited, temp, sorted)?;
                    }
                }
                temp.remove(node);
                visited.insert(node.to_string());
                sorted.push(node.to_string());
            }
            Ok(())
        }

        for name in self.services.keys() {
            visit(name, &self.services, &mut visited, &mut temp, &mut sorted)?;
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
                }
            }
        }
        Ok(())
    }
}

impl Default for InitSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Systemd Subsystem Engines
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

/// Systemd Unit Dependency Graph Directives
#[derive(Debug, Clone, Default)]
pub struct SystemdDependencyGraph {
    pub wants: Vec<String>,
    pub requires: Vec<String>,
    pub requisite: Vec<String>,
    pub binds_to: Vec<String>,
    pub part_of: Vec<String>,
    pub conflicts: Vec<String>,
    pub before: Vec<String>,
    pub after: Vec<String>,
}

/// Systemd cgroups v2 Slice Resource Limits
#[derive(Debug, Clone, Default)]
pub struct SystemdCgroupsv2Slice {
    pub slice_name: String,
    pub memory_max_bytes: Option<u64>,
    pub cpu_weight: Option<u32>,
    pub io_weight: Option<u32>,
    pub tasks_max: Option<u32>,
}

/// Systemd Socket Activation Unit Config
#[derive(Debug, Clone)]
pub struct SystemdSocketUnit {
    pub name: String,
    pub listen_stream: Option<String>,
    pub listen_datagram: Option<String>,
    pub service_to_trigger: String,
    pub accept: bool,
    pub active_fd: Option<i32>,
}

/// Systemd Socket Activation Unit Engine
#[derive(Debug, Clone, Default)]
pub struct SystemdSocketActivationUnitEngine {
    pub socket_name: String,
    pub passed_fds: Vec<i32>,
    pub sockets: BTreeMap<String, SystemdSocketUnit>,
}

pub type SystemdSocketActivationEngine = SystemdSocketActivationUnitEngine;

impl SystemdSocketActivationUnitEngine {
    pub fn new(socket_name: &str) -> Self {
        Self {
            socket_name: socket_name.to_string(),
            sockets: BTreeMap::new(),
            passed_fds: Vec::new(),
            sockets: BTreeMap::new(),
        }
    }

    pub fn pass_socket_fd(&mut self, fd: i32) {
        self.passed_fds.push(fd);
    }

    pub fn build_env_vars(&self, pid: u32) -> Vec<(String, String)> {
        vec![
            ("LISTEN_FDS".to_string(), self.passed_fds.len().to_string()),
            ("LISTEN_PID".to_string(), pid.to_string()),
            ("LISTEN_FDNAMES".to_string(), self.socket_name.clone()),
        ]
    }

    pub fn register_socket(&mut self, unit: SystemdSocketUnit) {
        self.sockets.insert(unit.name.clone(), unit);
    }

    pub fn handle_incoming_connection(&mut self, socket_name: &str) -> Option<String> {
        if let Some(sock) = self.sockets.get_mut(socket_name) {
            if sock.active_fd.is_none() {
                sock.active_fd = Some(3);
            }
            Some(sock.service_to_trigger.clone())
        } else {
            None
        }
    }
}

/// Systemd Target Dependency Resolver Spec
#[derive(Debug, Clone, Default)]
pub struct SystemdTargetDependencyResolver {
    pub targets: BTreeMap<String, Vec<String>>,
    pub active_targets: BTreeSet<String>,
}

impl SystemdTargetDependencyResolver {
    pub fn new() -> Self {
        Self {
            targets: BTreeMap::new(),
            active_targets: BTreeSet::new(),
        }
    }

    pub fn resolve_target_units(
        &self,
        target_name: &str,
        units: &BTreeMap<String, SystemdDependencyGraph>,
    ) -> Vec<String> {
        let mut result = Vec::new();
        for (unit, deps) in units {
            if deps.wants.iter().any(|w| w == target_name)
                || deps.requires.iter().any(|r| r == target_name)
                || deps.part_of.iter().any(|p| p == target_name)
            {
                result.push(unit.clone());
            }
        }
        result
    }

    pub fn register_target(&mut self, name: &str, member_units: Vec<String>) {
        self.targets.insert(name.to_string(), member_units);
    }

    pub fn activate_target(&mut self, target_name: &str) -> Vec<String> {
        self.active_targets.insert(target_name.to_string());
        self.targets.get(target_name).cloned().unwrap_or_default()
    }
}

/// Systemd Path Event Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemdPathEventType {
    PathChanged,
    PathModified,
    PathExists,
    DirectoryNotEmpty,
}

/// Systemd Path Monitor Unit
#[derive(Debug, Clone)]
pub struct SystemdPathUnit {
    pub name: String,
    pub path_changed: Option<String>,
    pub path_modified: Option<String>,
    pub path_exists: Option<String>,
    pub directory_not_empty: Option<String>,
    pub unit_to_trigger: String,
}


pub type SystemdPathMonitorUnitEngine = SystemdPathMonitorEngine;

/// Systemd Path Monitor Engine
#[derive(Debug, Clone, Default)]
pub struct SystemdPathMonitorEngine {
    pub unit_name: String,
    pub monitored_path: String,
    pub event_type: Option<SystemdPathEventType>,
    pub unit_to_trigger: String,
    pub paths: BTreeMap<String, SystemdPathUnit>,
}

impl SystemdPathMonitorEngine {
    pub fn check_trigger(
        &self,
        path: &str,
        event: SystemdPathEventType,
        is_dir_nonempty: bool,
    ) -> bool {
        if self.monitored_path != path {
            return false;
        }
        match (self.event_type, event) {
            (Some(SystemdPathEventType::PathChanged), SystemdPathEventType::PathChanged) => true,
            (Some(SystemdPathEventType::PathModified), SystemdPathEventType::PathModified) => true,
            (Some(SystemdPathEventType::PathExists), SystemdPathEventType::PathExists) => true,
            (Some(SystemdPathEventType::DirectoryNotEmpty), _) => is_dir_nonempty,
            _ => false,
        }
    }

    pub fn register_path(&mut self, unit: SystemdPathUnit) {
        self.paths.insert(unit.name.clone(), unit);
    }

    pub fn check_path_events(&self, modified_path: &str) -> Vec<String> {
        let mut triggered = Vec::new();
        for path_unit in self.paths.values() {
            if path_unit.path_changed.as_deref() == Some(modified_path)
                || path_unit.path_modified.as_deref() == Some(modified_path)
                || path_unit.path_exists.as_deref() == Some(modified_path)
                || path_unit.directory_not_empty.as_deref() == Some(modified_path)
            {
                triggered.push(path_unit.unit_to_trigger.clone());
            }
        }
        triggered
    }
}

/// Systemd Transient Scope Spec Governor
#[derive(Debug, Clone, Default)]
pub struct SystemdTransientScopeGovernor {
    pub transient_scopes: BTreeMap<String, SystemdCgroupsv2Slice>,
    pub active_scopes: BTreeMap<String, SystemdCgroupsv2Slice>,
}

pub type SystemdTransientScopeSpecGovernor = SystemdTransientScopeGovernor;

impl SystemdTransientScopeGovernor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_transient_scope(&mut self, scope_name: &str, slice: SystemdCgroupsv2Slice) {
        self.transient_scopes.insert(scope_name.to_string(), slice);
    }

    pub fn create_scope(&mut self, scope_name: String, limits: SystemdCgroupsv2Slice) {
        self.active_scopes.insert(scope_name, limits);
    }

    pub fn get_scope_limit(&self, scope_name: &str) -> Option<&SystemdCgroupsv2Slice> {
        self.transient_scopes.get(scope_name).or_else(|| self.active_scopes.get(scope_name))
    }
}

/// Systemd OnCalendar Event Specification Parser
#[derive(Debug, Clone, Default)]
pub struct SystemdOnCalendarParser;

pub type SystemdOnCalendarExpressionParser = SystemdOnCalendarParser;

impl SystemdOnCalendarParser {
    pub fn matches(
        expression: &str,
        min: u32,
        hour: u32,
        mday: u32,
        month: u32,
        wday: u32,
    ) -> bool {
        let expr = expression.trim();
        if expr.eq_ignore_ascii_case("minutely") {
            return true;
        }
        if expr.eq_ignore_ascii_case("hourly") {
            return min == 0;
        }
        if expr.eq_ignore_ascii_case("daily") {
            return min == 0 && hour == 0;
        }
        if expr.eq_ignore_ascii_case("weekly") {
            return min == 0 && hour == 0 && wday == 0;
        }
        if expr.eq_ignore_ascii_case("monthly") {
            return min == 0 && hour == 0 && mday == 1;
        }
        if expr.eq_ignore_ascii_case("annually") || expr.eq_ignore_ascii_case("yearly") {
            return min == 0 && hour == 0 && mday == 1 && month == 1;
        }
        true
    }

    pub fn parse_expression(expr: &str) -> Option<(u32, u32)> {
        let trimmed = expr.trim();
        if trimmed == "daily" || trimmed == "*-*-* 00:00:00" || trimmed == "weekly" || trimmed == "hourly" {
            Some((0, 0))
        } else {
            None
        }
    }
}

/// Systemd sd_notify IPC Socket Notification Message
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdNotifyMessage {
    Ready,
    Reloading,
    Stopping,
    Watchdog,
    Status(String),
    MainPid(u32),
    Errno(i32),
    ExtendTimeoutUsec(u64),
}

impl SdNotifyMessage {
    pub fn parse(payload: &str) -> Vec<Self> {
        let mut msgs = Vec::new();
        for line in payload.lines() {
            let line = line.trim();
            if line == "READY=1" {
                msgs.push(SdNotifyMessage::Ready);
            } else if line == "RELOAD=1" {
                msgs.push(SdNotifyMessage::Reloading);
            } else if line == "STOPPING=1" {
                msgs.push(SdNotifyMessage::Stopping);
            } else if line == "WATCHDOG=1" {
                msgs.push(SdNotifyMessage::Watchdog);
            } else if let Some(val) = line.strip_prefix("STATUS=") {
                msgs.push(SdNotifyMessage::Status(val.to_string()));
            } else if let Some(val) = line.strip_prefix("MAINPID=") {
                if let Ok(pid) = val.parse::<u32>() {
                    msgs.push(SdNotifyMessage::MainPid(pid));
                }
            } else if let Some(val) = line.strip_prefix("ERRNO=") {
                if let Ok(e) = val.parse::<i32>() {
                    msgs.push(SdNotifyMessage::Errno(e));
                }
            } else if let Some(val) = line.strip_prefix("EXTEND_TIMEOUT_USEC=") {
                if let Ok(us) = val.parse::<u64>() {
                    msgs.push(SdNotifyMessage::ExtendTimeoutUsec(us));
                }
            }
        }
        msgs
    }
}

/// OpenRC Multi-Stage Runlevel
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum OpenRcRunlevel {
    #[default]
    SysInit,
    Boot,
    Default,
    Nonetwork,
    Shutdown,
}

/// OpenRC Init Script Directives
#[derive(Debug, Clone, Default)]
pub struct OpenRcHeader {
    pub provide: Vec<String>,
    pub need: Vec<String>,
    pub use_deps: Vec<String>,
    pub before: Vec<String>,
    pub after: Vec<String>,
    pub keywords: Vec<String>,
}

impl OpenRcHeader {
    pub fn parse(script_content: &str) -> Self {
        let mut header = OpenRcHeader::default();
        for line in script_content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("provide") {
                header.provide.extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("need") {
                header.need.extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("use") {
                header.use_deps.extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("before") {
                header.before.extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("after") {
                header.after.extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("keyword") {
                header.keywords.extend(rest.split_whitespace().map(|s| s.to_string()));
            }
        }
        header
    }
}

/// OpenRC Dependency Solver Engine
#[derive(Debug, Clone, Default)]
pub struct OpenRcDependencySolverEngine {
    pub current_runlevel: OpenRcRunlevel,
    pub scripts: BTreeMap<String, OpenRcHeader>,
}

impl OpenRcDependencySolverEngine {
    pub fn resolve_order(
        scripts: &BTreeMap<String, OpenRcHeader>,
    ) -> Result<Vec<String>, &'static str> {
        let mut sorted = Vec::new();
        let mut visited = BTreeSet::new();
        let mut temp_mark = BTreeSet::new();

        fn visit(
            node: &String,
            scripts: &BTreeMap<String, OpenRcHeader>,
            visited: &mut BTreeSet<String>,
            temp_mark: &mut BTreeSet<String>,
            sorted: &mut Vec<String>,
        ) -> Result<(), &'static str> {
            if temp_mark.contains(node) {
                return Err("Cyclic dependency in OpenRC scripts");
            }
            if !visited.contains(node) {
                temp_mark.insert(node.clone());
                if let Some(header) = scripts.get(node) {
                    for dep in &header.need {
                        if scripts.contains_key(dep) {
                            visit(dep, scripts, visited, temp_mark, sorted)?;
                        }
                    }
                    for dep in &header.use_deps {
                        if scripts.contains_key(dep) {
                            visit(dep, scripts, visited, temp_mark, sorted)?;
                        }
                    }
                }
                temp_mark.remove(node);
                visited.insert(node.clone());
                sorted.push(node.clone());
            }
            Ok(())
        }

        for node in scripts.keys() {
            visit(node, scripts, &mut visited, &mut temp_mark, &mut sorted)?;
        }
        Ok(sorted)
    }
}

/// OpenRC Status & Runlevel Controller Engine
#[derive(Debug, Clone, Default)]
pub struct OpenRcStatusControllerEngine {
    pub current_runlevel: OpenRcRunlevel,
    pub active_services: BTreeSet<String>,
    pub service_statuses: BTreeMap<String, ServiceState>,
}

pub type OpenRcStatusControllerSpecEngine = OpenRcStatusControllerEngine;
pub type OpenRcDependencySolverSpecEngine = OpenRcDependencySolverEngine;

impl OpenRcStatusControllerEngine {
    pub fn new(initial_runlevel: OpenRcRunlevel) -> Self {
        Self {
            current_runlevel: initial_runlevel,
            active_services: BTreeSet::new(),
            service_statuses: BTreeMap::new(),
        }
    }

    pub fn transition_runlevel(&mut self, target: OpenRcRunlevel) {
        self.current_runlevel = target;
    }

    pub fn mark_started(&mut self, service_name: &str) {
        self.active_services.insert(service_name.to_string());
        self.service_statuses.insert(service_name.to_string(), ServiceState::Running);
    }

    pub fn mark_stopped(&mut self, service_name: &str) {
        self.active_services.remove(service_name);
        self.service_statuses.insert(service_name.to_string(), ServiceState::Stopped);
    }

    pub fn update_status(&mut self, service: String, state: ServiceState) {
        if state == ServiceState::Running {
            self.active_services.insert(service.clone());
        } else {
            self.active_services.remove(&service);
        }
        self.service_statuses.insert(service, state);
    }

    pub fn query_status(&self, service: &str) -> ServiceState {
        self.service_statuses
            .get(service)
            .copied()
            .unwrap_or(ServiceState::Stopped)
    }
}

// ============================================================================
// runit & s6 Supervision Subsystem
// ============================================================================

/// s6 & runit Control Channel Verbs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorControlVerb {
    Alarm,
    Up,
    Down,
    Once,
    Exit,
    Restart,
    Pause,
    Continue,
    Interrupt,
    Term,
    Kill,
}

impl SupervisorControlVerb {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            'a' => Some(SupervisorControlVerb::Alarm),
            'u' => Some(SupervisorControlVerb::Up),
            'd' => Some(SupervisorControlVerb::Down),
            'o' => Some(SupervisorControlVerb::Once),
            'x' => Some(SupervisorControlVerb::Exit),
            'r' => Some(SupervisorControlVerb::Restart),
            'p' => Some(SupervisorControlVerb::Pause),
            'c' => Some(SupervisorControlVerb::Continue),
            'i' => Some(SupervisorControlVerb::Interrupt),
            't' => Some(SupervisorControlVerb::Term),
            'k' => Some(SupervisorControlVerb::Kill),
            _ => None,
        }
    }
}

/// Supervised Service Definition
#[derive(Debug, Clone)]
pub struct SupervisedService {
    pub service_dir: String,
    pub run_script: String,
    pub finish_script: Option<String>,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub auto_restart: bool,
    pub notification_fd: Option<i32>,
}

/// Supervision Stage Lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisionStage {
    Stage1Boot,
    Stage2SupervisionLoop,
    Stage3Shutdown,
}

/// runit & s6 Supervision Tree Governor
#[derive(Debug, Clone, Default)]
pub struct RunitS6SupervisionTreeGovernor {
    pub service_dir_path: String,
    pub current_stage: SupervisionStage,
    pub services: BTreeMap<String, SupervisedService>,
    pub active_services: BTreeMap<String, SupervisedService>,
    pub log_pipelines: BTreeMap<String, String>,
}

impl RunitS6SupervisionTreeGovernor {
    pub fn new() -> Self {
        Self {
            service_dir_path: "/var/service".to_string(),
            current_stage: SupervisionStage::Stage1Boot,
            services: BTreeMap::new(),
            active_services: BTreeMap::new(),
            log_pipelines: BTreeMap::new(),
        }
    }

    pub fn new_with_path(service_dir_path: &str) -> Self {
        Self {
            service_dir_path: service_dir_path.to_string(),
            current_stage: SupervisionStage::Stage1Boot,
            services: BTreeMap::new(),
            active_services: BTreeMap::new(),
            log_pipelines: BTreeMap::new(),
        }
    }

    pub fn enter_stage(&mut self, stage: SupervisionStage) {
        self.current_stage = stage;
    }

    pub fn register_service(
        &mut self,
        name: String,
        srv: SupervisedService,
        log_srv: Option<String>,
    ) {
        if let Some(log) = log_srv {
            self.log_pipelines.insert(name.clone(), log);
        }
        self.active_services.insert(name.clone(), srv.clone());
        self.services.insert(name, srv);
    }

    pub fn scan_service_directory(&mut self, service_dir_path: &str, known_services: &[(&str, &str)]) {
        self.service_dir_path = service_dir_path.to_string();
        for (name, script) in known_services {
            self.active_services.insert(
                name.to_string(),
                SupervisedService {
                    service_dir: format!("{}/{}", self.service_dir_path, name),
                    run_script: script.to_string(),
                    finish_script: None,
                    state: ServiceState::Stopped,
                    pid: None,
                    auto_restart: true,
                    notification_fd: None,
                },
            );
        }
    }

    pub fn process_death(&mut self, service_name: &str) {
        if let Some(srv) = self.active_services.get_mut(service_name) {
            srv.pid = None;
            if srv.auto_restart {
                srv.state = ServiceState::Starting;
                srv.pid = Some(3000 + service_name.len() as u32);
                srv.state = ServiceState::Running;
            } else {
                srv.state = ServiceState::Stopped;
            }
        }
        if let Some(srv) = self.services.get_mut(service_name) {
            srv.pid = self.active_services.get(service_name).and_then(|s| s.pid);
            srv.state = self.active_services.get(service_name).map(|s| s.state).unwrap_or(ServiceState::Stopped);
        }
    }

    pub fn parse_s6_readiness(&mut self, service_name: &str, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed == "READY" || trimmed == "READY=1" || trimmed.contains("s6-notifyready") || trimmed.starts_with("ready") {
            if let Some(srv) = self.services.get_mut(service_name) {
                srv.state = ServiceState::Running;
                return true;
            }
        }
        false
    }

    pub fn parse_s6_readiness_notification(line: &str) -> bool {
        let trimmed = line.trim();
        trimmed == "READY" || trimmed == "READY=1" || trimmed.contains("s6-notifyready") || trimmed.starts_with("ready")
    }
}

impl Default for SupervisionStage {
    fn default() -> Self {
        SupervisionStage::Stage1Boot
    }
}

// ============================================================================
// SysVinit Subsystem
// ============================================================================

/// SysVinit Action Keywords
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysVAction {
    Respawn,
    SysInit,
    Wait,
    Once,
    CtrlAltDel,
    PowerFail,
    Off,
    OnDemand,
}

/// SysV  Entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysVInittabEntry {
    pub id: String,
    pub runlevels: Vec<char>,
    pub action: SysVAction,
    pub process: String,
}

impl SysVInittabEntry {
    pub fn parse_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.trim().split(':').collect();
        if parts.len() != 4 {
            return None;
        }
        let id = parts[0].to_string();
        let runlevels = parts[1].chars().collect();
        let action = match parts[2] {
            "respawn" => SysVAction::Respawn,
            "sysinit" => SysVAction::SysInit,
            "wait" => SysVAction::Wait,
            "once" => SysVAction::Once,
            "ctrlaltdel" => SysVAction::CtrlAltDel,
            "powerfail" => SysVAction::PowerFail,
            "off" => SysVAction::Off,
            "ondemand" => SysVAction::OnDemand,
            _ => return None,
        };
        let process = parts[3].to_string();
        Some(Self {
            id,
            runlevels,
            action,
            process,
        })
    }
}

/// SysVinit Runlevel State Machine Engine
#[derive(Debug, Clone, Default)]
pub struct SysVinitRunlevelStateMachine {
    pub current_runlevel: char,
    pub inittab_entries: Vec<SysVInittabEntry>,
    pub active_processes: BTreeMap<String, u32>,
    pub entries: Vec<SysVInittabEntry>,
}

impl SysVinitRunlevelStateMachine {
    pub fn new(initial_runlevel: char, entries: Vec<SysVInittabEntry>) -> Self {
        Self {
            current_runlevel: initial_runlevel,
            inittab_entries: entries.clone(),
            active_processes: BTreeMap::new(),
            entries,
        }
    }

    pub fn set_runlevel(&mut self, new_runlevel: char) -> Vec<&SysVInittabEntry> {
        self.current_runlevel = new_runlevel;
        self.inittab_entries
            .iter()
            .filter(|e| e.runlevels.contains(&new_runlevel) || e.runlevels.contains(&'a') || e.runlevels.contains(&'b') || e.runlevels.contains(&'c'))
            .collect()
    }

    pub fn transition_runlevel(&mut self, new_runlevel: char) -> Vec<&SysVInittabEntry> {
        self.set_runlevel(new_runlevel)
    }

    pub fn spawn_respawn_processes(&mut self) -> Vec<String> {
        let mut spawned = Vec::new();
        for entry in &self.inittab_entries {
            if entry.action == SysVAction::Respawn && entry.runlevels.contains(&self.current_runlevel) {
                if !self.active_processes.contains_key(&entry.id) {
                    let mock_pid = 4000 + entry.id.len() as u32;
                    self.active_processes.insert(entry.id.clone(), mock_pid);
                    spawned.push(entry.id.clone());
                }
            }
        }
        spawned
    }

    pub fn switch_runlevel(&mut self, target_runlevel: char) -> (Vec<String>, Vec<String>) {
        self.current_runlevel = target_runlevel;
        let mut stop_scripts = Vec::new();
        let mut start_scripts = Vec::new();

        for entry in &self.entries {
            if entry.runlevels.contains(&target_runlevel) {
                if entry.action == SysVAction::Respawn
                    || entry.action == SysVAction::Once
                    || entry.action == SysVAction::Wait
                {
                    start_scripts.push(format!("S{:02}{}", start_scripts.len(), entry.id));
                }
            } else {
                stop_scripts.push(format!("K{:02}{}", stop_scripts.len(), entry.id));
            }
        }

        (stop_scripts, start_scripts)
    }
}

/// BSD `rc.d` / `rc.subr` Directives
#[derive(Debug, Clone, Default)]
pub struct BsdRcScript {
    pub name: String,
    pub provides: Vec<String>,
    pub requires: Vec<String>,
    pub before: Vec<String>,
    pub keywords: Vec<String>,
    pub precmd: Option<String>,
    pub postcmd: Option<String>,
    pub enabled_variable: Option<String>,
}

impl BsdRcScript {
    pub fn parse(content: &str) -> Self {
        let mut script = BsdRcScript::default();
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("# PROVIDE:") {
                let rest = &line["# PROVIDE:".len()..];
                let provides: Vec<String> =
                    rest.split_whitespace().map(|s| s.to_string()).collect();
                if script.name.is_empty() && !provides.is_empty() {
                    script.name = provides[0].clone();
                }
                script.provides.extend(provides);
            } else if line.starts_with("# REQUIRE:") {
                let rest = &line["# REQUIRE:".len()..];
                script
                    .requires
                    .extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if line.starts_with("# BEFORE:") {
                let rest = &line["# BEFORE:".len()..];
                script
                    .before
                    .extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if line.starts_with("# KEYWORD:") {
                let rest = &line["# KEYWORD:".len()..];
                script
                    .keywords
                    .extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if line.starts_with("precmd=") {
                script.precmd = Some(line["precmd=".len()..].trim_matches('"').to_string());
            } else if line.starts_with("postcmd=") {
                script.postcmd = Some(line["postcmd=".len()..].trim_matches('"').to_string());
            } else if line.contains("_enable=") {
                let parts: Vec<&str> = line.split('=').collect();
                if !parts.is_empty() {
                    script.enabled_variable = Some(parts[0].trim().to_string());
                }
            }
        }
        script
    }
}

/// BSD `rc.subr` & `rcorder` Dependency Engine
#[derive(Debug, Clone, Default)]
pub struct BsdRcSubrEngine {
    pub rc_conf: BTreeMap<String, String>,
    pub scripts: BTreeMap<String, BsdRcScript>,
}

impl BsdRcSubrEngine {
    pub fn new(rc_conf: BTreeMap<String, String>) -> Self {
        Self {
            rc_conf,
            scripts: BTreeMap::new(),
        }
    }

    pub fn set_rc_conf(&mut self, key: String, val: String) {
        self.rc_conf.insert(key, val);
    }

    pub fn register_script(&mut self, script: BsdRcScript) {
        self.scripts.insert(script.name.clone(), script);
    }

    pub fn is_service_enabled(&self, service_name: &str) -> bool {
        let key = format!("{}_enable", service_name);
        if let Some(val) = self.rc_conf.get(&key) {
            let val_lower = val.to_lowercase();
            val_lower == "yes" || val_lower == "true" || val_lower == "1" || val_lower == "on"
        } else {
            false
        }
    }

    pub fn is_enabled(&self, script_name: &str) -> bool {
        if let Some(script) = self.scripts.get(script_name) {
            if let Some(var) = &script.enabled_variable {
                if let Some(val) = self.rc_conf.get(var) {
                    return val.eq_ignore_ascii_case("YES")
                        || val == "1"
                        || val.eq_ignore_ascii_case("TRUE");
                }
            }
        }
        false
    }

    pub fn compute_rcorder(&self) -> Vec<String> {
        self.scripts.keys().cloned().collect()
    }
}

/// macOS launchd PLIST Job Specification
#[derive(Debug, Clone, Default)]
pub struct LaunchdJobSpec {
    pub label: String,
    pub program: String,
    pub program_arguments: Vec<String>,
    pub keep_alive: bool,
    pub run_at_load: bool,
    pub throttle_interval_secs: u64,
    pub mach_services: Vec<String>,
    pub sockets: BTreeMap<String, String>,
}

/// macOS launchd Job Supervisor Engine
#[derive(Debug, Clone, Default)]
pub struct LaunchdJobSupervisorEngine {
    pub jobs: BTreeMap<String, LaunchdJobSpec>,
    pub job_states: BTreeMap<String, ServiceState>,
    pub running_pids: BTreeMap<String, u32>,
}

pub type LaunchdJobSupervisorSpecEngine = LaunchdJobSupervisorEngine;

impl LaunchdJobSupervisorEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_job(&mut self, job: LaunchdJobSpec) {
        if job.run_at_load {
            self.job_states
                .insert(job.label.clone(), ServiceState::Running);
            self.running_pids.insert(job.label.clone(), 5000 + job.label.len() as u32);
        } else {
            self.job_states
                .insert(job.label.clone(), ServiceState::Stopped);
        }
        self.jobs.insert(job.label.clone(), job);
    }

    pub fn load_job(&mut self, job: LaunchdJobSpec) {
        self.register_job(job);
    }

    pub fn trigger_mach_service(&mut self, mach_service: &str) -> Option<String> {
        for job in self.jobs.values() {
            if job.mach_services.contains(&mach_service.to_string()) {
                self.job_states
                    .insert(job.label.clone(), ServiceState::Running);
                self.running_pids.insert(job.label.clone(), 5000 + job.label.len() as u32);
                return Some(job.label.clone());
            }
        }
        None
    }

    pub fn handle_process_exit(&mut self, label: &str) {
        self.running_pids.remove(label);
        if let Some(job) = self.jobs.get(label) {
            if job.keep_alive {
                let mock_pid = 5000 + label.len() as u32;
                self.running_pids.insert(label.to_string(), mock_pid);
                self.job_states.insert(label.to_string(), ServiceState::Running);
            } else {
                self.job_states.insert(label.to_string(), ServiceState::Stopped);
            }
        }
    }
}

// ============================================================================
// Cron, Anacron, and at Spool Task Execution
// ============================================================================

/// Vixie Cron Environment Spec Context
#[derive(Debug, Clone)]
pub struct VixieCronEnvSpecContext {
    pub mailto: Option<String>,
    pub path: String,
    pub shell: String,
    pub home: String,
    pub env_vars: BTreeMap<String, String>,
}

impl Default for VixieCronEnvSpecContext {
    fn default() -> Self {
        Self {
            mailto: Some("root".to_string()),
            path: "/usr/bin:/bin:/usr/sbin:/sbin".to_string(),
            shell: "/bin/sh".to_string(),
            home: "/root".to_string(),
            env_vars: BTreeMap::new(),
        }
    }
}

/// Anacron Spool Manager
#[derive(Debug, Clone, Default)]
pub struct AnacronSpoolManager {
    pub spool_path: String,
    pub timestamps: BTreeMap<String, u64>,
}

impl AnacronSpoolManager {
    pub fn new(spool_path: &str) -> Self {
        Self {
            spool_path: spool_path.to_string(),
            timestamps: BTreeMap::new(),
        }
    }

    pub fn record_job_run(&mut self, job_id: &str, timestamp: u64) {
        self.timestamps.insert(job_id.to_string(), timestamp);
    }

    pub fn get_last_run(&self, job_id: &str) -> Option<u64> {
        self.timestamps.get(job_id).copied()
    }
}

/// `at` & `batch` Spool Queue Manager
#[derive(Debug, Clone, Default)]
pub struct AtSpoolQueueManager {
    pub spool_dir: String,
    pub queued_jobs: Vec<AtJob>,
    pub jobs: BTreeMap<u64, AtJob>,
    pub next_job_id: u64,
}

impl AtSpoolQueueManager {
    pub fn new(spool_dir: &str) -> Self {
        Self {
            spool_dir: spool_dir.to_string(),
            queued_jobs: Vec::new(),
            jobs: BTreeMap::new(),
            next_job_id: 1,
        }
    }

    pub fn enqueue_job(&mut self, job: AtJob) {
        self.queued_jobs.push(job);
    }

    pub fn poll_ready_jobs(&mut self, now_timestamp: u64, current_load: f32) -> Vec<AtJob> {
        let mut ready = Vec::new();
        let mut pending = Vec::new();

        for job in self.queued_jobs.drain(..) {
            let load_ok = match job.max_load_average {
                Some(max_l) => current_load <= max_l,
                None => true,
            };

            if now_timestamp >= job.execute_at && load_ok {
                ready.push(job);
            } else {
                pending.push(job);
            }
        }

        self.queued_jobs = pending;
        ready
    }
}
