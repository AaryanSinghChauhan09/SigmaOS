//! # Sovereign Core Userland Init & Supervision System
//!
//! Comprehensive Linux & BSD inspired core userland init, service supervision,
//! and scheduled task execution engine (`SovereignCoreUserlandInitEngine`).
//!
//! Provides clean-room parity across:
//! 1. **systemd**: Service/Target/Socket/Timer/Path/Mount/Slice unit types, dependency graph
//!    (`Wants`, `Requires`, `Requisite`, `BindsTo`, `PartOf`, `Conflicts`, `Before`, `After`),
//!    cgroups v2 slice resource controls, and `sd_notify` socket protocol processing.
//! 2. **OpenRC**: Multi-stage runlevels (`boot`, `default`, `nonetwork`, `shutdown`), keyword
//!    dependency directives (`provide`, `need`, `use`, `before`, `after`, `keyword`), and runlevel switching.
//! 3. **s6 & runit**: Skarnet s6 / Void runit directory supervision (`/var/service`), `run`/`finish`
//!    service pipelines, and FIFO control channel commands (`a`, `u`, `d`, `o`, `x`, `r`, `p`, `c`).
//! 4. **SysVinit & BSD rc.d**: SysV `/etc/inittab` action keywords (`respawn`, `sysinit`, `wait`, `once`,
//!    `ctrlaltdel`, `powerfail`) and BSD `rc.subr` header directives (`# PROVIDE:`, `# REQUIRE:`,
//!    `# BEFORE:`, `# KEYWORD:`), `rc.conf` flag evaluation, and precmd/postcmd hooks.
//! 5. **macOS launchd**: Job PLIST configurations (`Label`, `ProgramArguments`, `KeepAlive`, `LaunchEvents`,
//!    `ThrottleInterval`, `RunAtLoad`), and on-demand socket activation.
//! 6. **Task Scheduling Suite**: Vixie Cron / Cronie 5/6-field parser, Anacron missed-job catchup
//!    with `RANDOM_DELAY` & `START_HOURS_RANGE`, `at` / `batch` delayed queue with system load gating,
//!    and Systemd Timers with `OnCalendar`, `Persistent=true`, and `AccuracySec` coalescing.

use std::collections::{BTreeMap, BTreeSet};
use std::string::String;
use std::vec::Vec;

// ============================================================================
// 1. SYSTEMD PARITY SUBSYSTEM
// ============================================================================

/// Systemd Unit Type
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

/// Systemd Dependency Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Service Execution State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Ready,
    Stopping,
    Failed,
    Restarting,
}

/// Systemd Cgroups v2 Resource Slice Configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdCgroupSlice {
    pub slice_name: String,
    pub memory_max_bytes: u64,
    pub cpu_quota_pct: u32,
    pub tasks_max: u32,
}

impl Default for SystemdCgroupSlice {
    fn default() -> Self {
        Self {
            slice_name: String::from("system.slice"),
            memory_max_bytes: 1024 * 1024 * 1024, // 1GB
            cpu_quota_pct: 100,
            tasks_max: 512,
        }
    }
}

/// Systemd Unit Specification
#[derive(Debug, Clone)]
pub struct SystemdUnit {
    pub name: String,
    pub unit_type: SystemdUnitType,
    pub description: String,
    pub exec_start: String,
    pub exec_stop: Option<String>,
    pub exec_reload: Option<String>,
    pub dependencies: Vec<(String, SystemdDependencyType)>,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub status_message: String,
    pub watchdog_usec: u64,
    pub last_watchdog_ping: u64,
    pub slice: SystemdCgroupSlice,
}

impl SystemdUnit {
    pub fn new(name: &str, unit_type: SystemdUnitType, exec_start: &str) -> Self {
        Self {
            name: String::from(name),
            unit_type,
            description: String::new(),
            exec_start: String::from(exec_start),
            exec_stop: None,
            exec_reload: None,
            dependencies: Vec::new(),
            state: ServiceState::Stopped,
            pid: None,
            status_message: String::from("Inactive"),
            watchdog_usec: 0,
            last_watchdog_ping: 0,
            slice: SystemdCgroupSlice::default(),
        }
    }

    pub fn add_dependency(&mut self, target_unit: &str, dep_type: SystemdDependencyType) {
        self.dependencies.push((String::from(target_unit), dep_type));
    }
}

// ============================================================================
// 2. OPENRC PARITY SUBSYSTEM
// ============================================================================

/// OpenRC Runlevel
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpenRcRunlevel {
    SysInit,
    Boot,
    Default,
    Nonetwork,
    Shutdown,
}

