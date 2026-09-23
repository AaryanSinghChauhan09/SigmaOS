#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::new_without_default)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::{BTreeMap, HashMap, HashSet};

#[cfg(any(feature = "standalone_test", test))]
use std::collections::{BTreeMap, HashMap, HashSet};

// ============================================================================
// Linux & BSD Non-Blocking & Async I/O Multiplexing Subsystem
// Inspired by Linux epoll, io_uring, FreeBSD kqueue/kevent, OpenBSD poll/select
// ============================================================================

/// Event Reactor Paradigm / Backend Engine Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AsyncIoBackendKind {
    EpollReactor,   // Linux epoll_create1 / epoll_wait
    KqueueReactor,  // FreeBSD/OpenBSD kqueue / kevent
    IoUringEngine,  // Linux io_uring SQ / CQ ring buffers
    PollMultiplexer,// POSIX poll / select fallback
}

/// Filter / Event Types across epoll and kqueue
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AsyncIoFilterKind {
    Read,           // EPOLLIN / EVFILT_READ
    Write,          // EPOLLOUT / EVFILT_WRITE
    Except,         // EPOLLPRI / EVFILT_EXCEPT
    Signal,         // EVFILT_SIGNAL
    Proc,           // EVFILT_PROC (process state changes)
    Timer,          // EVFILT_TIMER / timerfd
    NetworkSocket,  // Socket I/O
}

/// Notification Trigger Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NotificationTriggerMode {
    LevelTriggered, // Default readiness notification
    EdgeTriggered,  // EPOLLET / EV_CLEAR
    OneShot,        // EPOLLONESHOT / EV_ONESHOT
}

/// I/O Operation Opcode for Linux `io_uring` style submissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoUringOpcode {
    Nop,
    Read,
    Write,
    Accept,
    Connect,
    Timeout,
    Fsync,
}

/// Submission Queue Entry (SQE) for `io_uring`
#[derive(Debug, Clone)]
pub struct IoUringSqe {
    pub user_data: u64,
    pub opcode: IoUringOpcode,
    pub fd: i32,
    pub buffer_offset: usize,
    pub buffer_length: usize,
}

/// Completion Queue Entry (CQE) for `io_uring`
#[derive(Debug, Clone)]
pub struct IoUringCqe {
    pub user_data: u64,
    pub result: i32, // Bytes read/written or error code
    pub flags: u32,
}

/// Registered Interest Event
#[derive(Debug, Clone)]
pub struct AsyncIoInterest {
    pub fd: i32,
    pub filter: AsyncIoFilterKind,
    pub trigger_mode: NotificationTriggerMode,
    pub is_ready: bool,
    pub bytes_available: usize,
    pub user_data: u64,
}

/// Master Async I/O Reactor Engine
pub struct SovereignAsyncIoEngine {
    pub backend: AsyncIoBackendKind,
    pub interests: HashMap<(i32, AsyncIoFilterKind), AsyncIoInterest>,
    pub sqe_ring: Vec<IoUringSqe>,
    pub cqe_ring: Vec<IoUringCqe>,
    pub total_events_processed: u64,
}

impl SovereignAsyncIoEngine {
    pub fn new(backend: AsyncIoBackendKind) -> Self {
        Self {
            backend,
            interests: HashMap::new(),
            sqe_ring: Vec::new(),
            cqe_ring: Vec::new(),
            total_events_processed: 0,
        }
    }

    /// Register interest for a file or socket descriptor (`epoll_ctl(ADD)` / `kevent(EV_ADD)`)
    pub fn register_interest(
        &mut self,
        fd: i32,
        filter: AsyncIoFilterKind,
        trigger_mode: NotificationTriggerMode,
        user_data: u64,
    ) {
        let interest = AsyncIoInterest {
            fd,
            filter,
            trigger_mode,
            is_ready: false,
            bytes_available: 0,
            user_data,
        };
        self.interests.insert((fd, filter), interest);
    }

    /// Mark an interest as ready (`epoll_wait` or `kevent` event trigger)
    pub fn notify_event(&mut self, fd: i32, filter: AsyncIoFilterKind, bytes_available: usize) {
        if let Some(interest) = self.interests.get_mut(&(fd, filter)) {
            interest.is_ready = true;
            interest.bytes_available = bytes_available;
        }
    }

