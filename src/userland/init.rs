use std::collections::{BTreeMap, BTreeSet};
use std::string::String;
use std::vec::Vec;

// ============================================================================
// Legacy / Compatible InitSystem Primitives
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

#[derive(Debug, Clone, Default)]
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
    }
}

// ============================================================================
// 1. Systemd Unit & Dependency Subsystem
// ============================================================================

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
pub enum SystemdDependency {
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
pub enum SystemdUnitState {
    Inactive,
    Activating,
    Active,
    Deactivating,
    Failed,
}

#[derive(Debug, Clone, Default)]
pub struct SystemdCgroupSliceLimits {
    pub memory_max_mb: Option<u64>,
    pub cpu_weight: Option<u32>,
    pub io_weight: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct SystemdUnit {
    pub name: String,
    pub unit_type: SystemdUnitType,
    pub description: String,
    pub dependencies: Vec<(SystemdDependency, String)>,
    pub exec_start: Option<String>,
    pub exec_stop: Option<String>,
    pub state: SystemdUnitState,
    pub pid: Option<u32>,
    pub cgroup_limits: SystemdCgroupSliceLimits,
    pub status_message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdNotifyMessage {
    Ready,
    Reloading,
    Stopping,
    WatchdogPing,
    Status(String),
    MainPid(u32),
    Errno(i32),
}

impl SdNotifyMessage {
    pub fn parse(payload: &str) -> Option<Self> {
        let trimmed = payload.trim();
        if trimmed == "READY=1" {
            Some(SdNotifyMessage::Ready)
        } else if trimmed == "RELOAD=1" {
            Some(SdNotifyMessage::Reloading)
        } else if trimmed == "STOPPING=1" {
            Some(SdNotifyMessage::Stopping)
        } else if trimmed == "WATCHDOG=1" {
            Some(SdNotifyMessage::WatchdogPing)
        } else if trimmed.starts_with("STATUS=") {
            Some(SdNotifyMessage::Status(trimmed[7..].to_string()))
        } else if trimmed.starts_with("MAINPID=") {
            trimmed[8..].parse::<u32>().ok().map(SdNotifyMessage::MainPid)
        } else if trimmed.starts_with("ERRNO=") {
            trimmed[6..].parse::<i32>().ok().map(SdNotifyMessage::Errno)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemdManager {
    pub units: BTreeMap<String, SystemdUnit>,
}

impl SystemdManager {
    pub fn new() -> Self {
        Self {
            units: BTreeMap::new(),
        }
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.insert(unit.name.clone(), unit);
    }

    pub fn process_sd_notify(&mut self, unit_name: &str, raw_msg: &str) -> Result<(), &'static str> {
        let msg = SdNotifyMessage::parse(raw_msg).ok_or("Invalid sd_notify message")?;
        let unit = self.units.get_mut(unit_name).ok_or("Unit not found")?;

        match msg {
            SdNotifyMessage::Ready => {
                unit.state = SystemdUnitState::Active;
                unit.status_message = "Ready".to_string();
            }
            SdNotifyMessage::Reloading => {
                unit.status_message = "Reloading configuration".to_string();
            }
            SdNotifyMessage::Stopping => {
                unit.state = SystemdUnitState::Deactivating;
                unit.status_message = "Stopping".to_string();
            }
            SdNotifyMessage::WatchdogPing => {
                unit.status_message = "Watchdog ping received".to_string();
            }
            SdNotifyMessage::Status(st) => {
                unit.status_message = st;
            }
            SdNotifyMessage::MainPid(pid) => {
                unit.pid = Some(pid);
            }
            SdNotifyMessage::Errno(e) => {
                unit.state = SystemdUnitState::Failed;
                unit.status_message = format!("Error code {}", e);
            }
        }
        Ok(())
    }

    pub fn start_unit(&mut self, name: &str) -> Result<(), &'static str> {
        let unit = self.units.get_mut(name).ok_or("Unit not found")?;
        unit.state = SystemdUnitState::Activating;
        unit.pid = Some(2000 + name.len() as u32);
        unit.state = SystemdUnitState::Active;
        unit.status_message = "Active (running)".to_string();
        Ok(())
    }

    pub fn stop_unit(&mut self, name: &str) -> Result<(), &'static str> {
        let unit = self.units.get_mut(name).ok_or("Unit not found")?;
        unit.state = SystemdUnitState::Deactivating;
        unit.pid = None;
        unit.state = SystemdUnitState::Inactive;
        unit.status_message = "Inactive (dead)".to_string();
        Ok(())
    }
}

// ============================================================================
// 2. OpenRC Multi-Stage Runlevel & Keyword Subsystem
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum OpenRcRunlevel {
    #[default]
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
    KeywordNoDocker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenRcServiceStatus {
    Stopped,
    Starting,
    Started,
    Stopping,
    Crashed,
}

#[derive(Debug, Clone)]
pub struct OpenRcService {
    pub name: String,
    pub script_path: String,
    pub runlevels: BTreeSet<OpenRcRunlevel>,
    pub keywords: Vec<OpenRcKeyword>,
    pub status: OpenRcServiceStatus,
}

#[derive(Debug, Clone, Default)]
pub struct OpenRcManager {
    pub active_runlevel: OpenRcRunlevel,
    pub services: BTreeMap<String, OpenRcService>,
}

impl OpenRcManager {
    pub fn new() -> Self {
        Self {
            active_runlevel: OpenRcRunlevel::SysInit,
            services: BTreeMap::new(),
        }
    }

    pub fn register_service(&mut self, service: OpenRcService) {
        self.services.insert(service.name.clone(), service);
    }

    pub fn switch_runlevel(&mut self, target: OpenRcRunlevel) -> Result<(), &'static str> {
        self.active_runlevel = target;
        for srv in self.services.values_mut() {
            if srv.runlevels.contains(&target) {
                srv.status = OpenRcServiceStatus::Started;
            } else if srv.status == OpenRcServiceStatus::Started {
                srv.status = OpenRcServiceStatus::Stopped;
            }
        }
        Ok(())
    }

    pub fn parse_service_header(content: &str) -> Vec<OpenRcKeyword> {
        let mut keywords = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("provide ") {
                keywords.push(OpenRcKeyword::Provide(trimmed[8..].trim().to_string()));
            } else if trimmed.starts_with("need ") {
                let deps = trimmed[5..].split_whitespace().map(|s| s.to_string()).collect();
                keywords.push(OpenRcKeyword::Need(deps));
            } else if trimmed.starts_with("use ") {
                let deps = trimmed[4..].split_whitespace().map(|s| s.to_string()).collect();
                keywords.push(OpenRcKeyword::Use(deps));
            } else if trimmed.starts_with("keyword ") && trimmed.contains("-docker") {
                keywords.push(OpenRcKeyword::KeywordNoDocker);
            }
        }
        keywords
    }
}

// ============================================================================
// 3. s6 & runit Supervision Subsystem
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S6RunitControlVerb {
    Up,        // 'u'
    Down,      // 'd'
    Once,      // 'o'
    Exit,      // 'x'
    Restart,   // 'r'
    Pause,     // 'p'
    Continue,  // 'c'
    Alarm,     // 'a'
}

impl S6RunitControlVerb {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'u' => Some(Self::Up),
            'd' => Some(Self::Down),
            'o' => Some(Self::Once),
            'x' => Some(Self::Exit),
            'r' => Some(Self::Restart),
            'p' => Some(Self::Pause),
            'c' => Some(Self::Continue),
            'a' => Some(Self::Alarm),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisionStatus {
    Up,
    Down,
    Paused,
}

#[derive(Debug, Clone)]
pub struct SupervisedService {
    pub service_dir: String,
    pub name: String,
    pub run_script: String,
    pub finish_script: Option<String>,
    pub pid: Option<u32>,
    pub status: SupervisionStatus,
    pub restart_count: u32,
    pub auto_restart: bool,
}

#[derive(Debug, Clone, Default)]
pub struct S6RunitSupervisor {
    pub services: BTreeMap<String, SupervisedService>,
}

impl S6RunitSupervisor {
    pub fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }

