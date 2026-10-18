#![no_std]
extern crate alloc;

extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet};
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
    Reloading,
}

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
        // Mock actual parallel spawn
        srv.pid = Some(1000 + name.len() as u32);
        srv.state = ServiceState::Running;
        srv.last_ping = 0; // set by current time

        Ok(())
    }

    pub fn check_watchdogs(&mut self, now: u64) {
        for srv in self.services.values_mut() {
            if srv.state == ServiceState::Running && srv.watchdog_usec > 0 {
                if now.saturating_sub(srv.last_ping) > srv.watchdog_usec {
                    srv.state = ServiceState::Failed;
                    srv.pid = None;
                    // Mock SIGKILL dispatch
                }
            }
        }
    }
}


// ============================================================================
// Linux & BSD Distro Core Userland Parity Extensions
// ============================================================================

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
#[derive(Debug, Clone)]
pub struct SystemdCgroupsv2Slice {
    pub slice_name: String,
    pub memory_max_bytes: Option<u64>,
    pub cpu_weight: Option<u32>,
    pub io_weight: Option<u32>,
    pub tasks_max: Option<u32>,
}

/// Systemd Socket Activation Engine
#[derive(Debug, Clone, Default)]
pub struct SystemdSocketActivationEngine {
    pub socket_name: String,
    pub listen_stream: Option<String>,
    pub listen_datagram: Option<String>,
    pub passed_fds: Vec<i32>,
    pub sockets: BTreeMap<String, SystemdSocketUnit>,
}

impl SystemdSocketActivationEngine {
    pub fn new(socket_name: &str) -> Self {
        Self {
            socket_name: socket_name.to_string(),
            listen_stream: None,
            listen_datagram: None,
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
}

/// Systemd Target Dependency Resolver
#[derive(Debug, Clone, Default)]
pub struct SystemdTargetDependencyResolver;

impl SystemdTargetDependencyResolver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SystemdTargetDependencyResolverSpec {
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
}

/// Systemd Path Event Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SystemdPathEventType {
    #[default]
    PathChanged,
    PathModified,
    PathExists,
    DirectoryNotEmpty,
}

/// Systemd Path Monitor Engine
#[derive(Debug, Clone)]
pub struct SystemdPathMonitorEngine {
    pub unit_name: String,
    pub monitored_path: String,
    pub event_type: Option<SystemdPathEventType>,
    pub unit_to_trigger: String,
    pub paths: BTreeMap<String, SystemdPathUnit>,
}

impl SystemdPathMonitorEngine {
    pub fn new() -> Self {
        Self::default()
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

#[derive(Debug, Clone, Default)]
pub struct SystemdTransientScopeGovernor {
    pub transient_scopes: BTreeMap<String, SystemdCgroupsv2Slice>,
}

impl SystemdTransientScopeGovernor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_scope(&mut self, scope_name: String, slice: SystemdCgroupsv2Slice) {
        self.transient_scopes.insert(scope_name.clone(), slice.clone());
        self.active_scopes.insert(scope_name, slice);
    }

    pub fn register_transient_scope(&mut self, scope_name: &str, slice: SystemdCgroupsv2Slice) {
        self.transient_scopes.insert(scope_name.to_string(), slice.clone());
        self.active_scopes.insert(scope_name.to_string(), slice);
    }

    pub fn get_scope_limit(&self, scope_name: &str) -> Option<&SystemdCgroupsv2Slice> {
        self.transient_scopes
            .get(scope_name)
            .or_else(|| self.active_scopes.get(scope_name))
    }
}

/// Systemd OnCalendar Event Specification Parser
#[derive(Debug, Clone, Default)]
pub struct SystemdOnCalendarExpressionParser;

pub type SystemdOnCalendarParser = SystemdOnCalendarExpressionParser;

impl SystemdOnCalendarExpressionParser {
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
        // General wildcard or standard format parsing
        true
    }

    pub fn parse_expression(expr: &str) -> Option<(u32, u32)> {
        let trimmed = expr.trim();
        if trimmed == "daily"
            || trimmed == "*-*-* 00:00:00"
            || trimmed == "weekly"
            || trimmed == "hourly"
        {
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





/// OpenRC Multi-Stage Runlevel
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpenRcRunlevel {
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
    pub keywords: Vec<String>, // e.g. "keyword -docker -jail"
}

impl OpenRcHeader {
    pub fn parse(script_content: &str) -> Self {
        let mut header = OpenRcHeader::default();
        for line in script_content.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("provide") {
                header
                    .provide
                    .extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("need") {
                header
                    .need
                    .extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("use") {
                header
                    .use_deps
                    .extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("before") {
                header
                    .before
                    .extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("after") {
                header
                    .after
                    .extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("keyword") {
                header
                    .keywords
                    .extend(rest.split_whitespace().map(|s| s.to_string()));
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
    pub active_services: BTreeSet<String>,
    pub service_statuses: BTreeMap<String, ServiceState>,
}

pub type OpenRcDependencySolverSpecEngine = OpenRcDependencySolverEngine;

impl OpenRcDependencySolverEngine {
    pub fn new() -> Self {
        Self::default()
    }

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

    pub fn register_script(&mut self, name: String, header: OpenRcHeader) {
        self.scripts.insert(name, header);
    }

    pub fn transition_runlevel(&mut self, target: OpenRcRunlevel) -> Vec<String> {
        self.current_runlevel = target;
        // Collect scripts for this runlevel order
        self.scripts.keys().cloned().collect()
    }
}

/// OpenRC Status & Runlevel Controller Engine
#[derive(Debug, Clone, Default)]
pub struct OpenRcStatusControllerEngine {
    pub current_runlevel: OpenRcRunlevel,
    pub active_services: BTreeSet<String>,
    pub service_statuses: BTreeMap<String, ServiceState>,
}

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
    }

    pub fn mark_stopped(&mut self, service_name: &str) {
        self.active_services.remove(service_name);
    }
}
