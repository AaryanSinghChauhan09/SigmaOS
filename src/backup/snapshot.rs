#![no_std]
#![no_main]

/// OOP-based Backup and Snapshot for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 161
/// Implements system snapshots and backup management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SnapshotID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SnapshotType { Full = 0, Incremental = 1, Differential = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BackupError { Success = 0, NotFound = 1, CreationFailed = 2, RestoreFailed = 3 }

pub trait Snapshot {
    fn id(&self) -> SnapshotID;
    fn snapshot_type(&self) -> SnapshotType;
    fn timestamp(&self) -> u64;
    fn size(&self) -> usize;
    fn is_valid(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSnapshot {
    pub id: SnapshotID,
    pub snapshot_type: AtomicUsize,
    pub timestamp: AtomicUsize,
    pub size: AtomicUsize,
    pub valid: AtomicUsize,
}

impl SimpleSnapshot {
    pub fn new(id: SnapshotID, snapshot_type: SnapshotType, size: usize) -> Self {
        SimpleSnapshot {
            id,
            snapshot_type: AtomicUsize::new(snapshot_type as usize),
            timestamp: AtomicUsize::new(1000000),
            size: AtomicUsize::new(size),
            valid: AtomicUsize::new(1),
        }
    }
}

impl Snapshot for SimpleSnapshot {
    fn id(&self) -> SnapshotID { self.id }
    fn snapshot_type(&self) -> SnapshotType { unsafe { core::mem::transmute(self.snapshot_type.load(Ordering::SeqCst)) } }
    fn timestamp(&self) -> u64 { self.timestamp.load(Ordering::SeqCst) as u64 }
    fn size(&self) -> usize { self.size.load(Ordering::SeqCst) }
    fn is_valid(&self) -> bool { self.valid.load(Ordering::SeqCst) == 1 }
}

pub trait BackupManager {
    fn create_snapshot(&mut self, snapshot_type: SnapshotType) -> Result<SnapshotID, BackupError>;
    fn delete_snapshot(&mut self, id: SnapshotID) -> Result<(), BackupError>;
    fn get_snapshot(&self, id: SnapshotID) -> Option<&dyn Snapshot>;
    fn restore_snapshot(&mut self, id: SnapshotID) -> Result<(), BackupError>;
    fn list_snapshots(&self) -> Vec<SnapshotID>;
}

#[repr(C)]
pub struct SimpleBackupManager {
    pub snapshots: Vec<Option<Box<dyn Snapshot>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBackupManager {
    pub fn new() -> Self {
        SimpleBackupManager {
            snapshots: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BackupManager for SimpleBackupManager {
    fn create_snapshot(&mut self, snapshot_type: SnapshotType) -> Result<SnapshotID, BackupError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let snapshot = SimpleSnapshot::new(id, snapshot_type, 1024 * 1024);
        self.snapshots.push(Some(Box::new(snapshot)));
        Ok(id)
    }
    
    fn delete_snapshot(&mut self, id: SnapshotID) -> Result<(), BackupError> {
        for snapshot_option in &mut self.snapshots {
            if let Some(ref snapshot) = *snapshot_option {
                if snapshot.id() == id {
                    return Ok(());
                }
            }
        }
        Err(BackupError::NotFound)
    }
    
    fn get_snapshot(&self, id: SnapshotID) -> Option<&dyn Snapshot> {
        for snapshot_option in &self.snapshots {
            if let Some(ref snapshot) = *snapshot_option {
                if snapshot.id() == id { return Some(snapshot.as_ref()); }
            }
        }
        None
    }
    
    fn restore_snapshot(&mut self, id: SnapshotID) -> Result<(), BackupError> {
        if self.get_snapshot(id).is_some() {
            Ok(())
        } else {
            Err(BackupError::NotFound)
        }
    }
    
    fn list_snapshots(&self) -> Vec<SnapshotID> {
        let mut ids = Vec::new();
        for snapshot_option in &self.snapshots {
            if let Some(ref snapshot) = *snapshot_option {
                ids.push(snapshot.id());
            }
        }
        ids
    }
}

pub trait BackupScheduler {
    fn schedule_backup(&mut self, interval_ms: u64) -> Result<usize, BackupError>;
    fn cancel_backup(&mut self, schedule_id: usize) -> Result<(), BackupError>;
    fn run_scheduled_backups(&mut self) -> Vec<SnapshotID>;
}

#[repr(C)]
pub struct SimpleBackupScheduler {
    pub schedules: Vec<(usize, u64, SnapshotType)>,
    pub next_id: AtomicUsize,
}

impl SimpleBackupScheduler {
    pub fn new() -> Self {
        SimpleBackupScheduler {
            schedules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BackupScheduler for SimpleBackupScheduler {
    fn schedule_backup(&mut self, interval_ms: u64) -> Result<usize, BackupError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.schedules.push((id, interval_ms, SnapshotType::Incremental));
        Ok(id)
    }
    
    fn cancel_backup(&mut self, schedule_id: usize) -> Result<(), BackupError> {
        for i in 0..self.schedules.len() {
            if self.schedules[i].0 == schedule_id {
                self.schedules.remove(i);
                return Ok(());
            }
        }
        Err(BackupError::NotFound)
    }
    
    fn run_scheduled_backups(&mut self) -> Vec<SnapshotID> {
        let mut created = Vec::new();
        for &(id, _, snapshot_type) in &self.schedules {
            let snapshot_id = id + 1000;
            created.push(snapshot_id);
        }
        created
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

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