    pub fn register_service(&mut self, srv: SupervisedService) {
        self.services.insert(srv.name.clone(), srv);
    }

    pub fn send_control(&mut self, service_name: &str, verb: S6RunitControlVerb) -> Result<(), &'static str> {
        let srv = self.services.get_mut(service_name).ok_or("Supervised service not found")?;
        match verb {
            S6RunitControlVerb::Up => {
                srv.status = SupervisionStatus::Up;
                srv.pid = Some(3000 + srv.name.len() as u32);
            }
            S6RunitControlVerb::Down => {
                srv.status = SupervisionStatus::Down;
                srv.pid = None;
            }
            S6RunitControlVerb::Once => {
                srv.status = SupervisionStatus::Up;
                srv.auto_restart = false;
                srv.pid = Some(3000 + srv.name.len() as u32);
            }
            S6RunitControlVerb::Exit => {
                srv.status = SupervisionStatus::Down;
                srv.pid = None;
                srv.auto_restart = false;
            }
            S6RunitControlVerb::Restart => {
                srv.restart_count += 1;
                srv.status = SupervisionStatus::Up;
                srv.pid = Some(3100 + srv.restart_count);
            }
            S6RunitControlVerb::Pause => {
                srv.status = SupervisionStatus::Paused;
            }
            S6RunitControlVerb::Continue => {
                if srv.status == SupervisionStatus::Paused {
                    srv.status = SupervisionStatus::Up;
                }
            }
            S6RunitControlVerb::Alarm => {
                // Trigger immediate health/watchdog ping
            }
        }
        Ok(())
    }
}

// ============================================================================
// 4. SysVinit & BSD rc Subsystem
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
pub struct SysVinitInittabEntry {
    pub id: String,
    pub runlevels: Vec<u8>,
    pub action: SysVinitAction,
    pub command: String,
}

#[derive(Debug, Clone)]
pub struct BsdRcScriptHeader {
    pub provide: Vec<String>,
    pub require: Vec<String>,
    pub before: Vec<String>,
    pub keyword: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BsdRcScript {
    pub name: String,
    pub header: BsdRcScriptHeader,
    pub precmd: Option<String>,
    pub postcmd: Option<String>,
    pub command: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SysVAndBsdRcManager {
    pub inittab: Vec<SysVinitInittabEntry>,
    pub bsd_scripts: BTreeMap<String, BsdRcScript>,
}

impl SysVAndBsdRcManager {
    pub fn new() -> Self {
        Self {
            inittab: Vec::new(),
            bsd_scripts: BTreeMap::new(),
        }
    }

    pub fn parse_inittab_line(line: &str) -> Option<SysVinitInittabEntry> {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 4 {
            return None;
        }
        let id = parts[0].to_string();
        let runlevels = parts[1]
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();

        let action = match parts[2] {
            "respawn" => SysVinitAction::Respawn,
            "sysinit" => SysVinitAction::SysInit,
            "wait" => SysVinitAction::Wait,
            "once" => SysVinitAction::Once,
            "ctrlaltdel" => SysVinitAction::CtrlAltDel,
            "powerfail" => SysVinitAction::PowerFail,
            _ => SysVinitAction::Once,
        };
        let command = parts[3].to_string();

        Some(SysVinitInittabEntry {
            id,
            runlevels,
            action,
            command,
        })
    }

    pub fn parse_bsd_rc_header(content: &str) -> BsdRcScriptHeader {
        let mut provide = Vec::new();
        let mut require = Vec::new();
        let mut before = Vec::new();
        let mut keyword = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# PROVIDE:") {
                provide = trimmed[10..].split_whitespace().map(|s| s.to_string()).collect();
            } else if trimmed.starts_with("# REQUIRE:") {
                require = trimmed[10..].split_whitespace().map(|s| s.to_string()).collect();
            } else if trimmed.starts_with("# BEFORE:") {
                before = trimmed[9..].split_whitespace().map(|s| s.to_string()).collect();
            } else if trimmed.starts_with("# KEYWORD:") {
                keyword = trimmed[10..].split_whitespace().map(|s| s.to_string()).collect();
            }
        }

        BsdRcScriptHeader {
            provide,
            require,
            before,
            keyword,
        }
    }

