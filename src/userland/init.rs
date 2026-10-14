use std::collections::{BTreeMap, BTreeSet};
use std::string::String;
use std::vec::Vec;

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
#[derive(Debug, Clone)]
pub struct SystemdCgroupsv2Slice {
    pub slice_name: String,
    pub memory_max_bytes: Option<u64>,
    pub cpu_weight: Option<u32>,
    pub io_weight: Option<u32>,
    pub tasks_max: Option<u32>,
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

/// s6 & runit Control Channel Verb Directives
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorControlVerb {
    Alarm,     // 'a': SIGALRM
    Up,        // 'u': start service
    Down,      // 'd': stop service
    Once,      // 'o': run once
    Exit,      // 'x': terminate supervisor when process exits
    Restart,   // 'r': restart service
    Pause,     // 'p': SIGSTOP
    Continue,  // 'c': SIGCONT
    Interrupt, // 'i': SIGINT
    Term,      // 't': SIGTERM
    Kill,      // 'k': SIGKILL
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

/// s6 & runit Service Definition
#[derive(Debug, Clone)]
pub struct SupervisedService {
    pub service_dir: String,
    pub run_script: String,
    pub finish_script: Option<String>,
    pub state: ServiceState,
    pub pid: Option<u32>,
    pub auto_restart: bool,
}

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

/// SysV `/etc/inittab` Line Entry
#[derive(Debug, Clone)]
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
}

impl BsdRcScript {
    pub fn parse(content: &str) -> Self {
        let mut script = BsdRcScript::default();
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("# PROVIDE:") {
                let rest = &line["# PROVIDE:".len()..];
                script.provides.extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if line.starts_with("# REQUIRE:") {
                let rest = &line["# REQUIRE:".len()..];
                script.requires.extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if line.starts_with("# BEFORE:") {
                let rest = &line["# BEFORE:".len()..];
                script.before.extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if line.starts_with("# KEYWORD:") {
                let rest = &line["# KEYWORD:".len()..];
                script.keywords.extend(rest.split_whitespace().map(|s| s.to_string()));
            } else if line.starts_with("precmd=") {
                script.precmd = Some(line["precmd=".len()..].trim_matches('"').to_string());
            } else if line.starts_with("postcmd=") {
                script.postcmd = Some(line["postcmd=".len()..].trim_matches('"').to_string());
            }
        }
        script
    }
}

/// macOS launchd PLIST Job Specification
#[derive(Debug, Clone)]
pub struct LaunchdJobSpec {
    pub label: String,
    pub program: String,
    pub program_arguments: Vec<String>,
    pub keep_alive: bool,
    pub run_at_load: bool,
    pub throttle_interval_secs: u64,
}

/// Vixie Cron 5/6-Field Expression and Macros
#[derive(Debug, Clone)]
pub struct VixieCronEntry {
    pub expression: String,
    pub command: String,
    pub is_macro: bool,
}

impl VixieCronEntry {
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        if line.starts_with('@') {
            let parts: Vec<&str> = line.splitn(2, char::is_whitespace).collect();
            if parts.len() == 2 {
                return Some(Self {
                    expression: parts[0].to_string(),
                    command: parts[1].trim().to_string(),
                    is_macro: true,
                });
            }
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            let expr = parts[0..5].join(" ");
            let cmd = parts[5..].join(" ");
            return Some(Self {
                expression: expr,
                command: cmd,
                is_macro: false,
            });
        }
        None
    }

    /// Evaluates if the cron expression matches given time parameters: (minute, hour, mday, month, wday).
    pub fn matches_time(&self, min: u32, hour: u32, mday: u32, month: u32, wday: u32) -> bool {
        if self.is_macro {
            return match self.expression.as_str() {
                "@hourly" => min == 0,
                "@daily" | "@midnight" => min == 0 && hour == 0,
                "@weekly" => min == 0 && hour == 0 && wday == 0,
                "@monthly" => min == 0 && hour == 0 && mday == 1,
                "@yearly" | "@annually" => min == 0 && hour == 0 && mday == 1 && month == 1,
                _ => false,
            };
        }

        let fields: Vec<&str> = self.expression.split_whitespace().collect();
        if fields.len() < 5 {
            return false;
        }

        fn field_matches(field: &str, val: u32) -> bool {
            if field == "*" {
                return true;
            }
            if let Ok(v) = field.parse::<u32>() {
                return v == val;
            }
            if field.contains(',') {
                return field.split(',').any(|sub| field_matches(sub, val));
            }
            if let Some((start, end)) = field.split_once('-') {
                if let (Ok(s), Ok(e)) = (start.parse::<u32>(), end.parse::<u32>()) {
                    return val >= s && val <= e;
                }
            }
            if let Some((lhs, step)) = field.split_once('/') {
                if let Ok(st) = step.parse::<u32>() {
                    if lhs == "*" {
                        return val % st == 0;
                    }
                }
            }
            false
        }

        field_matches(fields[0], min)
            && field_matches(fields[1], hour)
            && field_matches(fields[2], mday)
            && field_matches(fields[3], month)
            && field_matches(fields[4], wday)
    }
}

/// Anacron Catchup Job Specification
#[derive(Debug, Clone)]
pub struct AnacronJob {
    pub period_days: u32,
    pub delay_mins: u32,
    pub job_id: String,
    pub command: String,
    pub last_run_timestamp: u64,
    pub random_delay_mins: u32,
    pub start_hours_range: (u32, u32),
}

impl AnacronJob {
    pub fn is_due(&self, now_timestamp: u64, current_hour: u32) -> bool {
        let seconds_in_day = 86400u64;
        let elapsed_days = (now_timestamp.saturating_sub(self.last_run_timestamp)) / seconds_in_day;
        if elapsed_days >= self.period_days as u64 {
            let (start_h, end_h) = self.start_hours_range;
            if current_hour >= start_h && current_hour <= end_h {
                return true;
            }
        }
        false
    }
}

/// `at` and `batch` Delayed Job Queue Entry
#[derive(Debug, Clone)]
pub struct AtJob {
    pub job_id: u64,
    pub execute_at: u64,
    pub command: String,
    pub queue: char, // 'a'..'z' or 'b' for batch
    pub max_load_average: Option<f32>,
}

/// Systemd Timer Specification
#[derive(Debug, Clone)]
pub struct SystemdTimerSpec {
    pub name: String,
    pub unit_to_trigger: String,
    pub on_calendar: Option<String>,
    pub on_boot_sec: Option<u64>,
    pub on_unit_active_sec: Option<u64>,
    pub persistent: bool,
    pub accuracy_sec: u64,
    pub last_trigger_timestamp: u64,
}

impl SystemdTimerSpec {
    pub fn is_triggered(&self, now_timestamp: u64, boot_timestamp: u64, unit_active_timestamp: u64) -> bool {
        if let Some(boot_sec) = self.on_boot_sec {
            if now_timestamp >= boot_timestamp + boot_sec && (self.last_trigger_timestamp == 0 || self.last_trigger_timestamp < boot_timestamp) {
                return true;
            }
        }
        if let Some(active_sec) = self.on_unit_active_sec {
            if unit_active_timestamp > 0 && now_timestamp >= unit_active_timestamp + active_sec {
                if now_timestamp.saturating_sub(self.last_trigger_timestamp) >= active_sec {
                    return true;
                }
            }
        }
        false
    }
}

/// Master Core Userland Init & Task Supervision Engine
pub struct SovereignCoreUserlandInitEngine {
    pub systemd_units: BTreeMap<String, SystemdDependencyGraph>,
    pub systemd_slices: BTreeMap<String, SystemdCgroupsv2Slice>,
    pub openrc_runlevels: BTreeMap<OpenRcRunlevel, Vec<String>>,
    pub openrc_scripts: BTreeMap<String, OpenRcHeader>,
    pub supervised_services: BTreeMap<String, SupervisedService>,
    pub sysv_entries: Vec<SysVInittabEntry>,
    pub bsd_rc_scripts: BTreeMap<String, BsdRcScript>,
    pub launchd_jobs: BTreeMap<String, LaunchdJobSpec>,
    pub cron_entries: Vec<VixieCronEntry>,
    pub anacron_jobs: Vec<AnacronJob>,
    pub at_jobs: Vec<AtJob>,
    pub systemd_timers: BTreeMap<String, SystemdTimerSpec>,
}

impl SovereignCoreUserlandInitEngine {
    pub fn new() -> Self {
        Self {
            systemd_units: BTreeMap::new(),
            systemd_slices: BTreeMap::new(),
            openrc_runlevels: BTreeMap::new(),
            openrc_scripts: BTreeMap::new(),
            supervised_services: BTreeMap::new(),
            sysv_entries: Vec::new(),
            bsd_rc_scripts: BTreeMap::new(),
            launchd_jobs: BTreeMap::new(),
            cron_entries: Vec::new(),
            anacron_jobs: Vec::new(),
            at_jobs: Vec::new(),
            systemd_timers: BTreeMap::new(),
        }
    }

