#![no_std]
#![no_main]

/// OOP-based Thread Management for SigmaOS
/// Based on Roadmap Item 12: Thread management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ThreadID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThreadState { Ready = 0, Running = 1, Blocked = 2, Terminated = 3 }

pub trait Thread {
    fn id(&self) -> ThreadID;
    fn state(&self) -> ThreadState;
    fn start(&mut self) -> Result<(), ThreadError>;
    fn stop(&mut self) -> Result<(), ThreadError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThreadError { Success = 0, StartFailed = 1, StopFailed = 2 }

#[repr(C)]
pub struct SimpleThread {
    pub id: ThreadID,
    pub state: AtomicUsize,
}

impl SimpleThread {
    pub fn new(id: ThreadID) -> Self {
        SimpleThread { id, state: AtomicUsize::new(ThreadState::Ready as usize) }
    }
}

impl Thread for SimpleThread {
    fn id(&self) -> ThreadID { self.id }
    fn state(&self) -> ThreadState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn start(&mut self) -> Result<(), ThreadError> {
        self.state.store(ThreadState::Running as usize, Ordering::SeqCst);
        Ok(())
    }
    fn stop(&mut self) -> Result<(), ThreadError> {
        self.state.store(ThreadState::Terminated as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait ThreadManager {
    fn create_thread(&mut self) -> Result<ThreadID, ThreadError>;
    fn destroy_thread(&mut self, id: ThreadID) -> Result<(), ThreadError>;
    fn get_thread(&self, id: ThreadID) -> Option<&dyn Thread>;
}

pub struct SimpleThreadManager {
    threads: Vec<Option<Box<dyn Thread>>>,
    next_id: AtomicUsize,
}

impl SimpleThreadManager {
    pub fn new() -> Self { SimpleThreadManager { threads: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl ThreadManager for SimpleThreadManager {
    fn create_thread(&mut self) -> Result<ThreadID, ThreadError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let thread = SimpleThread::new(id);
        self.threads.push(Some(Box::new(thread)));
        Ok(id)
    }
    fn destroy_thread(&mut self, id: ThreadID) -> Result<(), ThreadError> {
        for thread_option in &mut self.threads {
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
        for thread_option in &self.threads {
            if let Some(ref thread) = *thread_option {
                if thread.id() == id { return Some(thread.as_ref()); }
            }
        }
        None
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
    fn clear(&mut self) { self.len = 0; }
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