    pub fn execute_bsd_script(&self, name: &str) -> Result<String, &'static str> {
        let script = self.bsd_scripts.get(name).ok_or("BSD rc script not found")?;
        if !script.enabled {
            return Err("Script disabled via rcvar");
        }
        let mut exec_log = String::new();
        if let Some(pre) = &script.precmd {
            exec_log.push_str(&format!("precmd: {}; ", pre));
        }
        exec_log.push_str(&format!("cmd: {}; ", script.command));
        if let Some(post) = &script.postcmd {
            exec_log.push_str(&format!("postcmd: {}", post));
        }
        Ok(exec_log)
    }
}

// ============================================================================
// 5. macOS launchd Supervision Subsystem
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchdKeepAlive {
    Always,
    SuccessfulExit(bool),
    PathState(String, bool),
}

#[derive(Debug, Clone)]
pub struct LaunchdJobSpec {
    pub label: String,
    pub program_arguments: Vec<String>,
    pub run_at_load: bool,
    pub keep_alive: Option<LaunchdKeepAlive>,
    pub throttle_interval_sec: u64,
    pub launch_events: Vec<String>,
    pub is_active: bool,
    pub last_launch_time: u64,
}

#[derive(Debug, Clone, Default)]
pub struct LaunchdSupervisor {
    pub jobs: BTreeMap<String, LaunchdJobSpec>,
}