    pub fn register_systemd_unit(&mut self, unit_name: String, deps: SystemdDependencyGraph) {
        self.systemd_units.insert(unit_name, deps);
    }

    pub fn register_openrc_script(&mut self, script_name: String, header: OpenRcHeader) {
        self.openrc_scripts.insert(script_name, header);
    }

    pub fn register_supervised_service(&mut self, name: String, service: SupervisedService) {
        self.supervised_services.insert(name, service);
    }

    pub fn dispatch_supervisor_verb(&mut self, service_name: &str, verb: SupervisorControlVerb) -> Result<ServiceState, &'static str> {
        let srv = self.supervised_services.get_mut(service_name).ok_or("Supervised service not found")?;
        match verb {
            SupervisorControlVerb::Up | SupervisorControlVerb::Continue => {
                srv.state = ServiceState::Running;
                srv.pid = Some(2000 + service_name.len() as u32);
            }
            SupervisorControlVerb::Down | SupervisorControlVerb::Term | SupervisorControlVerb::Kill => {
                srv.state = ServiceState::Stopped;
                srv.pid = None;
            }
            SupervisorControlVerb::Restart => {
                srv.state = ServiceState::Starting;
                srv.state = ServiceState::Running;
                srv.pid = Some(2000 + service_name.len() as u32);
            }
            SupervisorControlVerb::Pause => {
                srv.state = ServiceState::Stopped;
            }
            SupervisorControlVerb::Alarm | SupervisorControlVerb::Once | SupervisorControlVerb::Exit | SupervisorControlVerb::Interrupt => {}
        }
        Ok(srv.state)
    }

