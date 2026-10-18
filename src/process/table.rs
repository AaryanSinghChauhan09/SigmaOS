//! Bounded process table implementation supporting concurrent access and orphan adoption.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use super::pid::{Pid, PidAllocator, PidError};
use super::state::{ProcessLifecycleTracker, ProcessState, ProcessStateError};

#[derive(Debug)]
pub struct ProcessEntry {
    pub pid: Pid,
    pub ppid: AtomicU32,
    pub name: String,
    pub tracker: Mutex<ProcessLifecycleTracker>,
    pub reaped: AtomicBool,
}

impl ProcessEntry {
    pub fn new(pid: Pid, ppid: Pid, name: &str) -> Self {
        Self {
            pid,
            ppid: AtomicU32::new(ppid.0),
            name: String::from(name),
            tracker: Mutex::new(ProcessLifecycleTracker::new()),
            reaped: AtomicBool::new(false),
        }
    }

    pub fn parent_pid(&self) -> Pid {
        Pid(self.ppid.load(Ordering::SeqCst))
    }

    pub fn set_parent_pid(&self, ppid: Pid) {
        self.ppid.store(ppid.0, Ordering::SeqCst);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessTableError {
    PidError(PidError),
    StateError(ProcessStateError),
    TableFull,
    ProcessNotFound(u32),
    AlreadyReaped(u32),
    NotAZombie(u32),
}

impl From<PidError> for ProcessTableError {
    fn from(e: PidError) -> Self {
        ProcessTableError::PidError(e)
    }
}

impl From<ProcessStateError> for ProcessTableError {
    fn from(e: ProcessStateError) -> Self {
        ProcessTableError::StateError(e)
    }
}

pub struct ProcessTable {
    max_entries: usize,
    pid_allocator: Mutex<PidAllocator>,
    entries: Mutex<BTreeMap<u32, Arc<ProcessEntry>>>,
    orphan_reaper_pid: AtomicU32,
}

impl ProcessTable {
    pub fn new(max_entries: usize, min_pid: u32, max_pid: u32) -> Self {
        Self {
            max_entries,
            pid_allocator: Mutex::new(PidAllocator::new(min_pid, max_pid)),
            entries: Mutex::new(BTreeMap::new()),
            orphan_reaper_pid: AtomicU32::new(1), // Default init PID = 1
        }
    }

    pub fn set_orphan_reaper(&self, pid: Pid) {
        self.orphan_reaper_pid.store(pid.0, Ordering::SeqCst);
    }

    pub fn orphan_reaper_pid(&self) -> Pid {
        Pid(self.orphan_reaper_pid.load(Ordering::SeqCst))
    }

    pub fn create_process(&self, ppid: Pid, name: &str) -> Result<Arc<ProcessEntry>, ProcessTableError> {
        let mut entries = self.entries.lock().unwrap();
        if entries.len() >= self.max_entries {
            return Err(ProcessTableError::TableFull);
        }

        let mut allocator = self.pid_allocator.lock().unwrap();
        let pid = allocator.allocate()?;

        let entry = Arc::new(ProcessEntry::new(pid, ppid, name));
        entry.tracker.lock().unwrap().transition_to(ProcessState::Ready)?;

        entries.insert(pid.0, entry.clone());
        Ok(entry)
    }

    pub fn get_process(&self, pid: Pid) -> Option<Arc<ProcessEntry>> {
        let entries = self.entries.lock().unwrap();
        entries.get(&pid.0).cloned()
    }

    pub fn handle_parent_exit(&self, exiting_ppid: Pid) -> usize {
        let reaper_pid = self.orphan_reaper_pid();
        let entries = self.entries.lock().unwrap();
        let mut adopted_count = 0;

        for entry in entries.values() {
            if entry.parent_pid() == exiting_ppid {
                entry.set_parent_pid(reaper_pid);
                adopted_count += 1;
            }
        }
        adopted_count
    }

    pub fn reap_process(&self, pid: Pid) -> Result<i32, ProcessTableError> {
        let entry = {
            let entries = self.entries.lock().unwrap();
            entries
                .get(&pid.0)
                .cloned()
                .ok_or(ProcessTableError::ProcessNotFound(pid.0))?
        };

        if entry.reaped.swap(true, Ordering::SeqCst) {
            return Err(ProcessTableError::AlreadyReaped(pid.0));
        }

        let exit_code = {
            let mut tracker = entry.tracker.lock().unwrap();
            tracker.reap()?
        };

        // Remove entry from table and free PID
        {
            let mut entries = self.entries.lock().unwrap();
            entries.remove(&pid.0);
        }

        {
            let mut allocator = self.pid_allocator.lock().unwrap();
            let _ = allocator.free(pid);
        }

        Ok(exit_code)
    }

    pub fn len(&self) -> usize {
        self.entries.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.lock().unwrap().is_empty()
    }
}
