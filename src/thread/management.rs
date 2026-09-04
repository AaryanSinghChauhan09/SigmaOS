/// OOP-based Thread Management for SigmaOS
/// Based on Roadmap Item 12: Thread management
/// Absorbing Linux interruptible/alertable state concepts, CPU affinity, and nice prioritization values

use std::boxed::Box;
use core::mem;
use core::sync::atomic::{AtomicI32, AtomicUsize, Ordering};

pub type ThreadID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready = 0,
    Running = 1,
    Blocked = 2,
    Terminated = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadAlertableState {
    NonAlertable = 0,
    Alertable = 1,
}

pub trait Thread {
    fn id(&self) -> ThreadID;
    fn state(&self) -> ThreadState;
    fn start(&mut self) -> Result<(), ThreadError>;
    fn stop(&mut self) -> Result<(), ThreadError>;
    fn alertable_state(&self) -> ThreadAlertableState;
    fn set_alertable_state(&mut self, state: ThreadAlertableState);
    fn nice(&self) -> i32;
    fn set_nice(&mut self, value: i32) -> Result<(), ThreadError>;
    fn cpu_affinity(&self) -> u64;
    fn set_cpu_affinity(&mut self, mask: u64);
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThreadError {
    Success = 0,
    StartFailed = 1,
    StopFailed = 2,
    InvalidNiceValue = 3,
}

#[repr(C)]
pub struct SimpleThread {
    pub id: ThreadID,
    pub state: AtomicUsize,
    pub alert_state: AtomicUsize,
    pub nice_val: AtomicI32,
    pub affinity: AtomicUsize,
}

impl SimpleThread {
    pub fn new(id: ThreadID) -> Self {
        SimpleThread {
            id,
            state: AtomicUsize::new(ThreadState::Ready as usize),
            alert_state: AtomicUsize::new(ThreadAlertableState::NonAlertable as usize),
            nice_val: AtomicI32::new(0), // Default Nice level = 0
            affinity: AtomicUsize::new(0xFFFFFFFF), // Default affinity = all CPUs
        }
    }
}

impl Thread for SimpleThread {
    fn id(&self) -> ThreadID {
        self.id
    }
    fn state(&self) -> ThreadState {
        match self.state.load(Ordering::SeqCst) {
            1 => ThreadState::Running,
            2 => ThreadState::Blocked,
            3 => ThreadState::Terminated,
            _ => ThreadState::Ready,
        }
    }

    fn start(&mut self) -> Result<(), ThreadError> {
        self.state
            .store(ThreadState::Running as usize, Ordering::SeqCst);
        Ok(())
    }

    fn stop(&mut self) -> Result<(), ThreadError> {
        self.state
            .store(ThreadState::Terminated as usize, Ordering::SeqCst);
        Ok(())
    }

    fn alertable_state(&self) -> ThreadAlertableState {
        match self.alert_state.load(Ordering::SeqCst) {
            1 => ThreadAlertableState::Alertable,
            _ => ThreadAlertableState::NonAlertable,
        }
    }

    fn set_alertable_state(&mut self, state: ThreadAlertableState) {
        self.alert_state.store(state as usize, Ordering::SeqCst);
    }

    fn nice(&self) -> i32 {
        self.nice_val.load(Ordering::SeqCst)
    }

    fn set_nice(&mut self, value: i32) -> Result<(), ThreadError> {
        // Linux Nice priority levels must reside strictly within -20 and 19
        if value < -20 || value > 19 {
            return Err(ThreadError::InvalidNiceValue);
        }
        self.nice_val.store(value, Ordering::SeqCst);
        Ok(())
    }

    fn cpu_affinity(&self) -> u64 {
        self.affinity.load(Ordering::SeqCst) as u64
    }

    fn set_cpu_affinity(&mut self, mask: u64) {
        self.affinity.store(mask as usize, Ordering::SeqCst);
    }
}

pub trait ThreadManager {
    fn create_thread(&mut self) -> Result<ThreadID, ThreadError>;
    fn destroy_thread(&mut self, id: ThreadID) -> Result<(), ThreadError>;
    fn get_thread(&self, id: ThreadID) -> Option<&dyn Thread>;
    fn get_thread_mut(&mut self, id: ThreadID) -> Option<&mut dyn Thread>;
}

pub struct SimpleThreadManager {
    threads: Vec<Option<Box<dyn Thread>>>,
    next_id: AtomicUsize,
}

impl SimpleThreadManager {
    pub fn new() -> Self {
        SimpleThreadManager {
            threads: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ThreadManager for SimpleThreadManager {
    fn create_thread(&mut self) -> Result<ThreadID, ThreadError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let thread = SimpleThread::new(id);
        self.threads.push(Some(Box::new(thread)));
        Ok(id)
    }

    fn destroy_thread(&mut self, id: ThreadID) -> Result<(), ThreadError> {
        for thread_option in self.threads.as_slice_mut() {
            if let Some(ref mut thread) = *thread_option {
                if thread.id() == id {
                    self.threads.clear();
                    return Ok(());
                }
            }
        }
        Err(ThreadError::StopFailed)
    }

    fn get_thread(&self, id: ThreadID) -> Option<&dyn Thread> {
        for thread_option in self.threads.as_slice() {
            if let Some(ref thread) = *thread_option {
                if thread.id() == id {
                    return Some(thread.as_ref());
                }
            }
        }
        None
    }

    fn get_thread_mut(&mut self, id: ThreadID) -> Option<&mut dyn Thread> {
        for thread_option in self.threads.as_slice_mut() {
            if let Some(ref mut thread) = *thread_option {
                if thread.id() == id {
                    return Some(thread.as_mut());
                }
            }
        }
        None
    }
}

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn clear(&mut self) {
        self.len = 0;
    }
    fn as_slice(&self) -> &[T] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
    fn as_slice_mut(&mut self) -> &mut [T] {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

#[cfg(not(test))]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
unsafe fn alloc(size: usize) -> *mut u8 {
    std::alloc(core::alloc::Layout::from_size_align_unchecked(size, 8))
}

#[cfg(test)]
unsafe fn free(_ptr: *mut u8) {
    // In standard shims, we can just let OS reclaim heap on test exit or perform simple dummy dealloc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_nice_alertable_and_affinity() {
        let mut thread = SimpleThread::new(101);
        assert_eq!(thread.id(), 101);
        assert_eq!(thread.state(), ThreadState::Ready);
        assert_eq!(thread.alertable_state(), ThreadAlertableState::NonAlertable);
        assert_eq!(thread.nice(), 0);
        assert_eq!(thread.cpu_affinity(), 0xFFFFFFFF);

        // Modify alertable state
        thread.set_alertable_state(ThreadAlertableState::Alertable);
        assert_eq!(thread.alertable_state(), ThreadAlertableState::Alertable);

        // Modify nice levels with boundary check
        assert!(thread.set_nice(-15).is_ok());
        assert_eq!(thread.nice(), -15);
        assert!(thread.set_nice(-30).is_err()); // invalid nice level
        assert!(thread.set_nice(20).is_err()); // invalid nice level

        // Modify CPU affinity mask
        thread.set_cpu_affinity(0b1101);
        assert_eq!(thread.cpu_affinity(), 0b1101);
    }

    #[test]
    fn test_thread_state_transitions() {
        let mut thread = SimpleThread::new(102);
        assert_eq!(thread.state(), ThreadState::Ready);

        thread.start().unwrap();
        assert_eq!(thread.state(), ThreadState::Running);

        thread.stop().unwrap();
        assert_eq!(thread.state(), ThreadState::Terminated);
    }
}