    /// Poll for active ready events (`epoll_wait` / `kevent`)
    pub fn poll_events(&mut self, max_events: usize) -> Vec<AsyncIoInterest> {
        let mut ready_events = Vec::new();
        let mut keys_to_reset = Vec::new();

        for ((fd, filter), interest) in self.interests.iter_mut() {
            if interest.is_ready {
                ready_events.push(interest.clone());
                self.total_events_processed += 1;

                match interest.trigger_mode {
                    NotificationTriggerMode::EdgeTriggered | NotificationTriggerMode::OneShot => {
                        keys_to_reset.push((*fd, *filter));
                    }
                    NotificationTriggerMode::LevelTriggered => {}
                }

                if ready_events.len() >= max_events {
                    break;
                }
            }
        }

        // Reset or disarm edge-triggered / one-shot events
        for key in keys_to_reset {
            if let Some(interest) = self.interests.get_mut(&key) {
                if interest.trigger_mode == NotificationTriggerMode::OneShot {
                    interest.is_ready = false;
                } else {
                    interest.is_ready = false;
                }
            }
        }

        ready_events
    }

    /// Submit `io_uring` SQE ring buffer entry
    pub fn submit_io_uring_sqe(&mut self, opcode: IoUringOpcode, fd: i32, len: usize, user_data: u64) {
        let sqe = IoUringSqe {
            user_data,
            opcode,
            fd,
            buffer_offset: 0,
            buffer_length: len,
        };
        self.sqe_ring.push(sqe);
    }

    /// Process submission ring SQEs into completion ring CQEs
    pub fn process_io_uring_ring(&mut self) -> usize {
        let mut processed = 0;
        let sqes = core::mem::take(&mut self.sqe_ring);

        for sqe in sqes {
            let res_bytes = match sqe.opcode {
                IoUringOpcode::Read | IoUringOpcode::Write => sqe.buffer_length as i32,
                IoUringOpcode::Accept | IoUringOpcode::Connect => 0,
                IoUringOpcode::Nop | IoUringOpcode::Timeout | IoUringOpcode::Fsync => 0,
            };

            let cqe = IoUringCqe {
                user_data: sqe.user_data,
                result: res_bytes,
                flags: 0,
            };

            self.cqe_ring.push(cqe);
            self.total_events_processed += 1;
            processed += 1;
        }

        processed
    }

    /// Reap completed `io_uring` CQEs
    pub fn reap_cqes(&mut self) -> Vec<IoUringCqe> {
        core::mem::take(&mut self.cqe_ring)
    }
}

impl Default for SovereignAsyncIoEngine {
    fn default() -> Self {
        Self::new(AsyncIoBackendKind::EpollReactor)
    }
}

// ============================================================================
// Standalone Unit Test Suite
// ============================================================================

#[cfg(test)]
mod async_io_tests {
    use super::*;

    #[test]
    fn test_epoll_kqueue_event_reactor_flow() {
        let mut engine = SovereignAsyncIoEngine::new(AsyncIoBackendKind::KqueueReactor);

        // Register socket descriptor 12 for READ in Edge-Triggered mode
        engine.register_interest(12, AsyncIoFilterKind::Read, NotificationTriggerMode::EdgeTriggered, 0xA1);

        // Initially no events ready
        assert_eq!(engine.poll_events(10).len(), 0);

        // Notify socket descriptor 12 is ready with 1024 bytes
        engine.notify_event(12, AsyncIoFilterKind::Read, 1024);

        let active = engine.poll_events(10);
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].fd, 12);
        assert_eq!(active[0].bytes_available, 1024);

        // Edge-triggered: subsequent poll returns 0 until new notification
        assert_eq!(engine.poll_events(10).len(), 0);
    }

    #[test]
    fn test_io_uring_sqe_cqe_ring_buffer() {
        let mut engine = SovereignAsyncIoEngine::new(AsyncIoBackendKind::IoUringEngine);

        engine.submit_io_uring_sqe(IoUringOpcode::Read, 5, 4096, 0x101);
        engine.submit_io_uring_sqe(IoUringOpcode::Write, 6, 2048, 0x102);

        assert_eq!(engine.sqe_ring.len(), 2);

        let count = engine.process_io_uring_ring();
        assert_eq!(count, 2);
        assert_eq!(engine.sqe_ring.len(), 0);

        let cqes = engine.reap_cqes();
        assert_eq!(cqes.len(), 2);
        assert_eq!(cqes[0].result, 4096);
        assert_eq!(cqes[1].result, 2048);
    }
}
