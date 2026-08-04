// Sovereign Replacement System Utilities for SigmaOS
// This module implements zero-dependency, no-std compliant alternatives to standard Linux core utilities,
// init/supervision systems, syslog/journald, cron, capability-based sudo, and help manuals.

extern crate alloc;
||||||| 2139cb2f8
#[cfg(not(target_os = "none"))]
extern crate alloc as std_alloc;
#[cfg(not(target_os = "none"))]
use std_alloc::boxed::Box;

#![no_std]
#![no_main]

/// OOP-based Sigma Shell for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Milestone 0.1
/// Implements interactive shell with command parsing, echo, environment variables, aliases, and basic utilities
#[cfg(not(target_os = "none"))]
extern crate alloc as std_alloc;
#[cfg(not(target_os = "none"))]
use std_alloc::boxed::Box;
#[cfg(not(target_os = "none"))]
use std_alloc::vec::Vec;

/// OOP-based Sigma Shell for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Milestone 0.1
/// Implements interactive shell with command parsing, echo, environment variables, aliases, and basic utilities

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::time::Duration;

// ==========================================
// 1. CORE UTILS (sigma-core-utils / BusyBox)
// ==========================================
pub struct SigmaCoreUtils;

impl SigmaCoreUtils {
    pub fn cat(&self, file_data: &[u8]) -> String {
        String::from_utf8_lossy(file_data).to_string()
    }

    pub fn echo(&self, args: &[&str]) -> String {
        args.join(" ")
    }

    pub fn ls(&self, files: &[&str]) -> String {
        files.join("\n")
    }

    pub fn cp(&self, source: &[u8]) -> Vec<u8> {
        source.to_vec()
    }

    pub fn mv(&self, source: &[u8]) -> Vec<u8> {
        source.to_vec()
    }

    pub fn rm(&self) -> bool {
        true
    }
}

// ==========================================
// 2. INIT SYSTEM (sigma-init / systemd / s6)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TargetState {
    Emergency = 0,
    SingleUser = 1,
    MultiUser = 2,
    Graphical = 3,
}

#[derive(Debug, Clone)]
pub struct Service {
    pub name: String,
    pub dependencies: Vec<String>,
    pub running: bool,
    pub restart_count: usize,
    pub max_restarts: usize,
    pub socket_activated: bool,
    pub startup_time_ms: u64,
}

pub struct SigmaInit {
    pub services: Vec<Service>,
    pub current_target: TargetState,
}

