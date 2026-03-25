#![no_std]
#![feature(alloc_error_handler)]

//! SigmaOS Native Concurrency Engine (SMP & Parallel Processing)
//! =============================================================
//! Purpose: Handles multiple CPU cores using atomic locks, lock-free queues,
//! and spinlocks without any POSIX threading or external abstractions.
//! OOPS, Encapsulation, Concurrency, and Synchronization completely native.

use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

const MAX_CORES: usize = 256;

// OOP Encapsulation of Lock mechanics
pub struct SigmaSpinlock {
    locked: AtomicBool,
}

impl SigmaSpinlock {
    pub const fn new() -> Self {
        SigmaSpinlock {
            locked: AtomicBool::new(false),
        }
    }

    pub fn acquire(&self) {
        // Test-and-Set loop for concurrency synchronization
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // Spin loop backoff to reduce cache line bouncing (pause insn)
            unsafe { core::arch::asm!("pause"); }
        }
    }

    pub fn release(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

// Global Work Queue (Lock-free concept abstracted via Custom Types)
pub struct SigmaParallelJobQueue {
    head: AtomicU32,
    tail: AtomicU32,
    lock: SigmaSpinlock,
    // Using a primitive flat array queue (no heap alloc to prevent fragmentation)
    jobs: [fn(); 1024],
}

impl SigmaParallelJobQueue {
    pub const fn new() -> Self {
        SigmaParallelJobQueue {
            head: AtomicU32::new(0),
            tail: AtomicU32::new(0),
            lock: SigmaSpinlock::new(),
            // Empty job array initially points to no-op
            jobs: [Self::noop; 1024],
        }
    }

    fn noop() {
        // Do nothing empty task
    }

    pub fn enqueue_job(&mut self, job: fn()) -> bool {
        self.lock.acquire();
        let current_tail = self.tail.load(Ordering::Relaxed) as usize;
        let next_tail = (current_tail + 1) % 1024;
        
        let current_head = self.head.load(Ordering::Relaxed) as usize;
        if next_tail == current_head {
            self.lock.release();
            return false; // Queue Full
        }

        self.jobs[current_tail] = job;
        self.tail.store(next_tail as u32, Ordering::Relaxed);
        self.lock.release();
        true
    }

    pub fn dispatch_worker(&self) {
        loop {
            self.lock.acquire();
            let mut current_head = self.head.load(Ordering::Relaxed) as usize;
            let current_tail = self.tail.load(Ordering::Relaxed) as usize;

            if current_head != current_tail {
                let job = self.jobs[current_head];
                self.head.store(((current_head + 1) % 1024) as u32, Ordering::Relaxed);
                self.lock.release();

                // Execute fetched job natively (polymorphism at task level)
                job();
            } else {
                self.lock.release();
                // Hlt until next interrupt (I/O Management)
                unsafe { core::arch::asm!("hlt"); }
            }
        }
    }
}

/// APIC (Advanced Programmable Interrupt Controller) Timer Interface
pub struct SigmaLAPIC;

impl SigmaLAPIC {
    pub fn broadcast_ipi(vector: u8, dest_cpu: u32) {
        unsafe {
            let icr_low = 0xFEE0_0300 as *mut u32;
            let icr_high = 0xFEE0_0310 as *mut u32;

            core::ptr::write_volatile(icr_high, dest_cpu << 24);
            core::ptr::write_volatile(icr_low, (vector as u32) | 0x4000); // Send simple IPI
        }
    }
}