impl LaunchdSupervisor {
    pub fn new() -> Self {
        Self {
            jobs: BTreeMap::new(),
        }
    }

    pub fn register_job(&mut self, job: LaunchdJobSpec) {
        self.jobs.insert(job.label.clone(), job);
    }

    pub fn start_job(&mut self, label: &str, now: u64) -> Result<(), &'static str> {
        let job = self.jobs.get_mut(label).ok_or("Launchd job not found")?;
        if now.saturating_sub(job.last_launch_time) < job.throttle_interval_sec {
            return Err("Job launch throttled by ThrottleInterval");
        }
        job.is_active = true;
        job.last_launch_time = now;
        Ok(())
    }
}

// ============================================================================
// 6. Core Task & Timer Execution Suite (Cron, Anacron, at/batch, Systemd Timers)
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

impl CronMacro {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "@reboot" => Some(CronMacro::Reboot),
            "@hourly" => Some(CronMacro::Hourly),
            "@daily" => Some(CronMacro::Daily),
            "@weekly" => Some(CronMacro::Weekly),
            "@monthly" => Some(CronMacro::Monthly),
            "@yearly" | "@annually" => Some(CronMacro::Yearly),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VixieCronEntry {
    pub spec: String,
    pub macro_spec: Option<CronMacro>,
    pub fields: Vec<String>, // minute, hour, dom, month, dow, optional year
    pub command: String,
    pub last_executed: u64,
}

#[derive(Debug, Clone)]
pub struct AnacronEntry {
    pub period_days: u32,
    pub delay_min: u32,
    pub job_id: String,
    pub command: String,
    pub last_run_timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct AtBatchTask {
    pub task_id: u64,
    pub execute_at: u64,
    pub load_gated: bool,
    pub max_load_average: f32,
    pub command: String,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct SystemdTimerUnit {
    pub name: String,
    pub on_calendar: String,
    pub persistent: bool,
    pub accuracy_sec: u64,
    pub last_trigger: u64,
    pub next_trigger: u64,
}

#[derive(Debug, Clone, Default)]
pub struct CoreTaskScheduler {
    pub cron_entries: Vec<VixieCronEntry>,
    pub anacron_entries: Vec<AnacronEntry>,
    pub at_tasks: Vec<AtBatchTask>,
    pub systemd_timers: BTreeMap<String, SystemdTimerUnit>,
}

impl CoreTaskScheduler {
    pub fn new() -> Self {
        Self {
            cron_entries: Vec::new(),
            anacron_entries: Vec::new(),
            at_tasks: Vec::new(),
            systemd_timers: BTreeMap::new(),
        }
    }

    pub fn parse_crontab_line(line: &str) -> Option<VixieCronEntry> {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            return None;
        }

        if trimmed.starts_with('@') {
            let mut parts = trimmed.splitn(2, whitespace);
            let mac_str = parts.next()?;
            let cmd = parts.next()?.trim().to_string();
            let mac = CronMacro::parse(mac_str)?;
            Some(VixieCronEntry {
                spec: trimmed.to_string(),
                macro_spec: Some(mac),
                fields: Vec::new(),
                command: cmd,
                last_executed: 0,
            })
        } else {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() < 6 {
                return None;
            }
            let fields: Vec<String> = parts[..5].iter().map(|s| s.to_string()).collect();
            let command = parts[5..].join(" ");
            Some(VixieCronEntry {
                spec: trimmed.to_string(),
                macro_spec: None,
                fields,
                command,
                last_executed: 0,
            })
        }
    }

    pub fn evaluate_anacron_catchup(&mut self, now_timestamp: u64, current_system_load: f32) -> Vec<String> {
        let mut triggered_jobs = Vec::new();
        let day_sec = 86400u64;

        for job in &mut self.anacron_entries {
            let elapsed_days = now_timestamp.saturating_sub(job.last_run_timestamp) / day_sec;
            if elapsed_days >= job.period_days as u64 {
                triggered_jobs.push(format!("Anacron job {}: {}", job.job_id, job.command));
                job.last_run_timestamp = now_timestamp;
            }
        }

        for task in &mut self.at_tasks {
            if !task.completed && now_timestamp >= task.execute_at {
                if !task.load_gated || current_system_load <= task.max_load_average {
                    task.completed = true;
                    triggered_jobs.push(format!("at/batch task {}: {}", task.task_id, task.command));
                }
            }
        }

        for timer in self.systemd_timers.values_mut() {
            if now_timestamp >= timer.next_trigger {
                if timer.persistent && timer.last_trigger == 0 {
                    triggered_jobs.push(format!("Timer persistent catchup: {}", timer.name));
                } else {
                    triggered_jobs.push(format!("Timer fired: {}", timer.name));
                }
                timer.last_trigger = now_timestamp;
                timer.next_trigger = now_timestamp + 3600;
            }
        }

        triggered_jobs
    }
}

fn whitespace(c: char) -> bool {
    c.is_whitespace()
}

// ============================================================================
// 7. SovereignCoreUserlandInitEngine Master Orchestrator
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct SovereignCoreUserlandInitEngine {
    pub systemd_mgr: SystemdManager,
    pub openrc_mgr: OpenRcManager,
    pub s6_supervisor: S6RunitSupervisor,
    pub sysv_bsd_mgr: SysVAndBsdRcManager,
    pub launchd_mgr: LaunchdSupervisor,
    pub task_scheduler: CoreTaskScheduler,
    pub legacy_init: InitSystem,
}

impl SovereignCoreUserlandInitEngine {
    pub fn new() -> Self {
        Self {
            systemd_mgr: SystemdManager::new(),
            openrc_mgr: OpenRcManager::new(),
            s6_supervisor: S6RunitSupervisor::new(),
            sysv_bsd_mgr: SysVAndBsdRcManager::new(),
            launchd_mgr: LaunchdSupervisor::new(),
            task_scheduler: CoreTaskScheduler::new(),
            legacy_init: InitSystem::new(),
        }
    }

