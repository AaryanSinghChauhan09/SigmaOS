// OOP-based Unified Logging System and Diverse Targets for SigmaOS
// Inspired by Linux systemd-journald and rsyslog, providing Console, File, Network, and Memory logging targets.

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Syslog Facility enum inspired by BSD syslog and Linux syslog.h
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SyslogFacility {
    Kernel = 0,
    User = 1,
    Mail = 2,
    Daemon = 3,
    Auth = 4,
    Syslog = 5,
    Lpr = 6,
    News = 7,
    Uucp = 8,
    Cron = 9,
    AuthPriv = 10,
    Ftp = 11,
    Local0 = 16,
    Local1 = 17,
    Local2 = 18,
    Local3 = 19,
    Local4 = 20,
    Local5 = 21,
    Local6 = 22,
    Local7 = 23,
}

impl SyslogFacility {
    pub fn as_str(&self) -> &'static str {
        match self {
            SyslogFacility::Kernel => "kern",
            SyslogFacility::User => "user",
            SyslogFacility::Mail => "mail",
            SyslogFacility::Daemon => "daemon",
            SyslogFacility::Auth => "auth",
            SyslogFacility::Syslog => "syslog",
            SyslogFacility::Lpr => "lpr",
            SyslogFacility::News => "news",
            SyslogFacility::Uucp => "uucp",
            SyslogFacility::Cron => "cron",
            SyslogFacility::AuthPriv => "authpriv",
            SyslogFacility::Ftp => "ftp",
            SyslogFacility::Local0 => "local0",
            SyslogFacility::Local1 => "local1",
            SyslogFacility::Local2 => "local2",
            SyslogFacility::Local3 => "local3",
            SyslogFacility::Local4 => "local4",
            SyslogFacility::Local5 => "local5",
            SyslogFacility::Local6 => "local6",
            SyslogFacility::Local7 => "local7",
        }
    }
}

/// Log level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warning = 3,
    Error = 4,
    Fatal = 5,
}

impl LogLevel {
    pub fn syslog_severity(&self) -> u8 {
        match self {
            LogLevel::Trace => 7,   // Debug
            LogLevel::Debug => 7,   // Debug
            LogLevel::Info => 6,    // Informational
            LogLevel::Warning => 4, // Warning
            LogLevel::Error => 3,   // Error
            LogLevel::Fatal => 2,   // Critical
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
}

/// Structured key-value log attribute field for rich structured logging
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogField {
    pub key: String,
    pub value: String,
}

impl LogField {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: String::from(key),
            value: String::from(value),
        }
    }
}

/// Log entry (OOP: Structured Log entry object)
#[derive(Debug, Clone)]
pub struct UnifiedLogEntry {
    pub timestamp: u64,
    pub level: LogLevel,
    pub facility: SyslogFacility,
    pub pid: u32,
    pub component: [u8; 64],
    pub component_len: u8,
    pub message: [u8; 512],
    pub message_len: u16,
    pub module: [u8; 128],
    pub module_len: u8,
    pub line: u32,
    pub fields: Vec<LogField>,
}

impl UnifiedLogEntry {
    pub fn new(
        level: LogLevel,
        component: &[u8],
        message: &[u8],
        module: &[u8],
        line: u32,
    ) -> Self {
        let mut component_array = [0u8; 64];
        let mut message_array = [0u8; 512];
        let mut module_array = [0u8; 128];

        let component_len = component.len().min(63);
        let message_len = message.len().min(511);
        let module_len = module.len().min(127);

        component_array[..component_len].copy_from_slice(&component[..component_len]);
        message_array[..message_len].copy_from_slice(&message[..message_len]);
        module_array[..module_len].copy_from_slice(&module[..module_len]);

        UnifiedLogEntry {
            timestamp: get_current_time(),
            level,
            facility: SyslogFacility::User,
            pid: 1,
            component: component_array,
            component_len: component_len as u8,
            message: message_array,
            message_len: message_len as u16,
            module: module_array,
            module_len: module_len as u8,
            line,
            fields: Vec::new(),
        }
    }

    pub fn with_facility(mut self, facility: SyslogFacility) -> Self {
        self.facility = facility;
        self
    }

