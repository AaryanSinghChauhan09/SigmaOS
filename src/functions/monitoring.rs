//! System Monitoring Functions (systemd-journalctl/htop Inspiration)
//! System log viewer, system monitor, and system information tools
use alloc::format;
extern crate alloc;



use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Log priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogPriority {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Debug,
}

/// Journal entry
#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub timestamp: u64,
    pub priority: LogPriority,
    pub service: String,
    pub message: String,
    pub process_id: u32,
    pub hostname: String,
}

impl JournalEntry {
    pub fn new(priority: LogPriority, service: &str, message: &str) -> Self {
        Self {
            timestamp: 0,
            priority,
            service: service.to_string(),
            message: message.to_string(),
            process_id: 0,
            hostname: "sigmaos".to_string(),
        }
    }
}

/// Log filter
#[derive(Debug, Clone)]
pub struct LogFilter {
    pub priority_min: Option<LogPriority>,
    pub service: Option<String>,
    pub time_range: Option<(u64, u64)>,
}

impl LogFilter {
    pub fn new() -> Self {
        Self {
            priority_min: None,
            service: None,
            time_range: None,
        }
    }

    pub fn set_priority_min(&mut self, priority: LogPriority) {
        self.priority_min = Some(priority);
    }

    pub fn set_service(&mut self, service: &str) {
        self.service = Some(service.to_string());
    }

    pub fn set_time_range(&mut self, start: u64, end: u64) {
        self.time_range = Some((start, end));
    }

    pub fn matches(&self, entry: &JournalEntry) -> bool {
        if let Some(min) = self.priority_min {
            if entry.priority < min {
                return false;
            }
        }
        if let Some(service) = &self.service {
            if &entry.service != service {
                return false;
            }
        }
        if let Some((start, end)) = self.time_range {
            if entry.timestamp < start || entry.timestamp > end {
                return false;
            }
        }
        true
    }
}

/// Journal viewer
pub struct JournalViewer {
    pub entries: Vec<JournalEntry>,
    pub filters: Vec<LogFilter>,
    pub following: bool,
}

impl JournalViewer {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            filters: Vec::new(),
            following: false,
        }
    }

    pub fn add_entry(&mut self, entry: JournalEntry) {
        self.entries.push(entry);
    }

    pub fn add_filter(&mut self, filter: LogFilter) {
        self.filters.push(filter);
    }

    pub fn get_filtered_entries(&self) -> Vec<&JournalEntry> {
        let mut filtered = self.entries.iter().collect::<Vec<_>>();
        
        for filter in &self.filters {
            filtered = filtered.iter().filter(|e| filter.matches(e)).cloned().collect();
        }
        
        filtered
    }

    pub fn follow(&mut self) {
        self.following = true;
    }

    pub fn unfollow(&mut self) {
        self.following = false;
    }

    pub fn export(&self, format: ExportFormat) -> Result<String, JournalError> {
        match format {
            ExportFormat::Text => self.export_text(),
            ExportFormat::Json => self.export_json(),
            ExportFormat::Csv => self.export_csv(),
        }
    }

    fn export_text(&self) -> Result<String, JournalError> {
        let mut output = String::new();
        for entry in &self.entries {
            output.push_str(&format!("{} {} {}: {}\n", 
                entry.timestamp, 
                entry.priority as u8,
                entry.service,
                entry.message
            ));
        }
        Ok(output)
    }

    fn export_json(&self) -> Result<String, JournalError> {
        Ok("{}".to_string()) // Simplified JSON export
    }

    fn export_csv(&self) -> Result<String, JournalError> {
        let mut output = String::new();
        output.push_str("timestamp,priority,service,message\n");
        for entry in &self.entries {
            output.push_str(&format!("{},{},{},{}\n",
                entry.timestamp,
                entry.priority as u8,
                entry.service,
                entry.message
            ));
        }
        Ok(output)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Text,
    Json,
    Csv,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JournalError {
    ExportFailed,
    FilterError,
    ReadError,
}

/// CPU stats
#[derive(Debug, Clone)]
pub struct CpuStats {
    pub user: f64,
    pub system: f64,
    pub idle: f64,
    pub iowait: f64,
}

/// Memory stats
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_used: u64,
}

/// Process info
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub state: String,
    pub parent_pid: Option<u32>,
}

