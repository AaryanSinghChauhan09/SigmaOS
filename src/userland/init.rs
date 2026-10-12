// src/userland/init.rs
// Comprehensive Linux & BSD Inspired Init and Service Management Engine for SigmaOS

use std::collections::{BTreeMap, BTreeSet};
use std::string::String;
use std::vec::Vec;

// ============================================================================
// Legacy / Core Init Structures (Backward Compatibility)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Stopped,
    Starting,
    Running,
    Failed,
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

#[derive(Debug, Clone)]
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
// 1. Systemd Unit & Dependency Supervision Framework
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone)]
pub struct SystemdUnitDependency {
    pub target_unit: String,
    pub kind: SystemdDependencyKind,
}

#[derive(Debug, Clone)]
pub struct SystemdCgroupLimits {
    pub cpu_max_pct: u32,
    pub memory_max_bytes: u64,
    pub io_weight: u32,
}

#[derive(Debug, Clone)]
pub struct SystemdUnit {
    pub name: String,
    pub unit_type: SystemdUnitType,
    pub description: String,
    pub exec_start: String,
    pub exec_stop: Option<String>,
    pub dependencies: Vec<SystemdUnitDependency>,
    pub cgroup_limits: Option<SystemdCgroupLimits>,
    pub active: bool,
    pub pid: Option<u32>,
    pub status_text: String,
    pub errno_code: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdNotifyMessage {
    Ready,
    Reloading,
    Stopping,
    Watchdog,
    Status(String),
    MainPid(u32),
    Errno(u32),
}

pub struct SystemdUnitEngine {
    pub units: BTreeMap<String, SystemdUnit>,
}

impl SystemdUnitEngine {
    pub fn new() -> Self {
        Self {
            units: BTreeMap::new(),
        }
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.insert(unit.name.clone(), unit);
    }

    pub fn parse_sd_notify_payload(payload: &str) -> Vec<SdNotifyMessage> {
        let mut msgs = Vec::new();
        for line in payload.lines() {
            let line = line.trim();
            if line.starts_with("READY=1") {
                msgs.push(SdNotifyMessage::Ready);
            } else if line.starts_with("RELOADING=1") {
                msgs.push(SdNotifyMessage::Reloading);
            } else if line.starts_with("STOPPING=1") {
                msgs.push(SdNotifyMessage::Stopping);
            } else if line.starts_with("WATCHDOG=1") {
                msgs.push(SdNotifyMessage::Watchdog);
            } else if let Some(val) = line.strip_prefix("STATUS=") {
                msgs.push(SdNotifyMessage::Status(val.to_string()));
            } else if let Some(val) = line.strip_prefix("MAINPID=") {
                if let Ok(pid) = val.parse::<u32>() {
                    msgs.push(SdNotifyMessage::MainPid(pid));
                }
            } else if let Some(val) = line.strip_prefix("ERRNO=") {
                if let Ok(err) = val.parse::<u32>() {
                    msgs.push(SdNotifyMessage::Errno(err));
                }
            }
        }
        msgs
    }

