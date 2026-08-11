#![no_std]
extern crate alloc;
use alloc::vec::Vec;

pub const VIRTIO_MAGIC: u32 = 0x74726976;

#[repr(C)]
pub struct VirtqDesc {
    pub addr: u64,
    pub len: u32,
    pub flags: u16,
    pub next: u16,
}

#[repr(C)]
pub struct VirtqAvail {
    pub flags: u16,
    pub idx: u16,
    pub ring: [u16; 256], // variable size in real implementation
    pub used_event: u16,
}

#[repr(C)]
pub struct VirtqUsedElem {
    pub id: u32,
    pub len: u32,
}

#[repr(C)]
pub struct VirtqUsed {
    pub flags: u16,
    pub idx: u16,
    pub ring: [VirtqUsedElem; 256],
    pub avail_event: u16,
}

pub struct VirtQueue {
    pub size: u16,
    pub desc: *mut VirtqDesc,
    pub avail: *mut VirtqAvail,
    pub used: *mut VirtqUsed,
    pub last_used_idx: u16,
    pub free_head: u16,
    pub free_count: u16,
}

impl VirtQueue {
    pub fn new(size: u16, desc: *mut VirtqDesc, avail: *mut VirtqAvail, used: *mut VirtqUsed) -> Self {
        Self {
            size, desc, avail, used,
            last_used_idx: 0,
            free_head: 0,
            free_count: size,
        }
    }
    
    pub fn add_buf(&mut self, addr: u64, len: u32, flags: u16) -> Result<(), &'static str> {
        if self.free_count == 0 { return Err("No free descriptors"); }
        let head = self.free_head;
        unsafe {
            let desc = &mut *self.desc.add(head as usize);
            desc.addr = addr;
            desc.len = len;
            desc.flags = flags;
            self.free_head = desc.next;
            
            let avail = &mut *self.avail;
            let idx = avail.idx % self.size;
            avail.ring[idx as usize] = head;
            // Memory barrier needed here
            avail.idx = avail.idx.wrapping_add(1);
        }
        self.free_count -= 1;
        Ok(())
    }
}
