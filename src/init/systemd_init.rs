extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// Systemd-Grade Init and Target State Engine for SigmaOS
/// Provides robust target dependency graphs, wants/requires properties,
/// and target states to defeat Fedora's Systemd initialization.
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type UnitID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Service,
    Target,
    Socket,
    Timer,
    Path,
    Mount,
    Device,
    Slice,
    Scope,
    Swap,
}

/// Alternative Init System types (Artix, Devuan & BSD parity)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitSystemType {
    SigmaInit, // Default SigmaOS Sovereign Init
    Runit,
    S6,
    Dinit,
    Sysvinit,
    OpenRC,
    BsdRc,
}

/// FreeBSD & OpenBSD rc.d boot execution ordering level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BsdRcOrder {
    EarlyBoot, // e.g. REQUIRE: mountcritlocal, BEFORE: DAEMON
    CoreBoot,  // e.g. REQUIRE: NETWORKING, BEFORE: LOGIN
    LateBoot,  // e.g. REQUIRE: LOGIN
}

/// Linux ProtectSystem directive levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectSystemLevel {
    Off,
    Full,     // /usr and /boot mounted read-only
    Strict,   // /usr, /boot, /etc mounted read-only
}

/// Linux ProtectHome directive levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectHomeLevel {
    Off,
    Bool,     // /home, /root inaccessible
    ReadOnly, // /home, /root read-only
    Tmpfs,    // /home, /root mounted as tmpfs
}

/// Unit Security Hardening Profile (systemd security sandbox + BSD pledge/unveil parity)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemdUnitHardeningProfile {
    pub no_new_privileges: bool,
    pub protect_system: ProtectSystemLevel,
    pub protect_home: ProtectHomeLevel,
    pub private_tmp: bool,
    pub private_devices: bool,
    pub protect_kernel_tunables: bool,
    pub protect_kernel_modules: bool,
    pub restrict_namespaces: bool,
    pub memory_deny_write_execute: bool,
    pub lock_personality: bool,
    pub restrict_realtime: bool,
    pub capability_bounding_set: Vec<String>,
    pub system_call_filter: Vec<String>,
    pub unveil_paths: Vec<(String, String)>, // (path, permissions) - OpenBSD unveil parity
    pub pledge_promises: String,             // OpenBSD pledge parity e.g. "stdio rpath wpath"
}

impl Default for SystemdUnitHardeningProfile {
    fn default() -> Self {
        Self {
            no_new_privileges: false,
            protect_system: ProtectSystemLevel::Off,
            protect_home: ProtectHomeLevel::Off,
            private_tmp: false,
            private_devices: false,
            protect_kernel_tunables: false,
            protect_kernel_modules: false,
            restrict_namespaces: false,
            memory_deny_write_execute: false,
            lock_personality: false,
            restrict_realtime: false,
            capability_bounding_set: Vec::new(),
            system_call_filter: Vec::new(),
            unveil_paths: Vec::new(),
            pledge_promises: String::new(),
        }
    }
}

/// `systemd-analyze security` Auditor Engine
#[derive(Debug, Clone)]
pub struct SecurityAnalysisReport {
    pub unit_name: String,
    pub exposure_score: f32, // 0.0 (OK / Very Secure) to 10.0 (UNSAFE / Unprotected)
    pub rating: String,      // "OK", "EXPOSED", "UNSAFE"
    pub passed_checks: Vec<String>,
    pub warnings: Vec<String>,
}

pub struct SystemdSecurityAuditor;

impl SystemdSecurityAuditor {
    pub fn analyze_profile(unit_name: &str, profile: &SystemdUnitHardeningProfile) -> SecurityAnalysisReport {
        let mut score: f32 = 10.0;
        let mut passed = Vec::new();
        let mut warnings = Vec::new();

        if profile.no_new_privileges {
            score -= 1.0;
            passed.push("NoNewPrivileges=yes".to_string());
        } else {
            warnings.push("NoNewPrivileges is disabled".to_string());
        }

        match profile.protect_system {
            ProtectSystemLevel::Strict => {
                score -= 1.5;
                passed.push("ProtectSystem=strict".to_string());
            }
            ProtectSystemLevel::Full => {
                score -= 1.0;
                passed.push("ProtectSystem=full".to_string());
            }
            ProtectSystemLevel::Off => {
                warnings.push("ProtectSystem is disabled".to_string());
            }
        }

        if profile.protect_home != ProtectHomeLevel::Off {
            score -= 1.0;
            passed.push("ProtectHome enabled".to_string());
        } else {
            warnings.push("ProtectHome is disabled".to_string());
        }

        if profile.private_tmp {
            score -= 0.8;
            passed.push("PrivateTmp=yes".to_string());
        } else {
            warnings.push("PrivateTmp is disabled".to_string());
        }

        if profile.private_devices {
            score -= 0.8;
            passed.push("PrivateDevices=yes".to_string());
        } else {
            warnings.push("PrivateDevices is disabled".to_string());
        }

        if profile.protect_kernel_tunables {
            score -= 0.8;
            passed.push("ProtectKernelTunables=yes".to_string());
        } else {
            warnings.push("ProtectKernelTunables is disabled".to_string());
        }

        if profile.protect_kernel_modules {
            score -= 0.8;
            passed.push("ProtectKernelModules=yes".to_string());
        } else {
            warnings.push("ProtectKernelModules is disabled".to_string());
        }

        if profile.memory_deny_write_execute {
            score -= 1.0;
            passed.push("MemoryDenyWriteExecute=yes".to_string());
        } else {
            warnings.push("MemoryDenyWriteExecute is disabled".to_string());
        }

        if profile.restrict_namespaces {
            score -= 0.8;
            passed.push("RestrictNamespaces=yes".to_string());
        }

        if !profile.pledge_promises.is_empty() {
            score -= 0.8;
            passed.push(format!("OpenBSD Pledge promises: '{}'", profile.pledge_promises));
        }

        if !profile.unveil_paths.is_empty() {
            score -= 0.7;
            passed.push(format!("OpenBSD Unveil path count: {}", profile.unveil_paths.len()));
        }

        let exposure_score = score.max(0.0).min(10.0);
        let rating = if exposure_score <= 3.0 {
            "OK".to_string()
        } else if exposure_score <= 6.5 {
            "EXPOSED".to_string()
        } else {
            "UNSAFE".to_string()
        };

        SecurityAnalysisReport {
            unit_name: unit_name.to_string(),
            exposure_score,
            rating,
            passed_checks: passed,
            warnings,
        }
    }
}

/// Socket activation transport kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketTransportKind {
    StreamTcp,
    DatagramUdp,
    UnixDomain,
    SequentialPacket,
}

/// Extended Socket Unit Configuration (`.socket`)
#[derive(Debug, Clone)]
pub struct SocketConfig {
    pub transport: SocketTransportKind,
    pub listen_address: String,
    pub port: u16,
    pub socket_path: String,
    pub max_connections: u32,
    pub pass_credentials: bool,
    pub socket_user: String,
    pub socket_group: String,
}

impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            transport: SocketTransportKind::StreamTcp,
            listen_address: "0.0.0.0".to_string(),
            port: 8080,
            socket_path: String::new(),
            max_connections: 128,
            pass_credentials: true,
            socket_user: "root".to_string(),
            socket_group: "root".to_string(),
        }
    }
}

/// Timer activation trigger kind (`.timer`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimerTrigger {
    OnBootSec(u64),
    OnStartupSec(u64),
    OnUnitActiveSec(u64),
    OnCalendar(String), // e.g. "*-*-* 00:00:00" or "daily"
}

/// Extended Timer Unit Configuration (`.timer`)
#[derive(Debug, Clone)]
pub struct TimerConfig {
    pub triggers: Vec<TimerTrigger>,
    pub accuracy_sec: u64,
    pub persistent: bool,
    pub randomized_delay_sec: u64,
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            triggers: vec![TimerTrigger::OnBootSec(10)],
            accuracy_sec: 1,
            persistent: true,
            randomized_delay_sec: 0,
        }
    }
}

/// Parallel Stage Solver for BSD rc.d & systemd boot stages
pub struct BsdRcParallelStageSolver;

