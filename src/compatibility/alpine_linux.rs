//! Alpine Linux & BusyBox-inspired Minimal Footprint Subsystems
//! Implements APK database parser, busybox syslog circular logger, and multicall binary router.

extern crate alloc;
use alloc::collections::BTreeMap as HashMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ==========================================
// 1. APK Database Installed Package Parser
// ==========================================

#[derive(Debug, Clone)]
pub struct ApkInstalledPackage {
    pub name: String,
    pub version: String,
    pub size_bytes: u64,
    pub description: String,
}

pub struct ApkDatabaseIndex {
    pub installed: HashMap<String, ApkInstalledPackage>,
    pub total_footprint_bytes: u64,
}

impl ApkDatabaseIndex {
    pub fn new() -> Self {
        Self {
            installed: HashMap::new(),
            total_footprint_bytes: 0,
        }
    }

    /// Parses Alpine '/lib/apk/db/installed' flat-file blocks into package structures
    pub fn parse_installed_db(&mut self, content: &str) {
        let mut name = String::new();
        let mut version = String::new();
        let mut size = 0u64;
        let mut desc = String::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                // End of package block -> insert
                if !name.is_empty() {
                    self.total_footprint_bytes += size;
                    self.installed.insert(
                        name.clone(),
                        ApkInstalledPackage {
                            name: name.clone(),
                            version: version.clone(),
                            size_bytes: size,
                            description: desc.clone(),
                        },
                    );
                }
                // Reset for next block
                name.clear();
                version.clear();
                size = 0;
                desc.clear();
            } else if trimmed.starts_with("P:") {
                name = trimmed[2..].to_string();
            } else if trimmed.starts_with("V:") {
                version = trimmed[2..].to_string();
            } else if trimmed.starts_with("I:") {
                size = trimmed[2..].parse::<u64>().unwrap_or(0);
            } else if trimmed.starts_with("T:") {
                desc = trimmed[2..].to_string();
            }
        }

        // Handle final block if no trailing newline
        if !name.is_empty() {
            self.total_footprint_bytes += size;
            self.installed.insert(
                name.clone(),
                ApkInstalledPackage {
                    name,
                    version,
                    size_bytes: size,
                    description: desc,
                },
            );
        }
    }
}

impl Default for ApkDatabaseIndex {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. Alpine-style BusyBox Syslog Circular Logger
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyslogSeverity {
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
pub struct SyslogMessage {
    pub severity: SyslogSeverity,
    pub message: String,
    pub timestamp_ms: u64,
}

pub struct AlpineSyslogManager {
    pub ring_buffer: Vec<SyslogMessage>,
    pub max_capacity: usize,
}

impl AlpineSyslogManager {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            ring_buffer: Vec::with_capacity(max_capacity),
            max_capacity,
        }
    }

    /// Logs a system event. Evicts oldest messages when capacity is reached (zero heap reallocations)
    pub fn log_event(&mut self, severity: SyslogSeverity, message: &str, timestamp_ms: u64) {
        if self.ring_buffer.len() >= self.max_capacity {
            self.ring_buffer.remove(0); // Evict oldest
        }
        self.ring_buffer.push(SyslogMessage {
            severity,
            message: message.to_string(),
            timestamp_ms,
        });
    }

    pub fn query_logs_by_severity(&self, min_severity: SyslogSeverity) -> Vec<SyslogMessage> {
        let mut results = Vec::new();
        for msg in &self.ring_buffer {
            if msg.severity <= min_severity {
                results.push(msg.clone());
            }
        }
        results
    }
}

// ==========================================
// 3. BusyBox Multi-Call Binary Router
// ==========================================

pub struct BusyBoxMulticall {
    pub applet_registry: HashMap<String, String>, // AppletName -> HelpString
}

impl BusyBoxMulticall {
    pub fn new() -> Self {
        let mut registry = HashMap::new();
        registry.insert(
            "ls".to_string(),
            "BusyBox v1.36.0 ls: list files".to_string(),
        );
        registry.insert(
            "cat".to_string(),
            "BusyBox v1.36.0 cat: print file content".to_string(),
        );
        registry.insert(
            "grep".to_string(),
            "BusyBox v1.36.0 grep: search pattern".to_string(),
        );
        Self {
            applet_registry: registry,
        }
    }

    /// Simulates binary invocation via symbolic link routing (argv[0])
    pub fn invoke_applet(&self, argv0: &str) -> Result<String, &'static str> {
        if let Some(help) = self.applet_registry.get(argv0) {
            Ok(help.clone())
        } else {
            Err("BusyBox: applet not found")
        }
    }
}

impl Default for BusyBoxMulticall {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 4. Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apk_database_parsing() {
        let mut db = ApkDatabaseIndex::new();
        let db_content = "\
P:musl
V:1.2.4
I:614400
T:standard musl shared library

P:zlib
V:1.3
I:102400
T:zlib compression library
";
        db.parse_installed_db(db_content);
        assert_eq!(db.installed.len(), 2);
        assert_eq!(db.total_footprint_bytes, 716800);
        assert_eq!(db.installed.get("musl").unwrap().version, "1.2.4");
    }

    #[test]
    fn test_alpine_syslog_ring_buffer() {
        let mut syslog = AlpineSyslogManager::new(2); // physical limit of 2 entries
        syslog.log_event(SyslogSeverity::Critical, "Kernel panic trace", 100);
        syslog.log_event(SyslogSeverity::Warning, "Low disk space", 200);
        assert_eq!(syslog.ring_buffer.len(), 2);

        // Exceed capacity -> triggers oldest eviction
        syslog.log_event(SyslogSeverity::Info, "User login", 300);
        assert_eq!(syslog.ring_buffer.len(), 2);
        assert_eq!(syslog.ring_buffer[0].message, "Low disk space"); // kernel panic evicted

        let criticals = syslog.query_logs_by_severity(SyslogSeverity::Critical);
        assert_eq!(criticals.len(), 0); // critical was evicted
    }

    #[test]
    fn test_busybox_multicall() {
        let busybox = BusyBoxMulticall::new();
        assert_eq!(
            busybox.invoke_applet("ls").unwrap(),
            "BusyBox v1.36.0 ls: list files"
        );
        assert_eq!(
            busybox.invoke_applet("invalid_cmd"),
            Err("BusyBox: applet not found")
        );
    }
}