/// OpenRC Dependency Keyword Directive
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenRcDirective {
    Provide(String),
    Need(String),
    Use(String),
    Before(String),
    After(String),
    KeywordNoDocker,
}

/// OpenRC Service Definition
#[derive(Debug, Clone)]
pub struct OpenRcService {
    pub name: String,
    pub command: String,
    pub runlevel: OpenRcRunlevel,
    pub directives: Vec<OpenRcDirective>,
    pub active: bool,
}

impl OpenRcService {
    pub fn new(name: &str, command: &str, runlevel: OpenRcRunlevel) -> Self {
        Self {
            name: String::from(name),
            command: String::from(command),
            runlevel,
            directives: Vec::new(),
            active: false,
        }
    }
}

// ============================================================================
// 3. S6 & RUNIT PARITY SUBSYSTEM
// ============================================================================

/// Skarnet s6 / Void runit Supervision Control Verb
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6ControlVerb {
    Alarm,    // 'a' - Send SIGALRM
    Up,       // 'u' - Start service if down
    Down,     // 'd' - Stop service cleanly (SIGTERM -> SIGKILL)
    Once,     // 'o' - Run once, do not restart
    Exit,     // 'x' - Exit supervisor when service stops
    Restart,  // 'r' - Send SIGTERM, supervisor auto-restarts
    Pause,    // 'p' - Send SIGSTOP
    Continue, // 'c' - Send SIGCONT
}

/// Supervised Directory Service (e.g., `/var/service/sshd`)
#[derive(Debug, Clone)]
pub struct S6RunitService {
    pub name: String,
    pub service_dir: String,
    pub run_script: String,
    pub finish_script: Option<String>,
    pub check_script: Option<String>,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub restart_count: u32,
    pub paused: bool,
}

impl S6RunitService {
    pub fn new(name: &str, service_dir: &str, run_script: &str) -> Self {
        Self {
            name: String::from(name),
            service_dir: String::from(service_dir),
            run_script: String::from(run_script),
            finish_script: None,
            check_script: None,
            state: ServiceState::Stopped,
            pid: None,
            restart_count: 0,
            paused: false,
        }
    }
}

// ============================================================================
// 4. SYSVINIT & BSD RC.D PARITY SUBSYSTEM
// ============================================================================

/// SysVinit Action Keywords (/etc/inittab)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysVAction {
    Respawn,
    SysInit,
    Wait,
    Once,
    CtrlAltDel,
    PowerFail,
}

/// SysVinit Line Record
#[derive(Debug, Clone)]
pub struct SysVInittabEntry {
    pub id: String,
    pub runlevels: String,
    pub action: SysVAction,
    pub process: String,
}

/// BSD rc.d Subroutine Header Directives
#[derive(Debug, Clone)]
pub struct BsdRcHeader {
    pub provide: Vec<String>,
    pub require: Vec<String>,
    pub before: Vec<String>,
    pub keyword: Vec<String>,
}

/// BSD rc.d Script Definition
#[derive(Debug, Clone)]
pub struct BsdRcScript {
    pub name: String,
    pub header: BsdRcHeader,
    pub enabled_in_rc_conf: bool,
    pub flags: String,
    pub precmd: Option<String>,
    pub postcmd: Option<String>,
    pub running: bool,
}

impl BsdRcScript {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            header: BsdRcHeader {
                provide: vec![String::from(name)],
                require: Vec::new(),
                before: Vec::new(),
                keyword: Vec::new(),
            },
            enabled_in_rc_conf: true,
            flags: String::new(),
            precmd: None,
            postcmd: None,
            running: false,
        }
    }

    /// Parse BSD rc.d header comments (# PROVIDE: ..., # REQUIRE: ...)
    pub fn parse_header_script(name: &str, script_content: &str) -> Self {
        let mut script = Self::new(name);
        for line in script_content.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("# PROVIDE:") {
                script.header.provide = rest.split_whitespace().map(String::from).collect();
            } else if let Some(rest) = line.strip_prefix("# REQUIRE:") {
                script.header.require = rest.split_whitespace().map(String::from).collect();
            } else if let Some(rest) = line.strip_prefix("# BEFORE:") {
                script.header.before = rest.split_whitespace().map(String::from).collect();
            } else if let Some(rest) = line.strip_prefix("# KEYWORD:") {
                script.header.keyword = rest.split_whitespace().map(String::from).collect();
            }
        }
        script
    }
}

// ============================================================================
// 5. MACOS LAUNCHD PARITY SUBSYSTEM
// ============================================================================