    pub fn boot_system(&mut self) -> Result<(), &'static str> {
        // Boot OpenRC
        self.openrc_mgr.switch_runlevel(OpenRcRunlevel::SysInit)?;
        self.openrc_mgr.switch_runlevel(OpenRcRunlevel::Boot)?;
        self.openrc_mgr.switch_runlevel(OpenRcRunlevel::Default)?;

        // Start legacy services
        self.legacy_init.start_all()?;

        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_init_system() {
        let mut init = InitSystem::new();
        let db = Service {
            name: "db".to_string(),
            exec_start: "/usr/bin/db".to_string(),
            restart: true,
            dependencies: vec![],
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 5000,
            last_ping: 100,
        };
        let app = Service {
            name: "app".to_string(),
            exec_start: "/usr/bin/app".to_string(),
            restart: true,
            dependencies: vec!["db".to_string()],
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 0,
            last_ping: 0,
        };

        init.register_service(db);
        init.register_service(app);

        assert!(init.start_all().is_ok());
        assert_eq!(init.services.get("db").unwrap().state, ServiceState::Running);
        assert_eq!(init.services.get("app").unwrap().state, ServiceState::Running);

        // Check watchdog timeout
        init.check_watchdogs(10000);
        assert_eq!(init.services.get("db").unwrap().state, ServiceState::Failed);
    }

