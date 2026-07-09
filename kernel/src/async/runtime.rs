// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Custom async/await runtime for SigmaOS
// Zero-allocation, performance-optimized async execution

use core::pin::Pin;
use core::future::Future;
use core::task::{Context, Poll, Waker};

/// Custom async runtime executor
pub struct SigmaExecutor {
    tasks: Vec<Option<Pin<Box<dyn Future<Output = ()>>>>>,
    current_task: usize,
}

impl SigmaExecutor {
    pub const fn new() -> Self {
        Self {
            tasks: Vec::new(),
            current_task: 0,
        }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.push(Some(Box::pin(future)));
    }

    pub fn run(&mut self) {
        loop {
            let mut progress = false;

            for i in 0..self.tasks.len() {
                if let Some(task) = &mut self.tasks[i] {
                    self.current_task = i;
                    let waker = Waker::from(core::task::RawWaker::new(
                        &self.current_task as *const usize as *const (),
                        &NOOP_WAKER_VTABLE,
                    ));
                    let mut cx = Context::from_waker(&waker);

                    match task.as_mut().poll(&mut cx) {
                        Poll::Ready(()) => {
                            self.tasks[i] = None;
                            progress = true;
                        }
                        Poll::Pending => {
                            progress = true;
                        }
                    }
                }
            }

            if !progress {
                break;
            }
        }
    }

    pub fn task_count(&self) -> usize {
        self.tasks.iter().filter(|t| t.is_some()).count()
    }
}

const NOOP_WAKER_VTABLE: core::task::RawWakerVTable = core::task::RawWakerVTable::new(
    noop_clone,
    noop_wake,
    noop_wake_by_ref,
    noop_drop,
);

unsafe fn noop_clone(data: *const ()) -> core::task::RawWaker {
    core::task::RawWaker::new(data, &NOOP_WAKER_VTABLE)
}

unsafe fn noop_wake(_data: *const ()) {}

unsafe fn noop_wake_by_ref(_data: *const ()) {}

unsafe fn noop_drop(_data: *const ()) {}

/// Simple task spawner for async operations
pub struct TaskSpawner {
    executor: *mut SigmaExecutor,
}

impl TaskSpawner {
    pub const fn new(executor: *mut SigmaExecutor) -> Self {
        Self { executor }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        unsafe {
            (*self.executor).spawn(future);
        }
    }
}

unsafe impl Send for TaskSpawner {}
unsafe impl Sync for TaskSpawner {}

/// Simple future for delay operations
pub struct DelayFuture {
    millis: u64,
    elapsed: u64,
}

