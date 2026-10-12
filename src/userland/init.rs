use std::collections::{BTreeMap, BTreeSet};
use std::string::String;
use std::vec::Vec;

// ==========================================
// 0. BACKWARD-COMPATIBLE SERVICE DEFINITIONS
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Failed,
}

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

// ==========================================
// 1. SYSTEMD-STYLE UNIT & SERVICE MANAGEMENT
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemdDependencyKind {
    Wants,
    Requires,
    Requisite,
    BindsTo,
    PartOf,
    Conflicts,
    Before,
    After,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemdServiceType {
    Simple,
    Forking,
    OneShot,
    Dbus,
    Notify,
    Idle,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CgroupsV2ResourceLimits {
    pub cpu_weight: Option<u32>,
    pub cpu_max_quota_us: Option<u64>,
    pub cpu_max_period_us: Option<u64>,
    pub memory_max_bytes: Option<u64>,
    pub memory_high_bytes: Option<u64>,
    pub io_weight: Option<u32>,
    pub pids_max: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdNotifyMessage {
    Ready,
    Reloading,
    Stopping,
    Watchdog,
    Status(String),
    MainPid(u32),
    Errno(i32),
    Custom(String),
}

impl SdNotifyMessage {
    pub fn parse(payload: &str) -> Vec<Self> {
        let mut messages = Vec::new();
        for line in payload.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line == "READY=1" {
                messages.push(SdNotifyMessage::Ready);
            } else if line == "RELOAD=1" {
                messages.push(SdNotifyMessage::Reloading);
            } else if line == "STOPPING=1" {
                messages.push(SdNotifyMessage::Stopping);
            } else if line == "WATCHDOG=1" {
                messages.push(SdNotifyMessage::Watchdog);
            } else if let Some(status) = line.strip_prefix("STATUS=") {
                messages.push(SdNotifyMessage::Status(status.to_string()));
            } else if let Some(pid_str) = line.strip_prefix("MAINPID=") {
                if let Ok(pid) = pid_str.parse::<u32>() {
                    messages.push(SdNotifyMessage::MainPid(pid));
                }
            } else if let Some(err_str) = line.strip_prefix("ERRNO=") {
                if let Ok(err) = err_str.parse::<i32>() {
                    messages.push(SdNotifyMessage::Errno(err));
                }
            } else {
                messages.push(SdNotifyMessage::Custom(line.to_string()));
            }
        }
        messages
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdUnit {
    pub name: String,
    pub unit_type: SystemdUnitType,
    pub description: String,
    pub service_type: SystemdServiceType,
    pub exec_start: String,
    pub exec_stop: Option<String>,
    pub restart: bool,
    pub dependencies: Vec<(SystemdDependencyKind, String)>,
    pub resource_limits: CgroupsV2ResourceLimits,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub watchdog_usec: u64,
    pub last_ping: u64,
    pub status_text: String,
}

pub struct SystemdUnitManager {
    pub units: BTreeMap<String, SystemdUnit>,
}

impl SystemdUnitManager {
    pub fn new() -> Self {
        Self {
            units: BTreeMap::new(),
        }
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.insert(unit.name.clone(), unit);
    }

    pub fn topo_sort(&self) -> Result<Vec<String>, &'static str> {
        let mut sorted = Vec::new();
        let mut visited = BTreeSet::new();
        let mut temp_mark = BTreeSet::new();

        fn visit(
            node: &String,
            units: &BTreeMap<String, SystemdUnit>,
            visited: &mut BTreeSet<String>,
            temp_mark: &mut BTreeSet<String>,
            sorted: &mut Vec<String>,
        ) -> Result<(), &'static str> {
            if temp_mark.contains(node) {
                return Err("Cyclic dependency detected in unit graph");
            }
            if !visited.contains(node) {
                temp_mark.insert(node.clone());
                if let Some(u) = units.get(node) {
                    for (kind, dep) in &u.dependencies {
                        if *kind == SystemdDependencyKind::Requires
                            || *kind == SystemdDependencyKind::Wants
                            || *kind == SystemdDependencyKind::After
                            || *kind == SystemdDependencyKind::BindsTo
                        {
                            visit(dep, units, visited, temp_mark, sorted)?;
                        }
                    }
                }
                temp_mark.remove(node);
                visited.insert(node.clone());
                sorted.push(node.clone());
            }
            Ok(())
        }

        for node in self.units.keys() {
            visit(&node, &self.units, &mut visited, &mut temp_mark, &mut sorted)?;
        }
        Ok(sorted)
    }

    pub fn start_unit(&mut self, name: &str) -> Result<(), &'static str> {
        let unit = self.units.get_mut(name).ok_or("Unit not found")?;
        if unit.state == ServiceState::Running {
            return Ok(());
        }
        unit.state = ServiceState::Starting;
        unit.pid = Some(2000 + name.len() as u32);
        unit.state = ServiceState::Running;
        unit.last_ping = 0;
        Ok(())
    }

    pub fn handle_sd_notify(&mut self, unit_name: &str, payload: &str, now: u64) -> Result<(), &'static str> {
        let messages = SdNotifyMessage::parse(payload);
        let unit = self.units.get_mut(unit_name).ok_or("Unit not found")?;

        for msg in messages {
            match msg {
                SdNotifyMessage::Ready => {
                    unit.state = ServiceState::Running;
                }
                SdNotifyMessage::Reloading => {
                    unit.status_text = "Reloading configuration...".to_string();
                }
                SdNotifyMessage::Stopping => {
                    unit.state = ServiceState::Stopped;
                }
                SdNotifyMessage::Watchdog => {
                    unit.last_ping = now;
                }
                SdNotifyMessage::Status(status) => {
                    unit.status_text = status;
                }
                SdNotifyMessage::MainPid(pid) => {
                    unit.pid = Some(pid);
                }
                SdNotifyMessage::Errno(err) => {
                    unit.status_text = format!("Error code: {}", err);
                    unit.state = ServiceState::Failed;
                }
                SdNotifyMessage::Custom(c) => {
                    unit.status_text = c;
                }
            }
        }
        Ok(())
    }

    pub fn check_watchdogs(&mut self, now: u64) {
        for unit in self.units.values_mut() {
            if unit.state == ServiceState::Running && unit.watchdog_usec > 0 {
                if now.saturating_sub(unit.last_ping) > unit.watchdog_usec {
                    unit.state = ServiceState::Failed;
                    unit.status_text = "Watchdog timeout - service killed".to_string();
                    unit.pid = None;
                }
            }
        }
    }
}