    #[test]
    fn test_systemd_unit_and_sd_notify() {
        let mut mgr = SystemdManager::new();
        let unit = SystemdUnit {
            name: "sshd.service".to_string(),
            unit_type: SystemdUnitType::Service,
            description: "OpenSSH Daemon".to_string(),
            dependencies: vec![(SystemdDependency::Wants, "network.target".to_string())],
            exec_start: Some("/usr/sbin/sshd".to_string()),
            exec_stop: None,
            state: SystemdUnitState::Inactive,
            pid: None,
            cgroup_limits: SystemdCgroupSliceLimits {
                memory_max_mb: Some(512),
                cpu_weight: Some(100),
                io_weight: None,
            },
            status_message: "Inactive".to_string(),
        };

        mgr.register_unit(unit);
        assert!(mgr.start_unit("sshd.service").is_ok());

        assert!(mgr.process_sd_notify("sshd.service", "READY=1").is_ok());
        assert_eq!(mgr.units.get("sshd.service").unwrap().state, SystemdUnitState::Active);

        assert!(mgr.process_sd_notify("sshd.service", "STATUS=Accepting connections").is_ok());
        assert_eq!(mgr.units.get("sshd.service").unwrap().status_message, "Accepting connections");

        assert!(mgr.process_sd_notify("sshd.service", "MAINPID=4321").is_ok());
        assert_eq!(mgr.units.get("sshd.service").unwrap().pid, Some(4321));

        assert!(mgr.stop_unit("sshd.service").is_ok());
        assert_eq!(mgr.units.get("sshd.service").unwrap().state, SystemdUnitState::Inactive);
    }

    #[test]
    fn test_openrc_manager() {
        let mut mgr = OpenRcManager::new();
        let mut runlevels = BTreeSet::new();
        runlevels.insert(OpenRcRunlevel::Default);

        let service = OpenRcService {
            name: "nginx".to_string(),
            script_path: "/etc/init.d/nginx".to_string(),
            runlevels,
            keywords: vec![OpenRcKeyword::Provide("webserver".to_string())],
            status: OpenRcServiceStatus::Stopped,
        };

        mgr.register_service(service);
        assert!(mgr.switch_runlevel(OpenRcRunlevel::Default).is_ok());
        assert_eq!(mgr.services.get("nginx").unwrap().status, OpenRcServiceStatus::Started);

        let header_content = "provide webserver\nneed net localmount\nkeyword -docker";
        let keywords = OpenRcManager::parse_service_header(header_content);
        assert_eq!(keywords.len(), 3);
        assert!(keywords.contains(&OpenRcKeyword::Provide("webserver".to_string())));
        assert!(keywords.contains(&OpenRcKeyword::KeywordNoDocker));
    }

    #[test]
    fn test_s6_runit_supervisor() {
        let mut sup = S6RunitSupervisor::new();
        let srv = SupervisedService {
            service_dir: "/var/service/getty".to_string(),
            name: "getty".to_string(),
            run_script: "./run".to_string(),
            finish_script: None,
            pid: None,
            status: SupervisionStatus::Down,
            restart_count: 0,
            auto_restart: true,
        };

        sup.register_service(srv);
        assert!(sup.send_control("getty", S6RunitControlVerb::Up).is_ok());
        assert_eq!(sup.services.get("getty").unwrap().status, SupervisionStatus::Up);

        assert!(sup.send_control("getty", S6RunitControlVerb::Pause).is_ok());
        assert_eq!(sup.services.get("getty").unwrap().status, SupervisionStatus::Paused);

        assert!(sup.send_control("getty", S6RunitControlVerb::Continue).is_ok());
        assert_eq!(sup.services.get("getty").unwrap().status, SupervisionStatus::Up);

        assert!(sup.send_control("getty", S6RunitControlVerb::Restart).is_ok());
        assert_eq!(sup.services.get("getty").unwrap().restart_count, 1);
    }

