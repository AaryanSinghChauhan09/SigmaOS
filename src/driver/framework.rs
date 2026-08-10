use core::mem;
/// OOP-based Driver Framework for SigmaOS
/// Based on Roadmap Item 1: Driver framework
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DriverID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Block = 0,
    Char = 1,
    Network = 2,
    Storage = 3,
    Input = 4,
}