    pub fn handle_sd_notify(&mut self, unit_name: &str, payload: &str) -> Result<(), &'static str> {
        let msgs = Self::parse_sd_notify_payload(payload);
        let unit = self.units.get_mut(unit_name).ok_or("Unit not found")?;
        for msg in msgs {
            match msg {
                SdNotifyMessage::Ready => {
                    unit.active = true;
                }
                SdNotifyMessage::Stopping => {
                    unit.active = false;
                }
                SdNotifyMessage::Status(st) => {
                    unit.status_text = st;
                }
                SdNotifyMessage::MainPid(pid) => {
                    unit.pid = Some(pid);
                }
                SdNotifyMessage::Errno(err) => {
                    unit.errno_code = err;
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn evaluate_unit_dependencies(&self, unit_name: &str) -> Result<Vec<String>, &'static str> {
        let mut order = Vec::new();
        let mut visited = BTreeSet::new();

        fn traverse(
            name: &str,
            units: &BTreeMap<String, SystemdUnit>,
            visited: &mut BTreeSet<String>,
            order: &mut Vec<String>,
        ) -> Result<(), &'static str> {
            if visited.contains(name) {
                return Ok(());
            }
            visited.insert(name.to_string());

            if let Some(unit) = units.get(name) {
                for dep in &unit.dependencies {
                    match dep.kind {
                        SystemdDependencyKind::Requires | SystemdDependencyKind::Wants | SystemdDependencyKind::After => {
                            traverse(&dep.target_unit, units, visited, order)?;
                        }
                        SystemdDependencyKind::Conflicts => {
                            if let Some(target) = units.get(&dep.target_unit) {
                                if target.active {
                                    return Err("Conflicting unit is currently active");
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            order.push(name.to_string());
            Ok(())
        }

        traverse(unit_name, &self.units, &mut visited, &mut order)?;
        Ok(order)
    }
}

// ============================================================================
// 2. OpenRC Multi-Stage Runlevel & Keyword Dependency Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OpenRcRunlevel {
    SysInit,
    Boot,
    Default,
    Nonetwork,
    Shutdown,
}

#[derive(Debug, Clone)]
pub enum OpenRcDirective {
    Provide(String),
    Need(Vec<String>),
    Use(Vec<String>),
    Before(Vec<String>),
    After(Vec<String>),
    KeywordNoDocker,
}

#[derive(Debug, Clone)]
pub struct OpenRcScript {
    pub name: String,
    pub runlevel: OpenRcRunlevel,
    pub directives: Vec<OpenRcDirective>,
    pub exec_start: String,
    pub is_started: bool,
}

pub struct OpenRcEngine {
    pub scripts: BTreeMap<String, OpenRcScript>,
    pub current_runlevel: OpenRcRunlevel,
}

impl OpenRcEngine {
    pub fn new() -> Self {
        Self {
            scripts: BTreeMap::new(),
            current_runlevel: OpenRcRunlevel::SysInit,
        }
    }

    pub fn register_script(&mut self, script: OpenRcScript) {
        self.scripts.insert(script.name.clone(), script);
    }

    pub fn parse_header_directives(header_content: &str) -> Vec<OpenRcDirective> {
        let mut directives = Vec::new();
        for line in header_content.lines() {
            let line = line.trim();
            if let Some(val) = line.strip_prefix("provide") {
                directives.push(OpenRcDirective::Provide(val.trim().to_string()));
            } else if let Some(val) = line.strip_prefix("need") {
                let deps = val.split_whitespace().map(|s| s.to_string()).collect();
                directives.push(OpenRcDirective::Need(deps));
            } else if let Some(val) = line.strip_prefix("use") {
                let deps = val.split_whitespace().map(|s| s.to_string()).collect();
                directives.push(OpenRcDirective::Use(deps));
            } else if let Some(val) = line.strip_prefix("before") {
                let deps = val.split_whitespace().map(|s| s.to_string()).collect();
                directives.push(OpenRcDirective::Before(deps));
            } else if let Some(val) = line.strip_prefix("after") {
                let deps = val.split_whitespace().map(|s| s.to_string()).collect();
                directives.push(OpenRcDirective::After(deps));
            } else if line.contains("keyword") && line.contains("-docker") {
                directives.push(OpenRcDirective::KeywordNoDocker);
            }
        }
        directives
    }

    pub fn transition_to_runlevel(&mut self, target: OpenRcRunlevel) -> Result<u32, &'static str> {
        self.current_runlevel = target;
        let mut started_count = 0;
        let script_names: Vec<String> = self.scripts.keys().cloned().collect();

        for name in script_names {
            if let Some(script) = self.scripts.get_mut(&name) {
                if script.runlevel <= target && !script.is_started {
                    script.is_started = true;
                    started_count += 1;
                }
            }
        }
        Ok(started_count)
    }
}

// ============================================================================
// 3. s6 & Runit Supervised Directory Channel Control Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6RunitControlVerb {
    Up,        // 'u'
    Down,      // 'd'
    Once,      // 'o'
    Alarm,     // 'a'
    Restart,   // 'r'
    Pause,     // 'p'
    Continue,  // 'c'
    Exit,      // 'x'
}

#[derive(Debug, Clone)]
pub struct S6RunitService {
    pub name: String,
    pub service_dir: String,
    pub is_running: bool,
    pub is_paused: bool,
    pub pid: Option<u32>,
    pub restart_count: u32,
}

pub struct S6RunitEngine {
    pub services: BTreeMap<String, S6RunitService>,
}

impl S6RunitEngine {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }

    pub fn register_service(&mut self, name: &str, service_dir: &str) {
        self.services.insert(
            name.to_string(),
            S6RunitService {
                name: name.to_string(),
                service_dir: service_dir.to_string(),
                is_running: false,
                is_paused: false,
                pid: None,
                restart_count: 0,
            },
        );
    }

    pub fn send_control_verb(&mut self, name: &str, verb: S6RunitControlVerb) -> Result<(), &'static str> {
        let srv = self.services.get_mut(name).ok_or("Service not found in supervision tree")?;
        match verb {
            S6RunitControlVerb::Up | S6RunitControlVerb::Once => {
                srv.is_running = true;
                srv.is_paused = false;
                srv.pid = Some(2000 + name.len() as u32);
            }
            S6RunitControlVerb::Down | S6RunitControlVerb::Exit => {
                srv.is_running = false;
                srv.is_paused = false;
                srv.pid = None;
            }
            S6RunitControlVerb::Restart | S6RunitControlVerb::Alarm => {
                srv.restart_count += 1;
                srv.is_running = true;
                srv.pid = Some(3000 + srv.restart_count);
            }
            S6RunitControlVerb::Pause => {
                srv.is_paused = true;
            }
            S6RunitControlVerb::Continue => {
                srv.is_paused = false;
            }
        }
        Ok(())
    }
}

// ============================================================================
// 4. SysVinit / BSD rc.subr / Launchd Supervisor Integration
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysVInittabAction {
    Respawn,
    SysInit,
    Wait,
    Once,
    CtrlAltDel,
    PowerFail,
}

#[derive(Debug, Clone)]
pub struct SysVInittabEntry {
    pub id: String,
    pub runlevels: String,
    pub action: SysVInittabAction,
    pub process: String,
}

#[derive(Debug, Clone)]
pub struct BsdRcSubrHeader {
    pub provides: Vec<String>,
    pub requires: Vec<String>,
    pub before: Vec<String>,
    pub keyword: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LaunchdPlistJob {
    pub label: String,
    pub program_arguments: Vec<String>,
    pub keep_alive: bool,
    pub throttle_interval_sec: u32,
    pub start_calendar_interval: Option<String>,
}

pub struct SysVBsdLaunchdSupervisorEngine {
    pub inittab_entries: Vec<SysVInittabEntry>,
    pub bsd_scripts: BTreeMap<String, BsdRcSubrHeader>,
    pub launchd_jobs: BTreeMap<String, LaunchdPlistJob>,
}

impl SysVBsdLaunchdSupervisorEngine {
    pub fn new() -> Self {
        Self {
            inittab_entries: Vec::new(),
            bsd_scripts: BTreeMap::new(),
            launchd_jobs: BTreeMap::new(),
        }
    }

    pub fn parse_inittab_line(line: &str) -> Option<SysVInittabEntry> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            return None;
        }
        let action = match parts[2].trim() {
            "respawn" => SysVInittabAction::Respawn,
            "sysinit" => SysVInittabAction::SysInit,
            "wait" => SysVInittabAction::Wait,
            "once" => SysVInittabAction::Once,
            "ctrlaltdel" => SysVInittabAction::CtrlAltDel,
            "powerfail" => SysVInittabAction::PowerFail,
            _ => SysVInittabAction::Once,
        };
        Some(SysVInittabEntry {
            id: parts[0].trim().to_string(),
            runlevels: parts[1].trim().to_string(),
            action,
            process: parts[3].trim().to_string(),
        })
    }

    pub fn parse_bsd_rc_header(content: &str) -> BsdRcSubrHeader {
        let mut provides = Vec::new();
        let mut requires = Vec::new();
        let mut before = Vec::new();
        let mut keyword = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if let Some(val) = line.strip_prefix("# PROVIDE:") {
                provides.extend(val.split_whitespace().map(|s| s.to_string()));
            } else if let Some(val) = line.strip_prefix("# REQUIRE:") {
                requires.extend(val.split_whitespace().map(|s| s.to_string()));
            } else if let Some(val) = line.strip_prefix("# BEFORE:") {
                before.extend(val.split_whitespace().map(|s| s.to_string()));
            } else if let Some(val) = line.strip_prefix("# KEYWORD:") {
                keyword.extend(val.split_whitespace().map(|s| s.to_string()));
            }
        }

        BsdRcSubrHeader {
            provides,
            requires,
            before,
            keyword,
        }
    }