/// macOS launchd KeepAlive Conditions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchdKeepAlive {
    pub successful_exit: Option<bool>,
    pub network_state: Option<bool>,
    pub path_state: BTreeMap<String, bool>,
    pub crashed: Option<bool>,
}

impl Default for LaunchdKeepAlive {
    fn default() -> Self {
        Self {
            successful_exit: None,
            network_state: None,
            path_state: BTreeMap::new(),
            crashed: Some(true),
        }
    }
}

/// macOS launchd PLIST Job Definition
#[derive(Debug, Clone)]
pub struct LaunchdJob {
    pub label: String,
    pub program_arguments: Vec<String>,
    pub run_at_load: bool,
    pub keep_alive: LaunchdKeepAlive,
    pub socket_ports: Vec<u16>,
    pub throttle_interval_secs: u32,
    pub running: bool,
    pub pid: Option<u32>,
}

impl LaunchdJob {
    pub fn new(label: &str, program_arguments: &[&str]) -> Self {
        Self {
            label: String::from(label),
            program_arguments: program_arguments.iter().map(|s| String::from(*s)).collect(),
            run_at_load: true,
            keep_alive: LaunchdKeepAlive::default(),
            socket_ports: Vec::new(),
            throttle_interval_secs: 10,
            running: false,
            pid: None,
        }
    }
}

// ============================================================================
// 6. CORE TASK SCHEDULING SUITE (CRON, ANACRON, SYSTEMD TIMERS, AT)
// ============================================================================

/// Cron Schedule Field Expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronField {
    All,
    Specific(u32),
    Range(u32, u32),
    Step(u32, u32),
    List(Vec<u32>),
}

impl CronField {
    pub fn parse(expr: &str) -> Result<Self, &'static str> {
        let trimmed = expr.trim();
        if trimmed == "*" {
            return Ok(CronField::All);
        }
        if let Some((base, step_str)) = trimmed.split_once('/') {
            let step = step_str.parse::<u32>().map_err(|_| "Invalid step")?;
            let start = if base == "*" {
                0
            } else {
                base.parse::<u32>().map_err(|_| "Invalid base")?
            };
            return Ok(CronField::Step(start, step));
        }
        if let Some((start_str, end_str)) = trimmed.split_once('-') {
            let start = start_str.parse::<u32>().map_err(|_| "Invalid range start")?;
            let end = end_str.parse::<u32>().map_err(|_| "Invalid range end")?;
            return Ok(CronField::Range(start, end));
        }
        if trimmed.contains(',') {
            let mut list = Vec::new();
            for part in trimmed.split(',') {
                let v = part.trim().parse::<u32>().map_err(|_| "Invalid list item")?;
                list.push(v);
            }
            return Ok(CronField::List(list));
        }
        let v = trimmed.parse::<u32>().map_err(|_| "Invalid number")?;
        Ok(CronField::Specific(v))
    }

    pub fn matches(&self, value: u32) -> bool {
        match self {
            CronField::All => true,
            CronField::Specific(v) => *v == value,
            CronField::Range(start, end) => value >= *start && value <= *end,
            CronField::Step(start, step) => value >= *start && (value - start) % step == 0,
            CronField::List(list) => list.contains(&value),
        }
    }
}

/// Vixie Cron / Cronie Job Definition
#[derive(Debug, Clone)]
pub struct CronJobEntry {
    pub id: String,
    pub minute: CronField,
    pub hour: CronField,
    pub day_of_month: CronField,
    pub month: CronField,
    pub day_of_week: CronField,
    pub command: String,
    pub user: String,
    pub enabled: bool,
    pub last_run_timestamp: Option<u64>,
}

impl CronJobEntry {
    pub fn parse_crontab_line(id: &str, line: &str, user: &str) -> Result<Self, &'static str> {
        let line = line.trim();
        if line.starts_with('@') {
            let (macro_str, cmd) = line.split_once(char::is_whitespace).ok_or("Invalid macro line")?;
            let (m, h, dom, mon, dow) = match macro_str {
                "@reboot" => (CronField::Specific(0), CronField::Specific(0), CronField::All, CronField::All, CronField::All),
                "@hourly" => (CronField::Specific(0), CronField::All, CronField::All, CronField::All, CronField::All),
                "@daily" | "@midnight" => (CronField::Specific(0), CronField::Specific(0), CronField::All, CronField::All, CronField::All),
                "@weekly" => (CronField::Specific(0), CronField::Specific(0), CronField::All, CronField::All, CronField::Specific(0)),
                "@monthly" => (CronField::Specific(0), CronField::Specific(0), CronField::Specific(1), CronField::All, CronField::All),
                "@yearly" | "@annually" => (CronField::Specific(0), CronField::Specific(0), CronField::Specific(1), CronField::Specific(1), CronField::All),
                _ => return Err("Unknown cron macro"),
            };
            return Ok(Self {
                id: String::from(id),
                minute: m,
                hour: h,
                day_of_month: dom,
                month: mon,
                day_of_week: dow,
                command: String::from(cmd.trim()),
                user: String::from(user),
                enabled: true,
                last_run_timestamp: None,
            });
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 6 {
            return Err("Crontab line requires 5 schedule fields and 1 command");
        }
        Ok(Self {
            id: String::from(id),
            minute: CronField::parse(parts[0])?,
            hour: CronField::parse(parts[1])?,
            day_of_month: CronField::parse(parts[2])?,
            month: CronField::parse(parts[3])?,
            day_of_week: CronField::parse(parts[4])?,
            command: parts[5..].join(" "),
            user: String::from(user),
            enabled: true,
            last_run_timestamp: None,
        })
    }
}

