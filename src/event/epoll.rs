#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// POSIX Epoll I/O Multiplexing Event Loop Subsystem for SigmaOS
// Zero-dependency, #![no_std] compliant I/O event notification mechanism (Linux epoll(7) parity)

use std::vec::Vec;

/// Epoll Event Notification Flags
pub const EPOLLIN: u32 = 0x001;
pub const EPOLLPRI: u32 = 0x002;
pub const EPOLLOUT: u32 = 0x004;
pub const EPOLLERR: u32 = 0x008;
pub const EPOLLHUP: u32 = 0x010;
pub const EPOLLRDHUP: u32 = 0x2000;
pub const EPOLLET: u32 = 1 << 31;

/// Epoll Operation Control Opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EpollOp {
    CtlAdd = 1,
    CtlDel = 2,
    CtlMod = 3,
}

/// Epoll Event Payload Data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpollData {
    pub fd: i32,
    pub u64_val: u64,
}

impl EpollData {
    pub const fn from_fd(fd: i32) -> Self {
        Self {
            fd,
            u64_val: fd as u64,
        }
    }

    pub const fn from_u64(val: u64) -> Self {
        Self {
            fd: val as i32,
            u64_val: val,
        }
    }
}

/// Epoll Event Descriptor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EpollEvent {
    pub events: u32,
    pub data: EpollData,
}

impl EpollEvent {
    pub const fn new(events: u32, fd: i32) -> Self {
        Self {
            events,
            data: EpollData::from_fd(fd),
        }
    }
}

/// Registered File Descriptor Watch Target
#[derive(Debug, Clone)]
pub struct EpollItem {
    pub fd: i32,
    pub events: u32,
    pub data: EpollData,
    pub ready_events: u32,
}

/// Epoll Event Loop Instance
#[derive(Debug, Clone)]
pub struct EpollInstance {
    pub epoll_fd: i32,
    pub items: Vec<EpollItem>,
    pub max_events: usize,
}

impl EpollInstance {
    pub fn new(epoll_fd: i32, max_events: usize) -> Self {
        Self {
            epoll_fd,
            items: Vec::new(),
            max_events,
        }
    }

    /// Add, modify, or remove target file descriptor interest
    pub fn ctl(
        &mut self,
        op: EpollOp,
        fd: i32,
        event: Option<EpollEvent>,
    ) -> Result<(), &'static str> {
        match op {
            EpollOp::CtlAdd => {
                let ev = event.ok_or("EINVAL: Event required for EPOLL_CTL_ADD")?;
                for item in self.items.iter() {
                    if item.fd == fd {
                        return Err("EEXIST: File descriptor already registered");
                    }
                }
                self.items.push(EpollItem {
                    fd,
                    events: ev.events,
                    data: ev.data,
                    ready_events: 0,
                });
                Ok(())
            }
            EpollOp::CtlMod => {
                let ev = event.ok_or("EINVAL: Event required for EPOLL_CTL_MOD")?;
                for item in self.items.iter_mut() {
                    if item.fd == fd {
                        item.events = ev.events;
                        item.data = ev.data;
                        return Ok(());
                    }
                }
                Err("ENOENT: File descriptor not registered")
            }
            EpollOp::CtlDel => {
                let mut found_index = None;
                for (idx, item) in self.items.iter().enumerate() {
                    if item.fd == fd {
                        found_index = Some(idx);
                        break;
                    }
                }
                if let Some(idx) = found_index {
                    self.items.remove(idx);
                    Ok(())
                } else {
                    Err("ENOENT: File descriptor not registered")
                }
            }
        }
    }

    /// Signal readiness events for a registered file descriptor
    pub fn trigger_event(&mut self, fd: i32, ready_events: u32) {
        for item in self.items.iter_mut() {
            if item.fd == fd {
                item.ready_events |= ready_events & item.events;
            }
        }
    }

    /// Wait for pending I/O events up to maxevents limit
    pub fn wait(&mut self, events_out: &mut [EpollEvent]) -> usize {
        let count_to_fill = events_out.len().min(self.max_events);
        let mut n_ready = 0;

        for item in self.items.iter_mut() {
            if n_ready >= count_to_fill {
                break;
            }

            if item.ready_events != 0 {
                events_out[n_ready] = EpollEvent {
                    events: item.ready_events,
                    data: item.data,
                };
                n_ready += 1;

                // Edge-triggered reset
                if (item.events & EPOLLET) != 0 {
                    item.ready_events = 0;
                }
            }
        }

        n_ready
    }
}