impl BsdRcParallelStageSolver {
    pub fn compute_parallel_stages(engine: &SystemdEngine, unit_ids: &[UnitID]) -> Vec<Vec<UnitID>> {
        let mut ids = SystemdVec::new();
        for &id in unit_ids {
            ids.push(id);
        }
        if let Ok(sorted) = engine.topological_sort(&ids) {
            let mut stages = Vec::new();
            let mut current_stage = Vec::new();
            for &id in sorted.iter() {
                if let Some(unit) = engine.find_unit(id) {
                    let has_dep = !current_stage.is_empty() && (
                        unit.after.iter().any(|dep| current_stage.contains(dep)) ||
                        current_stage.iter().any(|&prev_id| {
                            if let Some(prev) = engine.find_unit(prev_id) {
                                prev.before.contains(&id)
                            } else {
                                false
                            }
                        })
                    );
                    if has_dep {
                        stages.push(current_stage);
                        current_stage = Vec::new();
                    }
                }
                current_stage.push(id);
            }
            if !current_stage.is_empty() {
                stages.push(current_stage);
            }
            stages
        } else {
            vec![unit_ids.to_vec()]
        }
    }
}

/// Multi-init abstraction bridge allowing boot-time switching across Linux & BSD init models
pub struct InitSystemBridge {
    pub active_init: InitSystemType,
}

impl InitSystemBridge {
    pub fn new(init_type: InitSystemType) -> Self {
        Self {
            active_init: init_type,
        }
    }

    pub fn convert_runit_service_script(&self, service_name: &str) -> Vec<u8> {
        let mut script = Vec::new();
        for &b in b"#!/bin/sh\nexec " {
            script.push(b);
        }
        for &b in service_name.as_bytes() {
            script.push(b);
        }
        for &b in b" --foreground\n" {
            script.push(b);
        }
        script
    }

    /// Converts service configuration to OpenRC runlevel script format (Gentoo/Alpine parity)
    pub fn convert_openrc_service_script(&self, service_name: &str, runlevel: &str) -> Vec<u8> {
        let mut script = Vec::new();
        script.extend_from_slice(b"#!/sbin/openrc-run\n# OpenRC Service Script for ");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"\nrunlevel=");
        script.extend_from_slice(runlevel.as_bytes());
        script.extend_from_slice(b"\ncommand=\"/usr/bin/");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"\"\ncommand_args=\"--daemon\"\n");
        script
    }

    /// Converts service configuration to FreeBSD / OpenBSD rc.d script format (BSD rc.d parity)
    pub fn convert_bsd_rc_script(&self, service_name: &str, order: BsdRcOrder) -> Vec<u8> {
        let mut script = Vec::new();
        script.extend_from_slice(b"#!/bin/sh\n# PROVIDE: ");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"\n");
        match order {
            BsdRcOrder::EarlyBoot => {
                script.extend_from_slice(b"# REQUIRE: mountcritlocal\n# BEFORE: DAEMON\n")
            }
            BsdRcOrder::CoreBoot => {
                script.extend_from_slice(b"# REQUIRE: NETWORKING\n# BEFORE: LOGIN\n")
            }
            BsdRcOrder::LateBoot => script.extend_from_slice(b"# REQUIRE: LOGIN\n"),
        }
        script.extend_from_slice(b"name=\"");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"\"\nrcvar=\"");
        script.extend_from_slice(service_name.as_bytes());
        script.extend_from_slice(b"_enable\"\nload_rc_config $name\nrun_rc_command \"$1\"\n");
        script
    }

    /// Converts a parsed systemd unit into equivalent shell init script according to active_init type
    pub fn export_unit_to_active_init(&self, unit: &ParsedSystemdUnitFile, unit_name: &str) -> Vec<u8> {
        match self.active_init {
            InitSystemType::Runit => self.convert_runit_service_script(unit_name),
            InitSystemType::OpenRC => self.convert_openrc_service_script(unit_name, "default"),
            InitSystemType::BsdRc => self.convert_bsd_rc_script(unit_name, BsdRcOrder::CoreBoot),
            _ => {
                let mut content = Vec::new();
                content.extend_from_slice(b"[Unit]\nDescription=");
                content.extend_from_slice(unit.unit_description.as_bytes());
                content.extend_from_slice(b"\n[Service]\nExecStart=");
                content.extend_from_slice(unit.exec_start.as_bytes());
                content.extend_from_slice(b"\n");
                content
            }
        }
    }
}

// ================= Systemd INI Unit Configuration Parser =================

#[derive(Debug, Clone, Default)]
pub struct ParsedSystemdUnitFile {
    pub unit_description: String,
    pub exec_start: String,
    pub restart_policy: String,
    pub wanted_by: String,
    pub watchdog_sec: u32,
    pub slice: String,
    pub environment: Vec<(String, String)>,
    pub requires: Vec<String>,
    pub wants: Vec<String>,
    pub requisite: Vec<String>,
    pub before: Vec<String>,
    pub after: Vec<String>,
    pub conflicts: Vec<String>,
    pub on_failure: Vec<String>,
    pub hardening_profile: SystemdUnitHardeningProfile,
    pub listen_stream: String,
    pub on_calendar: String,
    pub protect_system: String,
    pub protect_home: String,
    pub oom_score_adjust: i32,
}

pub struct SystemdUnitFileParser;

impl SystemdUnitFileParser {
    pub fn parse_unit_file(content: &str) -> ParsedSystemdUnitFile {
        let mut parsed = ParsedSystemdUnitFile::default();
        let mut current_section = "";

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                current_section = &trimmed[1..trimmed.len() - 1];
                continue;
            }

            if let Some(pos) = trimmed.find('=') {
                let key = trimmed[..pos].trim();
                let val = trimmed[pos + 1..].trim();

                match (current_section, key) {
                    ("Unit", "Description") => parsed.unit_description = val.to_string(),
                    ("Unit", "Requires") => {
                        for dep in val.split_whitespace() {
                            parsed.requires.push(dep.to_string());
                        }
                    }
                    ("Unit", "Wants") => {
                        for dep in val.split_whitespace() {
                            parsed.wants.push(dep.to_string());
                        }
                    }
                    ("Unit", "Requisite") => {
                        for dep in val.split_whitespace() {
                            parsed.requisite.push(dep.to_string());
                        }
                    }
                    ("Unit", "Before") => {
                        for dep in val.split_whitespace() {
                            parsed.before.push(dep.to_string());
                        }
                    }
                    ("Unit", "After") => {
                        for dep in val.split_whitespace() {
                            parsed.after.push(dep.to_string());
                        }
                    }
                    ("Unit", "Conflicts") => {
                        for dep in val.split_whitespace() {
                            parsed.conflicts.push(dep.to_string());
                        }
                    }
                    ("Unit", "OnFailure") => {
                        for dep in val.split_whitespace() {
                            parsed.on_failure.push(dep.to_string());
                        }
                    }
                    ("Service", "ExecStart") => parsed.exec_start = val.to_string(),
                    ("Service", "Restart") => parsed.restart_policy = val.to_string(),
                    ("Service", "WatchdogSec") => {
                        let sec_str = val.trim_end_matches('s');
                        parsed.watchdog_sec = sec_str.parse::<u32>().unwrap_or(0);
                    }
                    ("Service", "Slice") => parsed.slice = val.to_string(),
                    ("Service", "Environment") => {
                        if let Some(eq_idx) = val.find('=') {
                            let env_k = val[..eq_idx].trim().to_string();
                            let env_v = val[eq_idx + 1..].trim().trim_matches('"').to_string();
                            parsed.environment.push((env_k, env_v));
                        }
                    }
                    ("Service", "NoNewPrivileges") => {
                        parsed.hardening_profile.no_new_privileges = val.eq_ignore_ascii_case("yes") || val == "1" || val.eq_ignore_ascii_case("true");
                    }
                    ("Service", "ProtectSystem") => {
                        parsed.hardening_profile.protect_system = match val.to_lowercase().as_str() {
                            "strict" => ProtectSystemLevel::Strict,
                            "full" | "yes" | "true" => ProtectSystemLevel::Full,
                            _ => ProtectSystemLevel::Off,
                        };
                        parsed.protect_system = val.to_string();
                    }
                    ("Service", "ProtectHome") => {
                        parsed.hardening_profile.protect_home = match val.to_lowercase().as_str() {
                            "read-only" => ProtectHomeLevel::ReadOnly,
                            "tmpfs" => ProtectHomeLevel::Tmpfs,
                            "yes" | "true" => ProtectHomeLevel::Bool,
                            _ => ProtectHomeLevel::Off,
                        };
                        parsed.protect_home = val.to_string();
                    }
                    ("Service", "PrivateTmp") => {
                        parsed.hardening_profile.private_tmp = val.eq_ignore_ascii_case("yes") || val == "1" || val.eq_ignore_ascii_case("true");
                    }
                    ("Service", "PrivateDevices") => {
                        parsed.hardening_profile.private_devices = val.eq_ignore_ascii_case("yes") || val == "1" || val.eq_ignore_ascii_case("true");
                    }
                    ("Service", "ProtectKernelTunables") => {
                        parsed.hardening_profile.protect_kernel_tunables = val.eq_ignore_ascii_case("yes") || val == "1" || val.eq_ignore_ascii_case("true");
                    }
                    ("Service", "ProtectKernelModules") => {
                        parsed.hardening_profile.protect_kernel_modules = val.eq_ignore_ascii_case("yes") || val == "1" || val.eq_ignore_ascii_case("true");
                    }
                    ("Service", "MemoryDenyWriteExecute") => {
                        parsed.hardening_profile.memory_deny_write_execute = val.eq_ignore_ascii_case("yes") || val == "1" || val.eq_ignore_ascii_case("true");
                    }
                    ("Service", "Pledge") => {
                        parsed.hardening_profile.pledge_promises = val.to_string();
                    }
                    ("Service", "Unveil") => {
                        if let Some(space_idx) = val.find(' ') {
                            let p = val[..space_idx].trim().to_string();
                            let perm = val[space_idx + 1..].trim().to_string();
                            parsed.hardening_profile.unveil_paths.push((p, perm));
                        }
                    }
                    ("Socket", "ListenStream") => parsed.listen_stream = val.to_string(),
                    ("Timer", "OnCalendar") => parsed.on_calendar = val.to_string(),
                    ("Service", "OOMScoreAdjust") => {
                        parsed.oom_score_adjust = val.parse::<i32>().unwrap_or(0);
                    }
                    ("Install", "WantedBy") => parsed.wanted_by = val.to_string(),
                    _ => {}
                }
            }
        }

        parsed
    }
}