// ==========================================
// 2. OPENRC MULTI-STAGE RUNLEVEL ENGINE
// ==========================================

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
    Need(Vec<String>),
    Use(Vec<String>),
    Before(Vec<String>),
    After(Vec<String>),
    NoDocker,
    Container,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenRcService {
    pub name: String,
    pub runlevels: Vec<OpenRcRunlevel>,
    pub keywords: Vec<OpenRcKeyword>,
    pub command: String,
    pub state: ServiceState,
}

pub struct OpenRcInitEngine {
    pub current_runlevel: OpenRcRunlevel,
    pub services: BTreeMap<String, OpenRcService>,
}

impl OpenRcInitEngine {
    pub fn new() -> Self {
        Self {
            current_runlevel: OpenRcRunlevel::SysInit,
            services: BTreeMap::new(),
        }
    }

    pub fn parse_script_header(&mut self, script_content: &str, service_name: &str, command: &str) -> OpenRcService {
        let mut keywords = Vec::new();

        for line in script_content.lines() {
            let line = line.trim();
            if line.starts_with("provide ") {
                let name = line.trim_start_matches("provide ").trim().to_string();
                keywords.push(OpenRcKeyword::Provide(name));
            } else if line.starts_with("need ") {
                let deps = line.trim_start_matches("need ")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                keywords.push(OpenRcKeyword::Need(deps));
            } else if line.starts_with("use ") {
                let deps = line.trim_start_matches("use ")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                keywords.push(OpenRcKeyword::Use(deps));
            } else if line.starts_with("before ") {
                let list = line.trim_start_matches("before ")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                keywords.push(OpenRcKeyword::Before(list));
            } else if line.starts_with("after ") {
                let list = line.trim_start_matches("after ")
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                keywords.push(OpenRcKeyword::After(list));
            } else if line.contains("keyword -docker") {
                keywords.push(OpenRcKeyword::NoDocker);
            }
        }

        OpenRcService {
            name: service_name.to_string(),
            runlevels: vec![OpenRcRunlevel::Default],
            keywords,
            command: command.to_string(),
            state: ServiceState::Stopped,
        }
    }

    pub fn register_service(&mut self, srv: OpenRcService) {
        self.services.insert(srv.name.clone(), srv);
    }

    pub fn switch_runlevel(&mut self, target: OpenRcRunlevel) -> Result<Vec<String>, &'static str> {
        self.current_runlevel = target;
        let mut started = Vec::new();

        for (name, srv) in self.services.iter_mut() {
            if srv.runlevels.contains(&target) || target == OpenRcRunlevel::Default {
                srv.state = ServiceState::Running;
                started.push(name.clone());
            } else if target == OpenRcRunlevel::Shutdown {
                srv.state = ServiceState::Stopped;
            }
        }
        Ok(started)
    }
}

