//! Key Innovations Absorbed from Open Source OS Repositories:
//! Haiku OS (app_server UI responsiveness), Redox OS (Microkernel Scheme IPC Grants),
//! illumos/ZFS (Adaptive Replacement Cache - ARC), and Plan 9 (9P2000.u GPU device sharing).
extern crate alloc;

use alloc::vec::Vec;

/// 1. Haiku OS app_server Window Damage & UI Priority Booster
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HaikuWindow {
    pub window_id: u64,
    pub title: Vec<u8>,
    pub bounds: Rect,
    pub damage_region: Option<Rect>,
    pub ui_priority: i32,
}

pub struct HaikuApplicationServer {
    windows: Vec<HaikuWindow>,
    active_window_id: Option<u64>,
    render_queue: Vec<u64>,
}

impl HaikuApplicationServer {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_window_id: None,
            render_queue: Vec::new(),
        }
    }

    pub fn create_window(&mut self, window_id: u64, title: &[u8], bounds: Rect) {
        self.windows.push(HaikuWindow {
            window_id,
            title: title.to_vec(),
            bounds,
            damage_region: None,
            ui_priority: 0,
        });
    }

    /// Focuses window and boosts UI event loop priority (Haiku App Server responsiveness model)
    pub fn focus_window(&mut self, window_id: u64) -> bool {
        let mut found = false;
        for win in self.windows.iter_mut() {
            if win.window_id == window_id {
                win.ui_priority = 10; // Boost active window UI thread
                found = true;
            } else {
                win.ui_priority = 0;
            }
        }
        if found {
            self.active_window_id = Some(window_id);
        }
        found
    }

    /// Invalidates region for incremental damage redraws
    pub fn invalidate_region(&mut self, window_id: u64, damage: Rect) -> bool {
        for win in self.windows.iter_mut() {
            if win.window_id == window_id {
                win.damage_region = Some(damage);
                self.render_queue.push(window_id);
                return true;
            }
        }
        false
    }

    pub fn process_redraws(&mut self) -> usize {
        let count = self.render_queue.len();
        self.render_queue.clear();
        count
    }
}

/// 2. Redox OS Zero-Copy Microkernel Scheme Grant IPC
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrantRights {
    Read,
    Write,
    ReadWrite,
    Execute,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryGrant {
    pub grant_id: u64,
    pub physical_address: u64,
    pub size_pages: usize,
    pub rights: GrantRights,
}

pub struct RedoxSchemeChannel {
    pub scheme_name: Vec<u8>,
    grants: Vec<MemoryGrant>,
    next_grant_id: u64,
}

impl RedoxSchemeChannel {
    pub fn new(scheme_name: &[u8]) -> Self {
        Self {
            scheme_name: scheme_name.to_vec(),
            grants: Vec::new(),
            next_grant_id: 1,
        }
    }

    /// Grants memory pages from client to microkernel scheme handler without copying
    pub fn create_grant(&mut self, phys_addr: u64, size_pages: usize, rights: GrantRights) -> u64 {
        let id = self.next_grant_id;
        self.next_grant_id += 1;

        self.grants.push(MemoryGrant {
            grant_id: id,
            physical_address: phys_addr,
            size_pages,
            rights,
        });
        id
    }

    pub fn revoke_grant(&mut self, grant_id: u64) -> bool {
        let initial_len = self.grants.len();
        self.grants.retain(|g| g.grant_id != grant_id);
        self.grants.len() < initial_len
    }

    pub fn active_grants_count(&self) -> usize {
        self.grants.len()
    }
}

/// 3. illumos / ZFS Two-Tier Adaptive Replacement Cache (ARC)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArcBlock {
    pub block_id: u64,
    pub data: Vec<u8>,
    pub access_count: u32,
}

pub struct IllumosZfsArcCache {
    target_capacity: usize,
    mru_list: Vec<ArcBlock>, // Most Recently Used
    mfu_list: Vec<ArcBlock>, // Most Frequently Used
    mru_ghost: Vec<u64>,     // Evicted MRU block IDs
    mfu_ghost: Vec<u64>,     // Evicted MFU block IDs
    p_mru_target: usize,     // Dynamic adaptation target for MRU
}