    pub fn with_pid(mut self, pid: u32) -> Self {
        self.pid = pid;
        self
    }

    pub fn with_field(mut self, key: &str, value: &str) -> Self {
        self.fields.push(LogField::new(key, value));
        self
    }

    // Performance Optimization: Direct O(1) slice access using pre-calculated length fields
    // instead of running linear scan `.position(|&b| b == 0)` across fixed byte arrays on every log formatting call.
    pub fn get_message_str(&self) -> String {
        let msg_len = self.message_len as usize;
        String::from_utf8_lossy(&self.message[..msg_len]).into_owned()
    }

    pub fn get_component_str(&self) -> String {
        let comp_len = self.component_len as usize;
        String::from_utf8_lossy(&self.component[..comp_len]).into_owned()
    }

    pub fn get_module_str(&self) -> String {
        let mod_len = self.module_len as usize;
        String::from_utf8_lossy(&self.module[..mod_len]).into_owned()
    }

    /// Calculate RFC 5424 Syslog Priority field (Facility * 8 + Severity)
    pub fn syslog_pri(&self) -> u8 {
        (self.facility as u8) * 8 + self.level.syslog_severity()
    }

    /// Format entry as structured JSON string (systemd-journald / JSON output format)
    pub fn to_json(&self) -> String {
        let mut json = alloc::format!(
            "{{\"timestamp\":{},\"level\":\"{}\",\"facility\":\"{}\",\"pri\":{},\"pid\":{},\"component\":\"{}\",\"module\":\"{}\",\"line\":{},\"message\":\"{}\"",
            self.timestamp,
            self.level.as_str(),
            self.facility.as_str(),
            self.syslog_pri(),
            self.pid,
            self.get_component_str(),
            self.get_module_str(),
            self.line,
            self.get_message_str().replace('"', "\\\"")
        );

        if !self.fields.is_empty() {
            json.push_str(",\"fields\":{");
            for (idx, field) in self.fields.iter().enumerate() {
                if idx > 0 {
                    json.push(',');
                }
                let esc_val = field.value.replace('"', "\\\"");
                let field_str = alloc::format!("\"{}\":\"{}\"", field.key, esc_val);
                json.push_str(&field_str);
            }
            json.push('}');
        }

        json.push('}');
        json
    }

    /// Format entry into RFC 5424 structured syslog message framing
    /// `<PRI>VERSION TIMESTAMP HOSTNAME APP-NAME PROCID MSGID [SD-ID KEY="VAL"] MSG`
    pub fn to_rfc5424(&self, hostname: &str, app_name: &str) -> String {
        let pri = self.syslog_pri();
        let comp = self.get_component_str();
        let procid = if self.pid > 0 { self.pid } else { 1 };

        let mut sd = alloc::format!("[meta@53828 component=\"{}\" module=\"{}\" line=\"{}\"", comp, self.get_module_str(), self.line);
        for field in &self.fields {
            let field_str = alloc::format!(" {}=\"{}\"", field.key, field.value);
            sd.push_str(&field_str);
        }
        sd.push(']');

        alloc::format!(
            "<{}>1 {} {} {} {} - {} {}",
            pri,
            self.timestamp,
            hostname,
            if app_name.is_empty() { "sigmaos" } else { app_name },
            procid,
            sd,
            self.get_message_str()
        )
    }

    /// Format entry in Linux systemd-journald native export format (field=value key-value blocks)
    pub fn to_journald_native(&self) -> String {
        let mut journal = alloc::format!(
            "__REALTIME_TIMESTAMP={}\nPRIORITY={}\nSYSLOG_FACILITY={}\nSYSLOG_IDENTIFIER={}\n_PID={}\nCODE_FILE={}\nCODE_LINE={}\nMESSAGE={}\n",
            self.timestamp,
            self.level.syslog_severity(),
            self.facility as u8,
            self.get_component_str(),
            self.pid,
            self.get_module_str(),
            self.line,
            self.get_message_str()
        );

        for field in &self.fields {
            let upper_key = field.key.to_uppercase().replace('-', "_");
            let field_line = alloc::format!("{}={}\n", upper_key, field.value);
            journal.push_str(&field_line);
        }

        journal
    }
}