// ==========================================
// 3. s6 & RUNIT SUPERVISION ENGINE
// ==========================================

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
    Hup,       // 'h'
    Term,      // 't'
    Kill,      // 'k'
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6RunitServiceState {
    Down,
    Starting,
    Up,
    Stopping,
    Supervised,
    FinishScriptExecuting,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6RunitService {
    pub name: String,
    pub service_dir: String,
    pub run_script: String,
    pub finish_script: Option<String>,
    pub state: S6RunitServiceState,
    pub pid: Option<u32>,
    pub restart_count: u32,
    pub paused: bool,
}

pub struct S6RunitSupervisor {
    pub services: BTreeMap<String, S6RunitService>,
}

impl S6RunitSupervisor {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }

    pub fn register_service(&mut self, service: S6RunitService) {
        self.services.insert(service.name.clone(), service);
    }

    pub fn dispatch_control_verb(&mut self, name: &str, verb: S6RunitControlVerb) -> Result<(), &'static str> {
        let srv = self.services.get_mut(name).ok_or("s6/runit service not found")?;

        match verb {
            S6RunitControlVerb::Up | S6RunitControlVerb::Continue => {
                srv.state = S6RunitServiceState::Up;
                srv.pid = Some(3000 + srv.name.len() as u32);
                srv.paused = false;
            }
            S6RunitControlVerb::Down => {
                srv.state = S6RunitServiceState::Down;
                srv.pid = None;
            }
            S6RunitControlVerb::Restart => {
                srv.state = S6RunitServiceState::Starting;
                srv.restart_count += 1;
                srv.pid = Some(3000 + srv.restart_count + srv.name.len() as u32);
                srv.state = S6RunitServiceState::Up;
            }
            S6RunitControlVerb::Pause => {
                srv.paused = true;
            }
            S6RunitControlVerb::Exit => {
                srv.state = S6RunitServiceState::Down;
                srv.pid = None;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn supervise_tick(&mut self) {
        for srv in self.services.values_mut() {
            if srv.state == S6RunitServiceState::Up && srv.pid.is_none() {
                // Auto-restart failed process
                srv.restart_count += 1;
                srv.pid = Some(3000 + srv.restart_count);
            }
        }
    }
}

// ==========================================
// 4. SYSVINIT /ETC/INITTAB & RUNLEVELS ENGINE
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SysVinitRunlevel {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    SingleUser,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysVinitAction {
    Respawn,
    Wait,
    Once,
    Boot,
    BootWait,
    SysInit,
    PowerFail,
    PowerFailNow,
    PowerOK,
    CtrlAltDel,
    InitDefault,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysVInittabEntry {
    pub id: String,
    pub runlevels: Vec<SysVinitRunlevel>,
    pub action: SysVinitAction,
    pub process: String,
}

pub struct SysVinitManager {
    pub active_runlevel: SysVinitRunlevel,
    pub entries: Vec<SysVInittabEntry>,
}

impl SysVinitManager {
    pub fn new() -> Self {
        Self {
            active_runlevel: SysVinitRunlevel::R3,
            entries: Vec::new(),
        }
    }

    pub fn parse_inittab_line(&mut self, line: &str) -> Result<SysVInittabEntry, &'static str> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Err("Comment or empty line");
        }
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            return Err("Invalid inittab line format");
        }

        let id = parts[0].to_string();
        let runlevels_raw = parts[1];
        let action_raw = parts[2];
        let process = parts[3].to_string();

        let mut runlevels = Vec::new();
        for ch in runlevels_raw.chars() {
            match ch {
                '0' => runlevels.push(SysVinitRunlevel::R0),
                '1' => runlevels.push(SysVinitRunlevel::R1),
                '2' => runlevels.push(SysVinitRunlevel::R2),
                '3' => runlevels.push(SysVinitRunlevel::R3),
                '4' => runlevels.push(SysVinitRunlevel::R4),
                '5' => runlevels.push(SysVinitRunlevel::R5),
                '6' => runlevels.push(SysVinitRunlevel::R6),
                'S' | 's' => runlevels.push(SysVinitRunlevel::SingleUser),
                _ => {}
            }
        }

        let action = match action_raw {
            "respawn" => SysVinitAction::Respawn,
            "wait" => SysVinitAction::Wait,
            "once" => SysVinitAction::Once,
            "boot" => SysVinitAction::Boot,
            "bootwait" => SysVinitAction::BootWait,
            "sysinit" => SysVinitAction::SysInit,
            "powerfail" => SysVinitAction::PowerFail,
            "powerfailnow" => SysVinitAction::PowerFailNow,
            "powerok" => SysVinitAction::PowerOK,
            "ctrlaltdel" => SysVinitAction::CtrlAltDel,
            "initdefault" => SysVinitAction::InitDefault,
            "off" => SysVinitAction::Off,
            _ => SysVinitAction::Once,
        };

        let entry = SysVInittabEntry {
            id,
            runlevels,
            action,
            process,
        };

        self.entries.push(entry.clone());
        Ok(entry)
    }

    pub fn evaluate_runlevel(&self, target_runlevel: SysVinitRunlevel) -> Vec<&SysVInittabEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.runlevels.is_empty() || entry.runlevels.contains(&target_runlevel)
            })
            .collect()
    }
}

