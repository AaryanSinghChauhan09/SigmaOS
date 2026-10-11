#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rlimit {
    pub cur: u64,
    pub max: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RlimitError {
    LimitExceeded(&'static str),
}

pub struct ProcessResourceLimits {
    pub cpu_time_sec: Rlimit,
    pub max_memory_bytes: Rlimit,
    pub max_file_descriptors: Rlimit,
    pub max_processes: Rlimit,
}

impl ProcessResourceLimits {
    pub fn default_limits() -> Self {
        Self {
            cpu_time_sec: Rlimit { cur: 100, max: 1000 },
            max_memory_bytes: Rlimit { cur: 1024 * 1024 * 1024, max: 2 * 1024 * 1024 * 1024 },
            max_file_descriptors: Rlimit { cur: 1024, max: 4096 },
            max_processes: Rlimit { cur: 256, max: 1024 },
        }
    }

    pub fn check_fd_limit(&self, current_open_fds: u64) -> Result<(), RlimitError> {
        if current_open_fds >= self.max_file_descriptors.cur {
            return Err(RlimitError::LimitExceeded("EMFILE: Too many open files"));
        }
        Ok(())
    }

    pub fn check_process_limit(&self, current_procs: u64) -> Result<(), RlimitError> {
        if current_procs >= self.max_processes.cur {
            return Err(RlimitError::LimitExceeded("EAGAIN: Resource temporarily unavailable"));
        }
        Ok(())
    }
}

#[test]
fn test_resource_limit_enforcement() {
    let limits = ProcessResourceLimits::default_limits();

    assert!(limits.check_fd_limit(500).is_ok());
    assert_eq!(
        limits.check_fd_limit(1024),
        Err(RlimitError::LimitExceeded("EMFILE: Too many open files"))
    );

    assert!(limits.check_process_limit(100).is_ok());
    assert_eq!(
        limits.check_process_limit(256),
        Err(RlimitError::LimitExceeded("EAGAIN: Resource temporarily unavailable"))
    );
}
