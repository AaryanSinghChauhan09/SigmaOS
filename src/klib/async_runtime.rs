#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// 'sigma-async' Cooperative Runtime for SigmaOS
// A lightweight, `// #![no_std]  // crate-root only` cooperative task executor using raw Future polling.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// A spawnable cooperative asynchronous task
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
}

impl Task {
    /// Create a new asynchronous task from any future
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            future: Box::pin(future),
        }
    }
}

/// Dynamic cooperative async executor
pub struct AsyncExecutor {
    tasks: Vec<Task>,
}

impl AsyncExecutor {
    /// Initialise a new, empty async executor
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Spawn a new cooperative task into the executor queue
    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        self.tasks.push(Task::new(future));
    }

    /// Single round-robin cooperative polling iteration of all ready tasks
    pub fn run_ready_tasks(&mut self) {
        let waker = dummy_waker();
        let mut cx = Context::from_waker(&waker);

        let mut i = 0;
        while i < self.tasks.len() {
            match self.tasks[i].future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    self.tasks.remove(i);
                }
                Poll::Pending => {
                    i += 1;
                }
            }
        }
    }

    /// Check if there are no pending tasks remaining
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

impl Default for AsyncExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to construct a safe dummy waker under `// #![no_std]  // crate-root only`
fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn dummy_raw_waker() -> RawWaker {
    RawWaker::new(core::ptr::null(), &DUMMY_VTABLE)
}

const DUMMY_VTABLE: RawWakerVTable = RawWakerVTable::new(
    |_| dummy_raw_waker(), // clone
    |_| {},                // wake
    |_| {},                // wake_by_ref
    |_| {},                // drop
);

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::rc::Rc;
    use core::cell::RefCell;

    struct SimpleFuture {
        polls_remaining: u32,
        result_ref: Rc<RefCell<u32>>,
    }

    impl Future for SimpleFuture {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.polls_remaining > 0 {
                self.polls_remaining -= 1;
                Poll::Pending
            } else {
                *self.result_ref.borrow_mut() += 1;
                Poll::Ready(())
            }
        }
    }

    #[test]
    fn test_cooperative_async_execution() {
        let mut executor = AsyncExecutor::new();
        let counter = Rc::new(RefCell::new(0u32));
        let counter_clone = Rc::clone(&counter);

        executor.spawn(async move {
            let fut = SimpleFuture {
                polls_remaining: 2,
                result_ref: counter_clone,
            };
            fut.await;
        });

        assert!(!executor.is_empty());

        // Iteration 1: polls_remaining goes from 2 to 1 -> Pending
        executor.run_ready_tasks();
        assert_eq!(*counter.borrow(), 0);
        assert!(!executor.is_empty());

        // Iteration 2: polls_remaining goes from 1 to 0 -> Pending
        executor.run_ready_tasks();
        assert_eq!(*counter.borrow(), 0);
        assert!(!executor.is_empty());

        // Iteration 3: polls_remaining is 0 -> Ready -> Counter is incremented, task resolved and removed!
        executor.run_ready_tasks();
        assert_eq!(*counter.borrow(), 1);
        assert!(executor.is_empty());
    }
}
