#![no_std]
#![no_main]

/// Custom Threading Primitives for SigmaOS
/// Implements threading without relying on std::thread
/// Uses capability-based access control

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::mem;

/// Thread ID
pub type ThreadID = usize;

/// Thread state
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThreadState {
    Uninitialized = 0,
    Ready = 1,
    Running = 2,
    Blocked = 3,
    Terminated = 4,
}

/// Thread priority
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ThreadPriority {
    Idle = 0,
    Low = 1,
    Normal = 2,
    High = 3,
    Realtime = 4,
}

/// Thread stack
#[repr(C)]
pub struct ThreadStack {
    base: *mut u8,
    size: usize,
    top: *mut u8,
}

impl ThreadStack {
    pub unsafe fn new(size: usize) -> Option<Self> {
        let base = alloc(size);
        if base.is_null() {
            return None;
        }

        let top = base.add(size);
        
        Some(ThreadStack {
            base,
            size,
            top,
        })
    }

    pub unsafe fn push(&mut self, value: usize) {
        self.top = self.top.sub(mem::size_of::<usize>());
        *(self.top as *mut usize) = value;
    }

    pub unsafe fn pop(&mut self) -> usize {
        let value = *(self.top as *const usize);
        self.top = self.top.add(mem::size_of::<usize>());
        value
    }
}

impl Drop for ThreadStack {
    fn drop(&mut self) {
        unsafe {
            if !self.base.is_null() {
                free(self.base);
            }
        }
    }
}

/// Thread context (registers)
#[repr(C)]
pub struct ThreadContext {
    pub rbx: usize,
    pub rsp: usize,
    pub rbp: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub rip: usize,
}

impl ThreadContext {
    pub fn new() -> Self {
        ThreadContext {
            rbx: 0,
            rsp: 0,
            rbp: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rip: 0,
        }
    }
}

/// Thread control block
#[repr(C)]
pub struct Thread {
    pub id: ThreadID,
    pub state: AtomicUsize, // ThreadState as usize
    pub priority: ThreadPriority,
    pub stack: Option<ThreadStack>,
    pub context: ThreadContext,
    pub capability: ThreadCapability,
    pub exit_code: AtomicUsize,
}

/// Thread capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ThreadCapability {
    pub can_create: bool,
    pub can_terminate: bool,
    pub can_suspend: bool,
    pub can_resume: bool,
    pub can_set_priority: bool,
}

impl ThreadCapability {
    pub fn new() -> Self {
        ThreadCapability {
            can_create: false,
            can_terminate: false,
            can_suspend: false,
            can_resume: false,
            can_set_priority: false,
        }
    }

    pub fn full() -> Self {
        ThreadCapability {
            can_create: true,
            can_terminate: true,
            can_suspend: true,
            can_resume: true,
            can_set_priority: true,
        }
    }
}

impl Thread {
    pub unsafe fn new(id: ThreadID, entry: extern "C" fn(), stack_size: usize, capability: ThreadCapability) -> Option<Self> {
        let stack = ThreadStack::new(stack_size)?;
        
        // Set up initial stack context
        let mut context = ThreadContext::new();
        context.rsp = stack.top as usize;
        context.rip = entry as usize;

        Some(Thread {
            id,
            state: AtomicUsize::new(ThreadState::Ready as usize),
            priority: ThreadPriority::Normal,
            stack: Some(stack),
            context,
            capability,
            exit_code: AtomicUsize::new(0),
        })
    }

    pub fn get_state(&self) -> ThreadState {
        unsafe {
            core::mem::transmute(self.state.load(Ordering::SeqCst))
        }
    }

    pub fn set_state(&self, state: ThreadState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    pub fn set_exit_code(&self, code: usize) {
        self.exit_code.store(code, Ordering::SeqCst);
    }

    pub fn get_exit_code(&self) -> usize {
        self.exit_code.load(Ordering::SeqCst)
    }
}

/// Mutex (Mutual Exclusion)
#[repr(C)]
pub struct Mutex {
    locked: AtomicBool,
    owner: AtomicUsize,
    wait_queue: *mut Thread,
    capability: MutexCapability,
}

/// Mutex capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MutexCapability {
    pub can_lock: bool,
    pub can_unlock: bool,
}

impl MutexCapability {
    pub fn new() -> Self {
        MutexCapability {
            can_lock: false,
            can_unlock: false,
        }
    }

    pub fn full() -> Self {
        MutexCapability {
            can_lock: true,
            can_unlock: true,
        }
    }
}

