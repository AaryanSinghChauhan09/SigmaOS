//! SigmaOS Upload Log Manager
//!
//! Inspired by: Omarchy upload log (omancy-upload-log)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
/// SECURE_DEFAULTS, MEMORY_SAFETY, USER_CONTROL, TRANSPARENCY, COLLABORATION.

/// Upload status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UploadStatus {
    Pending,
    Uploading,
    Completed,
    Failed(String),
    Cancelled,
}

/// Log entry
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub log_type: LogType,
    pub created_at: u64,
    pub upload_status: UploadStatus,
    pub upload_url: Option<String>,
}

/// Log type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogType {
    System,
    Crash,
    Debug,
    Audit,
    Custom(String),
}

/// Upload log manager
pub struct UploadLogManager {
    pub logs: Vec<LogEntry>,
    pub next_id: u64,
    pub max_logs: usize,
}

impl UploadLogManager {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            next_id: 1,
            max_logs: 50,
        }
    }

    /// Create a log entry
    pub fn create_log(
        &mut self,
        title: String,
        content: String,
        log_type: LogType,
    ) -> Result<u64, String> {
        if title.is_empty() {
            return Err("Log title cannot be empty".to_string());
        }
        if content.is_empty() {
            return Err("Log content cannot be empty".to_string());
        }

        let id = self.next_id;
        self.next_id += 1;

        let entry = LogEntry {
            id,
            title,
            content,
            log_type,
            created_at: now_secs(),
            upload_status: UploadStatus::Pending,
            upload_url: None,
        };

        self.logs.push(entry);
        self.trim_logs();
        Ok(id)
    }

    /// Upload a log
    pub fn upload_log(&mut self, id: u64) -> Result<String, String> {
        let log = self
            .logs
            .iter_mut()
            .find(|l| l.id == id)
            .ok_or_else(|| format!("Log {} not found", id))?;

        if log.upload_status == UploadStatus::Uploading {
            return Err("Log already uploading".to_string());
        }

        log.upload_status = UploadStatus::Uploading;

        // Simulate upload
        let url = format!("https://sigmaos.example.com/logs/{}", id);
        log.upload_status = UploadStatus::Completed;
        log.upload_url = Some(url.clone());

        Ok(url)
    }

    /// List all logs
    pub fn list_logs(&self) -> Vec<&LogEntry> {
        self.logs.iter().rev().collect()
    }

    /// Get a log by ID
    pub fn get_log(&self, id: u64) -> Option<&LogEntry> {
        self.logs.iter().find(|l| l.id == id)
    }

    /// Delete a log
    pub fn delete_log(&mut self, id: u64) -> Result<(), String> {
        let pos = self
            .logs
            .iter()
            .position(|l| l.id == id)
            .ok_or_else(|| format!("Log {} not found", id))?;
        self.logs.remove(pos);
        Ok(())
    }

    /// Get pending uploads
    pub fn get_pending_uploads(&self) -> Vec<&LogEntry> {
        self.logs
            .iter()
            .filter(|l| l.upload_status == UploadStatus::Pending)
            .collect()
    }

    /// Log count
    pub fn log_count(&self) -> usize {
        self.logs.len()
    }

    fn trim_logs(&mut self) {
        while self.logs.len() > self.max_logs {
            self.logs.remove(0);
        }
    }
}

impl Default for UploadLogManager {
    fn default() -> Self {
        Self::new()
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_log() {
        let mut mgr = UploadLogManager::new();
        let id = mgr.create_log(
            "System Crash".to_string(),
            "Kernel panic at 0xdeadbeef".to_string(),
            LogType::Crash,
        )
        .unwrap();
        assert_eq!(id, 1);
        assert_eq!(mgr.log_count(), 1);
    }

    #[test]
    fn test_create_log_empty_title_fails() {
        let mut mgr = UploadLogManager::new();
        let result = mgr.create_log(String::new(), "content".to_string(), LogType::System);
        assert!(result.is_err());
    }

    #[test]
    fn test_upload_log() {
        let mut mgr = UploadLogManager::new();
        mgr.create_log("Test".to_string(), "content".to_string(), LogType::System)
            .unwrap();
        let url = mgr.upload_log(1).unwrap();
        assert!(url.contains("sigmaos.example.com"));
        let log = mgr.get_log(1).unwrap();
        assert_eq!(log.upload_status, UploadStatus::Completed);
        assert!(log.upload_url.is_some());
    }

    #[test]
    fn test_upload_nonexistent_fails() {
        let mut mgr = UploadLogManager::new();
        assert!(mgr.upload_log(999).is_err());
    }

    #[test]
    fn test_get_pending_uploads() {
        let mut mgr = UploadLogManager::new();
        mgr.create_log("Log1".to_string(), "content1".to_string(), LogType::System)
            .unwrap();
        mgr.create_log("Log2".to_string(), "content2".to_string(), LogType::Crash)
            .unwrap();
        assert_eq!(mgr.get_pending_uploads().len(), 2);
    }

    #[test]
    fn test_delete_log() {
        let mut mgr = UploadLogManager::new();
        mgr.create_log("Test".to_string(), "content".to_string(), LogType::System)
            .unwrap();
        assert!(mgr.delete_log(1).is_ok());
        assert_eq!(mgr.log_count(), 0);
    }
}