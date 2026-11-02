//! Buffer management for the SigmaOS compositor.

use std::collections::HashMap;

/// Unique buffer identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferId(pub u64);

/// The backing store for a buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferKind {
    /// Shared-memory buffer.
    Shm { fd: i32, offset: u32, stride: u32, format: u32 },
    /// DMA-BUF (zero-copy GPU buffer).
    DmaBuf { fd: i32, width: u32, height: u32, format: u32, modifier: u64 },
}

/// A compositor buffer object.
#[derive(Debug)]
pub struct Buffer {
    pub id: BufferId,
    pub width: u32,
    pub height: u32,
    pub kind: BufferKind,
    ref_count: u32,
}

impl Buffer {
    pub fn new(id: BufferId, width: u32, height: u32, kind: BufferKind) -> Self {
        Self { id, width, height, kind, ref_count: 0 }
    }

    pub fn acquire(&mut self) { self.ref_count += 1; }

    pub fn release(&mut self) -> bool {
        if self.ref_count > 0 { self.ref_count -= 1; }
        self.ref_count == 0
    }

    pub fn ref_count(&self) -> u32 { self.ref_count }
    pub fn is_dmabuf(&self) -> bool { matches!(self.kind, BufferKind::DmaBuf { .. }) }
    pub fn is_shm(&self) -> bool { matches!(self.kind, BufferKind::Shm { .. }) }
}

/// Pool that owns all live buffers.
#[derive(Debug, Default)]
pub struct BufferPool {
    buffers: HashMap<BufferId, Buffer>,
    next_id: u64,
}

impl BufferPool {
    pub fn new() -> Self { Self::default() }

    pub fn import(&mut self, width: u32, height: u32, kind: BufferKind) -> BufferId {
        let id = BufferId(self.next_id);
        self.next_id += 1;
        self.buffers.insert(id, Buffer::new(id, width, height, kind));
        id
    }

    pub fn acquire(&mut self, id: BufferId) -> Result<(), String> {
        self.buffers.get_mut(&id)
            .ok_or_else(|| format!("buffer {:?} not found", id))
            .map(|b| b.acquire())
    }

    pub fn release(&mut self, id: BufferId) -> Result<bool, String> {
        let ready = self.buffers.get_mut(&id)
            .ok_or_else(|| format!("buffer {:?} not found", id))?
            .release();
        if ready { self.buffers.remove(&id); }
        Ok(ready)
    }

    pub fn get(&self, id: BufferId) -> Option<&Buffer> { self.buffers.get(&id) }
    pub fn count(&self) -> usize { self.buffers.len() }
}
