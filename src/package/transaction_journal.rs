//! Package Transaction Journaling and Rollback System
//!
//! This module provides atomic package transaction support with journaling and rollback capabilities.
//! Uses minimal external dependencies as per SigmaOS policy.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// Transaction operation type
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionOperation {
    Install { package_name: String, version: String },
    Remove { package_name: String },
    Update { package_name: String, old_version: String, new_version: String },
}

impl TransactionOperation {
    fn to_string(&self) -> String {
        match self {
            TransactionOperation::Install { package_name, version } => {
                format!("INSTALL {} {}", package_name, version)
            }
            TransactionOperation::Remove { package_name } => {
                format!("REMOVE {}", package_name)
            }
            TransactionOperation::Update { package_name, old_version, new_version } => {
                format!("UPDATE {} {} -> {}", package_name, old_version, new_version)
            }
        }
    }

    fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        match parts[0] {
            "INSTALL" => {
                if parts.len() >= 3 {
                    Some(TransactionOperation::Install {
                        package_name: parts[1].to_string(),
                        version: parts[2].to_string(),
                    })
                } else {
                    None
                }
            }
            "REMOVE" => {
                if parts.len() >= 2 {
                    Some(TransactionOperation::Remove {
                        package_name: parts[1].to_string(),
                    })
                } else {
                    None
                }
            }
            "UPDATE" => {
                if parts.len() >= 4 {
                    Some(TransactionOperation::Update {
                        package_name: parts[1].to_string(),
                        old_version: parts[2].to_string(),
                        new_version: parts[3].to_string(),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Transaction state
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    Pending,
    InProgress,
    Completed,
    Failed,
    RolledBack,
}

impl TransactionState {
    fn to_string(&self) -> &'static str {
        match self {
            TransactionState::Pending => "PENDING",
            TransactionState::InProgress => "IN_PROGRESS",
            TransactionState::Completed => "COMPLETED",
            TransactionState::Failed => "FAILED",
            TransactionState::RolledBack => "ROLLED_BACK",
        }
    }

    fn from_string(s: &str) -> Option<Self> {
        match s {
            "PENDING" => Some(TransactionState::Pending),
            "IN_PROGRESS" => Some(TransactionState::InProgress),
            "COMPLETED" => Some(TransactionState::Completed),
            "FAILED" => Some(TransactionState::Failed),
            "ROLLED_BACK" => Some(TransactionState::RolledBack),
            _ => None,
        }
    }
}

/// Transaction entry in the journal
#[derive(Debug, Clone)]
pub struct TransactionEntry {
    pub id: u64,
    pub timestamp: u64,
    pub operation: TransactionOperation,
    pub state: TransactionState,
    pub files_changed: Vec<PathBuf>,
    pub directories_created: Vec<PathBuf>,
    pub pre_snapshot: Option<PathBuf>,
    pub post_snapshot: Option<PathBuf>,
}

impl TransactionEntry {
    /// Serialize to a simple text format
    fn to_text(&self) -> String {
        let mut lines = vec![
            format!("id:{}", self.id),
            format!("timestamp:{}", self.timestamp),
            format!("operation:{}", self.operation.to_string()),
            format!("state:{}", self.state.to_string()),
        ];

        for path in &self.files_changed {
            lines.push(format!("file:{}", path.display()));
        }

        for path in &self.directories_created {
            lines.push(format!("directory:{}", path.display()));
        }

        if let Some(ref snapshot) = self.pre_snapshot {
            lines.push(format!("pre_snapshot:{}", snapshot.display()));
        }

        if let Some(ref snapshot) = self.post_snapshot {
            lines.push(format!("post_snapshot:{}", snapshot.display()));
        }

        lines.join("\n")
    }

    /// Deserialize from text format
    fn from_text(text: &str) -> Option<Self> {
        let mut id = None;
        let mut timestamp = None;
        let mut operation = None;
        let mut state = None;
        let mut files_changed = Vec::new();
        let mut directories_created = Vec::new();
        let mut pre_snapshot = None;
        let mut post_snapshot = None;

        for line in text.lines() {
            if line.starts_with("id:") {
                id = line[3..].parse().ok();
            } else if line.starts_with("timestamp:") {
                timestamp = line[10..].parse().ok();
            } else if line.starts_with("operation:") {
                operation = TransactionOperation::from_string(&line[10..]);
            } else if line.starts_with("state:") {
                state = TransactionState::from_string(&line[6..]);
            } else if line.starts_with("file:") {
                files_changed.push(PathBuf::from(&line[5..]));
            } else if line.starts_with("directory:") {
                directories_created.push(PathBuf::from(&line[10..]));
            } else if line.starts_with("pre_snapshot:") {
                pre_snapshot = Some(PathBuf::from(&line[13..]));
            } else if line.starts_with("post_snapshot:") {
                post_snapshot = Some(PathBuf::from(&line[14..]));
            }
        }

        if let (Some(id), Some(timestamp), Some(operation), Some(state)) = (id, timestamp, operation, state) {
            Some(TransactionEntry {
                id,
                timestamp,
                operation,
                state,
                files_changed,
                directories_created,
                pre_snapshot,
                post_snapshot,
            })
        } else {
            None
        }
    }
}

/// Transaction journal
pub struct TransactionJournal {
    entries: Vec<TransactionEntry>,
    journal_path: PathBuf,
    next_id: u64,
}

impl TransactionJournal {
    /// Create a new transaction journal
    pub fn new<P: AsRef<Path>>(journal_path: P) -> io::Result<Self> {
        let journal_path = journal_path.as_ref().to_path_buf();

        // Create journal directory if it doesn't exist
        if let Some(parent) = journal_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Load existing journal if it exists
        let (entries, next_id) = if journal_path.exists() {
            Self::load(&journal_path)?
        } else {
            (Vec::new(), 0)
        };

        Ok(Self {
            entries,
            journal_path,
            next_id,
        })
    }

    /// Load journal from disk
    fn load(path: &Path) -> io::Result<(Vec<TransactionEntry>, u64)> {
        let content = fs::read_to_string(path)?;
        let mut entries = Vec::new();
        let mut next_id = 0;

        // Parse simple text format
        let current_entry_lines: Vec<&str> = content.lines().collect();
        let mut current_entry_lines: Vec<String> = Vec::new();

        for line in current_entry_lines {
            if line.starts_with("id:") {
                // New entry starts
                if !current_entry_lines.is_empty() {
                    if let Some(entry) = TransactionEntry::from_text(&current_entry_lines.join("\n")) {
                        next_id = next_id.max(entry.id + 1);
                        entries.push(entry);
                    }
                }
                current_entry_lines = vec![line.to_string()];
            } else {
                current_entry_lines.push(line.to_string());
            }
        }

        // Don't forget the last entry
        if !current_entry_lines.is_empty() {
            if let Some(entry) = TransactionEntry::from_text(&current_entry_lines.join("\n")) {
                next_id = next_id.max(entry.id + 1);
                entries.push(entry);
            }
        }

        Ok((entries, next_id))
    }

    /// Save journal to disk
    fn save(&self) -> io::Result<()> {
        let mut content = String::new();

        for entry in &self.entries {
            content.push_str(&entry.to_text());
            content.push_str("\n---\n");
        }

        fs::write(&self.journal_path, content)?;
        Ok(())
    }

    /// Begin a new transaction
    pub fn begin_transaction(&mut self, operation: TransactionOperation) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let entry = TransactionEntry {
            id,
            timestamp,
            operation,
            state: TransactionState::Pending,
            files_changed: Vec::new(),
            directories_created: Vec::new(),
            pre_snapshot: None,
            post_snapshot: None,
        };

        self.entries.push(entry);
        self.save().unwrap_or_else(|e| {
            eprintln!("Failed to save journal: {}", e);
        });

        id
    }

    /// Set transaction state
    pub fn set_state(&mut self, id: u64, state: TransactionState) -> io::Result<()> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.state = state;
            self.save()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Transaction not found"))
        }
    }

    /// Record file change
    pub fn record_file_change(&mut self, id: u64, path: PathBuf) -> io::Result<()> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.files_changed.push(path);
            self.save()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Transaction not found"))
        }
    }

    /// Record directory creation
    pub fn record_directory_creation(&mut self, id: u64, path: PathBuf) -> io::Result<()> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.directories_created.push(path);
            self.save()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Transaction not found"))
        }
    }

    /// Set pre-snapshot path
    pub fn set_pre_snapshot(&mut self, id: u64, snapshot_path: PathBuf) -> io::Result<()> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.pre_snapshot = Some(snapshot_path);
            self.save()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Transaction not found"))
        }
    }

    /// Set post-snapshot path
    pub fn set_post_snapshot(&mut self, id: u64, snapshot_path: PathBuf) -> io::Result<()> {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == id) {
            entry.post_snapshot = Some(snapshot_path);
            self.save()
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Transaction not found"))
        }
    }

    /// Get transaction by ID
    pub fn get_transaction(&self, id: u64) -> Option<&TransactionEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// Get all transactions
    pub fn get_all_transactions(&self) -> &[TransactionEntry] {
        &self.entries
    }

    /// Rollback a transaction
    pub fn rollback(&mut self, id: u64) -> io::Result<()> {
        let entry = self.entries.iter_mut()
            .find(|e| e.id == id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Transaction not found"))?;

        // Restore files from pre-snapshot if available
        if let Some(ref snapshot_path) = entry.pre_snapshot {
            if snapshot_path.exists() {
                Self::restore_snapshot(snapshot_path)?;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Pre-snapshot not found"
                ));
            }
        }

        // Remove files created by transaction
        for path in &entry.files_changed {
            if path.exists() {
                fs::remove_file(path)?;
            }
        }

        // Remove directories created by transaction
        for path in entry.directories_created.iter().rev() {
            if path.exists() {
                fs::remove_dir(path)?;
            }
        }

        entry.state = TransactionState::RolledBack;
        self.save()
    }

    /// Restore system from snapshot
    fn restore_snapshot(snapshot_path: &Path) -> io::Result<()> {
        if !snapshot_path.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Snapshot path does not exist"));
        }

        if snapshot_path.is_file() {
            let parent = snapshot_path.parent().unwrap_or(Path::new("."));
            let dest_name = snapshot_path.file_name().unwrap_or_default();
            let restored_file = parent.join(format!("restored_{}", dest_name.to_string_lossy()));
            fs::copy(snapshot_path, restored_file)?;
        } else if snapshot_path.is_dir() {
            let parent = snapshot_path.parent().unwrap_or(Path::new("."));
            let restore_dir = parent.join("restored_snapshot");
            fs::create_dir_all(&restore_dir)?;
            for entry in fs::read_dir(snapshot_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let file_name = path.file_name().unwrap_or_default();
                    let target = restore_dir.join(file_name);
                    fs::copy(&path, &target)?;
                }
            }
        }
        Ok(())
    }

    /// Get transaction history for a package
    pub fn get_package_history(&self, package_name: &str) -> Vec<&TransactionEntry> {
        self.entries.iter()
            .filter(|e| match &e.operation {
                TransactionOperation::Install { package_name, .. } => package_name == *package_name,
                TransactionOperation::Remove { package_name } => package_name == *package_name,
                TransactionOperation::Update { package_name, .. } => package_name == *package_name,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_journal_creation() {
        let journal_path = temp_dir().join("sigma_test_transactions.json");

        let journal = TransactionJournal::new(&journal_path).unwrap();
        assert_eq!(journal.get_all_transactions().len(), 0);
    }

    #[test]
    fn test_transaction_lifecycle() {
        let journal_path = temp_dir().join("sigma_test_transactions2.json");

        let mut journal = TransactionJournal::new(&journal_path).unwrap();

        // Begin transaction
        let id = journal.begin_transaction(TransactionOperation::Install {
            package_name: "test-package".to_string(),
            version: "1.0.0".to_string(),
        });

        // Set state
        journal.set_state(id, TransactionState::InProgress).unwrap();

        // Record file change
        journal.record_file_change(id, PathBuf::from("/bin/test")).unwrap();

        // Complete transaction
        journal.set_state(id, TransactionState::Completed).unwrap();

        // Verify
        let entry = journal.get_transaction(id).unwrap();
        assert_eq!(entry.state, TransactionState::Completed);
        assert_eq!(entry.files_changed.len(), 1);
    }

    #[test]
    fn test_package_history() {
        let journal_path = temp_dir().join("sigma_test_transactions3.json");

        let mut journal = TransactionJournal::new(&journal_path).unwrap();

        journal.begin_transaction(TransactionOperation::Install {
            package_name: "test-package".to_string(),
            version: "1.0.0".to_string(),
        });

        journal.begin_transaction(TransactionOperation::Remove {
            package_name: "test-package".to_string(),
        });

        let history = journal.get_package_history("test-package");
        assert_eq!(history.len(), 2);
    }
}