impl Mutex {
    pub unsafe fn new(capability: MutexCapability) -> Self {
        Mutex {
            locked: AtomicBool::new(false),
            owner: AtomicUsize::new(0),
            wait_queue: ptr::null_mut(),
            capability,
        }
    }

    pub unsafe fn lock(&self) -> bool {
        if !self.capability.can_lock {
            return false;
        }

        // Try to acquire lock
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // In a real implementation, this would yield to scheduler
            // For now, spin
            core::hint::spin_loop();
        }

        self.owner.store(get_current_thread_id(), Ordering::SeqCst);
        true
    }

    pub unsafe fn unlock(&self) -> bool {
        if !self.capability.can_unlock {
            return false;
        }

        if self.owner.load(Ordering::SeqCst) != get_current_thread_id() {
            return false;
        }

        self.locked.store(false, Ordering::Release);
        self.owner.store(0, Ordering::SeqCst);
        true
    }

    pub unsafe fn try_lock(&self) -> bool {
        if !self.capability.can_lock {
            return false;
        }

        if self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_ok() {
            self.owner.store(get_current_thread_id(), Ordering::SeqCst);
            true
        } else {
            false
        }
    }

    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::SeqCst)
    }
}

/// Semaphore
#[repr(C)]
pub struct Semaphore {
    count: AtomicUsize,
    max_count: usize,
    wait_queue: *mut Thread,
    capability: SemaphoreCapability,
}

/// Semaphore capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SemaphoreCapability {
    pub can_wait: bool,
    pub can_signal: bool,
}

impl SemaphoreCapability {
    pub fn new() -> Self {
        SemaphoreCapability {
            can_wait: false,
            can_signal: false,
        }
    }

    pub fn full() -> Self {
        SemaphoreCapability {
            can_wait: true,
            can_signal: true,
        }
    }
}

impl Semaphore {
    pub unsafe fn new(initial_count: usize, max_count: usize, capability: SemaphoreCapability) -> Self {
        Semaphore {
            count: AtomicUsize::new(initial_count),
            max_count,
            wait_queue: ptr::null_mut(),
            capability,
        }
    }

    pub unsafe fn wait(&self) -> bool {
        if !self.capability.can_wait {
            return false;
        }

        while self.count.load(Ordering::Acquire) == 0 {
            // In a real implementation, this would yield to scheduler
            // For now, spin
            core::hint::spin_loop();
        }

        self.count.fetch_sub(1, Ordering::AcqRel);
        true
    }

    pub unsafe fn signal(&self) -> bool {
        if !self.capability.can_signal {
            return false;
        }

        if self.count.load(Ordering::Acquire) < self.max_count {
            self.count.fetch_add(1, Ordering::AcqRel);
            true
        } else {
            false
        }
    }

    pub unsafe fn try_wait(&self) -> bool {
        if !self.capability.can_wait {
            return false;
        }

        if self.count.load(Ordering::Acquire) > 0 {
            self.count.fetch_sub(1, Ordering::AcqRel);
            true
        } else {
            false
        }
    }

    pub fn get_count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

/// Read-Write Lock
#[repr(C)]
pub struct RwLock {
    readers: AtomicUsize,
    writer: AtomicBool,
    write_wait_queue: *mut Thread,
    read_wait_queue: *mut Thread,
    capability: RwLockCapability,
}

/// Read-write lock capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RwLockCapability {
    pub can_read_lock: bool,
    pub can_write_lock: bool,
    pub can_unlock: bool,
}

impl RwLockCapability {
    pub fn new() -> Self {
        RwLockCapability {
            can_read_lock: false,
            can_write_lock: false,
            can_unlock: false,
        }
    }

    pub fn full() -> Self {
        RwLockCapability {
            can_read_lock: true,
            can_write_lock: true,
            can_unlock: true,
        }
    }
}

impl RwLock {
    pub unsafe fn new(capability: RwLockCapability) -> Self {
        RwLock {
            readers: AtomicUsize::new(0),
            writer: AtomicBool::new(false),
            write_wait_queue: ptr::null_mut(),
            read_wait_queue: ptr::null_mut(),
            capability,
        }
    }

    pub unsafe fn read_lock(&self) -> bool {
        if !self.capability.can_read_lock {
            return false;
        }

        while self.writer.load(Ordering::Acquire) {
            // In a real implementation, this would yield to scheduler
            core::hint::spin_loop();
        }

        self.readers.fetch_add(1, Ordering::AcqRel);
        true
    }

