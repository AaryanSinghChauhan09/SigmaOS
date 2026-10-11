//! Race-free PID allocator implementation with capacity limits and typed error handling.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pid(pub u32);

impl Pid {
    pub const INIT: Pid = Pid(1);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PidError {
    PidExhausted,
    InvalidPid(u32),
    PidAlreadyAllocated(u32),
    PidNotFound(u32),
}

pub struct PidAllocator {
    min_pid: u32,
    max_pid: u32,
    next_pid: u32,
    allocated: Vec<bool>,
    allocated_count: usize,
}

impl PidAllocator {
    pub fn new(min_pid: u32, max_pid: u32) -> Self {
        let capacity = (max_pid - min_pid + 1) as usize;
        Self {
            min_pid,
            max_pid,
            next_pid: min_pid,
            allocated: vec![false; capacity],
            allocated_count: 0,
        }
    }

    pub fn allocate(&mut self) -> Result<Pid, PidError> {
        let capacity = (self.max_pid - self.min_pid + 1) as usize;
        if self.allocated_count >= capacity {
            return Err(PidError::PidExhausted);
        }

        let start = self.next_pid;
        loop {
            let pid_candidate = self.next_pid;
            let idx = (pid_candidate - self.min_pid) as usize;

            self.next_pid += 1;
            if self.next_pid > self.max_pid {
                self.next_pid = self.min_pid;
            }

            if !self.allocated[idx] {
                self.allocated[idx] = true;
                self.allocated_count += 1;
                return Ok(Pid(pid_candidate));
            }

            if self.next_pid == start {
                return Err(PidError::PidExhausted);
            }
        }
    }

    pub fn free(&mut self, pid: Pid) -> Result<(), PidError> {
        if pid.0 < self.min_pid || pid.0 > self.max_pid {
            return Err(PidError::InvalidPid(pid.0));
        }

        let idx = (pid.0 - self.min_pid) as usize;
        if !self.allocated[idx] {
            return Err(PidError::PidNotFound(pid.0));
        }

        self.allocated[idx] = false;
        self.allocated_count -= 1;
        Ok(())
    }

    pub fn is_allocated(&self, pid: Pid) -> bool {
        if pid.0 < self.min_pid || pid.0 > self.max_pid {
            return false;
        }
        let idx = (pid.0 - self.min_pid) as usize;
        self.allocated[idx]
    }

    pub fn allocated_count(&self) -> usize {
        self.allocated_count
    }

    pub fn capacity(&self) -> usize {
        (self.max_pid - self.min_pid + 1) as usize
    }
}