// ================= NixOS / Declarative Unit Generator =================

#[derive(Debug, Clone, Default)]
pub struct DeclarativeUnitSpec {
    pub name: String,
    pub description: String,
    pub exec_command: String,
    pub dependencies: Vec<String>,
    pub environment: Vec<(String, String)>,
    pub restart_policy: String,
}

impl DeclarativeUnitSpec {
    pub fn new(name: &str, exec_command: &str) -> Self {
        Self {
            name: name.to_string(),
            description: String::new(),
            exec_command: exec_command.to_string(),
            dependencies: Vec::new(),
            environment: Vec::new(),
            restart_policy: "on-failure".to_string(),
        }
    }

    pub fn generate_unit_file(&self) -> String {
        let mut out = String::new();
        out.push_str("[Unit]\nDescription=");
        out.push_str(&self.description);
        out.push_str("\n\n[Service]\nExecStart=");
        out.push_str(&self.exec_command);
        out.push_str("\nRestart=");
        out.push_str(&self.restart_policy);
        for (k, v) in &self.environment {
            out.push_str("\nEnvironment=");
            out.push_str(k);
            out.push_str("=");
            out.push_str(v);
        }
        out.push_str("\n\n[Install]\nWantedBy=multi-user.target\n");
        out
    }
}

// ================= Systemd Socket Activation FD Manager =================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketKind {
    Stream,
    Datagram,
    SequentialPacket,
}

#[derive(Debug, Clone)]
pub struct SystemdSocketConfig {
    pub socket_id: UnitID,
    pub listen_address: String,
    pub port: u16,
    pub kind: SocketKind,
    pub bound_fd: i32,
    pub target_service_id: UnitID,
}

#[derive(Debug, Clone)]
pub struct SocketActivationEvent {
    pub socket_id: UnitID,
    pub incoming_bytes: usize,
    pub client_addr: String,
}

pub struct SystemdSocketActivationManager {
    pub sockets: Vec<SystemdSocketConfig>,
    pub active_fds: Vec<(i32, UnitID)>,
}

impl Default for SystemdSocketActivationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemdSocketActivationManager {
    pub fn new() -> Self {
        Self {
            sockets: Vec::new(),
            active_fds: Vec::new(),
        }
    }

    pub fn register_socket(&mut self, config: SystemdSocketConfig) {
        self.active_fds.push((config.bound_fd, config.socket_id));
        self.sockets.push(config);
    }

    pub fn handle_incoming_connection(
        &mut self,
        engine: &mut SystemdEngine,
        event: &SocketActivationEvent,
    ) -> Result<i32, &'static str> {
        let socket_config = self
            .sockets
            .iter()
            .find(|s| s.socket_id == event.socket_id)
            .ok_or("Socket configuration not found")?;
        let target_srv = socket_config.target_service_id;
        let fd = socket_config.bound_fd;

        engine.systemctl_start(target_srv)?;
        Ok(fd)
    }

    pub fn get_passed_fds_for_service(&self, service_id: UnitID) -> Vec<i32> {
        self.sockets
            .iter()
            .filter(|s| s.target_service_id == service_id)
            .map(|s| s.bound_fd)
            .collect()
    }
}

// ================= Systemd Watchdog Health Monitor =================

#[derive(Debug, Clone)]
pub struct SystemdServiceWatchdog {
    pub service_name: String,
    pub watchdog_interval_sec: u32,
    pub last_ping_sec: u64,
    pub is_healthy: bool,
}

impl SystemdServiceWatchdog {
    pub fn new(service_name: &str, watchdog_sec: u32) -> Self {
        Self {
            service_name: service_name.to_string(),
            watchdog_interval_sec: watchdog_sec,
            last_ping_sec: 0,
            is_healthy: true,
        }
    }

    pub fn ping_watchdog(&mut self, now_sec: u64) {
        self.last_ping_sec = now_sec;
        self.is_healthy = true;
    }

    pub fn check_health(&mut self, now_sec: u64) -> bool {
        if self.watchdog_interval_sec == 0 {
            return true;
        }
        let elapsed = now_sec.saturating_sub(self.last_ping_sec);
        if elapsed > (self.watchdog_interval_sec as u64) {
            self.is_healthy = false;
        }
        self.is_healthy
    }
}

// ================= Systemd Cgroup v2 Slice Governor =================

/// Debian Betsy / LMDE Systemd Compatibility Shim
pub struct BetsySystemdCompatShim {
    pub is_systemd_active: bool,
    pub cgroup_v2_mounted: bool,
}

impl BetsySystemdCompatShim {
    pub fn new() -> Self {
        Self {
            is_systemd_active: true,
            cgroup_v2_mounted: true,
        }
    }

    pub fn emulate_systemd_sysv_fallback(&self, init_script_path: &str) -> String {
        format!("systemd-sysv-generator: Emulating SysV init script at '{}'", init_script_path)
    }
}

/// Systemd Transient Unit (`systemd-run`) Service Generator
pub struct TransientServiceGenerator;

impl TransientServiceGenerator {
    pub fn create_transient_unit(cmd: &str, service_name: &str) -> SystemdUnit {
        let mut unit = SystemdUnit::new(999, service_name.as_bytes(), UnitType::Service);
        unit.restart_policy = RestartPolicy::No;
        unit
    }
}

/// Systemd Dynamic User Sandboxing Context (`DynamicUser=yes`)
#[derive(Debug, Clone)]
pub struct SystemdDynamicUserContext {
    pub allocated_uid: u32,
    pub allocated_gid: u32,
    pub private_tmp: bool,
    pub protect_system: String, // e.g. "strict", "full"
}

impl SystemdDynamicUserContext {
    pub fn allocate_ephemeral_user(uid_seed: u32) -> Self {
        Self {
            allocated_uid: 60000 + (uid_seed % 5000),
            allocated_gid: 60000 + (uid_seed % 5000),
            private_tmp: true,
            protect_system: "strict".to_string(),
        }
    }
}

/// Systemd Service Hardening Directives Evaluator (`systemd-analyze security`)
pub struct SystemdServiceHardeningEvaluator;

impl SystemdServiceHardeningEvaluator {
    pub fn calculate_hardening_score(dynamic_user: bool, private_tmp: bool, protect_home: bool) -> f32 {
        let mut score: f32 = 10.0; // Start at 10 (unprotected)
        if dynamic_user {
            score -= 3.0;
        }
        if private_tmp {
            score -= 2.0;
        }
        if protect_home {
            score -= 2.5;
        }
        score.max(0.0) // 0.0 is fully hardened
    }
}

pub struct SystemdCgroupSliceGovernor {
    pub slice_name: String,
    pub cpu_weight: u32,
    pub memory_max_bytes: u64,
    pub current_memory_bytes: u64,
}

impl SystemdCgroupSliceGovernor {
    pub fn new(slice_name: &str, cpu_weight: u32, memory_max_bytes: u64) -> Self {
        Self {
            slice_name: slice_name.to_string(),
            cpu_weight,
            memory_max_bytes,
            current_memory_bytes: 0,
        }
    }

