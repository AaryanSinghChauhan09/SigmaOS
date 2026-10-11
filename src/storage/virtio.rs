// SPDX-License-Identifier: MIT
//! VirtIO Block Device Driver Implementation for SigmaOS.

use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtIoBlockError {
    Success,
    QueueFull,
    DeviceError,
    InvalidSector,
    DoubleCompletion,
    LostCompletion,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    Read,
    Write,
    Flush,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtIoRequest {
    pub req_id: u64,
    pub req_type: RequestType,
    pub sector: u64,
    pub data: Vec<u8>,
    pub completed: bool,
}

pub struct VirtIoBlockDevice {
    pub device_id: usize,
    pub sector_size: u32,
    pub capacity_sectors: u64,
    pub max_queue_depth: usize,
    pub pending_queue: Vec<VirtIoRequest>,
    pub completed_requests: usize,
}

impl VirtIoBlockDevice {
    pub fn new(device_id: usize, capacity_sectors: u64, max_queue_depth: usize) -> Self {
        Self {
            device_id,
            sector_size: 512,
            capacity_sectors,
            max_queue_depth,
            pending_queue: Vec::new(),
            completed_requests: 0,
        }
    }

    pub fn enqueue_request(&mut self, req_id: u64, req_type: RequestType, sector: u64, data: Vec<u8>) -> Result<(), VirtIoBlockError> {
        if self.pending_queue.len() >= self.max_queue_depth {
            return Err(VirtIoBlockError::QueueFull);
        }
        if sector >= self.capacity_sectors {
            return Err(VirtIoBlockError::InvalidSector);
        }

        self.pending_queue.push(VirtIoRequest {
            req_id,
            req_type,
            sector,
            data,
            completed: false,
        });

        Ok(())
    }

    pub fn process_queue(&mut self) -> usize {
        let mut processed = 0;
        for req in self.pending_queue.iter_mut() {
            if !req.completed {
                req.completed = true;
                processed += 1;
            }
        }
        self.completed_requests += processed;
        processed
    }

    pub fn dequeue_completed(&mut self, req_id: u64) -> Result<VirtIoRequest, VirtIoBlockError> {
        if let Some(pos) = self.pending_queue.iter().position(|r| r.req_id == req_id) {
            let req = &self.pending_queue[pos];
            if !req.completed {
                return Err(VirtIoBlockError::LostCompletion);
            }
            Ok(self.pending_queue.remove(pos))
        } else {
            Err(VirtIoBlockError::DoubleCompletion)
        }
    }
}