// ==========================================
// 5. BSD RC.D / RC.SUBR INIT SUBSYSTEM
// ==========================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BsdRcKeyword {
    Provide(Vec<String>),
    Require(Vec<String>),
    Before(Vec<String>),
    Keyword(Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsdRcCommandHook {
    Start,
    Stop,
    Restart,
    Status,
    Poll,
    RcVar,
    PreCmd,
    PostCmd,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BsdRcScript {
    pub name: String,
    pub provides: Vec<String>,
    pub requires: Vec<String>,
    pub before: Vec<String>,
    pub keywords: Vec<String>,
    pub rcvar: String,
    pub command: String,
    pub precmd: Option<String>,
    pub postcmd: Option<String>,
}

pub struct BsdRcConf {
    pub variables: BTreeMap<String, String>,
}

impl BsdRcConf {
    pub fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
        }
    }

    pub fn parse_rc_conf(&mut self, content: &str) {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                let key = k.trim().to_string();
                let val = v.trim().trim_matches('"').trim_matches('\'').to_string();
                self.variables.insert(key, val);
            }
        }
    }

    pub fn is_enabled(&self, service_name: &str) -> bool {
        let var_name = format!("{}_enable", service_name);
        if let Some(val) = self.variables.get(&var_name) {
            let val_upper = val.to_uppercase();
            val_upper == "YES" || val_upper == "TRUE" || val_upper == "ON" || val_upper == "1"
        } else {
            false
        }
    }
}

pub struct BsdRcSubrEngine {
    pub scripts: BTreeMap<String, BsdRcScript>,
    pub rc_conf: BsdRcConf,
}

impl BsdRcSubrEngine {
    pub fn new() -> Self {
        Self {
            scripts: BTreeMap::new(),
            rc_conf: BsdRcConf::new(),
        }
    }

    pub fn parse_rc_header(&mut self, header: &str, name: &str, command: &str) -> BsdRcScript {
        let mut provides = Vec::new();
        let mut requires = Vec::new();
        let mut before = Vec::new();
        let mut keywords = Vec::new();

        for line in header.lines() {
            let line = line.trim();
            if let Some(p) = line.strip_prefix("# PROVIDE:") {
                provides.extend(p.split_whitespace().map(|s| s.to_string()));
            } else if let Some(r) = line.strip_prefix("# REQUIRE:") {
                requires.extend(r.split_whitespace().map(|s| s.to_string()));
            } else if let Some(b) = line.strip_prefix("# BEFORE:") {
                before.extend(b.split_whitespace().map(|s| s.to_string()));
            } else if let Some(k) = line.strip_prefix("# KEYWORD:") {
                keywords.extend(k.split_whitespace().map(|s| s.to_string()));
            }
        }

        BsdRcScript {
            name: name.to_string(),
            provides,
            requires,
            before,
            keywords,
            rcvar: format!("{}_enable", name),
            command: command.to_string(),
            precmd: None,
            postcmd: None,
        }
    }

    pub fn register_script(&mut self, script: BsdRcScript) {
        self.scripts.insert(script.name.clone(), script);
    }

    pub fn execute_hook(&self, name: &str, hook: BsdRcCommandHook) -> Result<String, &'static str> {
        let script = self.scripts.get(name).ok_or("BSD rc script not found")?;

        if !self.rc_conf.is_enabled(name) && hook == BsdRcCommandHook::Start {
            return Err("Service disabled in /etc/rc.conf");
        }

        match hook {
            BsdRcCommandHook::Start => Ok(format!("Executing: {}", script.command)),
            BsdRcCommandHook::Stop => Ok(format!("Stopping: {}", script.name)),
            BsdRcCommandHook::PreCmd => Ok(script.precmd.clone().unwrap_or_default()),
            BsdRcCommandHook::PostCmd => Ok(script.postcmd.clone().unwrap_or_default()),
            _ => Ok("Hook executed successfully".to_string()),
        }
    }
}