    pub fn allocate_slice_memory(&mut self, bytes: u64) -> Result<(), &'static str> {
        if self.current_memory_bytes + bytes > self.memory_max_bytes {
            return Err("Systemd Slice Cgroup: Memory quota exceeded for slice");
        }
        self.current_memory_bytes += bytes;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitState {
    Active,
    Inactive,
    Activating,
    Deactivating,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    No,
    Always,
    OnFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum JournalPriority {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub unit_id: UnitID,
    pub message: [u8; 64],
    pub from_state: UnitState,
    pub to_state: UnitState,
    pub priority: JournalPriority,
    pub timestamp_ms: u64,
}

impl JournalEntry {
    pub fn new(unit_id: UnitID, msg: &[u8], from_state: UnitState, to_state: UnitState) -> Self {
        let mut msg_arr = [0u8; 64];
        let len = msg.len().min(63);
        msg_arr[..len].copy_from_slice(&msg[..len]);
        JournalEntry {
            unit_id,
            message: msg_arr,
            from_state,
            to_state,
            priority: JournalPriority::Info,
            timestamp_ms: 0,
        }
    }

    pub fn with_priority(mut self, priority: JournalPriority) -> Self {
        self.priority = priority;
        self
    }
}

/// Critical Chain Dependency Path Entry for `systemd-analyze critical-chain`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalChainEntry {
    pub unit_id: UnitID,
    pub unit_name: String,
    pub start_time_ms: u64,
    pub finish_time_ms: u64,
    pub duration_ms: u64,
    pub dependent_unit_id: Option<UnitID>,
}

#[derive(Debug, Clone)]
pub struct SystemdUnit {
    pub id: UnitID,
    pub name: [u8; 32],
    pub unit_type: UnitType,
    pub state: UnitState,
    pub requires: Vec<UnitID>,
    pub requisites: Vec<UnitID>,
    pub wants: Vec<UnitID>,
    pub requisite: Vec<UnitID>,
    pub before: Vec<UnitID>,
    pub after: Vec<UnitID>,
    pub conflicts: Vec<UnitID>,
    pub binds_to: Vec<UnitID>,
    pub part_of: Vec<UnitID>,
    pub on_failure: Vec<UnitID>,
    pub restart_policy: RestartPolicy,
    pub restart_count: usize,
    pub startup_time_ms: u64,
    pub duration_ms: u64,
    pub is_enabled: bool,
    pub triggered_unit: Option<UnitID>,
    pub hardening_profile: SystemdUnitHardeningProfile,
    pub socket_config: Option<SocketConfig>,
    pub timer_config: Option<TimerConfig>,
    pub upholds: Vec<UnitID>,
    pub oom_score_adjust: i32,
}

impl SystemdUnit {
    pub fn new(id: UnitID, name: &[u8], unit_type: UnitType) -> Self {
        let mut name_arr = [0u8; 32];
        let len = name.len().min(31);
        name_arr[..len].copy_from_slice(&name[..len]);
        SystemdUnit {
            id,
            name: name_arr,
            unit_type,
            state: UnitState::Inactive,
            requires: Vec::new(),
            requisites: Vec::new(),
            wants: Vec::new(),
            requisite: Vec::new(),
            before: Vec::new(),
            after: Vec::new(),
            conflicts: Vec::new(),
            binds_to: Vec::new(),
            part_of: Vec::new(),
            on_failure: Vec::new(),
            restart_policy: RestartPolicy::No,
            restart_count: 0,
            startup_time_ms: 0,
            duration_ms: 0,
            is_enabled: true,
            triggered_unit: None,
            hardening_profile: SystemdUnitHardeningProfile::default(),
            socket_config: None,
            timer_config: None,
            upholds: Vec::new(),
            oom_score_adjust: 0,
        }
    }

    pub fn name_as_str(&self) -> &str {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(self.name.len());
        core::str::from_utf8(&self.name[..end]).unwrap_or("unknown")
    }
}

pub struct SystemdEngine {
    pub units: Vec<SystemdUnit>,
    pub current_target: AtomicUsize, // stores UnitID of active target
    pub journal: Vec<JournalEntry>,
}

impl Default for SystemdEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemdEngine {
    pub fn new() -> Self {
        SystemdEngine {
            units: Vec::new(),
            current_target: AtomicUsize::new(0),
            journal: Vec::new(),
        }
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.units.push(unit);
    }

    pub fn find_unit(&self, id: UnitID) -> Option<&SystemdUnit> {
        for unit in self.units.iter() {
            if unit.id == id {
                return Some(unit);
            }
        }
        None
    }

    pub fn find_unit_mut(&mut self, id: UnitID) -> Option<&mut SystemdUnit> {
        for unit in self.units.iter_mut() {
            if unit.id == id {
                return Some(unit);
            }
        }
        None
    }

    pub fn log_journal(
        &mut self,
        unit_id: UnitID,
        message: &[u8],
        from_state: UnitState,
        to_state: UnitState,
    ) {
        let entry = JournalEntry::new(unit_id, message, from_state, to_state);
        self.journal.push(entry);
    }

    pub fn log_journal_with_priority(
        &mut self,
        unit_id: UnitID,
        message: &[u8],
        from_state: UnitState,
        to_state: UnitState,
        priority: JournalPriority,
    ) {
        let entry = JournalEntry::new(unit_id, message, from_state, to_state).with_priority(priority);
        self.journal.push(entry);
    }

    pub fn isolate_target(&mut self, target_id: UnitID) -> Result<(), &'static str> {
        let mut target_idx = None;
        for (i, unit) in self.units.iter().enumerate() {
            if unit.id == target_id && unit.unit_type == UnitType::Target {
                target_idx = Some(i);
                break;
            }
        }

        let _idx = target_idx.ok_or("Target unit not found")?;

        self.units[_idx].state = UnitState::Activating;

        let mut active_set = Vec::new();
        self.collect_dependencies(target_id, &mut active_set);

        for unit in self.units.iter_mut() {
            if !active_set.contains(&unit.id) && unit.id != target_id {
                unit.state = UnitState::Inactive;
            } else if active_set.contains(&unit.id) {
                unit.state = UnitState::Active;
            }
        }

        self.units[_idx].state = UnitState::Active;
        self.current_target.store(target_id, Ordering::SeqCst);
        Ok(())
    }

    fn collect_dependencies(&self, unit_id: UnitID, set: &mut Vec<UnitID>) {
        if set.contains(&unit_id) {
            return;
        }

        for unit in self.units.iter() {
            if unit.id == unit_id {
                for &req in unit.requires.iter() {
                    set.push(req);
                    self.collect_dependencies(req, set);
                }
                for &want in unit.wants.iter() {
                    set.push(want);
                    self.collect_dependencies(want, set);
                }
            }
        }
    }

    pub fn get_active_target_id(&self) -> usize {
        self.current_target.load(Ordering::SeqCst)
    }

    pub fn topological_sort(
        &self,
        unit_ids: &SystemdVec<UnitID>,
    ) -> Result<SystemdVec<UnitID>, &'static str> {
        let mut sorted = SystemdVec::new();
        let mut visiting = SystemdVec::new();
        let mut visited = SystemdVec::new();