    pub fn register_launchd_job(&mut self, job: LaunchdPlistJob) {
        self.launchd_jobs.insert(job.label.clone(), job);
    }
}

// ============================================================================
// 5. Integrated Scheduled Task Execution Suite (Cron, Anacron, Systemd Timers, At)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronMacro {
    Reboot,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone)]
pub struct CronTask {
    pub minute: String,
    pub hour: String,
    pub day_of_month: String,
    pub month: String,
    pub day_of_week: String,
    pub command: String,
    pub cron_macro: Option<CronMacro>,
}

impl CronTask {
    pub fn parse(expression: &str, command: &str) -> Self {
        let expr = expression.trim();
        let cron_macro = match expr {
            "@reboot" => Some(CronMacro::Reboot),
            "@hourly" => Some(CronMacro::Hourly),
            "@daily" => Some(CronMacro::Daily),
            "@weekly" => Some(CronMacro::Weekly),
            "@monthly" => Some(CronMacro::Monthly),
            "@yearly" | "@annually" => Some(CronMacro::Yearly),
            _ => None,
        };

        if cron_macro.is_some() {
            return Self {
                minute: "*".to_string(),
                hour: "*".to_string(),
                day_of_month: "*".to_string(),
                month: "*".to_string(),
                day_of_week: "*".to_string(),
                command: command.to_string(),
                cron_macro,
            };
        }

        let parts: Vec<&str> = expr.split_whitespace().collect();
        if parts.len() >= 5 {
            Self {
                minute: parts[0].to_string(),
                hour: parts[1].to_string(),
                day_of_month: parts[2].to_string(),
                month: parts[3].to_string(),
                day_of_week: parts[4].to_string(),
                command: command.to_string(),
                cron_macro: None,
            }
        } else {
            Self {
                minute: "*".to_string(),
                hour: "*".to_string(),
                day_of_month: "*".to_string(),
                month: "*".to_string(),
                day_of_week: "*".to_string(),
                command: command.to_string(),
                cron_macro: None,
            }
        }
    }

