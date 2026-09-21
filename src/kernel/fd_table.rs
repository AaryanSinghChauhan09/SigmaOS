// Linux-inspired file descriptor table manager
// Provides file descriptor management for processes

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// File descriptor flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FdFlags {
    pub close_on_exec: bool,
}

impl FdFlags {
    pub fn new() -> Self {
        Self {
            close_on_exec: false,
        }
    }

    pub fn with_close_on_exec(mut self, value: bool) -> Self {
        self.close_on_exec = value;
        self
    }
}

impl Default for FdFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// File descriptor entry
#[derive(Debug, Clone)]
pub struct FdEntry {
    pub fd: i32,
    pub file_type: String,
    pub flags: FdFlags,
    pub offset: u64,
}

impl FdEntry {
    pub fn new(fd: i32, file_type: String, flags: FdFlags) -> Self {
        Self {
            fd,
            file_type,
            flags,
            offset: 0,
        }
    }

    /// Get file descriptor
    pub fn get_fd(&self) -> i32 {
        self.fd
    }

    /// Get file type
    pub fn get_file_type(&self) -> &str {
        &self.file_type
    }

    /// Get flags
    pub fn get_flags(&self) -> FdFlags {
        self.flags
    }

    /// Get offset
    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    /// Set offset
    pub fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }
}

/// File descriptor table
#[derive(Debug, Clone)]
pub struct FdTable {
    pub entries: HashMap<i32, FdEntry>,
    pub next_fd: i32,
}

impl FdTable {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_fd: 3, // Start from 3 (0=stdin, 1=stdout, 2=stderr)
        }
    }

    /// Allocate a new file descriptor
    pub fn allocate(&mut self, file_type: String, flags: FdFlags) -> i32 {
        let fd = self.next_fd;
        self.next_fd += 1;

        let entry = FdEntry::new(fd, file_type, flags);
        self.entries.insert(fd, entry);

        fd
    }

    /// Get a file descriptor entry
    pub fn get(&self, fd: i32) -> Option<&FdEntry> {
        self.entries.get(&fd)
    }

    /// Get a mutable file descriptor entry
    pub fn get_mut(&mut self, fd: i32) -> Option<&mut FdEntry> {
        self.entries.get_mut(&fd)
    }

    /// Close a file descriptor
    pub fn close(&mut self, fd: i32) -> Result<(), String> {
        match self.entries.remove(&fd) {
            Some(_) => Ok(()),
            None => Err(format!("File descriptor {} not found", fd)),
        }
    }

    /// Duplicate a file descriptor
    pub fn dup(&mut self, old_fd: i32) -> Result<i32, String> {
        let entry = self.entries.get(&old_fd).ok_or_else(|| format!("File descriptor {} not found", old_fd))?.clone();
        
        let new_fd = self.next_fd;
        self.next_fd += 1;

        let new_entry = FdEntry::new(new_fd, entry.file_type.clone(), entry.flags);
        self.entries.insert(new_fd, new_entry);

        Ok(new_fd)
    }

    /// Duplicate a file descriptor to a specific fd
    pub fn dup2(&mut self, old_fd: i32, new_fd: i32) -> Result<i32, String> {
        let entry = self.entries.get(&old_fd).ok_or_else(|| format!("File descriptor {} not found", old_fd))?.clone();
        
        // Close the new_fd if it exists
        if self.entries.contains_key(&new_fd) {
            self.entries.remove(&new_fd);
        }

        let new_entry = FdEntry::new(new_fd, entry.file_type.clone(), entry.flags);
        self.entries.insert(new_fd, new_entry);

        Ok(new_fd)
    }

    /// Get file descriptor count
    pub fn fd_count(&self) -> usize {
        self.entries.len()
    }

    /// Check if file descriptor exists
    pub fn contains(&self, fd: i32) -> bool {
        self.entries.contains_key(&fd)
    }
}

impl Default for FdTable {
    fn default() -> Self {
        Self::new()
    }
}

/// File descriptor table manager for system-wide fd table management
pub struct FdTableManager {
    tables: Arc<Mutex<HashMap<u64, FdTable>>>,
    next_table_id: Arc<Mutex<u64>>,
}