    #[test]
    fn test_sysv_and_bsd_rc_manager() {
        let entry = SysVAndBsdRcManager::parse_inittab_line("id:2345:respawn:/sbin/getty 38400 tty1").unwrap();
        assert_eq!(entry.id, "id");
        assert_eq!(entry.action, SysVinitAction::Respawn);

        let bsd_hdr = "# PROVIDE: ntpd\n# REQUIRE: DAEMON NETWORKING\n# BEFORE: LOGIN";
        let hdr = SysVAndBsdRcManager::parse_bsd_rc_header(bsd_hdr);
        assert_eq!(hdr.provide, vec!["ntpd"]);
        assert_eq!(hdr.require, vec!["DAEMON", "NETWORKING"]);
        assert_eq!(hdr.before, vec!["LOGIN"]);

        let mut mgr = SysVAndBsdRcManager::new();
        mgr.bsd_scripts.insert("ntpd".to_string(), BsdRcScript {
            name: "ntpd".to_string(),
            header: hdr,
            precmd: Some("echo pre".to_string()),
            postcmd: Some("echo post".to_string()),
            command: "/usr/sbin/ntpd -g".to_string(),
            enabled: true,
        });

        let res = mgr.execute_bsd_script("ntpd").unwrap();
        assert!(res.contains("precmd: echo pre"));
        assert!(res.contains("cmd: /usr/sbin/ntpd -g"));
        assert!(res.contains("postcmd: echo post"));
    }

    #[test]
    fn test_launchd_supervisor() {
        let mut sup = LaunchdSupervisor::new();
        let job = LaunchdJobSpec {
            label: "com.apple.syslogd".to_string(),
            program_arguments: vec!["/usr/sbin/syslogd".to_string()],
            run_at_load: true,
            keep_alive: Some(LaunchdKeepAlive::Always),
            throttle_interval_sec: 10,
            launch_events: vec![],
            is_active: false,
            last_launch_time: 100,
        };

        sup.register_job(job);
        assert!(sup.start_job("com.apple.syslogd", 102).is_err()); // Throttled
        assert!(sup.start_job("com.apple.syslogd", 120).is_ok());
        assert!(sup.jobs.get("com.apple.syslogd").unwrap().is_active);
    }

    #[test]
    fn test_core_task_scheduler() {
        let macro_entry = CoreTaskScheduler::parse_crontab_line("@daily /usr/bin/backup").unwrap();
        assert_eq!(macro_entry.macro_spec, Some(CronMacro::Daily));

        let standard_entry = CoreTaskScheduler::parse_crontab_line("0 2 * * * /usr/bin/daily-cleanup").unwrap();
        assert_eq!(standard_entry.fields.len(), 5);

        let mut sched = CoreTaskScheduler::new();
        sched.anacron_entries.push(AnacronEntry {
            period_days: 1,
            delay_min: 5,
            job_id: "cron.daily".to_string(),
            command: "run-parts /etc/cron.daily".to_string(),
            last_run_timestamp: 0,
        });

        sched.at_tasks.push(AtBatchTask {
            task_id: 1,
            execute_at: 500,
            load_gated: true,
            max_load_average: 2.0,
            command: "echo delayed task".to_string(),
            completed: false,
        });

        sched.systemd_timers.insert("backup.timer".to_string(), SystemdTimerUnit {
            name: "backup.timer".to_string(),
            on_calendar: "daily".to_string(),
            persistent: true,
            accuracy_sec: 60,
            last_trigger: 0,
            next_trigger: 1000,
        });

        // 100000s > 86400s (1 day) so anacron triggers, at_task execute_at 500 triggers, timer next_trigger 1000 triggers
        let triggered = sched.evaluate_anacron_catchup(100000, 1.0);
        assert_eq!(triggered.len(), 3);
    }

    #[test]
    fn test_sovereign_core_userland_init_engine() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        assert!(engine.boot_system().is_ok());
        assert_eq!(engine.openrc_mgr.active_runlevel, OpenRcRunlevel::Default);
    }
}