/// Log target trait (OOP interface)
pub trait LogTarget {
    /// Write log entry
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError>;
    /// Flush buffered logs
    fn flush(&mut self) -> Result<(), LogError>;
    /// Get target info
    fn info(&self) -> TargetInfo;
}

/// Log error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogError {
    Success = 0,
    BufferFull = 1,
    WriteFailed = 2,
    PermissionDenied = 3,
    InvalidFormat = 4,
}

/// Target info
pub struct TargetInfo {
    pub target_type: TargetType,
    pub buffer_size: usize,
    pub entries_written: u64,
    pub capability: TargetCapability,
}

impl TargetInfo {
    pub fn new(target_type: TargetType) -> Self {
        TargetInfo {
            target_type,
            buffer_size: 0,
            entries_written: 0,
            capability: TargetCapability::new(),
        }
    }
}

/// Target type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetType {
    Console = 0,
    File = 1,
    Network = 2,
    Memory = 3,
    Remote = 4,
}

/// Target capability
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetCapability {
    pub can_write: bool,
    pub can_flush: bool,
    pub can_rotate: bool,
}

impl TargetCapability {
    pub const fn new() -> Self {
        TargetCapability {
            can_write: false,
            can_flush: false,
            can_rotate: false,
        }
    }

    pub const fn full() -> Self {
        TargetCapability {
            can_write: true,
            can_flush: true,
            can_rotate: true,
        }
    }
}

impl Default for TargetCapability {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 1. MEMORY LOG TARGET
// ==========================================

pub struct MemoryLogTarget {
    pub entries: Vec<Option<UnifiedLogEntry>>,
    pub max_entries: usize,
    pub entries_written: AtomicUsize,
    pub capability: TargetCapability,
}

impl MemoryLogTarget {
    pub fn new(max_entries: usize, capability: TargetCapability) -> Self {
        MemoryLogTarget {
            entries: Vec::new(),
            max_entries,
            entries_written: AtomicUsize::new(0),
            capability,
        }
    }

    pub fn clear(&mut self) -> Result<(), LogError> {
        if !self.capability.can_rotate {
            return Err(LogError::PermissionDenied);
        }

        self.entries = Vec::new();
        self.entries_written.store(0, Ordering::SeqCst);
        Ok(())
    }
}

impl LogTarget for MemoryLogTarget {
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError> {
        if !self.capability.can_write {
            return Err(LogError::PermissionDenied);
        }

        if self.entries.len() >= self.max_entries {
            return Err(LogError::BufferFull);
        }

        let mut entry_copy = UnifiedLogEntry::new(
            entry.level,
            &entry.component,
            &entry.message,
            &entry.module,
            entry.line,
        );
        entry_copy.timestamp = entry.timestamp;

        self.entries.push(Some(entry_copy));
        self.entries_written.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), LogError> {
        if !self.capability.can_flush {
            return Err(LogError::PermissionDenied);
        }
        Ok(())
    }