impl SigmaInit {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
            current_target: TargetState::MultiUser,
        }
    }

    pub fn set_target(&mut self, target: TargetState) {
        self.current_target = target;
    }

    pub fn register_service(&mut self, name: &str, dependencies: &[&str]) {
        let mut deps = Vec::new();
        for &dep in dependencies {
            deps.push(dep.to_string());
        }
        self.services.push(Service {
            name: name.to_string(),
            dependencies: deps,
            running: false,
            restart_count: 0,
            max_restarts: 3,
            socket_activated: false,
            startup_time_ms: 100, // mock startup time
        });
    }

    pub fn register_socket_activated_service(&mut self, name: &str, dependencies: &[&str]) {
        let mut deps = Vec::new();
        for &dep in dependencies {
            deps.push(dep.to_string());
        }
        self.services.push(Service {
            name: name.to_string(),
            dependencies: deps,
            running: false,
            restart_count: 0,
            max_restarts: 3,
            socket_activated: true,
            startup_time_ms: 150,
        });
    }

    /// Simulate a connection attempt triggering standard socket activation for a service.
    pub fn socket_trigger(&mut self, service_name: &str) -> bool {
        if let Some(service) = self.services.iter_mut().find(|s| e_eq(&s.name, service_name)) {
            if service.socket_activated && !service.running {
                service.running = true;
                return true;
            }
        }
        false
    }

    /// Automatically identify failed/terminated services and trigger a restart supervision policy.
    pub fn check_and_restart_failed_services(&mut self) -> Vec<String> {
        let mut restarted = Vec::new();
        for service in &mut self.services {
            if !service.running && service.restart_count < service.max_restarts {
                service.restart_count += 1;
                service.running = true;
                restarted.push(service.name.clone());
            }
        }
        restarted
    }

    /// Systemd-analyze: return list of services ordered by startup duration time (descending).
    pub fn systemd_analyze(&self) -> Vec<(String, u64)> {
        let mut list = Vec::new();
        for service in &self.services {
            if service.running {
                list.push((service.name.clone(), service.startup_time_ms));
            }
        }
        list.sort_by(|a, b| b.1.cmp(&a.1));
        list
    }

    /// Supervise and resolve startup sequence under dependency constraints.
    pub fn start_services_ordered(&mut self) -> Vec<String> {
        let mut started = Vec::new();
        let mut attempts = 0;
        let total_services = self.services.len();

        while started.len() < total_services && attempts < 10 {
            attempts += 1;
            for i in 0..total_services {
                if self.services[i].running || self.services[i].socket_activated {
                    // Skip running or socket-activated units during initial bootstrap
                    if self.services[i].running && !started.contains(&self.services[i].name) {
                        started.push(self.services[i].name.clone());
                    }
                    continue;
                }
                let mut deps_satisfied = true;
                for dep in &self.services[i].dependencies {
                    if !started.contains(dep) {
                        deps_satisfied = false;
                        break;
                    }
                }
                if deps_satisfied {
                    self.services[i].running = true;
                    started.push(self.services[i].name.clone());
                }
            }
        }
        started
    }
}

/// Helper method for string matching
fn e_eq(a: &str, b: &str) -> bool {
    a == b
}

// ==========================================
// 3. LOGGER SYSTEM (sigma-log / journald)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub level: LogLevel,
    pub tag: String,
    pub message: String,
    pub timestamp_ms: u64,
}

/// A lock-free circular journal log ring buffer.
pub struct SigmaLog {
    pub buffer: [Option<LogEntry>; 128],
    pub head: AtomicUsize,
    pub count: AtomicUsize,
}

impl SigmaLog {
    pub fn new() -> Self {
        const INIT_OPTION: Option<LogEntry> = None;
        Self {
            buffer: [INIT_OPTION; 128],
            head: AtomicUsize::new(0),
            count: AtomicUsize::new(0),
        }
    }

    /// Record a structured journal event safely into the cyclic buffer.
    pub fn write_log(&mut self, level: LogLevel, tag: &str, message: &str, timestamp_ms: u64) {
        let entry = LogEntry {
            level,
            tag: tag.to_string(),
            message: message.to_string(),
            timestamp_ms,
        };
        let index = self.head.fetch_add(1, Ordering::SeqCst) % 128;
        self.buffer[index] = Some(entry);
        let curr_count = self.count.load(Ordering::SeqCst);
        if curr_count < 128 {
            self.count.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// Filter journal logs matching severity.
    pub fn query_logs(&self, min_level: LogLevel) -> Vec<LogEntry> {
        let mut results = Vec::new();
        for item in &self.buffer {
            if let Some(ref entry) = item {
                if entry.level >= min_level {
                    results.push(entry.clone());
                }
            }
        }
        results.sort_by_key(|e| e.timestamp_ms);
        results
    }
}

// ==========================================
// 4. CRON SYSTEM (sigma-cron)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CronScheduleType {
    Interval(Duration),
    Hourly,
    Daily,
    Weekly,
    Reboot,
}

pub struct CronJob {
    pub job_id: usize,
    pub schedule_type: CronScheduleType,
    pub interval: Duration,
    pub last_run: Duration,
    pub command: String,
    pub enabled: bool,
    pub last_run_success: bool,
    pub output_log: String,
}

pub struct SigmaCron {
    pub jobs: Vec<CronJob>,
}

impl SigmaCron {
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn schedule_job(&mut self, id: usize, interval: Duration, command: &str) {
        self.jobs.push(CronJob {
            job_id: id,
            schedule_type: CronScheduleType::Interval(interval),
            interval,
            last_run: Duration::ZERO,
            command: command.to_string(),
            enabled: true,
            last_run_success: false,
            output_log: String::new(),
        });
    }