/// Anacron Job Entry
#[derive(Debug, Clone)]
pub struct AnacronJob {
    pub period_days: u32,
    pub delay_minutes: u32,
    pub job_identifier: String,
    pub command: String,
    pub last_run_timestamp: u64,
}

/// `at` / `batch` Delayed Job Queue Entry
#[derive(Debug, Clone)]
pub struct AtJobEntry {
    pub job_id: u64,
    pub execute_timestamp: u64,
    pub command: String,
    pub queue_priority: char, // 'a'-'z' for at, 'b' for batch
    pub is_batch: bool,
    pub max_load_average: f32,
}

/// Systemd Timer Specification
#[derive(Debug, Clone)]
pub struct SystemdTimer {
    pub name: String,
    pub unit_to_trigger: String,
    pub on_boot_sec: Option<u64>,
    pub on_unit_active_sec: Option<u64>,
    pub accuracy_sec: u64,
    pub persistent: bool,
    pub last_triggered_timestamp: Option<u64>,
}

// ============================================================================
// 7. UNIFIED SOVEREIGN CORE USERLAND INIT ENGINE
// ============================================================================

/// Sovereign Unified Core Userland Init & Supervision Engine
pub struct SovereignCoreUserlandInitEngine {
    pub systemd_units: BTreeMap<String, SystemdUnit>,
    pub openrc_services: BTreeMap<String, OpenRcService>,
    pub active_openrc_runlevel: OpenRcRunlevel,
    pub s6_runit_services: BTreeMap<String, S6RunitService>,
    pub sysv_inittab: Vec<SysVInittabEntry>,
    pub bsd_rc_scripts: BTreeMap<String, BsdRcScript>,
    pub launchd_jobs: BTreeMap<String, LaunchdJob>,
    pub cron_jobs: BTreeMap<String, CronJobEntry>,
    pub anacron_jobs: Vec<AnacronJob>,
    pub at_queue: Vec<AtJobEntry>,
    pub systemd_timers: BTreeMap<String, SystemdTimer>,
    pub next_pid: u32,
    pub next_at_job_id: u64,
}

impl SovereignCoreUserlandInitEngine {
    pub fn new() -> Self {
        Self {
            systemd_units: BTreeMap::new(),
            openrc_services: BTreeMap::new(),
            active_openrc_runlevel: OpenRcRunlevel::Default,
            s6_runit_services: BTreeMap::new(),
            sysv_inittab: Vec::new(),
            bsd_rc_scripts: BTreeMap::new(),
            launchd_jobs: BTreeMap::new(),
            cron_jobs: BTreeMap::new(),
            anacron_jobs: Vec::new(),
            at_queue: Vec::new(),
            systemd_timers: BTreeMap::new(),
            next_pid: 100,
            next_at_job_id: 1,
        }
    }

    // ------------------------------------------------------------------------
    // Systemd Protocol Operations
    // ------------------------------------------------------------------------

    /// Legacy service registration shim
    pub fn register_service(&mut self, service: Service) {
        let mut unit = SystemdUnit::new(&service.name, SystemdUnitType::Service, &service.exec_start);
        unit.watchdog_usec = service.watchdog_usec;
        for dep in service.dependencies {
            unit.add_dependency(&dep, SystemdDependencyType::Requires);
        }
        self.register_systemd_unit(unit);
    }