        let slice: &[UnitID] = &**unit_ids;
        for &id in unit_ids.iter() {
            if !visited.contains(&id) {
                self.topo_visit(id, slice, &mut sorted, &mut visiting, &mut visited)?;
            }
        }
        Ok(sorted)
    }

    fn topo_visit(
        &self,
        id: UnitID,
        all_ids: &[UnitID],
        sorted: &mut SystemdVec<UnitID>,
        visiting: &mut SystemdVec<UnitID>,
        visited: &mut SystemdVec<UnitID>,
    ) -> Result<(), &'static str> {
        if visiting.contains(&id) {
            return Err("Dependency cycle detected");
        }
        if visited.contains(&id) {
            return Ok(());
        }

        visiting.push(id);

        for &other_id in all_ids.iter() {
            if other_id == id {
                continue;
            }
            let mut is_prereq = false;
            if let Some(unit) = self.find_unit(id) {
                if unit.after.contains(&other_id) {
                    is_prereq = true;
                }
            }
            if let Some(other_unit) = self.find_unit(other_id) {
                if other_unit.before.contains(&id) {
                    is_prereq = true;
                }
            }
            if is_prereq && !visited.contains(&other_id) {
                self.topo_visit(other_id, all_ids, sorted, visiting, visited)?;
            }
        }

        visiting.retain(|&x| x != id);
        visited.push(id);
        sorted.push(id);
        Ok(())
    }

    pub fn systemctl_start(&mut self, id: UnitID) -> Result<(), &'static str> {
        let (is_enabled, conflicts, requires, wants, requisite, on_failure_list) = if let Some(u) = self.find_unit(id) {
            (
                u.is_enabled,
                u.conflicts.clone(),
                u.requires.clone(),
                u.wants.clone(),
                u.requisite.clone(),
                u.on_failure.clone(),
            )
        } else {
            return Err("Unit not found");
        };

        if !is_enabled {
            return Err("Unit is disabled");
        }

        // Check Requisite dependency: if any requisite unit is not currently Active, fail immediately
        for &req_site_id in requisite.iter() {
            let is_req_active = self.find_unit(req_site_id).map_or(false, |u| u.state == UnitState::Active);
            if !is_req_active {
                if let Some(u) = self.find_unit_mut(id) {
                    u.state = UnitState::Failed;
                }
                self.log_journal_with_priority(
                    id,
                    b"Requisite dependency is not active; failing unit start",
                    UnitState::Inactive,
                    UnitState::Failed,
                    JournalPriority::Error,
                );
                self.trigger_on_failure_cascade(&on_failure_list);
                return Err("Requisite dependency is not active");
            }
        }

        for &conflict_id in conflicts.iter() {
            if let Some(conf_unit) = self.find_unit(conflict_id) {
                if conf_unit.state == UnitState::Active {
                    self.systemctl_stop(conflict_id)?;
                }
            }
        }

        let mut units_to_stop = Vec::new();
        for u in self.units.iter() {
            if u.conflicts.contains(&id) && u.state == UnitState::Active {
                units_to_stop.push(u.id);
            }
        }
        for &stop_id in units_to_stop.iter() {
            self.systemctl_stop(stop_id)?;
        }

        let requisites = if let Some(u) = self.find_unit(id) {
            u.requisites.clone()
        } else {
            Vec::new()
        };

        for &req_id in requisites.iter() {
            let is_active = self.find_unit(req_id).map(|u| u.state == UnitState::Active).unwrap_or(false);
            if !is_active {
                if let Some(u) = self.find_unit_mut(id) {
                    u.state = UnitState::Failed;
                }
                self.log_journal_with_priority(
                    id,
                    b"Requisite dependency is not active",
                    UnitState::Inactive,
                    UnitState::Failed,
                    JournalPriority::Error,
                );
                return Err("Requisite dependency is not active");
            }
        }

        for &req_id in requires.iter() {
            if self.systemctl_start(req_id).is_err() {
                if let Some(u) = self.find_unit_mut(id) {
                    u.state = UnitState::Failed;
                }
                self.log_journal_with_priority(
                    id,
                    b"Required dependency failed to start",
                    UnitState::Inactive,
                    UnitState::Failed,
                    JournalPriority::Error,
                );
                self.trigger_on_failure_cascade(&on_failure_list);
                return Err("Required dependency failed to start");
            }
        }

        for &want_id in wants.iter() {
            let _ = self.systemctl_start(want_id);
        }

        let upholds = if let Some(u) = self.find_unit(id) {
            u.upholds.clone()
        } else {
            Vec::new()
        };

        for &uphold_id in upholds.iter() {
            let _ = self.systemctl_start(uphold_id);
        }

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Activating;
        }
        self.log_journal(
            id,
            b"Unit starting/activating",
            UnitState::Inactive,
            UnitState::Activating,
        );

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Active;
            u.startup_time_ms = 100;
            u.duration_ms = 150;
        }
        self.log_journal(
            id,
            b"Unit started successfully",
            UnitState::Activating,
            UnitState::Active,
        );

        Ok(())
    }

    fn trigger_on_failure_cascade(&mut self, on_failure_units: &[UnitID]) {
        for &fail_target in on_failure_units {
            let _ = self.systemctl_start(fail_target);
        }
    }

    pub fn systemctl_stop(&mut self, id: UnitID) -> Result<(), &'static str> {
        let exists = self.find_unit(id).is_some();
        if !exists {
            return Err("Unit not found");
        }

        if let Some(u) = self.find_unit_mut(id) {
            if u.state == UnitState::Inactive {
                return Ok(());
            }
            u.state = UnitState::Deactivating;
        }
        self.log_journal(
            id,
            b"Unit stopping/deactivating",
            UnitState::Active,
            UnitState::Deactivating,
        );

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Inactive;
        }
        self.log_journal(
            id,
            b"Unit stopped successfully",
            UnitState::Deactivating,
            UnitState::Inactive,
        );

        let mut bound_units = Vec::new();
        for u in self.units.iter() {
            if u.binds_to.contains(&id) && u.state == UnitState::Active {
                bound_units.push(u.id);
            }
        }

        for &b_id in bound_units.iter() {
            self.systemctl_stop(b_id)?;
        }

        Ok(())
    }

    pub fn systemctl_restart(&mut self, id: UnitID) -> Result<(), &'static str> {
        self.systemctl_stop(id)?;
        self.systemctl_start(id)?;
        Ok(())
    }

    pub fn systemctl_enable(&mut self, id: UnitID) -> Result<(), &'static str> {
        if let Some(u) = self.find_unit_mut(id) {
            u.is_enabled = true;
            Ok(())
        } else {
            Err("Unit not found")
        }
    }

    pub fn systemctl_disable(&mut self, id: UnitID) -> Result<(), &'static str> {
        if let Some(u) = self.find_unit_mut(id) {
            u.is_enabled = false;
            Ok(())
        } else {
            Err("Unit not found")
        }
    }

    pub fn systemctl_reload(&mut self, id: UnitID) -> Result<(), &'static str> {
        let (state, is_active) = if let Some(u) = self.find_unit(id) {
            (u.state, u.state == UnitState::Active)
        } else {
            return Err("Unit not found");
        };
        if !is_active {
            return Err("Cannot reload inactive unit");
        }

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Activating;
        }
        self.log_journal(
            id,
            b"Reloading unit configuration",
            state,
            UnitState::Activating,
        );

        if let Some(u) = self.find_unit_mut(id) {
            u.state = UnitState::Active;
        }
        self.log_journal(
            id,
            b"Unit reloaded successfully",
            UnitState::Activating,
            UnitState::Active,
        );
        Ok(())
    }

    pub fn systemctl_status(&self, id: UnitID) -> Option<UnitState> {
        self.find_unit(id).map(|u| u.state)
    }

    pub fn handle_unit_failure(&mut self, id: UnitID) -> Result<bool, &'static str> {
        let mut should_restart = false;
        let (policy, count, on_failure_list) = if let Some(unit) = self.find_unit(id) {
            (unit.restart_policy, unit.restart_count, unit.on_failure.clone())
        } else {
            return Err("Unit not found");
        };

        if policy == RestartPolicy::Always || policy == RestartPolicy::OnFailure {
            if count < 3 {
                should_restart = true;
            }
        }

        if should_restart {
            if let Some(unit) = self.find_unit_mut(id) {
                unit.restart_count += 1;
                unit.state = UnitState::Activating;
            }
            self.log_journal(
                id,
                b"Auto-restarting failed unit based on restart policy",
                UnitState::Failed,
                UnitState::Activating,
            );
            self.systemctl_start(id)?;
            Ok(true)
        } else {
            if let Some(unit) = self.find_unit_mut(id) {
                unit.state = UnitState::Failed;
            }
            self.log_journal_with_priority(
                id,
                b"Unit entered Failed state, restart policy not met or limit exceeded",
                UnitState::Inactive,
                UnitState::Failed,
                JournalPriority::Error,
            );
            self.trigger_on_failure_cascade(&on_failure_list);
            Ok(false)
        }
    }

    pub fn trigger_socket_activation(&mut self, socket_id: UnitID) -> Result<(), &'static str> {
        let mut triggered_id = None;
        if let Some(unit) = self.find_unit(socket_id) {
            if unit.unit_type != UnitType::Socket {
                return Err("Unit is not a Socket");
            }
            triggered_id = unit.triggered_unit;
        }

        if let Some(srv_id) = triggered_id {
            self.log_journal(
                socket_id,
                b"Socket activation triggered",
                UnitState::Active,
                UnitState::Active,
            );
            self.systemctl_start(srv_id)?;
            Ok(())
        } else {
            Err("No triggered unit configured for this socket")
        }
    }

    pub fn trigger_path_activation(&mut self, path_id: UnitID) -> Result<(), &'static str> {
        let mut triggered_id = None;
        if let Some(unit) = self.find_unit(path_id) {
            if unit.unit_type != UnitType::Path {
                return Err("Unit is not a Path");
            }
            triggered_id = unit.triggered_unit;
        }

        if let Some(srv_id) = triggered_id {
            self.log_journal(
                path_id,
                b"Path modification activation triggered",
                UnitState::Active,
                UnitState::Active,
            );
            self.systemctl_start(srv_id)?;
            Ok(())
        } else {
            Err("No triggered unit configured for this path")
        }
    }

    pub fn trigger_timer_activation(&mut self, timer_id: UnitID) -> Result<(), &'static str> {
        let mut triggered_id = None;
        if let Some(unit) = self.find_unit(timer_id) {
            if unit.unit_type != UnitType::Timer {
                return Err("Unit is not a Timer");
            }
            triggered_id = unit.triggered_unit;
        }

        if let Some(srv_id) = triggered_id {
            self.log_journal(
                timer_id,
                b"Timer activation triggered",
                UnitState::Active,
                UnitState::Active,
            );
            self.systemctl_start(srv_id)?;
            Ok(())
        } else {
            Err("No triggered unit configured for this timer")
        }
    }

    pub fn systemd_analyze_blame(&self) -> SystemdVec<(UnitID, u64)> {
        let mut blame_list = SystemdVec::new();
        for unit in self.units.iter() {
            if unit.state == UnitState::Active {
                blame_list.push((unit.id, unit.duration_ms));
            }
        }
        for i in 0..blame_list.len() {
            for j in 0..blame_list.len() - 1 - i {
                if blame_list[j].1 < blame_list[j + 1].1 {
                    let temp = blame_list[j].clone();
                    blame_list[j] = blame_list[j + 1].clone();
                    blame_list[j + 1] = temp;
                }
            }
        }
        blame_list
    }

    /// Traces the critical boot dependency chain leading to target_id (`systemd-analyze critical-chain`)
    pub fn systemd_analyze_critical_chain(&self, target_id: UnitID) -> Vec<CriticalChainEntry> {
        let mut chain = Vec::new();
        let mut visited = Vec::new();
        let mut current_id = target_id;
        let mut current_time_offset: u64 = 0;

        while let Some(unit) = self.find_unit(current_id) {
            if visited.contains(&current_id) {
                break; // Dependency cycle guard
            }
            visited.push(current_id);

            let start = current_time_offset;
            let finish = start + unit.duration_ms;
            current_time_offset = finish;

            // Find prerequisite dependency (Requires/After) that took longest or was primary
            let mut prev_dep = None;
            for &req in unit.requires.iter().chain(unit.after.iter()) {
                if !visited.contains(&req) && self.find_unit(req).is_some() {
                    prev_dep = Some(req);
                    break;
                }
            }

            chain.push(CriticalChainEntry {
                unit_id: unit.id,
                unit_name: unit.name_as_str().to_string(),
                start_time_ms: start,
                finish_time_ms: finish,
                duration_ms: unit.duration_ms,
                dependent_unit_id: prev_dep,
            });

            if let Some(next_id) = prev_dep {
                current_id = next_id;
            } else {
                break;
            }
        }

        chain
    }

    /// Evaluates `systemd-analyze security` across all registered units
    pub fn systemd_analyze_security(&self) -> Vec<SecurityAnalysisReport> {
        let mut reports = Vec::new();
        for unit in self.units.iter() {
            if unit.unit_type == UnitType::Service {
                let report = SystemdSecurityAuditor::analyze_profile(unit.name_as_str(), &unit.hardening_profile);
                reports.push(report);
            }
        }
        reports
    }

    pub fn query_target_by_name(&self, name: &[u8]) -> Option<UnitID> {
        for unit in self.units.iter() {
            if unit.unit_type == UnitType::Target {
                let len = name.len().min(32);
                if unit.name[..len] == name[..len] {
                    return Some(unit.id);
                }
            }
        }
        None
    }
}

