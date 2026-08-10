#![no_std]

use core::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct TaskId(pub u32);

#[derive(Debug, Clone, Copy)]
pub struct Task {
    pub id: TaskId,
    pub vruntime: u64,
    pub priority: u32,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.vruntime == other.vruntime