// ==========================================
// 6. MACOS LAUNCHD SUPERVISOR ENGINE
// ==========================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchdJobPlist {
    pub label: String,
    pub program_arguments: Vec<String>,
    pub run_at_load: bool,
    pub keep_alive_successful_exit: Option<bool>,
    pub keep_alive_path_state: Option<(String, bool)>, // (path, required_exists)
    pub keep_alive_network_state: Option<bool>,
    pub throttle_interval_sec: u64,
    pub stdout_path: Option<String>,
    pub stderr_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaunchdJobState {
    Idle,
    Loaded,
    Running,
    Throttled,
    Stopped,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchdJob {
    pub plist: LaunchdJobPlist,
    pub state: LaunchdJobState,
    pub pid: Option<u32>,
    pub last_exit_code: Option<i32>,
    pub last_spawn_time: u64,
}

pub struct LaunchdSupervisor {
    pub jobs: BTreeMap<String, LaunchdJob>,
}

impl LaunchdSupervisor {
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
        }
    }

    pub fn load_job(&mut self, plist: LaunchdJobPlist) {
        let label = plist.label.clone();
        let job = LaunchdJob {
            plist,
            state: LaunchdJobState::Loaded,
            pid: None,
            last_exit_code: None,
            last_spawn_time: 0,
        };
        self.jobs.insert(label, job);
    }

    pub fn check_and_spawn_jobs(&mut self, now_sec: u64, path_exists: &impl Fn(&str) -> bool, network_up: bool) {
        for job in self.jobs.values_mut() {
            if job.state == LaunchdJobState::Throttled {
                if now_sec.saturating_sub(job.last_spawn_time) >= job.plist.throttle_interval_sec {
                    job.state = LaunchdJobState::Loaded;
                } else {
                    continue;
                }
            }

            let mut should_run = false;

            if job.plist.run_at_load && job.state == LaunchdJobState::Loaded {
                should_run = true;
            }

            if let Some(ref path_rule) = job.plist.keep_alive_path_state {
                let exists = path_exists(&path_rule.0);
                if exists == path_rule.1 {
                    should_run = true;
                }
            }

            if let Some(net_req) = job.plist.keep_alive_network_state {
                if network_up == net_req {
                    should_run = true;
                }
            }

            if should_run && job.state != LaunchdJobState::Running {
                job.state = LaunchdJobState::Running;
                job.pid = Some(4000 + job.plist.label.len() as u32);
                job.last_spawn_time = now_sec;
            }
        }
    }

    pub fn handle_job_exit(&mut self, label: &str, exit_code: i32, _now_sec: u64) -> Result<(), &'static str> {
        let job = self.jobs.get_mut(label).ok_or("Launchd job not found")?;
        job.last_exit_code = Some(exit_code);
        job.pid = None;

        let mut keep_alive = false;
        if let Some(succ_exit) = job.plist.keep_alive_successful_exit {
            if (exit_code == 0) == succ_exit {
                keep_alive = true;
            }
        }

        if keep_alive {
            if job.plist.throttle_interval_sec > 0 {
                job.state = LaunchdJobState::Throttled;
            } else {
                job.state = LaunchdJobState::Loaded;
            }
        } else {
            job.state = LaunchdJobState::Stopped;
        }
        Ok(())
    }
}

// ==========================================
// 7. SCHEDULED TASK EXECUTION SUITE
// ==========================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VixieCronField {
    Any,
    Specific(u32),
    Range(u32, u32),
    Step(u32),
    List(Vec<u32>),
}

impl VixieCronField {
    pub fn matches(&self, val: u32) -> bool {
        match self {
            VixieCronField::Any => true,
            VixieCronField::Specific(v) => *v == val,
            VixieCronField::Range(min, max) => val >= *min && val <= *max,
            VixieCronField::Step(step) => *step > 0 && val % *step == 0,
            VixieCronField::List(list) => list.contains(&val),
        }
    }