    pub fn matches_schedule(&self, minute: u32, hour: u32) -> bool {
        if let Some(ref m) = self.cron_macro {
            return match m {
                CronMacro::Hourly => minute == 0,
                CronMacro::Daily => minute == 0 && hour == 0,
                CronMacro::Reboot => false,
                _ => minute == 0 && hour == 0,
            };
        }

        let min_match = self.minute == "*" || self.minute.parse::<u32>().map_or(false, |m| m == minute);
        let hr_match = self.hour == "*" || self.hour.parse::<u32>().map_or(false, |h| h == hour);
        min_match && hr_match
    }
}

#[derive(Debug, Clone)]
pub struct AnacronJob {
    pub period_days: u32,
    pub delay_minutes: u32,
    pub job_id: String,
    pub command: String,
    pub last_run_timestamp: u64,
}

pub struct AnacronEngine {
    pub jobs: Vec<AnacronJob>,
    pub random_delay_max_sec: u32,
    pub start_hours_range: (u32, u32),
}

impl AnacronEngine {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            random_delay_max_sec: 300,
            start_hours_range: (6, 22),
        }
    }

    pub fn evaluate_due_jobs(&mut self, current_time: u64, current_hour: u32) -> Vec<String> {
        let mut due = Vec::new();
        if current_hour < self.start_hours_range.0 || current_hour > self.start_hours_range.1 {
            return due;
        }

        for job in &mut self.jobs {
            let period_sec = (job.period_days as u64) * 86400;
            if current_time.saturating_sub(job.last_run_timestamp) >= period_sec {
                due.push(job.job_id.clone());
                job.last_run_timestamp = current_time;
            }
        }
        due
    }
}