    pub fn schedule_profile_job(&mut self, id: usize, profile: CronScheduleType, command: &str) {
        let interval = match profile {
            CronScheduleType::Interval(d) => d,
            CronScheduleType::Hourly => Duration::from_secs(3600),
            CronScheduleType::Daily => Duration::from_secs(86400),
            CronScheduleType::Weekly => Duration::from_secs(604800),
            CronScheduleType::Reboot => Duration::ZERO,
        };
        self.jobs.push(CronJob {
            job_id: id,
            schedule_type: profile,
            interval,
            last_run: Duration::ZERO,
            command: command.to_string(),
            enabled: true,
            last_run_success: false,
            output_log: String::new(),
        });
    }
||||||| 2139cb2f8
        
        if in_arg {
            args.push(&line[start..line.len()]);
        }
        
        if args.is_empty() {
            return Ok(());
        }
        
        // 1. Resolve Command Aliases (udev/bash inspiration)
        let resolved_cmd_name = if let Some(alias_target) = self.get_alias(args[0]) {
            alias_target
        } else {
            args[0]
        };
        
        if in_arg {
            args.push(&line[start..line.len()]);
        }
        
        if args.is_empty() {
            return Ok(());
        }
        
        // 1. Resolve Command Aliases (udev/bash inspiration)
        let mut resolved_buf = [0u8; 64];
        let mut resolved_len = 0;
        {
            let resolved_cmd_ref = if let Some(alias_target) = self.get_alias(args[0]) {
                alias_target
            } else {
                args[0]
            };
            resolved_len = resolved_cmd_ref.len().min(64);
            resolved_buf[..resolved_len].copy_from_slice(&resolved_cmd_ref[..resolved_len]);
        }
        let resolved_cmd_name = &resolved_buf[..resolved_len];

    /// Suspend or resume a specific cron job dynamically at runtime.
    pub fn toggle_job(&mut self, id: usize, enabled: bool) -> bool {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.job_id == id) {
            job.enabled = enabled;
            return true;
        }
        false
    }

    /// Anacron-like Catch-up: identify and run jobs that missed their schedule.
    pub fn check_catchup_jobs(&mut self, current_time: Duration) -> Vec<String> {
        let mut catchup_commands = Vec::new();
        for job in &mut self.jobs {
            if job.enabled && job.last_run == Duration::ZERO && current_time > job.interval {
                job.last_run = current_time;
                job.last_run_success = true;
                job.output_log = "Anacron catch-up run successful".to_string();
                catchup_commands.push(job.command.clone());
            }
        }
        catchup_commands
    }

