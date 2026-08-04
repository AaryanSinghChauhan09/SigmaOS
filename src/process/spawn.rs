// OOP-based Process Spawning and POSIX Signals Framework for SigmaOS
// Implements process lifecycles, fork, exec, and signals (SIGKILL, SIGTERM, SIGINT) under `#![no_std]`.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ProcessID = usize;
pub type SignalHandlerFn = fn(ProcessID, u8);

/// Standard POSIX Signals
pub const SIGINT: u8 = 2; // Interrupt (graceful / catchable)
pub const SIGKILL: u8 = 9; // Force Kill (un-catchable, immediate)
pub const SIGUSR1: u8 = 10; // User defined 1 (catchable)
pub const SIGTERM: u8 = 15; // Terminate (graceful / catchable)

pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;

pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState { Created = 0, Running = 1, Sleeping = 2, Zombie = 3, Terminated = 4 }