/// Sovereign Systemd Betsy Engine uniting Init supervision, Unit parsing, Watchdogs, Cgroup slices, and Multi-init bridging
pub struct SystemdBetsyEngine {
    pub engine: SystemdEngine,
    pub watchdogs: BTreeMap<String, SystemdServiceWatchdog>,
    pub slice_governors: BTreeMap<String, SystemdCgroupSliceGovernor>,
    pub init_bridge: InitSystemBridge,
    pub target_name: String,
}

impl SystemdBetsyEngine {
    pub fn new(default_target: &str) -> Self {
        Self {
            engine: SystemdEngine::new(),
            watchdogs: BTreeMap::new(),
            slice_governors: BTreeMap::new(),
            init_bridge: InitSystemBridge::new(InitSystemType::SigmaInit),
            target_name: default_target.to_string(),
        }
    }

    pub fn register_unit(&mut self, unit: SystemdUnit) {
        self.engine.register_unit(unit);
    }

    pub fn parse_and_load_unit_file(&mut self, unit_id: UnitID, unit_name: &str, file_content: &str, unit_type: UnitType) -> ParsedSystemdUnitFile {
        let parsed = SystemdUnitFileParser::parse_unit_file(file_content);
        let mut unit = SystemdUnit::new(unit_id, unit_name.as_bytes(), unit_type);
        if parsed.restart_policy == "always" {
            unit.restart_policy = RestartPolicy::Always;
        } else if parsed.restart_policy == "on-failure" {
            unit.restart_policy = RestartPolicy::OnFailure;
        }
        self.engine.register_unit(unit);

        if parsed.watchdog_sec > 0 {
            self.watchdogs.insert(
                unit_name.to_string(),
                SystemdServiceWatchdog::new(unit_name, parsed.watchdog_sec),
            );
        }

        if !parsed.slice.is_empty() {
            self.slice_governors.entry(parsed.slice.clone()).or_insert_with(|| {
                SystemdCgroupSliceGovernor::new(&parsed.slice, 100, 512 * 1024 * 1024)
            });
        }

        parsed
    }

    pub fn systemctl_start(&mut self, id: UnitID) -> Result<(), &'static str> {
        self.engine.systemctl_start(id)
    }

    pub fn systemctl_stop(&mut self, id: UnitID) -> Result<(), &'static str> {
        self.engine.systemctl_stop(id)
    }

    pub fn systemctl_restart(&mut self, id: UnitID) -> Result<(), &'static str> {
        self.engine.systemctl_restart(id)
    }

    pub fn query_unit_state(&self, id: UnitID) -> Option<UnitState> {
        self.engine.systemctl_status(id)
    }

    pub fn analyze_blame(&self) -> SystemdVec<(UnitID, u64)> {
        self.engine.systemd_analyze_blame()
    }

    pub fn register_watchdog(&mut self, service_name: &str, interval_sec: u32) {
        self.watchdogs.insert(
            service_name.to_string(),
            SystemdServiceWatchdog::new(service_name, interval_sec),
        );
    }

    pub fn ping_watchdog(&mut self, service_name: &str, now_sec: u64) -> bool {
        if let Some(wd) = self.watchdogs.get_mut(service_name) {
            wd.ping_watchdog(now_sec);
            true
        } else {
            false
        }
    }

    pub fn check_watchdog_health(&mut self, service_name: &str, now_sec: u64) -> bool {
        if let Some(wd) = self.watchdogs.get_mut(service_name) {
            wd.check_health(now_sec)
        } else {
            true
        }
    }

    pub fn configure_slice(&mut self, slice_name: &str, cpu_weight: u32, memory_max_bytes: u64) -> Result<(), &'static str> {
        let governor = SystemdCgroupSliceGovernor::new(slice_name, cpu_weight, memory_max_bytes);
        self.slice_governors.insert(slice_name.to_string(), governor);
        Ok(())
    }
}

impl Default for SystemdBetsyEngine {
    fn default() -> Self {
        Self::new("multi-user.target")
    }
}

pub struct SystemdVec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> core::ops::Deref for SystemdVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T: PartialEq> SystemdVec<T> {
    pub fn contains(&self, item: &T) -> bool {
        for i in 0..self.len {
            unsafe {
                if &*self.data.add(i) == item {
                    return true;
                }
            }
        }
        false
    }
}

impl<T: Clone> Clone for SystemdVec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = SystemdVec::new();
        for item in self.iter() {
            new_vec.push(item.clone());
        }
        new_vec
    }
}

impl<T> SystemdVec<T> {
    pub fn new() -> Self {
        SystemdVec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut write_idx = 0;
        for i in 0..self.len {
            unsafe {
                let item = &*self.data.add(i);
                if f(item) {
                    if write_idx != i {
                        core::ptr::copy_nonoverlapping(
                            self.data.add(i),
                            self.data.add(write_idx),
                            1,
                        );
                    }
                    write_idx += 1;
                } else {
                    core::ptr::drop_in_place(self.data.add(i));
                }
            }
        }
        self.len = write_idx;
    }