    fn info(&self) -> TargetInfo {
        TargetInfo {
            target_type: TargetType::Memory,
            buffer_size: self.max_entries,
            entries_written: self.entries_written.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

// ==========================================
// 2. FILE LOG TARGET (/var/log/syslog)
// ==========================================

pub struct FileLogTarget {
    pub file_path: String,
    pub file_buffer: Vec<String>,
    pub entries_written: AtomicUsize,
    pub capability: TargetCapability,
    pub max_entries_before_rotate: usize,
    pub rotated_generations: Vec<Vec<String>>,
    pub is_json_format: bool,
}

impl FileLogTarget {
    pub fn new(path: &str, capability: TargetCapability) -> Self {
        Self {
            file_path: String::from(path),
            file_buffer: Vec::new(),
            entries_written: AtomicUsize::new(0),
            capability,
            max_entries_before_rotate: 1000,
            rotated_generations: Vec::new(),
            is_json_format: false,
        }
    }

    pub fn with_json_formatting(mut self, enable_json: bool) -> Self {
        self.is_json_format = enable_json;
        self
    }

    pub fn with_max_entries(mut self, max_entries: usize) -> Self {
        self.max_entries_before_rotate = max_entries;
        self
    }

    pub fn rotate_now(&mut self) {
        if !self.file_buffer.is_empty() {
            let gen = core::mem::take(&mut self.file_buffer);
            self.rotated_generations.push(gen);
            if self.rotated_generations.len() > 5 {
                self.rotated_generations.remove(0);
            }
        }
    }
}

impl LogTarget for FileLogTarget {
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError> {
        if !self.capability.can_write {
            return Err(LogError::PermissionDenied);
        }

        if self.file_buffer.len() >= self.max_entries_before_rotate {
            self.rotate_now();
        }

        let formatted = if self.is_json_format {
            entry.to_json()
        } else {
            let msg_str = entry.get_message_str();
            alloc::format!("[FILE][{:?}]: {}", entry.level, msg_str)
        };

        self.file_buffer.push(formatted);
        self.entries_written.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), LogError> {
        Ok(())
    }

    fn info(&self) -> TargetInfo {
        TargetInfo {
            target_type: TargetType::File,
            buffer_size: 10000,
            entries_written: self.entries_written.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

// ==========================================
// 3. CONSOLE LOG TARGET (/dev/console)
// ==========================================

pub struct ConsoleLogTarget {
    pub output_history: Vec<String>,
    pub entries_written: AtomicUsize,
    pub capability: TargetCapability,
}

impl ConsoleLogTarget {
    pub fn new(capability: TargetCapability) -> Self {
        Self {
            output_history: Vec::new(),
            entries_written: AtomicUsize::new(0),
            capability,
        }
    }
}

impl LogTarget for ConsoleLogTarget {
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError> {
        if !self.capability.can_write {
            return Err(LogError::PermissionDenied);
        }

        let msg_len = entry.message.iter().position(|&b| b == 0).unwrap_or(512);
        let msg_str = String::from_utf8_lossy(&entry.message[..msg_len]);

        let formatted = alloc::format!("[STDOUT][{:?}]: {}", entry.level, msg_str);
        self.output_history.push(formatted);
        self.entries_written.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), LogError> {
        Ok(())
    }

    fn info(&self) -> TargetInfo {
        TargetInfo {
            target_type: TargetType::Console,
            buffer_size: 5000,
            entries_written: self.entries_written.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

// ==========================================
// 4. NETWORK LOG TARGET (rsyslog / syslog-ng RFC 5424 / UDP / TCP)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkProtocol {
    Udp = 0,
    Tcp = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkFramingFormat {
    LegacySyslog = 0, // <34>message
    Rfc5424 = 1,      // RFC 5424 structured syslog
    JsonStream = 2,   // JSON line streaming
}

pub struct NetworkLogTarget {
    pub server_ip: String,
    pub server_port: u16,
    pub secondary_ip: Option<String>,
    pub protocol: NetworkProtocol,
    pub framing: NetworkFramingFormat,
    pub forwarded_packets: Vec<String>,
    pub offline_ring_buffer: Vec<String>,
    pub max_ring_buffer_size: usize,
    pub is_connected: bool,
    pub retry_attempts: usize,
    pub hostname: String,
    pub app_name: String,
    pub entries_written: AtomicUsize,
    pub capability: TargetCapability,
}

impl NetworkLogTarget {
    pub fn new(server_ip: &str, server_port: u16, capability: TargetCapability) -> Self {
        Self {
            server_ip: String::from(server_ip),
            server_port,
            secondary_ip: None,
            protocol: NetworkProtocol::Udp,
            framing: NetworkFramingFormat::LegacySyslog,
            forwarded_packets: Vec::new(),
            offline_ring_buffer: Vec::new(),
            max_ring_buffer_size: 500,
            is_connected: true,
            retry_attempts: 0,
            hostname: String::from("sigmaos-node-1"),
            app_name: String::from("sigmaos"),
            entries_written: AtomicUsize::new(0),
            capability,
        }
    }

    pub fn with_protocol(mut self, protocol: NetworkProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn with_framing(mut self, framing: NetworkFramingFormat) -> Self {
        self.framing = framing;
        self
    }

    pub fn with_failover_ip(mut self, failover_ip: &str) -> Self {
        self.secondary_ip = Some(String::from(failover_ip));
        self
    }

    pub fn set_connection_status(&mut self, connected: bool) {
        self.is_connected = connected;
        if connected && !self.offline_ring_buffer.is_empty() {
            // Drain offline buffer upon re-connection
            let offline_msgs = core::mem::take(&mut self.offline_ring_buffer);
            for msg in offline_msgs {
                self.forwarded_packets.push(msg);
            }
        }
    }

    pub fn format_packet(&self, entry: &UnifiedLogEntry) -> String {
        match self.framing {
            NetworkFramingFormat::LegacySyslog => {
                let msg_str = entry.get_message_str();
                alloc::format!("<{}>{}", entry.syslog_pri(), msg_str)
            }
            NetworkFramingFormat::Rfc5424 => entry.to_rfc5424(&self.hostname, &self.app_name),
            NetworkFramingFormat::JsonStream => entry.to_json(),
        }
    }
}

impl LogTarget for NetworkLogTarget {
    fn write(&mut self, entry: &UnifiedLogEntry) -> Result<(), LogError> {
        if !self.capability.can_write {
            return Err(LogError::PermissionDenied);
        }

        let packet = self.format_packet(entry);

        if self.is_connected {
            self.forwarded_packets.push(packet);
            self.entries_written.fetch_add(1, Ordering::SeqCst);
            Ok(())
        } else {
            // Offline buffering / queueing
            if self.offline_ring_buffer.len() >= self.max_ring_buffer_size {
                self.offline_ring_buffer.remove(0); // Evict oldest entry in ring buffer
            }
            self.offline_ring_buffer.push(packet);

            // If failover target exists, try secondary IP
            if self.secondary_ip.is_some() {
                self.retry_attempts += 1;
            }

            Err(LogError::WriteFailed)
        }
    }

    fn flush(&mut self) -> Result<(), LogError> {
        if !self.is_connected {
            return Err(LogError::WriteFailed);
        }
        Ok(())
    }

    fn info(&self) -> TargetInfo {
        TargetInfo {
            target_type: TargetType::Network,
            buffer_size: self.max_ring_buffer_size,
            entries_written: self.entries_written.load(Ordering::SeqCst) as u64,
            capability: self.capability,
        }
    }
}

// ==========================================
// LOGGER MANAGER & IMPLEMENTATION
// ==========================================

/// Unified logger trait (OOP interface)
pub trait UnifiedLogger {
    /// Log message at level
    fn log(&mut self, level: LogLevel, component: &[u8], message: &[u8], module: &[u8], line: u32);
    /// Set global log level
    fn set_level(&mut self, level: LogLevel);
    /// Get global log level
    fn level(&self) -> LogLevel;
    /// Add target
    fn add_target(&mut self, target: Box<dyn LogTarget>) -> Result<(), LogError>;
    /// Remove target
    fn remove_target(&mut self, index: usize) -> Result<(), LogError>;
    /// Flush all targets
    fn flush(&mut self) -> Result<(), LogError>;
    /// Get logger statistics
    fn stats(&self) -> UnifiedLogStats;
}

/// Unified log statistics
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnifiedLogStats {
    pub total_entries: u64,
    pub entries_by_level: [u64; 6],
    pub entries_by_component: [u64; 16],
    pub targets: usize,
}

impl UnifiedLogStats {
    pub const fn new() -> Self {
        UnifiedLogStats {
            total_entries: 0,
            entries_by_level: [0; 6],
            entries_by_component: [0; 16],
            targets: 0,
        }
    }
}

impl Default for UnifiedLogStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple unified logger (OOP: Concrete logger class)
pub struct SimpleUnifiedLogger {
    pub level: AtomicUsize, // LogLevel as usize
    pub targets: Vec<Option<Box<dyn LogTarget>>>,
    pub stats: UnifiedLogStats,
    pub capability: LoggerCapability,
}

/// Logger capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoggerCapability {
    pub can_log: bool,
    pub can_set_level: bool,
    pub can_manage_targets: bool,
}

impl LoggerCapability {
    pub const fn new() -> Self {
        LoggerCapability {
            can_log: false,
            can_set_level: false,
            can_manage_targets: false,
        }
    }

    pub const fn full() -> Self {
        LoggerCapability {
            can_log: true,
            can_set_level: true,
            can_manage_targets: true,
        }
    }
}

impl Default for LoggerCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleUnifiedLogger {
    pub fn new(initial_level: LogLevel, capability: LoggerCapability) -> Self {
        SimpleUnifiedLogger {
            level: AtomicUsize::new(initial_level as usize),
            targets: Vec::new(),
            stats: UnifiedLogStats::new(),
            capability,
        }
    }

    fn get_component_index(&self, component: &[u8]) -> usize {
        let mut hash: usize = 0;
        for (i, &byte) in component.iter().enumerate() {
            hash = hash.wrapping_add((byte as usize) * (i + 1));
        }
        hash % 16
    }
}

impl UnifiedLogger for SimpleUnifiedLogger {
    fn log(&mut self, level: LogLevel, component: &[u8], message: &[u8], module: &[u8], line: u32) {
        if !self.capability.can_log {
            return;
        }

        let current_level = match self.level.load(Ordering::SeqCst) {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warning,
            4 => LogLevel::Error,
            _ => LogLevel::Fatal,
        };

        if level < current_level {
            return;
        }

        let entry = UnifiedLogEntry::new(level, component, message, module, line);

        for target_option in &mut self.targets {
            if let Some(ref mut target) = *target_option {
                let _ = target.write(&entry);
            }
        }

        self.stats.total_entries += 1;
        self.stats.entries_by_level[level as usize] += 1;

        let component_index = self.get_component_index(component);
        self.stats.entries_by_component[component_index] += 1;
    }

    fn set_level(&mut self, level: LogLevel) {
        if !self.capability.can_set_level {
            return;
        }
        self.level.store(level as usize, Ordering::SeqCst);
    }

    fn level(&self) -> LogLevel {
        match self.level.load(Ordering::SeqCst) {
            0 => LogLevel::Trace,
            1 => LogLevel::Debug,
            2 => LogLevel::Info,
            3 => LogLevel::Warning,
            4 => LogLevel::Error,
            _ => LogLevel::Fatal,
        }
    }

    fn add_target(&mut self, target: Box<dyn LogTarget>) -> Result<(), LogError> {
        if !self.capability.can_manage_targets {
            return Err(LogError::PermissionDenied);
        }

        self.targets.push(Some(target));
        self.stats.targets += 1;
        Ok(())
    }

    fn remove_target(&mut self, index: usize) -> Result<(), LogError> {
        if !self.capability.can_manage_targets {
            return Err(LogError::PermissionDenied);
        }

        if index < self.targets.len() {
            self.targets[index] = None;
            self.stats.targets -= 1;
            Ok(())
        } else {
            Err(LogError::WriteFailed)
        }
    }

    fn flush(&mut self) -> Result<(), LogError> {
        for target_option in &mut self.targets {
            if let Some(ref mut target) = *target_option {
                let _ = target.flush();
            }
        }
        Ok(())
    }

    fn stats(&self) -> UnifiedLogStats {
        self.stats
    }
}

/// Get current simulated time (nanoseconds)
fn get_current_time() -> u64 {
    static mut COUNTER: u64 = 0;
    unsafe {
        COUNTER += 1_000_000;
        COUNTER
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_log_entry_formatting() {
        let entry = UnifiedLogEntry::new(
            LogLevel::Error,
            b"KERNEL",
            b"Memory allocation failure",
            b"mm/slab.rs",
            128,
        )
        .with_facility(SyslogFacility::Kernel)
        .with_pid(4096)
        .with_field("subsystem", "slab")
        .with_field("err_code", "ENOMEM");

        assert_eq!(entry.syslog_pri(), 3); // Kernel(0)*8 + Error(3) = 3
        let json = entry.to_json();
        assert!(json.contains("\"level\":\"ERROR\""));
        assert!(json.contains("\"facility\":\"kern\""));
        assert!(json.contains("\"subsystem\":\"slab\""));

        let rfc5424 = entry.to_rfc5424("sigma-node-1", "kernel");
        assert!(rfc5424.starts_with("<3>1 "));
        assert!(rfc5424.contains("subsystem=\"slab\""));

        let journald = entry.to_journald_native();
        assert!(journald.contains("SYSLOG_IDENTIFIER=KERNEL"));
        assert!(journald.contains("_PID=4096"));
        assert!(journald.contains("SUBSYSTEM=slab"));
    }

    #[test]
    fn test_remote_network_log_forwarding() {
        let mut net_target = NetworkLogTarget::new("10.0.0.100", 514, TargetCapability::full())
            .with_protocol(NetworkProtocol::Tcp)
            .with_framing(NetworkFramingFormat::Rfc5424)
            .with_failover_ip("10.0.0.101");

        let entry1 = UnifiedLogEntry::new(LogLevel::Error, b"AUTH", b"Failed SSH login attempt", b"auth/pam.rs", 44)
            .with_facility(SyslogFacility::Auth)
            .with_pid(882)
            .with_field("ip", "192.168.1.50");

        // Send while connected
        net_target.write(&entry1).unwrap();
        assert_eq!(net_target.forwarded_packets.len(), 1);
        assert!(net_target.forwarded_packets[0].contains("Failed SSH login attempt"));
        assert!(net_target.forwarded_packets[0].contains("ip=\"192.168.1.50\""));

        // Simulate network outage -> buffer offline
        net_target.set_connection_status(false);
        let entry2 = UnifiedLogEntry::new(LogLevel::Warning, b"AUTH", b"Account locked", b"auth/pam.rs", 50);
        assert!(net_target.write(&entry2).is_err());
        assert_eq!(net_target.offline_ring_buffer.len(), 1);

        // Reconnect -> drain offline ring buffer
        net_target.set_connection_status(true);
        assert_eq!(net_target.offline_ring_buffer.len(), 0);
        assert_eq!(net_target.forwarded_packets.len(), 2);
    }

    #[test]
    fn test_file_log_target_rotation() {
        let mut target = FileLogTarget::new("/var/log/syslog.log", TargetCapability::full())
            .with_json_formatting(true)
            .with_max_entries(2);

        let entry1 = UnifiedLogEntry::new(LogLevel::Info, b"APP", b"Start service", b"main.rs", 10);
        let entry2 = UnifiedLogEntry::new(LogLevel::Info, b"APP", b"Process request", b"main.rs", 15);
        let entry3 = UnifiedLogEntry::new(LogLevel::Info, b"APP", b"Stop service", b"main.rs", 20);

        target.write(&entry1).unwrap();
        target.write(&entry2).unwrap();
        assert_eq!(target.file_buffer.len(), 2);

        // Entry 3 exceeds threshold, triggers automatic rotation
        target.write(&entry3).unwrap();
        assert_eq!(target.rotated_generations.len(), 1);
        assert_eq!(target.file_buffer.len(), 1);
        assert!(target.rotated_generations[0][0].contains("Start service"));
    }

    #[test]
    fn test_logger_routing_to_multiple_targets() {
        let mut logger = SimpleUnifiedLogger::new(LogLevel::Debug, LoggerCapability::full());

        let mem_target = MemoryLogTarget::new(5, TargetCapability::full());
        let file_target = FileLogTarget::new("/var/log/syslog", TargetCapability::full());
        let console_target = ConsoleLogTarget::new(TargetCapability::full());
        let net_target = NetworkLogTarget::new("192.168.1.50", 514, TargetCapability::full());

        logger.add_target(Box::new(mem_target)).unwrap();
        logger.add_target(Box::new(file_target)).unwrap();
        logger.add_target(Box::new(console_target)).unwrap();
        logger.add_target(Box::new(net_target)).unwrap();

        assert_eq!(logger.stats().targets, 4);

        // Dispatch log entry
        logger.log(
            LogLevel::Warning,
            b"SYS",
            b"Kernel panic averted",
            b"main.rs",
            42,
        );

        // Verify statistics
        assert_eq!(logger.stats().total_entries, 1);
        assert_eq!(
            logger.stats().entries_by_level[LogLevel::Warning as usize],
            1
        );
    }
}