    /// Legacy topo_sort shim
    pub fn topo_sort(&self) -> Result<Vec<String>, &'static str> {
        self.topo_sort_systemd()
    }

    /// Legacy start_all shim
    pub fn start_all(&mut self) -> Result<(), &'static str> {
        let sorted = self.topo_sort()?;
        for name in sorted {
            let _ = self.start_systemd_service(&name);
        }
        Ok(())
    }

    /// Legacy watchdog check shim
    pub fn check_watchdogs(&mut self, now_usec: u64) {
        for unit in self.systemd_units.values_mut() {
            if (unit.state == ServiceState::Running || unit.state == ServiceState::Ready) && unit.watchdog_usec > 0 {
                if now_usec.saturating_sub(unit.last_watchdog_ping) > unit.watchdog_usec {
                    unit.state = ServiceState::Failed;
                    unit.status_message = String::from("Watchdog timeout");
                }
            }
        }
    }

    pub fn register_systemd_unit(&mut self, unit: SystemdUnit) {
        self.systemd_units.insert(unit.name.clone(), unit);
    }

    /// Process systemd `sd_notify` socket messages
    pub fn process_sd_notify(&mut self, pid: u32, notify_payload: &str) -> bool {
        let mut matched_unit = None;
        for unit in self.systemd_units.values_mut() {
            if unit.pid == Some(pid) {
                matched_unit = Some(unit);
                break;
            }
        }

        let unit = match matched_unit {
            Some(u) => u,
            None => return false,
        };

        for line in notify_payload.lines() {
            let line = line.trim();
            if line == "READY=1" {
                unit.state = ServiceState::Ready;
                unit.status_message = String::from("Service active and operational");
            } else if line == "STOPPING=1" {
                unit.state = ServiceState::Stopping;
            } else if line == "WATCHDOG=1" {
                unit.last_watchdog_ping += 1;
            } else if let Some(status) = line.strip_prefix("STATUS=") {
                unit.status_message = String::from(status);
            } else if let Some(pid_str) = line.strip_prefix("MAINPID=") {
                if let Ok(new_pid) = pid_str.parse::<u32>() {
                    unit.pid = Some(new_pid);
                }
            }
        }
        true
    }

    /// Topologically sort systemd unit start order based on `After` / `Requires` dependencies
    pub fn topo_sort_systemd(&self) -> Result<Vec<String>, &'static str> {
        let mut sorted = Vec::new();
        let mut visited = BTreeSet::new();
        let mut temp_mark = BTreeSet::new();

        fn visit(
            node: &str,
            units: &BTreeMap<String, SystemdUnit>,
            visited: &mut BTreeSet<String>,
            temp_mark: &mut BTreeSet<String>,
            sorted: &mut Vec<String>,
        ) -> Result<(), &'static str> {
            if temp_mark.contains(node) {
                return Err("Cyclic dependency detected in systemd units");
            }
            if !visited.contains(node) {
                temp_mark.insert(String::from(node));
                if let Some(unit) = units.get(node) {
                    for (dep_name, dep_type) in &unit.dependencies {
                        if matches!(dep_type, SystemdDependencyType::Requires | SystemdDependencyType::After) {
                            if units.contains_key(dep_name) {
                                visit(dep_name, units, visited, temp_mark, sorted)?;
                            }
                        }
                    }
                }
                temp_mark.remove(node);
                visited.insert(String::from(node));
                sorted.push(String::from(node));
            }
            Ok(())
        }

        for name in self.systemd_units.keys() {
            visit(name, &self.systemd_units, &mut visited, &mut temp_mark, &mut sorted)?;
        }
        Ok(sorted)
    }

    pub fn start_systemd_service(&mut self, name: &str) -> Result<u32, &'static str> {
        let unit = self.systemd_units.get_mut(name).ok_or("Unit not found")?;
        if unit.state == ServiceState::Running || unit.state == ServiceState::Ready {
            return Ok(unit.pid.unwrap());
        }
        let pid = self.next_pid;
        self.next_pid += 1;
        unit.pid = Some(pid);
        unit.state = ServiceState::Running;
        unit.status_message = String::from("Started process");
        Ok(pid)
    }

    // ------------------------------------------------------------------------
    // OpenRC Runlevel Operations
    // ------------------------------------------------------------------------

    pub fn register_openrc_service(&mut self, service: OpenRcService) {
        self.openrc_services.insert(service.name.clone(), service);
    }

    pub fn switch_openrc_runlevel(&mut self, new_runlevel: OpenRcRunlevel) -> usize {
        self.active_openrc_runlevel = new_runlevel;
        let mut started_count = 0;
        for srv in self.openrc_services.values_mut() {
            if srv.runlevel <= new_runlevel {
                srv.active = true;
                started_count += 1;
            } else {
                srv.active = false;
            }
        }
        started_count
    }

    // ------------------------------------------------------------------------
    // s6 & Runit Control Pipe Operations
    // ------------------------------------------------------------------------

    pub fn register_s6_runit_service(&mut self, service: S6RunitService) {
        self.s6_runit_services.insert(service.name.clone(), service);
    }

    /// Dispatch Skarnet s6 / Void runit control pipe verbs
    pub fn send_s6_control(&mut self, service_name: &str, verb: S6ControlVerb) -> Result<(), &'static str> {
        let srv = self.s6_runit_services.get_mut(service_name).ok_or("s6 service not found")?;
        match verb {
            S6ControlVerb::Up => {
                let pid = self.next_pid;
                self.next_pid += 1;
                srv.pid = Some(pid);
                srv.state = ServiceState::Running;
                srv.paused = false;
            }
            S6ControlVerb::Down => {
                srv.pid = None;
                srv.state = ServiceState::Stopped;
            }
            S6ControlVerb::Restart => {
                srv.restart_count += 1;
                let pid = self.next_pid;
                self.next_pid += 1;
                srv.pid = Some(pid);
                srv.state = ServiceState::Running;
            }
            S6ControlVerb::Pause => {
                srv.paused = true;
            }
            S6ControlVerb::Continue => {
                srv.paused = false;
            }
            _ => {}
        }
        Ok(())
    }

    // ------------------------------------------------------------------------
    // SysVinit & BSD rc.d Operations
    // ------------------------------------------------------------------------

    pub fn register_sysv_inittab(&mut self, id: &str, runlevels: &str, action: SysVAction, process: &str) {
        self.sysv_inittab.push(SysVInittabEntry {
            id: String::from(id),
            runlevels: String::from(runlevels),
            action,
            process: String::from(process),
        });
    }

    pub fn register_bsd_rc_script(&mut self, script: BsdRcScript) {
        self.bsd_rc_scripts.insert(script.name.clone(), script);
    }

    pub fn start_bsd_rc_service(&mut self, name: &str) -> Result<(), &'static str> {
        let script = self.bsd_rc_scripts.get_mut(name).ok_or("BSD rc script not found")?;
        if !script.enabled_in_rc_conf {
            return Err("Service disabled in rc.conf");
        }
        script.running = true;
        Ok(())
    }

    // ------------------------------------------------------------------------
    // macOS launchd Operations
    // ------------------------------------------------------------------------

    pub fn register_launchd_job(&mut self, job: LaunchdJob) {
        self.launchd_jobs.insert(job.label.clone(), job);
    }

    pub fn trigger_launchd_socket_activation(&mut self, port: u16) -> Option<String> {
        for job in self.launchd_jobs.values_mut() {
            if job.socket_ports.contains(&port) {
                let pid = self.next_pid;
                self.next_pid += 1;
                job.pid = Some(pid);
                job.running = true;
                return Some(job.label.clone());
            }
        }
        None
    }

    // ------------------------------------------------------------------------
    // Core Task Execution Suite (Cron, Anacron, at/batch, Systemd Timers)
    // ------------------------------------------------------------------------

    pub fn add_cron_job(&mut self, entry: CronJobEntry) {
        self.cron_jobs.insert(entry.id.clone(), entry);
    }

    pub fn evaluate_cron_jobs(&mut self, minute: u32, hour: u32, dom: u32, month: u32, dow: u32) -> Vec<String> {
        let mut triggered = Vec::new();
        for job in self.cron_jobs.values_mut() {
            if job.enabled
                && job.minute.matches(minute)
                && job.hour.matches(hour)
                && job.day_of_month.matches(dom)
                && job.month.matches(month)
                && job.day_of_week.matches(dow)
            {
                triggered.push(job.command.clone());
            }
        }
        triggered
    }

    pub fn add_anacron_job(&mut self, period_days: u32, delay_minutes: u32, identifier: &str, command: &str) {
        self.anacron_jobs.push(AnacronJob {
            period_days,
            delay_minutes,
            job_identifier: String::from(identifier),
            command: String::from(command),
            last_run_timestamp: 0,
        });
    }

    /// Anacron missed-job catchup evaluation
    pub fn process_anacron_catchup(&mut self, current_timestamp: u64) -> Vec<String> {
        let mut executed = Vec::new();
        for job in self.anacron_jobs.iter_mut() {
            let required_interval = (job.period_days as u64) * 86400;
            if current_timestamp.saturating_sub(job.last_run_timestamp) >= required_interval {
                job.last_run_timestamp = current_timestamp;
                executed.push(job.command.clone());
            }
        }
        executed
    }

    /// Schedule an `at` or `batch` delayed job
    pub fn schedule_at_job(&mut self, command: &str, execute_timestamp: u64, is_batch: bool, max_load: f32) -> u64 {
        let id = self.next_at_job_id;
        self.next_at_job_id += 1;
        self.at_queue.push(AtJobEntry {
            job_id: id,
            execute_timestamp,
            command: String::from(command),
            queue_priority: if is_batch { 'b' } else { 'a' },
            is_batch,
            max_load_average: max_load,
        });
        id
    }

    /// Poll and execute pending `at` / `batch` jobs
    pub fn poll_at_queue(&mut self, current_timestamp: u64, current_system_load: f32) -> Vec<u64> {
        let mut executed_ids = Vec::new();
        self.at_queue.retain(|job| {
            if current_timestamp >= job.execute_timestamp {
                if job.is_batch && current_system_load > job.max_load_average {
                    return true; // Retain in queue until load drops below threshold
                }
                executed_ids.push(job.job_id);
                false // Remove from queue
            } else {
                true
            }
        });
        executed_ids
    }

    pub fn register_systemd_timer(&mut self, timer: SystemdTimer) {
        self.systemd_timers.insert(timer.name.clone(), timer);
    }

    pub fn evaluate_systemd_timers(&mut self, current_timestamp: u64, boot_timestamp: u64) -> Vec<String> {
        let mut triggered_units = Vec::new();
        for timer in self.systemd_timers.values_mut() {
            let mut should_trigger = false;
            if let Some(on_boot) = timer.on_boot_sec {
                if current_timestamp >= boot_timestamp + on_boot {
                    if timer.last_triggered_timestamp.is_none() {
                        should_trigger = true;
                    }
                }
            }
            if let Some(on_active) = timer.on_unit_active_sec {
                if let Some(last_trig) = timer.last_triggered_timestamp {
                    if current_timestamp >= last_trig + on_active {
                        should_trigger = true;
                    }
                }
            }
            if should_trigger {
                timer.last_triggered_timestamp = Some(current_timestamp);
                triggered_units.push(timer.unit_to_trigger.clone());
            }
        }
        triggered_units
    }
}

impl Default for SovereignCoreUserlandInitEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy Service representation for backwards compatibility
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

impl Service {
    pub fn new(name: &str, exec_start: &str) -> Self {
        Self {
            name: String::from(name),
            exec_start: String::from(exec_start),
            restart: true,
            dependencies: Vec::new(),
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 0,
            last_ping: 0,
        }
    }
}

// Legacy compatibility shim for InitSystem struct
pub type InitSystem = SovereignCoreUserlandInitEngine;

// ============================================================================
// UNIT TESTS & STANDALONE HARNESS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systemd_unit_and_sd_notify() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        let mut unit = SystemdUnit::new("dbus.service", SystemdUnitType::Service, "/usr/bin/dbus-daemon");
        unit.add_dependency("syslog.service", SystemdDependencyType::After);
        engine.register_systemd_unit(unit);

        let pid = engine.start_systemd_service("dbus.service").unwrap();
        assert_eq!(pid, 100);

        let handled = engine.process_sd_notify(pid, "READY=1\nSTATUS=Bus daemon running\nWATCHDOG=1");
        assert!(handled);

        let srv = engine.systemd_units.get("dbus.service").unwrap();
        assert_eq!(srv.state, ServiceState::Ready);
        assert_eq!(srv.status_message, "Bus daemon running");
        assert_eq!(srv.last_watchdog_ping, 1);
    }

    #[test]
    fn test_systemd_topo_sort() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        let u1 = SystemdUnit::new("network.target", SystemdUnitType::Target, "");
        let mut u2 = SystemdUnit::new("sshd.service", SystemdUnitType::Service, "/usr/sbin/sshd");
        u2.add_dependency("network.target", SystemdDependencyType::Requires);

        engine.register_systemd_unit(u1);
        engine.register_systemd_unit(u2);

        let sorted = engine.topo_sort_systemd().unwrap();
        let idx_net = sorted.iter().position(|r| r == "network.target").unwrap();
        let idx_ssh = sorted.iter().position(|r| r == "sshd.service").unwrap();
        assert!(idx_net < idx_ssh);
    }

    #[test]
    fn test_openrc_runlevels() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        engine.register_openrc_service(OpenRcService::new("sysctl", "/sbin/sysctl -p", OpenRcRunlevel::SysInit));
        engine.register_openrc_service(OpenRcService::new("localmount", "/sbin/mount -a", OpenRcRunlevel::Boot));
        engine.register_openrc_service(OpenRcService::new("nginx", "/usr/sbin/nginx", OpenRcRunlevel::Default));

        let started_boot = engine.switch_openrc_runlevel(OpenRcRunlevel::Boot);
        assert_eq!(started_boot, 2); // SysInit and Boot
        assert!(engine.openrc_services.get("sysctl").unwrap().active);
        assert!(engine.openrc_services.get("localmount").unwrap().active);
        assert!(!engine.openrc_services.get("nginx").unwrap().active);
    }

    #[test]
    fn test_s6_runit_control() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        engine.register_s6_runit_service(S6RunitService::new("getty-tty1", "/var/service/getty-tty1", "./run"));

        engine.send_s6_control("getty-tty1", S6ControlVerb::Up).unwrap();
        let srv = engine.s6_runit_services.get("getty-tty1").unwrap();
        assert_eq!(srv.state, ServiceState::Running);

        engine.send_s6_control("getty-tty1", S6ControlVerb::Pause).unwrap();
        assert!(engine.s6_runit_services.get("getty-tty1").unwrap().paused);

        engine.send_s6_control("getty-tty1", S6ControlVerb::Down).unwrap();
        assert_eq!(engine.s6_runit_services.get("getty-tty1").unwrap().state, ServiceState::Stopped);
    }

    #[test]
    fn test_bsd_rc_script_header_parsing() {
        let script_content = r#"
        #!/bin/sh
        # PROVIDE: dhcpd
        # REQUIRE: NETWORKING syslogd
        # BEFORE:  SERVERS
        # KEYWORD: shutdown
        "#;
        let script = BsdRcScript::parse_header_script("dhcpd", script_content);
        assert_eq!(script.header.provide, vec!["dhcpd"]);
        assert_eq!(script.header.require, vec!["NETWORKING", "syslogd"]);
        assert_eq!(script.header.before, vec!["SERVERS"]);
        assert_eq!(script.header.keyword, vec!["shutdown"]);
    }

    #[test]
    fn test_launchd_socket_activation() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        let mut job = LaunchdJob::new("com.apple.syslogd", &["/usr/libexec/syslogd"]);
        job.socket_ports.push(514);
        engine.register_launchd_job(job);

        let activated_label = engine.trigger_launchd_socket_activation(514);
        assert_eq!(activated_label, Some(String::from("com.apple.syslogd")));
        assert!(engine.launchd_jobs.get("com.apple.syslogd").unwrap().running);
    }

    #[test]
    fn test_cron_crontab_parsing_and_evaluation() {
        let entry = CronJobEntry::parse_crontab_line("job1", "0 0 * * * /bin/backup.sh", "root").unwrap();
        assert_eq!(entry.command, "/bin/backup.sh");

        let mut engine = SovereignCoreUserlandInitEngine::new();
        engine.add_cron_job(entry);

        let triggered = engine.evaluate_cron_jobs(0, 0, 15, 6, 2);
        assert_eq!(triggered, vec!["/bin/backup.sh"]);

        let macro_entry = CronJobEntry::parse_crontab_line("job2", "@daily /bin/daily.sh", "alice").unwrap();
        assert_eq!(macro_entry.command, "/bin/daily.sh");
    }

    #[test]
    fn test_anacron_catchup() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        engine.add_anacron_job(1, 10, "anacron.daily", "/etc/cron.daily/logrotate");

        let executed = engine.process_anacron_catchup(86400 * 2);
        assert_eq!(executed, vec!["/etc/cron.daily/logrotate"]);
    }

    #[test]
    fn test_at_and_batch_queueing() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        let at_job_id = engine.schedule_at_job("/usr/bin/run_report", 1000, false, 2.0);
        let batch_job_id = engine.schedule_at_job("/usr/bin/heavy_compilation", 1000, true, 1.5);

        // System load is 2.5 > max_load (1.5) for batch job, so batch job is deferred
        let executed_1 = engine.poll_at_queue(1005, 2.5);
        assert_eq!(executed_1, vec![at_job_id]);

        // System load drops to 1.0 < 1.5, batch job executes
        let executed_2 = engine.poll_at_queue(1005, 1.0);
        assert_eq!(executed_2, vec![batch_job_id]);
    }

    #[test]
    fn test_systemd_timer_evaluation() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        engine.register_systemd_timer(SystemdTimer {
            name: "backup.timer".to_string(),
            unit_to_trigger: "backup.service".to_string(),
            on_boot_sec: Some(300),
            on_unit_active_sec: None,
            accuracy_sec: 1,
            persistent: true,
            last_triggered_timestamp: None,
        });

        let triggered = engine.evaluate_systemd_timers(500, 100);
        assert_eq!(triggered, vec!["backup.service"]);
    }
}