    /// Trigger enabled jobs whose timing parameters have elapsed.
    pub fn tick_jobs(&mut self, current_time: Duration) -> Vec<String> {
        let mut triggered_commands = Vec::new();
        for job in &mut self.jobs {
            if job.enabled && current_time >= job.last_run + job.interval {
                job.last_run = current_time;
                job.last_run_success = true;
                job.output_log = "Scheduled cron execution successful".to_string();
                triggered_commands.push(job.command.clone());
||||||| 2139cb2f8
        let cmd_args: Vec<&[u8]> = expanded_args.to_vec();
        
        for cmd_option in &mut self.commands {
            if let Some(ref mut cmd) = *cmd_option {
                if cmd.name() == resolved_cmd_name {
                    return cmd.execute(&cmd_args);
                }
        let cmd_args = expanded_args;
        
        for cmd_option in &mut self.commands {
            if let Some(ref mut cmd) = *cmd_option {
                if cmd.name() == resolved_cmd_name {
                    return cmd.execute(&cmd_args);
                }
            }
        }
        triggered_commands
    }
}

// ==========================================
// 5. PRIVILEGE DELEGATOR (sigma-priv / sudo)
// ==========================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Privilege {
    SysAdmin,
    NetworkConfigure,
    HardwareWrite,
    UserRead,
}

pub struct SigmaPriv {
    pub user_capabilities: Vec<(String, Privilege)>,
}

impl SigmaPriv {
    pub fn new() -> Self {
        Self {
            user_capabilities: Vec::new(),
        }
    }

    pub fn grant_capability(&mut self, username: &str, privilege: Privilege) {
        self.user_capabilities
            .push((username.to_string(), privilege));
    }
||||||| 2139cb2f8
    
    fn get(&self, index: usize) -> Option<&[u8]> {
        if index >= self.history.len() {
            return None;
        }
        let len = self.lengths[index];
        Some(&self.history[index][..len])
    }
    
    fn get_last(&self) -> Option<&[u8]>;
}
    
    fn get(&self, index: usize) -> Option<&[u8]> {
        if index >= self.history.len() {
            return None;
        }
        let len = self.lengths[index];
        Some(&self.history[index][..len])
    }
    
    fn get_last(&self) -> Option<&[u8]> {
        self.get_last_impl()
    }
}

    /// Authorize a command delegate strictly based on capability matrices.
    pub fn check_authority(&self, username: &str, required: Privilege) -> bool {
        for (user, priv_item) in &self.user_capabilities {
            if user == username && *priv_item == required {
                return true;
||||||| 2139cb2f8
impl SimpleShellHistory {
    fn get_last_impl(&self) -> Option<&[u8]> {
        if self.history.is_empty() {
            return None;
        }
        let index = self.history.len() - 1;
        self.get(index)
    }
}

impl ShellHistory for SimpleShellHistory {
    fn get_last(&self) -> Option<&[u8]> {
        self.get_last_impl()
    }
}

pub trait ShellEnvironment {
    fn set(&mut self, key: &[u8], value: &[u8]);
    fn get(&self, key: &[u8]) -> Option<&[u8]>;
    fn unset(&mut self, key: &[u8]);
}

#[repr(C)]
pub struct SimpleShellEnvironment {
    pub keys: Vec<[u8; 64]>,
    pub values: Vec<[u8; 256]>,
    pub key_lengths: Vec<usize>,
    pub value_lengths: Vec<usize>,
}

impl SimpleShellEnvironment {
    pub fn new() -> Self {
        SimpleShellEnvironment {
            keys: Vec::new(),
            values: Vec::new(),
            key_lengths: Vec::new(),
            value_lengths: Vec::new(),
        }
    }
}

impl ShellEnvironment for SimpleShellEnvironment {
    fn set(&mut self, key: &[u8], value: &[u8]) {
        let key_len = key.len().min(63);
        let value_len = value.len().min(255);
        
        let mut key_entry = [0u8; 64];
        let mut value_entry = [0u8; 256];
        
        for i in 0..key_len {
            key_entry[i] = key[i];
        }
        for i in 0..value_len {
            value_entry[i] = value[i];
        }
        
        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                self.values[i] = value_entry;
                self.value_lengths[i] = value_len;
                return;
            }
        }
        
        self.keys.push(key_entry);
        self.values.push(value_entry);
        self.key_lengths.push(key_len);
        self.value_lengths.push(value_len);
    }
    
    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        let key_len = key.len();
        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                let value_len = self.value_lengths[i];
                return Some(&self.values[i][..value_len]);
            }
        }
        None
    }
    