    pub fn register_bsd_rc_script(&mut self, script: BsdRcScript) {
        self.bsd_rc_scripts.insert(script.name.clone(), script);
    }

    pub fn register_launchd_job(&mut self, job: LaunchdJobSpec) {
        self.launchd_jobs.insert(job.label.clone(), job);
    }

    pub fn process_cron_tasks(&self, min: u32, hour: u32, mday: u32, month: u32, wday: u32) -> Vec<&VixieCronEntry> {
        self.cron_entries
            .iter()
            .filter(|e| e.matches_time(min, hour, mday, month, wday))
            .collect()
    }

    pub fn process_due_anacron_jobs(&mut self, now_timestamp: u64, current_hour: u32) -> Vec<AnacronJob> {
        let mut due = Vec::new();
        for job in &mut self.anacron_jobs {
            if job.is_due(now_timestamp, current_hour) {
                due.push(job.clone());
                job.last_run_timestamp = now_timestamp;
            }
        }
        due
    }

    pub fn process_at_jobs(&mut self, now_timestamp: u64, current_system_load: f32) -> Vec<AtJob> {
        let mut ready_jobs = Vec::new();
        let mut remaining_jobs = Vec::new();

        for job in self.at_jobs.drain(..) {
            let load_ok = match job.max_load_average {
                Some(limit) => current_system_load <= limit,
                None => true,
            };

            if now_timestamp >= job.execute_at && load_ok {
                ready_jobs.push(job);
            } else {
                remaining_jobs.push(job);
            }
        }

        self.at_jobs = remaining_jobs;
        ready_jobs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_system_topo_sort() {
        let mut init = InitSystem::new();
        init.register_service(Service {
            name: "network".to_string(),
            exec_start: "/bin/net".to_string(),
            restart: true,
            dependencies: vec![],
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 0,
            last_ping: 0,
        });
        init.register_service(Service {
            name: "httpd".to_string(),
            exec_start: "/bin/httpd".to_string(),
            restart: true,
            dependencies: vec!["network".to_string()],
            state: ServiceState::Stopped,
            pid: None,
            watchdog_usec: 0,
            last_ping: 0,
        });

        let sorted = init.topo_sort().unwrap();
        assert_eq!(sorted, vec!["network", "httpd"]);

        init.start_all().unwrap();
        assert_eq!(init.services.get("httpd").unwrap().state, ServiceState::Running);
    }

    #[test]
    fn test_sd_notify_message_parsing() {
        let raw = "READY=1\nSTATUS=Processing requests\nMAINPID=1234\nEXTEND_TIMEOUT_USEC=5000000";
        let msgs = SdNotifyMessage::parse(raw);
        assert_eq!(msgs.len(), 4);
        assert_eq!(msgs[0], SdNotifyMessage::Ready);
        assert_eq!(msgs[1], SdNotifyMessage::Status("Processing requests".to_string()));
        assert_eq!(msgs[2], SdNotifyMessage::MainPid(1234));
        assert_eq!(msgs[3], SdNotifyMessage::ExtendTimeoutUsec(5000000));
    }

    #[test]
    fn test_openrc_header_parse() {
        let script = r#"
            provide net
            need localmount
            use dns
            before sshd
            keyword -docker
        "#;
        let header = OpenRcHeader::parse(script);
        assert_eq!(header.provide, vec!["net"]);
        assert_eq!(header.need, vec!["localmount"]);
        assert_eq!(header.use_deps, vec!["dns"]);
        assert_eq!(header.before, vec!["sshd"]);
        assert_eq!(header.keywords, vec!["-docker"]);
    }

    #[test]
    fn test_supervisor_control_verbs() {
        assert_eq!(SupervisorControlVerb::from_char('u'), Some(SupervisorControlVerb::Up));
        assert_eq!(SupervisorControlVerb::from_char('d'), Some(SupervisorControlVerb::Down));

        let mut engine = SovereignCoreUserlandInitEngine::new();
        engine.register_supervised_service("syslogd".to_string(), SupervisedService {
            service_dir: "/var/service/syslogd".to_string(),
            run_script: "./run".to_string(),
            finish_script: None,
            state: ServiceState::Stopped,
            pid: None,
            auto_restart: true,
        });

        let state = engine.dispatch_supervisor_verb("syslogd", SupervisorControlVerb::Up).unwrap();
        assert_eq!(state, ServiceState::Running);
    }

    #[test]
    fn test_sysv_inittab_parse() {
        let line = "id:2345:respawn:/sbin/getty 38400 tty1";
        let entry = SysVInittabEntry::parse_line(line).unwrap();
        assert_eq!(entry.id, "id");
        assert_eq!(entry.runlevels, vec!['2', '3', '4', '5']);
        assert_eq!(entry.action, SysVAction::Respawn);
        assert_eq!(entry.process, "/sbin/getty 38400 tty1");
    }

    #[test]
    fn test_bsd_rc_script_parse() {
        let content = r#"
            # PROVIDE: sshd
            # REQUIRE: LOGIN
            # BEFORE: SECURELEVEL
            # KEYWORD: shutdown
            precmd="sshd_precmd"
            postcmd="sshd_postcmd"
        "#;
        let script = BsdRcScript::parse(content);
        assert_eq!(script.provides, vec!["sshd"]);
        assert_eq!(script.requires, vec!["LOGIN"]);
        assert_eq!(script.before, vec!["SECURELEVEL"]);
        assert_eq!(script.keywords, vec!["shutdown"]);
        assert_eq!(script.precmd, Some("sshd_precmd".to_string()));
        assert_eq!(script.postcmd, Some("sshd_postcmd".to_string()));
    }

    #[test]
    fn test_vixie_cron_and_macros() {
        let entry = VixieCronEntry::parse("*/15 * * * * /usr/bin/check_updates").unwrap();
        assert!(!entry.is_macro);
        assert!(entry.matches_time(0, 10, 1, 1, 1));
        assert!(entry.matches_time(15, 10, 1, 1, 1));
        assert!(!entry.matches_time(7, 10, 1, 1, 1));

        let macro_entry = VixieCronEntry::parse("@hourly /usr/bin/hourly_cleanup").unwrap();
        assert!(macro_entry.is_macro);
        assert!(macro_entry.matches_time(0, 12, 1, 1, 1));
        assert!(!macro_entry.matches_time(30, 12, 1, 1, 1));
    }

    #[test]
    fn test_anacron_catchup_jobs() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        engine.anacron_jobs.push(AnacronJob {
            period_days: 7,
            delay_mins: 10,
            job_id: "cron.weekly".to_string(),
            command: "/usr/bin/weekly_report".to_string(),
            last_run_timestamp: 100000,
            random_delay_mins: 5,
            start_hours_range: (6, 22),
        });

        // 8 days passed (8 * 86400 = 691200), current hour 10 (within 6..22)
        let due = engine.process_due_anacron_jobs(100000 + 691200, 10);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].job_id, "cron.weekly");
    }

    #[test]
    fn test_systemd_timer_spec() {
        let timer = SystemdTimerSpec {
            name: "backup.timer".to_string(),
            unit_to_trigger: "backup.service".to_string(),
            on_calendar: Some("daily".to_string()),
            on_boot_sec: Some(300),
            on_unit_active_sec: Some(3600),
            persistent: true,
            accuracy_sec: 1,
            last_trigger_timestamp: 0,
        };

        // At timestamp 100 with boot at 0: boot timer (300s) not reached yet
        assert!(!timer.is_triggered(100, 0, 0));

        // At timestamp 350 with boot at 0: boot timer (300s) triggered
        assert!(timer.is_triggered(350, 0, 0));
    }

    #[test]
    fn test_at_and_batch_jobs() {
        let mut engine = SovereignCoreUserlandInitEngine::new();
        engine.at_jobs.push(AtJob {
            job_id: 1,
            execute_at: 1000,
            command: "echo 'hello'".to_string(),
            queue: 'a',
            max_load_average: None,
        });
        engine.at_jobs.push(AtJob {
            job_id: 2,
            execute_at: 1000,
            command: "echo 'heavy batch'".to_string(),
            queue: 'b',
            max_load_average: Some(1.5),
        });

        // at timestamp 1000, load is 2.0. Job 1 executes, Job 2 skipped due to load limit.
        let ready = engine.process_at_jobs(1000, 2.0);
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].job_id, 1);
        assert_eq!(engine.at_jobs.len(), 1);

        // at timestamp 1000, load drops to 1.0. Job 2 executes.
        let ready2 = engine.process_at_jobs(1000, 1.0);
        assert_eq!(ready2.len(), 1);
        assert_eq!(ready2[0].job_id, 2);
        assert_eq!(engine.at_jobs.len(), 0);
    }
}
