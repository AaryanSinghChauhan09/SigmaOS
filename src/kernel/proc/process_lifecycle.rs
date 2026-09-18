// SigmaOS Advanced Process Control & Lifecycle Management
// Absorbs Linux fork/exec/exit/waitpid, copy-on-write namespaces, BSD rlimits, Windows Priority Classes, and Orphan Re-parenting.


#[cfg(test)]


use std::string::String;
use std::string::ToString;
use std::format;
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::time::Duration;

use crate::klib::HashMap;

#[cfg(test)]
pub(crate) mod mock_scheduler {
    use core::time::Duration;
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Priority {
        Idle = 0,
        Low = 1,
        Normal = 2,
        High = 3,
        Realtime = 4,
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ProcessState {
        Running,
        Ready,
        Blocked,
        Terminated,
    }
    #[derive(Debug, Clone)]
    pub struct Process {
        pub pid: u64,
        pub name: String,
        pub priority: Priority,
        pub state: ProcessState,
        pub runtime: Duration,
        pub time_slice: Duration,
    }
    impl Process {
        pub fn new(pid: u64, name: String, priority: Priority) -> Self {
            Self {
                pid,
                name,
                priority,
                state: ProcessState::Ready,
                runtime: Duration::from_secs(0),
                time_slice: Duration::from_millis(10),
            }
        }
    }
}

#[cfg(test)]
pub use mock_scheduler::{Priority, Process, ProcessState};

#[cfg(not(test))]
pub use crate::kernel::scheduler::{Priority, Process, ProcessState};

/// Windows-style Process Creation Priority Classes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
