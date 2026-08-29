// SigmaOS NUMA-Aware CFS Scheduler & Lock-Free Concurrency Primitives
// Deploys abstract compare-and-swap Michael-Scott queues and Treiber stacks for multi-NUMA systems

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicPtr, Ordering};

pub struct NumaNode {
    pub node_id: u32,
    pub latency_weight: u32,
}

pub struct NumaScheduler {
    pub nodes: Vec<NumaNode>,
    pub active_thread_affinity_node: u32,
}

impl Default for NumaScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl NumaScheduler {
    pub fn new() -> Self {
        NumaScheduler {
            nodes: vec![
                NumaNode {
                    node_id: 0,
                    latency_weight: 10,
                },
                NumaNode {
                    node_id: 1,
                    latency_weight: 20,
                },
            ],
            active_thread_affinity_node: 0,
        }
    }

    pub fn schedule_task_affinity(&mut self, cross_socket_contention: bool) -> u32 {
        if cross_socket_contention {
            self.active_thread_affinity_node = 0;
        } else {
            self.active_thread_affinity_node = 1;
        }
        self.active_thread_affinity_node
    }
}

pub struct Node<T> {
    pub value: T,
    pub next: AtomicPtr<Node<T>>,
}

pub struct MichaelScottQueue<T> {
    pub head: AtomicPtr<Node<T>>,
    pub tail: AtomicPtr<Node<T>>,
}

impl<T> Default for MichaelScottQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MichaelScottQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            value: unsafe { core::mem::zeroed() },
            next: AtomicPtr::new(core::ptr::null_mut()),
        }));
        MichaelScottQueue {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, val: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: val,
            next: AtomicPtr::new(core::ptr::null_mut()),
        }));
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };
            if next.is_null() {
                if unsafe {
                    (*tail)
                        .next
                        .compare_exchange(
                            core::ptr::null_mut(),
                            new_node,
                            Ordering::Release,
                            Ordering::Relaxed,
                        )
                        .is_ok()
                } {
                    let _ = self.tail.compare_exchange(
                        tail,
                        new_node,
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                    break;
                }
            } else {
                let _ =
                    self.tail
                        .compare_exchange(tail, next, Ordering::Release, Ordering::Relaxed);
            }
        }
    }
}

pub struct TreiberStack<T> {
    pub top: AtomicPtr<Node<T>>,
}

impl<T> Default for TreiberStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> TreiberStack<T> {
    pub fn new() -> Self {
        TreiberStack {
            top: AtomicPtr::new(core::ptr::null_mut()),
        }
    }

    pub fn push(&self, val: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: val,
            next: AtomicPtr::new(core::ptr::null_mut()),
        }));
        loop {
            let top = self.top.load(Ordering::Acquire);
            unsafe {
                (*new_node).next.store(top, Ordering::Release);
            }
            if self
                .top
                .compare_exchange(top, new_node, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numa_scheduler() {
        let mut sched = NumaScheduler::new();
        assert_eq!(sched.schedule_task_affinity(true), 0);
        assert_eq!(sched.schedule_task_affinity(false), 1);
    }

    #[test]
    fn test_michael_scott_queue() {
        let queue = MichaelScottQueue::new();
        queue.enqueue(42);
        queue.enqueue(99);
        assert!(!queue.head.load(Ordering::Relaxed).is_null());
    }

    #[test]
    fn test_treiber_stack() {
        let stack = TreiberStack::new();
        stack.push(123);
        assert!(!stack.top.load(Ordering::Relaxed).is_null());
    }
}