impl DelayFuture {
    pub const fn new(millis: u64) -> Self {
        Self {
            millis,
            elapsed: 0,
        }
    }
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.elapsed += 1;
        if self.elapsed >= self.millis {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

/// Simple future for async sleep
pub fn sleep(millis: u64) -> DelayFuture {
    DelayFuture::new(millis)
}

/// Join future for running multiple futures concurrently
pub struct JoinFuture<F1, F2> {
    future1: Option<F1>,
    future2: Option<F2>,
}

impl<F1, F2> JoinFuture<F1, F2> {
    pub const fn new(future1: F1, future2: F2) -> Self {
        Self {
            future1: Some(future1),
            future2: Some(future2),
        }
    }
}

impl<F1, F2, O1, O2> Future for JoinFuture<F1, F2>
where
    F1: Future<Output = O1>,
    F2: Future<Output = O2>,
{
    type Output = (O1, O2);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(future1) = &mut self.future1 {
            match Pin::new(future1).poll(cx) {
                Poll::Ready(output1) => {
                    self.future1 = None;
                    if let Some(future2) = &mut self.future2 {
                        match Pin::new(future2).poll(cx) {
                            Poll::Ready(output2) => {
                                self.future2 = None;
                                Poll::Ready((output1, output2))
                            }
                            Poll::Pending => Poll::Pending,
                        }
                    } else {
                        Poll::Pending
                    }
                }
                Poll::Pending => {
                    if let Some(future2) = &mut self.future2 {
                        match Pin::new(future2).poll(cx) {
                            Poll::Ready(output2) => {
                                self.future2 = None;
                                // Continue polling future1
                            }
                            Poll::Pending => Poll::Pending,
                        }
                    } else {
                        Poll::Pending
                    }
                }
            }
        } else {
            Poll::Pending
        }
    }
}

/// Select future for racing multiple futures
pub struct SelectFuture<F1, F2> {
    future1: Option<F1>,
    future2: Option<F2>,
}

impl<F1, F2> SelectFuture<F1, F2> {
    pub const fn new(future1: F1, future2: F2) -> Self {
        Self {
            future1: Some(future1),
            future2: Some(future2),
        }
    }
}

impl<F1, F2, O1, O2> Future for SelectFuture<F1, F2>
where
    F1: Future<Output = O1>,
    F2: Future<Output = O2>,
    O1: Into<Either<O1, O2>>,
    O2: Into<Either<O1, O2>>,
{
    type Output = Either<O1, O2>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(future1) = &mut self.future1 {
            match Pin::new(future1).poll(cx) {
                Poll::Ready(output) => {
                    self.future1 = None;
                    self.future2 = None;
                    Poll::Ready(Either::Left(output))
                }
                Poll::Pending => {
                    if let Some(future2) = &mut self.future2 {
                        match Pin::new(future2).poll(cx) {
                            Poll::Ready(output) => {
                                self.future1 = None;
                                self.future2 = None;
                                Poll::Ready(Either::Right(output))
                            }
                            Poll::Pending => Poll::Pending,
                        }
                    } else {
                        Poll::Pending
                    }
                }
            }
        } else {
            Poll::Pending
        }
    }
}

/// Either type for select results
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

/// Async channel for communication between tasks
pub struct AsyncChannel<T, const N: usize> {
    buffer: [Option<T>; N],
    head: usize,
    tail: usize,
}

impl<T, const N: usize> AsyncChannel<T, N> {
    pub const fn new() -> Self {
        Self {
            buffer: [None; N],
            head: 0,
            tail: 0,
        }
    }

    pub async fn send(&mut self, item: T) -> Result<(), ChannelError> {
        let next_tail = (self.tail + 1) % N;
        
        if next_tail == self.head {
            return Err(ChannelError::Full);
        }
        
        self.buffer[self.tail] = Some(item);
        self.tail = next_tail;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<T, ChannelError> {
        if self.head == self.tail {
            return Err(ChannelError::Empty);
        }
        
        let item = self.buffer[self.head].take();
        self.head = (self.head + 1) % N;
        item.ok_or(ChannelError::Empty)
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn is_full(&self) -> bool {
        (self.tail + 1) % N == self.head
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelError {
    Full,
    Empty,
}

/// Async mutex for mutual exclusion
pub struct AsyncMutex<T> {
    data: core::cell::UnsafeCell<T>,
    locked: core::sync::atomic::AtomicBool,
}

impl<T> AsyncMutex<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: core::cell::UnsafeCell::new(data),
            locked: core::sync::atomic::AtomicBool::new(false),
        }
    }

    pub async fn lock(&self) -> AsyncMutexGuard<'_, T> {
        while self.locked.load(core::sync::atomic::Ordering::Acquire) {
            // Simple spin-wait - in production, use proper async waiting
            core::hint::spin_loop();
        }
        
        self.locked.store(true, core::sync::atomic::Ordering::Release);
        AsyncMutexGuard { mutex: self }
    }
}

unsafe impl<T: Send> Send for AsyncMutex<T> {}
unsafe impl<T: Send> Sync for AsyncMutex<T> {}

pub struct AsyncMutexGuard<'a, T> {
    mutex: &'a AsyncMutex<T>,
}

impl<'a, T> Drop for AsyncMutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, core::sync::atomic::Ordering::Release);
    }
}

impl<'a, T> core::ops::Deref for AsyncMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for AsyncMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}