impl FdTableManager {
    pub fn new() -> Self {
        Self {
            tables: Arc::new(Mutex::new(HashMap::new())),
            next_table_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new fd table
    pub fn create_table(&self) -> u64 {
        let mut next_id = self.next_table_id.lock().unwrap();
        let table_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let table = FdTable::new();
        let mut tables = self.tables.lock().unwrap();
        tables.insert(table_id, table);

        table_id
    }

    /// Get a fd table by ID
    pub fn get_table(&self, table_id: u64) -> Option<FdTable> {
        let tables = self.tables.lock().unwrap();
        tables.get(&table_id).cloned()
    }

    /// Remove a fd table
    pub fn remove_table(&self, table_id: u64) -> Result<(), String> {
        let mut tables = self.tables.lock().unwrap();
        match tables.remove(&table_id) {
            Some(_) => Ok(()),
            None => Err(format!("Fd table {} not found", table_id)),
        }
    }

    /// Allocate a file descriptor
    pub fn allocate(&self, table_id: u64, file_type: String, flags: FdFlags) -> Result<i32, String> {
        let mut tables = self.tables.lock().unwrap();
        match tables.get_mut(&table_id) {
            Some(table) => Ok(table.allocate(file_type, flags)),
            None => Err(format!("Fd table {} not found", table_id)),
        }
    }

    /// Close a file descriptor
    pub fn close(&self, table_id: u64, fd: i32) -> Result<(), String> {
        let mut tables = self.tables.lock().unwrap();
        match tables.get_mut(&table_id) {
            Some(table) => table.close(fd),
            None => Err(format!("Fd table {} not found", table_id)),
        }
    }

    /// Duplicate a file descriptor
    pub fn dup(&self, table_id: u64, old_fd: i32) -> Result<i32, String> {
        let mut tables = self.tables.lock().unwrap();
        match tables.get_mut(&table_id) {
            Some(table) => table.dup(old_fd),
            None => Err(format!("Fd table {} not found", table_id)),
        }
    }

    /// Duplicate a file descriptor to a specific fd
    pub fn dup2(&self, table_id: u64, old_fd: i32, new_fd: i32) -> Result<i32, String> {
        let mut tables = self.tables.lock().unwrap();
        match tables.get_mut(&table_id) {
            Some(table) => table.dup2(old_fd, new_fd),
            None => Err(format!("Fd table {} not found", table_id)),
        }
    }

    /// Get table count
    pub fn table_count(&self) -> usize {
        let tables = self.tables.lock().unwrap();
        tables.len()
    }
}

impl Default for FdTableManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fd_flags() {
        let flags = FdFlags::new().with_close_on_exec(true);
        assert!(flags.close_on_exec);
    }

    #[test]
    fn test_fd_entry() {
        let flags = FdFlags::new();
        let entry = FdEntry::new(3, "file".to_string(), flags);

        assert_eq!(entry.get_fd(), 3);
        assert_eq!(entry.get_file_type(), "file");
        assert_eq!(entry.get_offset(), 0);
    }

    #[test]
    fn test_fd_entry_set_offset() {
        let flags = FdFlags::new();
        let mut entry = FdEntry::new(3, "file".to_string(), flags);

        entry.set_offset(100);
        assert_eq!(entry.get_offset(), 100);
    }

    #[test]
    fn test_fd_table() {
        let table = FdTable::new();
        assert_eq!(table.next_fd, 3);
        assert_eq!(table.fd_count(), 0);
    }

    #[test]
    fn test_fd_table_allocate() {
        let mut table = FdTable::new();

        let flags = FdFlags::new();
        let fd = table.allocate("file".to_string(), flags);

        assert_eq!(fd, 3);
        assert_eq!(table.fd_count(), 1);
    }

    #[test]
    fn test_fd_table_get() {
        let mut table = FdTable::new();

        let flags = FdFlags::new();
        let fd = table.allocate("file".to_string(), flags);

        let entry = table.get(fd);
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().get_fd(), fd);
    }

    #[test]
    fn test_fd_table_close() {
        let mut table = FdTable::new();

        let flags = FdFlags::new();
        let fd = table.allocate("file".to_string(), flags);

        table.close(fd).unwrap();
        assert_eq!(table.fd_count(), 0);
    }

    #[test]
    fn test_fd_table_dup() {
        let mut table = FdTable::new();

        let flags = FdFlags::new();
        let old_fd = table.allocate("file".to_string(), flags);

        let new_fd = table.dup(old_fd).unwrap();
        assert_eq!(new_fd, 4);
        assert_eq!(table.fd_count(), 2);
    }

    #[test]
    fn test_fd_table_dup2() {
        let mut table = FdTable::new();

        let flags = FdFlags::new();
        let old_fd = table.allocate("file".to_string(), flags);

        let new_fd = table.dup2(old_fd, 10).unwrap();
        assert_eq!(new_fd, 10);
        assert_eq!(table.fd_count(), 1);
    }

    #[test]
    fn test_fd_table_manager() {
        let manager = FdTableManager::new();

        let table_id = manager.create_table();
        assert_eq!(table_id, 1);
        assert_eq!(manager.table_count(), 1);
    }

    #[test]
    fn test_fd_table_manager_allocate() {
        let manager = FdTableManager::new();

        let table_id = manager.create_table();
        let flags = FdFlags::new();
        let fd = manager.allocate(table_id, "file".to_string(), flags).unwrap();

        assert_eq!(fd, 3);
    }

    #[test]
    fn test_fd_table_manager_close() {
        let manager = FdTableManager::new();

        let table_id = manager.create_table();
        let flags = FdFlags::new();
        let fd = manager.allocate(table_id, "file".to_string(), flags).unwrap();

        manager.close(table_id, fd).unwrap();
    }

    #[test]
    fn test_fd_table_manager_dup() {
        let manager = FdTableManager::new();

        let table_id = manager.create_table();
        let flags = FdFlags::new();
        let old_fd = manager.allocate(table_id, "file".to_string(), flags).unwrap();

        let new_fd = manager.dup(table_id, old_fd).unwrap();
        assert_eq!(new_fd, 4);
    }

    #[test]
    fn test_fd_table_manager_invalid() {
        let manager = FdTableManager::new();
        assert!(manager.allocate(999, "file".to_string(), FdFlags::new()).is_err());
        assert!(manager.close(999, 3).is_err());
    }
}