    fn unset(&mut self, key: &[u8]) {
        let key_len = key.len();
        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                self.keys.remove(i);
                self.values.remove(i);
                self.key_lengths.remove(i);
                self.value_lengths.remove(i);
                return;
impl SimpleShellHistory {
    fn get_last_impl(&self) -> Option<&[u8]> {
        if self.history.is_empty() {
            return None;
        }
        let index = self.history.len() - 1;
        self.get(index)
    }
}

pub trait ShellEnvironment {
    fn set(&mut self, key: &[u8], value: &[u8]);
    fn get(&self, key: &[u8]) -> Option<&[u8]>;
    fn unset(&mut self, key: &[u8]);
}

#[repr(C)]
pub struct SimpleShellEnvironment {
    pub keys: Vec<[u8; 64]>,
    pub values: Vec<[u8; 256]>,
    pub key_lengths: Vec<usize>,
    pub value_lengths: Vec<usize>,
}

impl SimpleShellEnvironment {
    pub fn new() -> Self {
        SimpleShellEnvironment {
            keys: Vec::new(),
            values: Vec::new(),
            key_lengths: Vec::new(),
            value_lengths: Vec::new(),
        }
    }
}

impl ShellEnvironment for SimpleShellEnvironment {
    fn set(&mut self, key: &[u8], value: &[u8]) {
        let key_len = key.len().min(63);
        let value_len = value.len().min(255);
        
        let mut key_entry = [0u8; 64];
        let mut value_entry = [0u8; 256];
        
        for i in 0..key_len {
            key_entry[i] = key[i];
        }
        for i in 0..value_len {
            value_entry[i] = value[i];
        }
        
        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                self.values[i] = value_entry;
                self.value_lengths[i] = value_len;
                return;
            }
        }
        
        self.keys.push(key_entry);
        self.values.push(value_entry);
        self.key_lengths.push(key_len);
        self.value_lengths.push(value_len);
    }
    
    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        let key_len = key.len();
        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                let value_len = self.value_lengths[i];
                return Some(&self.values[i][..value_len]);
            }
        }
        None
    }
    
    fn unset(&mut self, key: &[u8]) {
        let key_len = key.len();
        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                self.keys.remove(i);
                self.values.remove(i);
                self.key_lengths.remove(i);
                self.value_lengths.remove(i);
                return;
            }
        }
        false
    }
}

// ==========================================
// 6. SOVEREIGN DOCS (sigma-doc / tldr)
// ==========================================
pub struct SigmaDoc {
    pub topic: String,
    pub description: String,
    pub examples: Vec<String>,
}
||||||| 2139cb2f8
struct Vec<T> { data: *mut T, len: usize, capacity: usize }
#[cfg(target_os = "none")]
struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl SigmaDoc {
    pub fn new(topic: &str, description: &str) -> Self {
        Self {
            topic: topic.to_string(),
            description: description.to_string(),
            examples: Vec::new(),
||||||| 2139cb2f8
impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                new_vec.push((*self.data.add(i)).clone());
            }
#[cfg(target_os = "none")]
impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                new_vec.push((*self.data.add(i)).clone());
            }
        }
    }

    pub fn add_example(&mut self, example: &str) {
        self.examples.push(example.to_string());
    }

    pub fn render_tldr(&self) -> String {
        let mut output = format!("# {}\n\n> {}\n\n", self.topic, self.description);
        for ex in &self.examples {
            output.push_str(&format!("- {}\n", ex));
        }
        output
||||||| 2139cb2f8
        new_vec
    }
}

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn is_empty(&self) -> bool { self.len == 0 }
    fn len(&self) -> usize { self.len }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
        new_vec
    }
}

#[cfg(target_os = "none")]
impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn is_empty(&self) -> bool { self.len == 0 }
    fn len(&self) -> usize { self.len }
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

#[cfg(target_os = "none")]
extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }

#[cfg(target_os = "none")]
impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

#[cfg(target_os = "none")]
impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

#[cfg(target_os = "none")]
impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

#[cfg(target_os = "none")]
impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