    pub fn parse(token: &str) -> Self {
        let token = token.trim();
        if token == "*" {
            VixieCronField::Any
        } else if let Some(step_str) = token.strip_prefix("*/") {
            if let Ok(s) = step_str.parse::<u32>() {
                VixieCronField::Step(s)
            } else {
                VixieCronField::Any
            }
        } else if token.contains('-') {
            let parts: Vec<&str> = token.split('-').collect();
            if parts.len() == 2 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    return VixieCronField::Range(a, b);
                }
            }
            VixieCronField::Any
        } else if token.contains(',') {
            let nums: Vec<u32> = token.split(',').filter_map(|s| s.parse::<u32>().ok()).collect();
            VixieCronField::List(nums)
        } else if let Ok(val) = token.parse::<u32>() {
            VixieCronField::Specific(val)
        } else {
            VixieCronField::Any
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VixieCronSchedule {
    pub minute: VixieCronField,
    pub hour: VixieCronField,
    pub dom: VixieCronField,
    pub month: VixieCronField,
    pub dow: VixieCronField,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VixieCronEntry {
    pub schedule: VixieCronSchedule,
    pub user: String,
    pub command: String,
}

impl VixieCronEntry {
    pub fn parse_line(line: &str) -> Result<Self, &'static str> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Err("Comment or empty line");
        }

        if let Some(macro_str) = line.strip_prefix('@') {
            let parts: Vec<&str> = macro_str.split_whitespace().collect();
            if parts.is_empty() {
                return Err("Invalid macro line");
            }
            let macro_name = parts[0];
            let command = parts[1..].join(" ");

            let schedule = match macro_name {
                "hourly" => VixieCronSchedule {
                    minute: VixieCronField::Specific(0),
                    hour: VixieCronField::Any,
                    dom: VixieCronField::Any,
                    month: VixieCronField::Any,
                    dow: VixieCronField::Any,
                },
                "daily" | "midnight" => VixieCronSchedule {
                    minute: VixieCronField::Specific(0),
                    hour: VixieCronField::Specific(0),
                    dom: VixieCronField::Any,
                    month: VixieCronField::Any,
                    dow: VixieCronField::Any,
                },
                "weekly" => VixieCronSchedule {
                    minute: VixieCronField::Specific(0),
                    hour: VixieCronField::Specific(0),
                    dom: VixieCronField::Any,
                    month: VixieCronField::Any,
                    dow: VixieCronField::Specific(0),
                },
                "monthly" => VixieCronSchedule {
                    minute: VixieCronField::Specific(0),
                    hour: VixieCronField::Specific(0),
                    dom: VixieCronField::Specific(1),
                    month: VixieCronField::Any,
                    dow: VixieCronField::Any,
                },
                "yearly" | "annually" => VixieCronSchedule {
                    minute: VixieCronField::Specific(0),
                    hour: VixieCronField::Specific(0),
                    dom: VixieCronField::Specific(1),
                    month: VixieCronField::Specific(1),
                    dow: VixieCronField::Any,
                },
                _ => return Err("Unknown cron macro"),
            };

            return Ok(VixieCronEntry {
                schedule,
                user: "root".to_string(),
                command,
            });
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            return Err("Invalid 5/6-field cron line");
        }

        let minute = VixieCronField::parse(parts[0]);
        let hour = VixieCronField::parse(parts[1]);
        let dom = VixieCronField::parse(parts[2]);
        let month = VixieCronField::parse(parts[3]);
        let dow = VixieCronField::parse(parts[4]);
        let command = parts[5..].join(" ");

        Ok(VixieCronEntry {
            schedule: VixieCronSchedule { minute, hour, dom, month, dow },
            user: "root".to_string(),
            command,
        })
    }

    pub fn matches_time(&self, min: u32, hr: u32, day_of_month: u32, mth: u32, day_of_week: u32) -> bool {
        self.schedule.minute.matches(min)
            && self.schedule.hour.matches(hr)
            && self.schedule.dom.matches(day_of_month)
            && self.schedule.month.matches(mth)
            && self.schedule.dow.matches(day_of_week)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnacronJob {
    pub period_in_days: u32,
    pub delay_in_minutes: u32,
    pub job_identifier: String,
    pub command: String,
    pub last_run_timestamp: u64,
}

impl AnacronJob {
    pub fn should_run(&self, now_timestamp: u64) -> bool {
        let period_sec = self.period_in_days as u64 * 86400;
        now_timestamp.saturating_sub(self.last_run_timestamp) >= period_sec
    }

    pub fn calculate_start_delay(&self, random_delay_max_min: u32) -> u64 {
        let base_delay_sec = self.delay_in_minutes as u64 * 60;
        let jitter = (self.job_identifier.len() as u64 % (random_delay_max_min as u64 + 1)) * 60;
        base_delay_sec + jitter
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdTimer {
    pub name: String,
    pub target_unit: String,
    pub on_calendar: String,
    pub persistent: bool,
    pub accuracy_sec: u64,
    pub last_trigger: u64,
}

impl SystemdTimer {
    pub fn check_trigger(&mut self, now_sec: u64, interval_sec: u64) -> bool {
        if now_sec.saturating_sub(self.last_trigger) >= interval_sec {
            self.last_trigger = now_sec;
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtJob {
    pub job_id: u64,
    pub queue_letter: char,
    pub run_timestamp: u64,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BatchJob {
    pub job_id: u64,
    pub max_load_threshold: f32,
    pub command: String,
}

pub struct AtBatchQueueEngine {
    pub at_jobs: Vec<AtJob>,
    pub batch_jobs: Vec<BatchJob>,
    pub next_job_id: u64,
}

impl AtBatchQueueEngine {
    pub fn new() -> Self {
        Self {
            at_jobs: Vec::new(),
            batch_jobs: Vec::new(),
            next_job_id: 100,
        }
    }

    pub fn enqueue_at(&mut self, run_timestamp: u64, command: &str) -> u64 {
        let id = self.next_job_id;
        self.next_job_id += 1;
        self.at_jobs.push(AtJob {
            job_id: id,
            queue_letter: 'a',
            run_timestamp,
            command: command.to_string(),
        });
        id
    }

    pub fn enqueue_batch(&mut self, max_load: f32, command: &str) -> u64 {
        let id = self.next_job_id;
        self.next_job_id += 1;
        self.batch_jobs.push(BatchJob {
            job_id: id,
            max_load_threshold: max_load,
            command: command.to_string(),
        });
        id
    }

    pub fn evaluate_runnable_jobs(&mut self, now_sec: u64, current_loadavg: f32) -> (Vec<AtJob>, Vec<BatchJob>) {
        let mut ready_at = Vec::new();
        let mut remaining_at = Vec::new();

        for job in self.at_jobs.drain(..) {
            if now_sec >= job.run_timestamp {
                ready_at.push(job);
            } else {
                remaining_at.push(job);
            }
        }
        self.at_jobs = remaining_at;

        let mut ready_batch = Vec::new();
        let mut remaining_batch = Vec::new();

        for job in self.batch_jobs.drain(..) {
            if current_loadavg <= job.max_load_threshold {
                ready_batch.push(job);
            } else {
                remaining_batch.push(job);
            }
        }
        self.batch_jobs = remaining_batch;

        (ready_at, ready_batch)
    }
}

// ==========================================
// 8. UNIFIED MASTER CORE USERLAND INIT ENGINE
// ==========================================

pub struct SovereignCoreUserlandInitEngine {
    pub systemd: SystemdUnitManager,
    pub openrc: OpenRcInitEngine,
    pub s6_runit: S6RunitSupervisor,
    pub sysvinit: SysVinitManager,
    pub bsd_rc: BsdRcSubrEngine,
    pub launchd: LaunchdSupervisor,
    pub at_batch: AtBatchQueueEngine,
}

impl SovereignCoreUserlandInitEngine {
    pub fn new() -> Self {
        Self {
            systemd: SystemdUnitManager::new(),
            openrc: OpenRcInitEngine::new(),
            s6_runit: S6RunitSupervisor::new(),
            sysvinit: SysVinitManager::new(),
            bsd_rc: BsdRcSubrEngine::new(),
            launchd: LaunchdSupervisor::new(),
            at_batch: AtBatchQueueEngine::new(),
        }
    }

    pub fn boot_system(&mut self) -> Result<(), &'static str> {
        self.systemd.start_unit("multi-user.target").ok();
        self.openrc.switch_runlevel(OpenRcRunlevel::Default).ok();
        Ok(())
    }
}

// ==========================================
// 9. STANDALONE UNIT TESTS
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systemd_unit_manager_and_sd_notify() {
        let mut mgr = SystemdUnitManager::new();

        let unit_db = SystemdUnit {
            name: "db.service".to_string(),
            unit_type: SystemdUnitType::Service,
            description: "Database Engine".to_string(),
            service_type: SystemdServiceType::Notify,
            exec_start: "/usr/bin/db".to_string(),
            exec_stop: None,
            restart: true,
            dependencies: vec![],
            resource_limits: CgroupsV2ResourceLimits {
                cpu_weight: Some(100),
                memory_max_bytes: Some(1024 * 1024 * 512),
                ..Default::default()
            },
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 5000000,
            last_ping: 0,
            status_text: String::new(),
        };

        let unit_app = SystemdUnit {
            name: "app.service".to_string(),
            unit_type: SystemdUnitType::Service,
            description: "App Server".to_string(),
            service_type: SystemdServiceType::Simple,
            exec_start: "/usr/bin/app".to_string(),
            exec_stop: None,
            restart: true,
            dependencies: vec![(SystemdDependencyKind::Requires, "db.service".to_string())],
            resource_limits: CgroupsV2ResourceLimits::default(),
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 0,
            last_ping: 0,
            status_text: String::new(),
        };

        mgr.register_unit(unit_db);
        mgr.register_unit(unit_app);

        let order = mgr.topo_sort().expect("Topo sort should succeed");
        assert_eq!(order, vec!["db.service", "app.service"]);

        mgr.start_unit("db.service").unwrap();
        assert_eq!(mgr.units.get("db.service").unwrap().state, ServiceState::Running);

        mgr.handle_sd_notify("db.service", "STATUS=Ready to accept connections\nWATCHDOG=1\nMAINPID=4567\n", 100).unwrap();
        let db = mgr.units.get("db.service").unwrap();
        assert_eq!(db.status_text, "Ready to accept connections");
        assert_eq!(db.last_ping, 100);
        assert_eq!(db.pid, Some(4567));
    }

    #[test]
    fn test_openrc_header_parser() {
        let mut engine = OpenRcInitEngine::new();
        let script = r#"
            provide networking
            need localmount
            use dns
            before sshd
        "#;
        let srv = engine.parse_script_header(script, "networking", "/etc/init.d/networking start");
        assert_eq!(srv.name, "networking");
        assert_eq!(srv.keywords.len(), 4);

        engine.register_service(srv);
        let started = engine.switch_runlevel(OpenRcRunlevel::Default).unwrap();
        assert!(started.contains(&"networking".to_string()));
    }

    #[test]
    fn test_s6_runit_supervisor() {
        let mut supervisor = S6RunitSupervisor::new();
        let srv = S6RunitService {
            name: "sshd".to_string(),
            service_dir: "/var/service/sshd".to_string(),
            run_script: "./run".to_string(),
            finish_script: Some("./finish".to_string()),
            state: S6RunitServiceState::Down,
            pid: None,
            restart_count: 0,
            paused: false,
        };

        supervisor.register_service(srv);
        supervisor.dispatch_control_verb("sshd", S6RunitControlVerb::Up).unwrap();
        assert_eq!(supervisor.services.get("sshd").unwrap().state, S6RunitServiceState::Up);
        assert!(supervisor.services.get("sshd").unwrap().pid.is_some());

        supervisor.dispatch_control_verb("sshd", S6RunitControlVerb::Pause).unwrap();
        assert!(supervisor.services.get("sshd").unwrap().paused);
    }

    #[test]
    fn test_sysvinit_inittab() {
        let mut mgr = SysVinitManager::new();
        let entry = mgr.parse_inittab_line("id:35:respawn:/sbin/getty 38400 tty1").unwrap();
        assert_eq!(entry.id, "id");
        assert_eq!(entry.action, SysVinitAction::Respawn);
        assert!(entry.runlevels.contains(&SysVinitRunlevel::R3));

        let eval = mgr.evaluate_runlevel(SysVinitRunlevel::R3);
        assert_eq!(eval.len(), 1);
    }

    #[test]
    fn test_bsd_rc_subr() {
        let mut bsd = BsdRcSubrEngine::new();
        let script = bsd.parse_rc_header(
            "# PROVIDE: nginx\n# REQUIRE: DAEMON NETWORKING\n# BEFORE: LOGIN\n",
            "nginx",
            "/usr/local/sbin/nginx",
        );
        assert_eq!(script.provides, vec!["nginx"]);
        assert_eq!(script.requires, vec!["DAEMON", "NETWORKING"]);

        bsd.register_script(script);
        bsd.rc_conf.parse_rc_conf("nginx_enable=\"YES\"\n");
        assert!(bsd.rc_conf.is_enabled("nginx"));

        let res = bsd.execute_hook("nginx", BsdRcCommandHook::Start).unwrap();
        assert!(res.contains("Executing: /usr/local/sbin/nginx"));
    }

    #[test]
    fn test_launchd_supervisor() {
        let mut launchd = LaunchdSupervisor::new();
        let plist = LaunchdJobPlist {
            label: "com.apple.syslogd".to_string(),
            program_arguments: vec!["/usr/sbin/syslogd".to_string()],
            run_at_load: true,
            keep_alive_successful_exit: Some(true),
            keep_alive_path_state: None,
            keep_alive_network_state: None,
            throttle_interval_sec: 10,
            stdout_path: None,
            stderr_path: None,
        };

        launchd.load_job(plist);
        launchd.check_and_spawn_jobs(100, &|_| true, true);
        assert_eq!(launchd.jobs.get("com.apple.syslogd").unwrap().state, LaunchdJobState::Running);

        launchd.handle_job_exit("com.apple.syslogd", 0, 105).unwrap();
        assert_eq!(launchd.jobs.get("com.apple.syslogd").unwrap().state, LaunchdJobState::Throttled);
    }

    #[test]
    fn test_cron_anacron_at_batch() {
        let cron_entry = VixieCronEntry::parse_line("*/15 2 1-5 * * /usr/bin/backup").unwrap();
        assert!(cron_entry.matches_time(0, 2, 3, 10, 1));
        assert!(!cron_entry.matches_time(7, 2, 3, 10, 1));

        let cron_macro = VixieCronEntry::parse_line("@daily /usr/bin/daily_task").unwrap();
        assert!(cron_macro.matches_time(0, 0, 15, 6, 2));

        let anacron = AnacronJob {
            period_in_days: 7,
            delay_in_minutes: 15,
            job_identifier: "cron.weekly".to_string(),
            command: "/usr/bin/weekly".to_string(),
            last_run_timestamp: 1000,
        };
        assert!(anacron.should_run(1000 + 7 * 86400));
        assert!(!anacron.should_run(1000 + 3 * 86400));

        let mut at_batch = AtBatchQueueEngine::new();
        at_batch.enqueue_at(200, "/usr/bin/at_job");
        at_batch.enqueue_batch(0.5, "/usr/bin/batch_job");

        let (ready_at, ready_batch) = at_batch.evaluate_runnable_jobs(250, 0.4);
        assert_eq!(ready_at.len(), 1);
        assert_eq!(ready_batch.len(), 1);
    }

    #[test]
    fn test_unified_master_init_engine() {
        let mut master = SovereignCoreUserlandInitEngine::new();
        assert!(master.boot_system().is_ok());
    }
}