impl IllumosZfsArcCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            target_capacity: capacity,
            mru_list: Vec::new(),
            mfu_list: Vec::new(),
            mru_ghost: Vec::new(),
            mfu_ghost: Vec::new(),
            p_mru_target: capacity / 2,
        }
    }

    pub fn read_block(&mut self, block_id: u64) -> Option<Vec<u8>> {
        // Check MFU
        for block in self.mfu_list.iter_mut() {
            if block.block_id == block_id {
                block.access_count += 1;
                return Some(block.data.clone());
            }
        }

        // Check MRU -> promote to MFU on hit
        if let Some(pos) = self.mru_list.iter().position(|b| b.block_id == block_id) {
            let mut block = self.mru_list.remove(pos);
            block.access_count += 1;
            let data = block.data.clone();
            self.mfu_list.push(block);
            return Some(data);
        }

        None
    }

    pub fn insert_block(&mut self, block_id: u64, data: Vec<u8>) {
        // Adapt target size p based on ghost hits
        if self.mru_ghost.contains(&block_id) {
            self.p_mru_target = (self.p_mru_target + 1).min(self.target_capacity);
            self.mru_ghost.retain(|&id| id != block_id);
        } else if self.mfu_ghost.contains(&block_id) {
            self.p_mru_target = self.p_mru_target.saturating_sub(1);
            self.mfu_ghost.retain(|&id| id != block_id);
        }

        // Evict if over capacity
        if self.mru_list.len() + self.mfu_list.len() >= self.target_capacity {
            if self.mru_list.len() >= self.p_mru_target && !self.mru_list.is_empty() {
                let evicted = self.mru_list.remove(0);
                self.mru_ghost.push(evicted.block_id);
            } else if !self.mfu_list.is_empty() {
                let evicted = self.mfu_list.remove(0);
                self.mfu_ghost.push(evicted.block_id);
            }
        }

        self.mru_list.push(ArcBlock {
            block_id,
            data,
            access_count: 1,
        });
    }

    pub fn stats(&self) -> (usize, usize, usize, usize) {
        (
            self.mru_list.len(),
            self.mfu_list.len(),
            self.mru_ghost.len(),
            self.mfu_ghost.len(),
        )
    }
}

/// 4. Plan 9 / 9P2000.u GPU Virtualization & Frame Streaming Protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Plan9GpuOp {
    AllocSurface { width: u32, height: u32, bpp: u32 },
    SubmitCommandBuffer { commands: Vec<u8> },
    PresentSurface { surface_id: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan9GpuCommand {
    pub tag: u16,
    pub container_id: u64,
    pub op: Plan9GpuOp,
}

pub struct Plan9GpuDevice {
    allocated_surfaces: usize,
    processed_commands: usize,
}

impl Plan9GpuDevice {
    pub fn new() -> Self {
        Self {
            allocated_surfaces: 0,
            processed_commands: 0,
        }
    }

    pub fn handle_9p_gpu_command(&mut self, cmd: Plan9GpuCommand) -> bool {
        self.processed_commands += 1;
        match cmd.op {
            Plan9GpuOp::AllocSurface { .. } => {
                self.allocated_surfaces += 1;
                true
            }
            Plan9GpuOp::SubmitCommandBuffer { .. } => true,
            Plan9GpuOp::PresentSurface { .. } => true,
        }
    }

    pub fn stats(&self) -> (usize, usize) {
        (self.allocated_surfaces, self.processed_commands)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haiku_app_server() {
        let mut server = HaikuApplicationServer::new();
        server.create_window(
            1,
            b"Terminal",
            Rect {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            },
        );
        assert!(server.focus_window(1));
        assert!(server.invalidate_region(
            1,
            Rect {
                x: 10,
                y: 10,
                width: 100,
                height: 100
            }
        ));
        assert_eq!(server.process_redraws(), 1);
    }

    #[test]
    fn test_redox_scheme_grant() {
        let mut scheme = RedoxSchemeChannel::new(b"display");
        let grant_id = scheme.create_grant(0x1000, 4, GrantRights::ReadWrite);
        assert_eq!(grant_id, 1);
        assert_eq!(scheme.active_grants_count(), 1);
        assert!(scheme.revoke_grant(grant_id));
        assert_eq!(scheme.active_grants_count(), 0);
    }

    #[test]
    fn test_illumos_zfs_arc() {
        let mut arc = IllumosZfsArcCache::new(2);
        arc.insert_block(101, vec![1, 2, 3]);
        arc.insert_block(102, vec![4, 5, 6]);

        // Access 101 to promote to MFU
        assert_eq!(arc.read_block(101), Some(vec![1, 2, 3]));

        // Insert 103 -> evicts unpromoted MRU 102
        arc.insert_block(103, vec![7, 8, 9]);

        let (_mru_len, mfu_len, mru_ghost, _mfu_ghost) = arc.stats();
        assert_eq!(mfu_len, 1); // 101
        assert_eq!(mru_ghost, 1); // 102
    }

    #[test]
    fn test_plan9_gpu_virtualization() {
        let mut gpu = Plan9GpuDevice::new();
        let cmd = Plan9GpuCommand {
            tag: 1,
            container_id: 42,
            op: Plan9GpuOp::AllocSurface {
                width: 1920,
                height: 1080,
                bpp: 32,
            },
        };
        assert!(gpu.handle_9p_gpu_command(cmd));
        let (surfaces, cmds) = gpu.stats();
        assert_eq!(surfaces, 1);
        assert_eq!(cmds, 1);
    }
}