/// IO stats
#[derive(Debug, Clone)]
pub struct IOStats {
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_count: u64,
    pub write_count: u64,
}

/// System monitor
pub struct SystemMonitor {
    pub cpu_usage: CpuStats,
    pub memory_usage: MemoryStats,
    pub process_list: Vec<ProcessInfo>,
    pub io_stats: IOStats,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            cpu_usage: CpuStats {
                user: 0.0,
                system: 0.0,
                idle: 100.0,
                iowait: 0.0,
            },
            memory_usage: MemoryStats {
                total: 8192,
                used: 4096,
                free: 4096,
                cached: 1024,
                swap_total: 4096,
                swap_used: 0,
            },
            process_list: Vec::new(),
            io_stats: IOStats {
                read_bytes: 0,
                write_bytes: 0,
                read_count: 0,
                write_count: 0,
            },
        }
    }

    pub fn update(&mut self) {
        // Update system statistics
        self.cpu_usage.user = 25.0;
        self.cpu_usage.system = 10.0;
        self.cpu_usage.idle = 65.0;
        self.cpu_usage.iowait = 0.0;
    }

    pub fn get_process_by_pid(&mut self, pid: u32) -> Option<&mut ProcessInfo> {
        self.process_list.iter_mut().find(|p| p.pid == pid)
    }

    pub fn sort_processes_by_cpu(&mut self) {
        self.process_list.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
    }

    pub fn sort_processes_by_memory(&mut self) {
        self.process_list.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
    }

    pub fn get_monitor_stats(&self) -> MonitorStats {
        MonitorStats {
            total_processes: self.process_list.len(),
            cpu_usage: self.cpu_usage.user + self.cpu_usage.system,
            memory_usage: (self.memory_usage.used as f64 / self.memory_usage.total as f64) * 100.0,
            total_io: self.io_stats.read_bytes + self.io_stats.write_bytes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonitorStats {
    pub total_processes: usize,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub total_io: u64,
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// System info
pub struct SystemInfo {
    pub hostname: String,
    pub os_version: String,
    pub kernel_version: String,
    pub uptime: u64,
    pub hardware: HardwareInfo,
}

#[derive(Debug, Clone)]
pub struct HardwareInfo {
    pub cpu_model: String,
    pub cpu_cores: u32,
    pub total_memory: u64,
    pub disk_size: u64,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            hostname: "sigmaos".to_string(),
            os_version: "1.0.0".to_string(),
            kernel_version: "5.0.0-sigma".to_string(),
            uptime: 0,
            hardware: HardwareInfo {
                cpu_model: "SigmaOS CPU".to_string(),
                cpu_cores: 4,
                total_memory: 8192,
                disk_size: 102400,
            },
        }
    }

    pub fn update_uptime(&mut self, uptime: u64) {
        self.uptime = uptime;
    }

    pub fn format_uptime(&self) -> String {
        let hours = self.uptime / 3600;
        let minutes = (self.uptime % 3600) / 60;
        format!("{}h {}m", hours, minutes)
    }

    pub fn get_info_summary(&self) -> String {
        format!(
            "{} {} {} CPU: {} {} ({})",
            self.hostname,
            self.os_version,
            self.kernel_version,
            self.hardware.cpu_cores,
            self.hardware.cpu_model,
            self.format_uptime()
        )
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_entry() {
        let entry = JournalEntry::new(LogPriority::Info, "test-service", "test message");
        assert_eq!(entry.service, "test-service");
    }

    #[test]
    fn test_log_filter() {
        let filter = LogFilter::new();
        filter.set_priority_min(LogPriority::Warning);
        let entry = JournalEntry::new(LogPriority::Error, "test", "msg");
        assert!(filter.matches(&entry));
    }

    #[test]
    fn test_journal_viewer() {
        let mut viewer = JournalViewer::new();
        let entry = JournalEntry::new(LogPriority::Info, "test", "msg");
        viewer.add_entry(entry);
        assert_eq!(viewer.entries.len(), 1);
    }

    #[test]
    fn test_system_monitor() {
        let mut monitor = SystemMonitor::new();
        monitor.update();
        assert!(monitor.cpu_usage.user > 0.0);
    }

    #[test]
    fn test_system_info() {
        let info = SystemInfo::new();
        assert_eq!(info.hostname, "sigmaos");
    }
}