// ==========================================
// UNIT TESTS MODULE
// ==========================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigma_core_utils() {
        let utils = SigmaCoreUtils;
        assert_eq!(utils.echo(&["Sovereign", "OS"]), "Sovereign OS");
        assert_eq!(utils.cat(b"kernel config"), "kernel config");
        assert_eq!(utils.ls(&["bin", "etc"]), "bin\netc");
        assert_eq!(utils.cp(b"file1"), b"file1");
        assert_eq!(utils.mv(b"file2"), b"file2");
        assert!(utils.rm());
    }

    #[test]
    fn test_sigma_init() {
        let mut init = SigmaInit::new();
        init.register_service("db", &[]);
        init.register_service("network", &[]);
        init.register_service("api-gateway", &["db", "network"]);

        let started = init.start_services_ordered();
        assert_eq!(started.len(), 3);
        assert_eq!(started[2], "api-gateway");
    }

    #[test]
    fn test_socket_activation() {
        let mut init = SigmaInit::new();
        init.register_socket_activated_service("web-server", &[]);

        // Should not start during initial bootstrap
        let started = init.start_services_ordered();
        assert!(started.is_empty());
        assert!(!init.services[0].running);

        // Simulate connection trigger -> socket activation launches the service
        assert!(init.socket_trigger("web-server"));
        assert!(init.services[0].running);
    }

    #[test]
    fn test_service_restart_policy() {
        let mut init = SigmaInit::new();
        init.register_service("faulty-service", &[]);
        init.services[0].running = false; // Simulate failure

        let restarted = init.check_and_restart_failed_services();
        assert_eq!(restarted.len(), 1);
        assert_eq!(restarted[0], "faulty-service");
        assert_eq!(init.services[0].restart_count, 1);
        assert!(init.services[0].running);
    }
||||||| 2139cb2f8
        // Inspect captured variable inside spy command
        if let Some(ref cmd_box) = shell.commands[0] {
            // Unsafe cast to access captured properties (since we can't downcast Box<dyn ShellCommand>)
            let spy_ptr = cmd_box as *const Box<dyn ShellCommand> as *const SpyCommand;
            unsafe {
                let captured = &(*spy_ptr).captured_arg[..(*spy_ptr).captured_len];
                assert_eq!(captured, b"sovereign_pass_123");
            }
        }
        // Inspect captured variable inside spy command
        if let Some(ref cmd_box) = shell.commands[0] {
            // Unsafe cast to access captured properties (since we can't downcast Box<dyn ShellCommand>)
            let spy_ptr = &**cmd_box as *const dyn ShellCommand as *const SpyCommand;
            unsafe {
                let captured = &(&(*spy_ptr).captured_arg)[..(*spy_ptr).captured_len];
                assert_eq!(captured, b"sovereign_pass_123");
            }
        }

    #[test]
    fn test_systemd_analyze() {
        let mut init = SigmaInit::new();
        init.register_service("db", &[]);
        init.register_service("network", &[]);
        init.services[0].running = true;
        init.services[1].running = true;
        init.services[0].startup_time_ms = 80;
        init.services[1].startup_time_ms = 250;

        let analyze = init.systemd_analyze();
        assert_eq!(analyze.len(), 2);
        assert_eq!(analyze[0].0, "network"); // network should be first (slowest, 250ms)
        assert_eq!(analyze[1].0, "db");
    }