#[derive(Debug, Clone)]
pub struct AtJob {
    pub job_id: u32,
    pub scheduled_timestamp: u64,
    pub command: String,
    pub max_load_average: f32,
}

pub struct AtBatchEngine {
    pub queue: Vec<AtJob>,
    pub next_job_id: u32,
}

impl AtBatchEngine {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
            next_job_id: 1,
        }
    }

    pub fn schedule_at(&mut self, timestamp: u64, command: &str, max_load: f32) -> u32 {
        let id = self.next_job_id;
        self.next_job_id += 1;
        self.queue.push(AtJob {
            job_id: id,
            scheduled_timestamp: timestamp,
            command: command.to_string(),
            max_load_average: max_load,
        });
        id
    }

    pub fn process_due_jobs(&mut self, current_time: u64, current_system_load: f32) -> Vec<AtJob> {
        let mut executed = Vec::new();
        let mut remaining = Vec::new();

        for job in self.queue.drain(..) {
            if current_time >= job.scheduled_timestamp && current_system_load <= job.max_load_average {
                executed.push(job);
            } else {
                remaining.push(job);
            }
        }
        self.queue = remaining;
        executed
    }
}

#[derive(Debug, Clone)]
pub struct SystemdTimerJob {
    pub name: String,
    pub on_calendar: String,
    pub persistent: bool,
    pub accuracy_sec: u32,
    pub target_service: String,
    pub last_trigger_time: u64,
}

pub struct SystemdTimerEngine {
    pub timers: BTreeMap<String, SystemdTimerJob>,
}

impl SystemdTimerEngine {
    pub fn new() -> Self {
        Self {
            timers: BTreeMap::new(),
        }
    }

    pub fn register_timer(&mut self, timer: SystemdTimerJob) {
        self.timers.insert(timer.name.clone(), timer);
    }

    pub fn check_timers(&mut self, current_time: u64) -> Vec<String> {
        let mut triggered = Vec::new();
        for timer in self.timers.values_mut() {
            if current_time >= timer.last_trigger_time + 3600 {
                triggered.push(timer.target_service.clone());
                timer.last_trigger_time = current_time;
            }
        }
        triggered
    }
}

// ============================================================================
// 6. Unified Master Coordinator SovereignCoreUserlandInitEngine
// ============================================================================

pub struct SovereignCoreUserlandInitEngine {
    pub legacy_init: InitSystem,
    pub systemd_engine: SystemdUnitEngine,
    pub openrc_engine: OpenRcEngine,
    pub s6_runit_engine: S6RunitEngine,
    pub sysv_bsd_launchd_engine: SysVBsdLaunchdSupervisorEngine,
    pub cron_tasks: Vec<CronTask>,
    pub anacron_engine: AnacronEngine,
    pub at_batch_engine: AtBatchEngine,
    pub systemd_timer_engine: SystemdTimerEngine,
}