    pub fn iter(&self) -> VecIter<'_, T> {
        VecIter {
            vec: self,
            index: 0,
        }
    }

    pub fn iter_mut(&mut self) -> VecIterMut<'_, T> {
        VecIterMut {
            data: self.data,
            len: self.len,
            index: 0,
            _marker: core::marker::PhantomData,
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * core::mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            if self.capacity > 0 && !self.data.is_null() {
                for i in 0..self.len {
                    core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
                }
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for SystemdVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for SystemdVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for SystemdVec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 && !self.data.is_null() {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

pub struct VecIter<'a, T> {
    vec: &'a SystemdVec<T>,
    index: usize,
}

impl<'a, T> Iterator for VecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len() {
            let item = unsafe { &*self.vec.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

pub struct VecIterMut<'a, T> {
    data: *mut T,
    len: usize,
    index: usize,
    _marker: core::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for VecIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let item = unsafe { &mut *self.data.add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use alloc::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_systemd_engine_dependency_isolation() {
        let mut engine = SystemdEngine::new();

        let mut graphical = SystemdUnit::new(100, b"graphical.target", UnitType::Target);
        graphical.requires.push(200);
        graphical.wants.push(300);

        let multi_user = SystemdUnit::new(200, b"multi-user.target", UnitType::Target);
        let network = SystemdUnit::new(300, b"network.target", UnitType::Target);
        let service = SystemdUnit::new(400, b"apache.service", UnitType::Service);

        engine.register_unit(graphical);
        engine.register_unit(multi_user);
        engine.register_unit(network);
        engine.register_unit(service);

        engine.isolate_target(100).unwrap();

        assert_eq!(engine.get_active_target_id(), 100);

        for unit in engine.units.iter() {
            if unit.id == 200 || unit.id == 300 || unit.id == 100 {
                assert_eq!(unit.state, UnitState::Active);
            } else if unit.id == 400 {
                assert_eq!(unit.state, UnitState::Inactive);
            }
        }
    }

    #[test]
    fn test_systemd_topological_sort_and_cycle_detection() {
        let mut engine = SystemdEngine::new();

        let mut a = SystemdUnit::new(1, b"a.service", UnitType::Service);
        a.before.push(2);

        let mut b = SystemdUnit::new(2, b"b.service", UnitType::Service);
        b.before.push(3);

        let c = SystemdUnit::new(3, b"c.service", UnitType::Service);

        engine.register_unit(a);
        engine.register_unit(b);
        engine.register_unit(c);

        let mut ids = SystemdVec::new();
        ids.push(3);
        ids.push(2);
        ids.push(1);

        let sorted = engine.topological_sort(&ids).unwrap();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0], 1);
        assert_eq!(sorted[1], 2);
        assert_eq!(sorted[2], 3);

        let mut engine_cycle = SystemdEngine::new();
        let mut u1 = SystemdUnit::new(10, b"u1.service", UnitType::Service);
        u1.before.push(20);
        let mut u2 = SystemdUnit::new(20, b"u2.service", UnitType::Service);
        u2.before.push(10);

        engine_cycle.register_unit(u1);
        engine_cycle.register_unit(u2);

        let mut cycle_ids = SystemdVec::new();
        cycle_ids.push(10);
        cycle_ids.push(20);

        assert!(engine_cycle.topological_sort(&cycle_ids).is_err());
    }

    #[test]
    fn test_systemd_conflicts_and_binds_to() {
        let mut engine = SystemdEngine::new();

        let mut a = SystemdUnit::new(1, b"a.service", UnitType::Service);
        a.conflicts.push(2);

        let mut b = SystemdUnit::new(2, b"b.service", UnitType::Service);
        b.state = UnitState::Active;

        let mut c = SystemdUnit::new(3, b"c.service", UnitType::Service);
        c.binds_to.push(1);
        c.state = UnitState::Active;

        engine.register_unit(a);
        engine.register_unit(b);
        engine.register_unit(c);

        engine.systemctl_start(1).unwrap();

        assert_eq!(engine.systemctl_status(1), Some(UnitState::Active));
        assert_eq!(engine.systemctl_status(2), Some(UnitState::Inactive));

        engine.systemctl_stop(1).unwrap();
        assert_eq!(engine.systemctl_status(1), Some(UnitState::Inactive));
        assert_eq!(engine.systemctl_status(3), Some(UnitState::Inactive));
    }

    #[test]
    fn test_systemd_activation_triggers() {
        let mut engine = SystemdEngine::new();

        let mut socket = SystemdUnit::new(1, b"test.socket", UnitType::Socket);
        socket.triggered_unit = Some(10);

        let srv = SystemdUnit::new(10, b"test.service", UnitType::Service);

        engine.register_unit(socket);
        engine.register_unit(srv);

        assert_eq!(engine.systemctl_status(10), Some(UnitState::Inactive));

        engine.trigger_socket_activation(1).unwrap();
        assert_eq!(engine.systemctl_status(10), Some(UnitState::Active));
    }

    #[test]
    fn test_systemd_restart_policy() {
        let mut engine = SystemdEngine::new();

        let mut fail_srv = SystemdUnit::new(1, b"fail.service", UnitType::Service);
        fail_srv.restart_policy = RestartPolicy::Always;

        engine.register_unit(fail_srv);

        let restarted = engine.handle_unit_failure(1).unwrap();
        assert!(restarted);
        assert_eq!(engine.find_unit(1).unwrap().restart_count, 1);
        assert_eq!(engine.systemctl_status(1), Some(UnitState::Active));
    }

    #[test]
    fn test_systemd_analyze_blame() {
        let mut engine = SystemdEngine::new();

        let mut a = SystemdUnit::new(1, b"a.service", UnitType::Service);
        a.state = UnitState::Active;
        a.duration_ms = 500;

        let mut b = SystemdUnit::new(2, b"b.service", UnitType::Service);
        b.state = UnitState::Active;
        b.duration_ms = 1200;

        engine.register_unit(a);
        engine.register_unit(b);

        let blame = engine.systemd_analyze_blame();
        assert_eq!(blame.len(), 2);
        assert_eq!(blame[0].0, 2);
        assert_eq!(blame[0].1, 1200);
        assert_eq!(blame[1].0, 1);
        assert_eq!(blame[1].1, 500);
    }

    #[test]
    fn test_advanced_dependency_sorting() {
        let mut engine = SystemdEngine::new();

        let mut basic = SystemdUnit::new(1, b"basic.target", UnitType::Target);
        basic.before.push(2);

        let mut network = SystemdUnit::new(2, b"network.target", UnitType::Target);
        network.before.push(3);

        let multi_user = SystemdUnit::new(3, b"multi-user.target", UnitType::Target);

        engine.register_unit(basic);
        engine.register_unit(network);
        engine.register_unit(multi_user);

        let mut ids = SystemdVec::new();
        ids.push(3);
        ids.push(2);
        ids.push(1);

        let sorted = engine.topological_sort(&ids).unwrap();
        assert_eq!(sorted.len(), 3);
        assert_eq!(sorted[0], 1);
        assert_eq!(sorted[1], 2);
        assert_eq!(sorted[2], 3);
    }

    #[test]
    fn test_systemctl_management() {
        let mut engine = SystemdEngine::new();
        let target = SystemdUnit::new(99, b"multi-user.target", UnitType::Target);
        engine.register_unit(target);

        let queried = engine.query_target_by_name(b"multi-user.target");
        assert_eq!(queried, Some(99));

        let non_existent = engine.query_target_by_name(b"non-existent.target");
        assert_eq!(non_existent, None);
    }

    #[test]
    fn test_systemd_betsy_engine_workflow() {
        let mut betsy = SystemdBetsyEngine::new("multi-user.target");
        let mut srv = SystemdUnit::new(1, b"nginx.service", UnitType::Service);
        srv.restart_policy = RestartPolicy::OnFailure;
        srv.startup_time_ms = 50;
        srv.duration_ms = 120;

        betsy.register_unit(srv);
        assert!(betsy.systemctl_start(1).is_ok());

        assert_eq!(betsy.query_unit_state(1), Some(UnitState::Active));

        let blame = betsy.analyze_blame();
        assert_eq!(blame.len(), 1);

        betsy.register_watchdog("nginx.service", 30);
        betsy.ping_watchdog("nginx.service", 90);
        let healthy = betsy.check_watchdog_health("nginx.service", 100);
        assert!(healthy);

        let slice_ok = betsy.configure_slice("system.slice", 100, 1024 * 1024);
        assert!(slice_ok.is_ok());
    }

    #[test]
    fn test_alternative_init_bridge() {
        let bridge = InitSystemBridge::new(InitSystemType::Runit);
        assert_eq!(bridge.active_init, InitSystemType::Runit);

        let runit_script = bridge.convert_runit_service_script("apache2");
        assert!(&runit_script[..].starts_with(b"#!/bin/sh\nexec apache2 --foreground\n"));
    }

    #[test]
    fn test_betsy_systemd_compat_shim() {
        let shim = BetsySystemdCompatShim::new();
        assert!(shim.is_systemd_active);
        let msg = shim.emulate_systemd_sysv_fallback("/etc/init.d/nginx");
        assert!(msg.contains("nginx"));
    }

    #[test]
    fn test_transient_service_generator() {
        let unit = TransientServiceGenerator::create_transient_unit("/usr/bin/curl", "transient-fetch");
        assert_eq!(unit.unit_type, UnitType::Service);
        assert_eq!(unit.restart_policy, RestartPolicy::No);
    }

    #[test]
    fn test_systemd_dynamic_user_context() {
        let ctx = SystemdDynamicUserContext::allocate_ephemeral_user(123);
        assert!(ctx.allocated_uid >= 60000);
        assert!(ctx.private_tmp);
        assert_eq!(ctx.protect_system, "strict");
    }

    #[test]
    fn test_systemd_service_hardening_evaluator() {
        let score_full = SystemdServiceHardeningEvaluator::calculate_hardening_score(true, true, true);
        assert_eq!(score_full, 2.5);

        let mut engine = SystemdEngine::new();
        let mut target = SystemdUnit::new(1, b"graphical.target", UnitType::Target);
        target.duration_ms = 100;
        target.requires.push(2);

        let mut service = SystemdUnit::new(2, b"display-manager.service", UnitType::Service);
        service.duration_ms = 150;
        service.requires.push(3);

        let mut network = SystemdUnit::new(3, b"network.target", UnitType::Target);
        network.duration_ms = 200;

        engine.register_unit(target);
        engine.register_unit(service);
        engine.register_unit(network);

        let chain = engine.systemd_analyze_critical_chain(1);
        assert_eq!(chain.len(), 3);
        assert_eq!(chain[0].unit_name, "graphical.target");
        assert_eq!(chain[1].unit_name, "display-manager.service");
        assert_eq!(chain[2].unit_name, "network.target");
    }

    #[test]
    fn test_requisite_and_on_failure_cascade() {
        let mut engine = SystemdEngine::new();

        let backup_service = SystemdUnit::new(99, b"fallback.service", UnitType::Service);
        engine.register_unit(backup_service);

        let mut dep = SystemdUnit::new(1, b"db.service", UnitType::Service);
        dep.state = UnitState::Inactive;
        engine.register_unit(dep);

        let mut app = SystemdUnit::new(2, b"app.service", UnitType::Service);
        app.requisite.push(1);
        app.on_failure.push(99);
        engine.register_unit(app);

        let res = engine.systemctl_start(2);
        assert!(res.is_err());
        assert_eq!(engine.systemctl_status(2), Some(UnitState::Failed));
        assert_eq!(engine.systemctl_status(99), Some(UnitState::Active));
    }

    #[test]
    fn test_systemd_unit_file_parser_extended() {
        let unit_content = r#"
[Unit]
Description=Sovereign Secure Web Service
Requires=network.target
Requisite=db.service
OnFailure=fallback.service

[Service]
ExecStart=/usr/bin/web-server --config /etc/web.conf
Restart=always
WatchdogSec=15s
NoNewPrivileges=yes
ProtectSystem=strict
ProtectHome=read-only
PrivateTmp=yes
MemoryDenyWriteExecute=yes
Pledge=stdio rpath wpath inet
Unveil=/etc/web.conf r
Environment=PORT=8080

[Install]
WantedBy=multi-user.target
"#;

        let parsed = SystemdUnitFileParser::parse_unit_file(unit_content);
        assert_eq!(parsed.unit_description, "Sovereign Secure Web Service");
        assert_eq!(parsed.requires, vec!["network.target"]);
        assert_eq!(parsed.requisite, vec!["db.service"]);
        assert_eq!(parsed.on_failure, vec!["fallback.service"]);
        assert_eq!(parsed.exec_start, "/usr/bin/web-server --config /etc/web.conf");
        assert_eq!(parsed.watchdog_sec, 15);
        assert!(parsed.hardening_profile.no_new_privileges);
        assert_eq!(parsed.hardening_profile.protect_system, ProtectSystemLevel::Strict);
        assert_eq!(parsed.hardening_profile.protect_home, ProtectHomeLevel::ReadOnly);
        assert!(parsed.hardening_profile.private_tmp);
        assert!(parsed.hardening_profile.memory_deny_write_execute);
        assert_eq!(parsed.hardening_profile.pledge_promises, "stdio rpath wpath inet");
        assert_eq!(parsed.hardening_profile.unveil_paths.len(), 1);
        assert_eq!(parsed.environment, vec![("PORT".to_string(), "8080".to_string())]);
    }

    #[test]
    fn test_systemd_socket_activation_manager() {
        let mut engine = SystemdEngine::new();
        let srv = SystemdUnit::new(10, b"httpd.service", UnitType::Service);
        engine.register_unit(srv);

        let mut mgr = SystemdSocketActivationManager::new();
        let socket_cfg = SystemdSocketConfig {
            socket_id: 1,
            listen_address: "0.0.0.0".to_string(),
            port: 80,
            kind: SocketKind::Stream,
            bound_fd: 5,
            target_service_id: 10,
        };
        mgr.register_socket(socket_cfg);

        let fds = mgr.get_passed_fds_for_service(10);
        assert_eq!(fds, vec![5]);

        let event = SocketActivationEvent {
            socket_id: 1,
            incoming_bytes: 128,
            client_addr: "192.168.1.5:54321".to_string(),
        };

        let fd = mgr.handle_incoming_connection(&mut engine, &event).unwrap();
        assert_eq!(fd, 5);
        assert_eq!(engine.systemctl_status(10), Some(UnitState::Active));
    }

    #[test]
    fn test_declarative_unit_generator() {
        let mut spec = DeclarativeUnitSpec::new("my-service", "/usr/bin/my-service --daemon");
        spec.description = "My Custom Declarative Service".to_string();
        spec.environment.push(("PORT".to_string(), "8080".to_string()));

        let unit_file = spec.generate_unit_file();
        assert!(unit_file.contains("Description=My Custom Declarative Service"));
        assert!(unit_file.contains("ExecStart=/usr/bin/my-service --daemon"));
        assert!(unit_file.contains("Environment=PORT=8080"));

        let parsed = SystemdUnitFileParser::parse_unit_file(&unit_file);
        assert_eq!(parsed.unit_description, "My Custom Declarative Service");
        assert_eq!(parsed.exec_start, "/usr/bin/my-service --daemon");
    }

    #[test]
    fn test_bsd_rc_parallel_stage_solver() {
        let mut engine = SystemdEngine::new();

        let mut u1 = SystemdUnit::new(1, b"mount.service", UnitType::Service);
        u1.before.push(2);
        u1.before.push(3);

        let u2 = SystemdUnit::new(2, b"net1.service", UnitType::Service);
        let u3 = SystemdUnit::new(3, b"net2.service", UnitType::Service);

        let mut u4 = SystemdUnit::new(4, b"app.service", UnitType::Service);
        u4.after.push(2);
        u4.after.push(3);

        engine.register_unit(u1);
        engine.register_unit(u2);
        engine.register_unit(u3);
        engine.register_unit(u4);

        let stages = BsdRcParallelStageSolver::compute_parallel_stages(&engine, &[1, 2, 3, 4]);
        assert_eq!(stages.len(), 3);
        assert_eq!(stages[0], vec![1]);
        assert_eq!(stages[1], vec![2, 3]);
        assert_eq!(stages[2], vec![4]);
    }

    #[test]
    fn test_extended_unit_dependencies() {
        let mut engine = SystemdEngine::new();

        let req_unit = SystemdUnit::new(1, b"dep.service", UnitType::Service);
        let mut main_unit = SystemdUnit::new(2, b"main.service", UnitType::Service);
        main_unit.requisites.push(1);

        engine.register_unit(req_unit);
        engine.register_unit(main_unit);

        // Fail to start because requisite unit 1 is inactive
        assert!(engine.systemctl_start(2).is_err());

        // Start requisite unit 1 first
        engine.systemctl_start(1).unwrap();
        assert_eq!(engine.systemctl_status(1), Some(UnitState::Active));

        // Now main unit 2 starts successfully
        engine.systemctl_start(2).unwrap();
        assert_eq!(engine.systemctl_status(2), Some(UnitState::Active));
    }
}