    pub unsafe fn write_lock(&self) -> bool {
        if !self.capability.can_write_lock {
            return false;
        }

        // Acquire writer lock
        while self.writer.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }

        // Wait for all readers to finish
        while self.readers.load(Ordering::Acquire) > 0 {
            core::hint::spin_loop();
        }

        true
    }

    pub unsafe fn unlock(&self) -> bool {
        if !self.capability.can_unlock {
            return false;
        }

        if self.writer.load(Ordering::SeqCst) {
            self.writer.store(false, Ordering::Release);
        } else {
            self.readers.fetch_sub(1, Ordering::AcqRel);
        }

        true
    }

    pub fn is_write_locked(&self) -> bool {
        self.writer.load(Ordering::SeqCst)
    }

    pub fn reader_count(&self) -> usize {
        self.readers.load(Ordering::SeqCst)
    }
}

/// Thread manager
pub struct ThreadManager {
    threads: [Option<NonNull<Thread>>; 256],
    next_thread_id: AtomicUsize,
    current_thread: AtomicUsize,
}

impl ThreadManager {
    pub fn new() -> Self {
        ThreadManager {
            threads: [None; 256],
            next_thread_id: AtomicUsize::new(1),
            current_thread: AtomicUsize::new(0),
        }
    }

    pub unsafe fn create_thread(&mut self, entry: extern "C" fn(), stack_size: usize, capability: ThreadCapability) -> Option<ThreadID> {
        let id = self.next_thread_id.fetch_add(1, Ordering::SeqCst);
        if id >= 256 {
            return None;
        }

        let thread = Thread::new(id, entry, stack_size, capability)?;
        let thread_ptr = alloc(mem::size_of::<Thread>()) as *mut Thread;
        if thread_ptr.is_null() {
            return None;
        }

        ptr::write(thread_ptr, thread);
        self.threads[id] = Some(NonNull::new_unchecked(thread_ptr));

        Some(id)
    }

    pub unsafe fn get_thread(&self, id: ThreadID) -> Option<&Thread> {
        if id < 256 {
            self.threads[id].map(|ptr| unsafe { &*ptr.as_ptr() })
        } else {
            None
        }
    }

    pub unsafe fn terminate_thread(&mut self, id: ThreadID) -> bool {
        if id >= 256 {
            return false;
        }

        if let Some(thread_ptr) = self.threads[id] {
            let thread = &*thread_ptr.as_ptr();
            if !thread.capability.can_terminate {
                return false;
            }

            thread.set_state(ThreadState::Terminated);
            true
        } else {
            false
        }
    }

    pub unsafe fn set_thread_priority(&mut self, id: ThreadID, priority: ThreadPriority) -> bool {
        if id >= 256 {
            return false;
        }

        if let Some(thread_ptr) = self.threads[id] {
            let thread = &mut *thread_ptr.as_ptr();
            if !thread.capability.can_set_priority {
                return false;
            }

            thread.priority = priority;
            true
        } else {
            false
        }
    }

    pub fn get_current_thread(&self) -> ThreadID {
        self.current_thread.load(Ordering::SeqCst)
    }

    pub unsafe fn set_current_thread(&self, id: ThreadID) {
        self.current_thread.store(id, Ordering::SeqCst);
    }
}

/// Global thread manager
static mut GLOBAL_THREAD_MANAGER: Option<ThreadManager> = None;

/// Initialize thread manager
pub unsafe fn init_thread_manager() {
    GLOBAL_THREAD_MANAGER = Some(ThreadManager::new());
}

/// Get current thread ID
pub unsafe fn get_current_thread_id() -> ThreadID {
    if let Some(ref manager) = GLOBAL_THREAD_MANAGER {
        manager.get_current_thread()
    } else {
        0
    }
}

/// Create thread
pub unsafe fn create_thread(entry: extern "C" fn(), stack_size: usize, capability: ThreadCapability) -> Option<ThreadID> {
    if let Some(ref mut manager) = GLOBAL_THREAD_MANAGER {
        manager.create_thread(entry, stack_size, capability)
    } else {
        None
    }
}

/// Terminate thread
pub unsafe fn terminate_thread(id: ThreadID) -> bool {
    if let Some(ref mut manager) = GLOBAL_THREAD_MANAGER {
        manager.terminate_thread(id)
    } else {
        false
    }
}

/// Set thread priority
pub unsafe fn set_thread_priority(id: ThreadID, priority: ThreadPriority) -> bool {
    if let Some(ref mut manager) = GLOBAL_THREAD_MANAGER {
        manager.set_thread_priority(id, priority)
    } else {
        false
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