impl SovereignCoreUserlandInitEngine {
    pub fn new() -> Self {
        Self {
            legacy_init: InitSystem::new(),
            systemd_engine: SystemdUnitEngine::new(),
            openrc_engine: OpenRcEngine::new(),
            s6_runit_engine: S6RunitEngine::new(),
            sysv_bsd_launchd_engine: SysVBsdLaunchdSupervisorEngine::new(),
            cron_tasks: Vec::new(),
            anacron_engine: AnacronEngine::new(),
            at_batch_engine: AtBatchEngine::new(),
            systemd_timer_engine: SystemdTimerEngine::new(),
        }
    }

    pub fn run_diagnostics(&self) -> String {
        format!(
            "InitEngine Summary: legacy_services={}, systemd_units={}, openrc_scripts={}, s6_services={}, launchd_jobs={}, cron_tasks={}",
            self.legacy_init.services.len(),
            self.systemd_engine.units.len(),
            self.openrc_engine.scripts.len(),
            self.s6_runit_engine.services.len(),
            self.sysv_bsd_launchd_engine.launchd_jobs.len(),
            self.cron_tasks.len()
        )
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_init_topo_sort() {
        let mut init = InitSystem::new();
        init.register_service(Service {
            name: "db".to_string(),
            exec_start: "/usr/bin/db".to_string(),
            restart: true,
            dependencies: vec![],
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 0,
            last_ping: 0,
        });
        init.register_service(Service {
            name: "web".to_string(),
            exec_start: "/usr/bin/web".to_string(),
            restart: true,
            dependencies: vec!["db".to_string()],
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 0,
            last_ping: 0,
        });

        let sorted = init.topo_sort().expect("Topo sort failed");
        assert_eq!(sorted, vec!["db", "web"]);

        init.start_all().expect("Start all failed");
        assert_eq!(init.services.get("web").unwrap().state, ServiceState::Running);
    }

    #[test]
    fn test_systemd_unit_notify_and_deps() {
        let mut engine = SystemdUnitEngine::new();
        engine.register_unit(SystemdUnit {
            name: "network.service".to_string(),
            unit_type: SystemdUnitType::Service,
            description: "Network Daemon".to_string(),
            exec_start: "/sbin/netd".to_string(),
            exec_stop: None,
            dependencies: vec![],
            cgroup_limits: None,
            active: false,
            pid: None,
            status_text: String::new(),
            errno_code: 0,
        });
        engine.register_unit(SystemdUnit {
            name: "httpd.service".to_string(),
            unit_type: SystemdUnitType::Service,
            description: "Web Server".to_string(),
            exec_start: "/usr/sbin/httpd".to_string(),
            exec_stop: None,
            dependencies: vec![SystemdUnitDependency {
                target_unit: "network.service".to_string(),
                kind: SystemdDependencyKind::Requires,
            }],
            cgroup_limits: Some(SystemdCgroupLimits {
                cpu_max_pct: 80,
                memory_max_bytes: 1024 * 1024 * 512,
                io_weight: 100,
            }),
            active: false,
            pid: None,
            status_text: String::new(),
            errno_code: 0,
        });

        let order = engine.evaluate_unit_dependencies("httpd.service").unwrap();
        assert_eq!(order, vec!["network.service", "httpd.service"]);

        let payload = "READY=1\nSTATUS=Processing requests\nMAINPID=4321\n";
        engine.handle_sd_notify("httpd.service", payload).unwrap();

        let srv = engine.units.get("httpd.service").unwrap();
        assert!(srv.active);
        assert_eq!(srv.pid, Some(4321));
        assert_eq!(srv.status_text, "Processing requests");
    }

    #[test]
    fn test_openrc_header_directives_and_runlevels() {
        let header = "provide net\nneed localmount\nuse dns\nbefore firewall\nkeyword -docker\n";
        let directives = OpenRcEngine::parse_header_directives(header);
        assert_eq!(directives.len(), 5);

        let mut engine = OpenRcEngine::new();
        engine.register_script(OpenRcScript {
            name: "networking".to_string(),
            runlevel: OpenRcRunlevel::Boot,
            directives,
            exec_start: "/etc/init.d/networking start".to_string(),
            is_started: false,
        });

        let started = engine.transition_to_runlevel(OpenRcRunlevel::Default).unwrap();
        assert_eq!(started, 1);
        assert!(engine.scripts.get("networking").unwrap().is_started);
    }

    #[test]
    fn test_s6_runit_control_verbs() {
        let mut engine = S6RunitEngine::new();
        engine.register_service("sshd", "/var/service/sshd");

        engine.send_control_verb("sshd", S6RunitControlVerb::Up).unwrap();
        let srv = engine.services.get("sshd").unwrap();
        assert!(srv.is_running);
        assert!(srv.pid.is_some());

        engine.send_control_verb("sshd", S6RunitControlVerb::Pause).unwrap();
        assert!(engine.services.get("sshd").unwrap().is_paused);

        engine.send_control_verb("sshd", S6RunitControlVerb::Down).unwrap();
        assert!(!engine.services.get("sshd").unwrap().is_running);
    }

    #[test]
    fn test_sysv_bsd_launchd_parsers() {
        let entry = SysVBsdLaunchdSupervisorEngine::parse_inittab_line("id:35:respawn:/sbin/getty 38400 tty1").unwrap();
        assert_eq!(entry.id, "id");
        assert_eq!(entry.action, SysVInittabAction::Respawn);

        let bsd_content = "# PROVIDE: netif\n# REQUIRE: ATM\n# BEFORE:  MOUNTCRITLOCAL\n";
        let bsd_hdr = SysVBsdLaunchdSupervisorEngine::parse_bsd_rc_header(bsd_content);
        assert_eq!(bsd_hdr.provides, vec!["netif"]);
        assert_eq!(bsd_hdr.requires, vec!["ATM"]);
        assert_eq!(bsd_hdr.before, vec!["MOUNTCRITLOCAL"]);
    }

    #[test]
    fn test_cron_and_anacron_scheduling() {
        let cron = CronTask::parse("0 12 * * *", "/usr/bin/backup");
        assert!(cron.matches_schedule(0, 12));
        assert!(!cron.matches_schedule(15, 12));

        let cron_macro = CronTask::parse("@hourly", "/usr/bin/sync");
        assert!(cron_macro.matches_schedule(0, 5));

        let mut anacron = AnacronEngine::new();
        anacron.jobs.push(AnacronJob {
            period_days: 1,
            delay_minutes: 5,
            job_id: "daily_logrotate".to_string(),
            command: "/usr/sbin/logrotate".to_string(),
            last_run_timestamp: 100000,
        });

        let due = anacron.evaluate_due_jobs(100000 + 86400 + 10, 10);
        assert_eq!(due, vec!["daily_logrotate"]);
    }

    #[test]
    fn test_at_batch_and_timers() {
        let mut at_engine = AtBatchEngine::new();
        let id = at_engine.schedule_at(1000, "/usr/bin/batch_job", 2.0);
        assert_eq!(id, 1);

        // System load too high (3.0 > 2.0) -> job postponed
        let executed = at_engine.process_due_jobs(1000, 3.0);
        assert!(executed.is_empty());

        // System load normal (1.0 <= 2.0) -> job executed
        let executed = at_engine.process_due_jobs(1000, 1.0);
        assert_eq!(executed.len(), 1);

        let mut timer_engine = SystemdTimerEngine::new();
        timer_engine.register_timer(SystemdTimerJob {
            name: "backup.timer".to_string(),
            on_calendar: "daily".to_string(),
            persistent: true,
            accuracy_sec: 60,
            target_service: "backup.service".to_string(),
            last_trigger_time: 1000,
        });

        let triggered = timer_engine.check_timers(5000);
        assert_eq!(triggered, vec!["backup.service"]);
    }

    #[test]
    fn test_sovereign_core_userland_init_engine() {
        let master = SovereignCoreUserlandInitEngine::new();
        let diag = master.run_diagnostics();
        assert!(diag.contains("InitEngine Summary"));
    }
}