||||||| 2139cb2f8
        if let Some(ref cmd_box) = shell.commands[0] {
            let spy_ptr = cmd_box as *const Box<dyn ShellCommand> as *const SpyCommand;
            unsafe {
                let captured = &(*spy_ptr).captured_arg[..(*spy_ptr).captured_len];
                assert_eq!(captured, b"sovereign");
            }
        }
        if let Some(ref cmd_box) = shell.commands[0] {
            let spy_ptr = &**cmd_box as *const dyn ShellCommand as *const SpyCommand;
            unsafe {
                let captured = &(&(*spy_ptr).captured_arg)[..(*spy_ptr).captured_len];
                assert_eq!(captured, b"sovereign");
            }
        }

    #[test]
    fn test_sigma_log() {
        let mut logger = SigmaLog::new();
        logger.write_log(LogLevel::Info, "INIT", "Booting kernel...", 1000);
        logger.write_log(LogLevel::Warning, "HW", "Throttling detected", 2000);
        logger.write_log(LogLevel::Debug, "SCHED", "Yielding idle task", 500);

        let queries = logger.query_logs(LogLevel::Warning);
        assert_eq!(queries.len(), 1);
        assert_eq!(queries[0].tag, "HW");
    }

    #[test]
    fn test_sigma_cron() {
        let mut cron = SigmaCron::new();
        cron.schedule_job(1, Duration::from_secs(60), "backup-db");
        cron.schedule_job(2, Duration::from_secs(3600), "rotate-logs");

        // Tick at 10 seconds -> nothing triggered
        let trig_10 = cron.tick_jobs(Duration::from_secs(10));
        assert!(trig_10.is_empty());

        // Tick at 70 seconds -> backup-db triggered
        let trig_70 = cron.tick_jobs(Duration::from_secs(70));
        assert_eq!(trig_70.len(), 1);
        assert_eq!(trig_70[0], "backup-db");
    }

    #[test]
    fn test_cron_toggle() {
        let mut cron = SigmaCron::new();
        cron.schedule_job(1, Duration::from_secs(60), "backup-db");

        // Suspend job 1
        assert!(cron.toggle_job(1, false));
        assert!(!cron.jobs[0].enabled);

        // Tick at 70 seconds -> should not trigger because it is suspended
        let triggered = cron.tick_jobs(Duration::from_secs(70));
        assert!(triggered.is_empty());
    }

    #[test]
    fn test_anacron_catchup() {
        let mut cron = SigmaCron::new();
        cron.schedule_job(1, Duration::from_secs(60), "backup-db");

        // Simulate reboot catch-up check: system was offline, last_run is ZERO, and current time > interval (100 > 60)
        let catchup = cron.check_catchup_jobs(Duration::from_secs(100));
        assert_eq!(catchup.len(), 1);
        assert_eq!(catchup[0], "backup-db");
        assert!(cron.jobs[0].last_run_success);
        assert_eq!(cron.jobs[0].output_log, "Anacron catch-up run successful");
    }

    #[test]
    fn test_cron_profile_scheduling() {
        let mut cron = SigmaCron::new();
        cron.schedule_profile_job(1, CronScheduleType::Hourly, "hourly-clean");
        cron.schedule_profile_job(2, CronScheduleType::Daily, "daily-backup");

        assert_eq!(cron.jobs[0].interval, Duration::from_secs(3600));
        assert_eq!(cron.jobs[1].interval, Duration::from_secs(86400));
    }

    #[test]
    fn test_sigma_priv() {
        let mut delegator = SigmaPriv::new();
        delegator.grant_capability("admin_bob", Privilege::SysAdmin);
        delegator.grant_capability("operator_alice", Privilege::NetworkConfigure);

        assert!(delegator.check_authority("admin_bob", Privilege::SysAdmin));
        assert!(!delegator.check_authority("operator_alice", Privilege::SysAdmin));
        assert!(delegator.check_authority("operator_alice", Privilege::NetworkConfigure));
    }

    #[test]
    fn test_sigma_doc() {
        let mut doc = SigmaDoc::new("cp", "Copy files and directories.");
        doc.add_example("cp [source] [destination]");
        doc.add_example("cp -r [source_dir] [destination_dir]");

        let rendered = doc.render_tldr();
        assert!(rendered.contains("# cp"));
        assert!(rendered.contains("> Copy files and directories."));
        assert!(rendered.contains("- cp [source] [destination]"));
    }
}
