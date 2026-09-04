use core::mem;
/// JehanneOS & Plan 9-inspired 9P Distributed Filesystem and Namespace Suite for SigmaOS
/// Provides 9P protocol serialization transactions, custom namespace binds,
/// and distributed compute process execution handoffs.
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plan9pMsgType {
    Tversion = 100,
    Rversion = 101,
    Tattach = 104,
    Rattach = 105,
    Twalk = 110,
    Rwalk = 111,
    Tread = 116,
    Rread = 117,
    Twrite = 118,
    Rwrite = 119,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JehanneError {
    Success = 0,
    Invalid9pMessage = 1,
    MountFailed = 2,
    NodeOffline = 3,
}

/// Simulated Plan 9 9P Protocol Message Packet
pub struct Plan9pMessage {
    pub msg_type: Plan9pMsgType,
    pub tag: u16,
    pub fid: u32,
    pub payload: [u8; 64],
    pub payload_len: usize,
}

impl Plan9pMessage {
    pub fn new(msg_type: Plan9pMsgType, tag: u16, fid: u32, payload: &[u8]) -> Self {
        let mut pay_arr = [0u8; 64];
        let len = payload.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(payload.as_ptr(), pay_arr.as_mut_ptr(), len);
        }
        Plan9pMessage {
            msg_type,
            tag,
            fid,
            payload: pay_arr,
            payload_len: len,
        }
    }
}

/// Decoupled Directory Namespace Bind mapping representation
pub struct NamespaceBindEntry {
    pub target_path: [u8; 32],
    pub source_path: [u8; 32],
}

impl NamespaceBindEntry {
    pub fn new(target: &[u8], source: &[u8]) -> Self {
        let mut target_arr = [0u8; 32];
        let mut source_arr = [0u8; 32];
        let t_len = target.len().min(31);
        let s_len = source.len().min(31);

        unsafe {
            core::ptr::copy_nonoverlapping(target.as_ptr(), target_arr.as_mut_ptr(), t_len);
            core::ptr::copy_nonoverlapping(source.as_ptr(), source_arr.as_mut_ptr(), s_len);
        }

        NamespaceBindEntry {
            target_path: target_arr,
            source_path: source_arr,
        }
    }
}

/// JehanneOS-style process directory namespace mapper
pub struct JehanneNamespace {
    pub binds: Vec<Option<NamespaceBindEntry>>,
}

impl JehanneNamespace {
    pub fn new() -> Self {
        JehanneNamespace { binds: Vec::new() }
    }

    /// Securely overlay/bind a folder namespace (Plan 9 'bind' command equivalent)
    pub fn bind_namespace(&mut self, target: &[u8], source: &[u8]) {
        let entry = NamespaceBindEntry::new(target, source);
        self.binds.push(Some(entry));
    }

    /// Resolve virtual namespace target path back to real source physical path
    pub fn resolve_path(&self, target: &[u8]) -> Option<[u8; 32]> {
        for i in 0..self.binds.len {
            if let Some(ref entry) = self.binds[i] {
                let len = entry.target_path.iter().position(|&b| b == 0).unwrap_or(32);
                if &entry.target_path[..len] == target {
                    return Some(entry.source_path);
                }
            }
        }
        None
    }
}

/// Distributed Node representation for compute offloading
pub struct ComputeNode {
    pub node_id: usize,
    pub endpoint: [u8; 32],
    pub available_cores: usize,
}

/// Jehanne-inspired Distributed Compute Process Handoff Manager
pub struct DistributedComputeHandoff {
    pub nodes: Vec<Option<ComputeNode>>,
    pub total_handoffs: AtomicUsize,
}

impl DistributedComputeHandoff {
    pub fn new() -> Self {
        DistributedComputeHandoff {
            nodes: Vec::new(),
            total_handoffs: AtomicUsize::new(0),
        }
    }

    pub fn register_compute_node(&mut self, node_id: usize, endpoint: &[u8], cores: usize) {
        let mut ep_arr = [0u8; 32];
        let len = endpoint.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(endpoint.as_ptr(), ep_arr.as_mut_ptr(), len);
        }
        let node = ComputeNode {
            node_id,
            endpoint: ep_arr,
            available_cores: cores,
        };
        self.nodes.push(Some(node));
    }

    /// Offloads dynamic process thread execution onto a target distributed node
    pub fn handoff_execution(
        &self,
        node_id: usize,
        task_bytecode: &[u8],
    ) -> Result<usize, JehanneError> {
        let mut found = false;
        for i in 0..self.nodes.len {
            if let Some(ref node) = self.nodes[i] {
                if node.node_id == node_id {
                    found = true;
                    break;
                }
            }
        }

        if !found {
            return Err(JehanneError::NodeOffline);
        }

        // Mock compute calculation of task bytes
        let mut result_acc = 0usize;
        for &b in task_bytecode {
            result_acc = result_acc.wrapping_add(b as usize);
        }

        self.total_handoffs.fetch_add(1, Ordering::SeqCst);
        Ok(result_acc)
    }
}

pub struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use alloc::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9p_message_creation() {
        let payload = b"version=9P2000";
        let msg = Plan9pMessage::new(Plan9pMsgType::Tversion, 0, 1, payload);

        assert_eq!(msg.msg_type, Plan9pMsgType::Tversion);
        assert_eq!(msg.tag, 0);
        assert_eq!(msg.fid, 1);
        assert_eq!(&msg.payload[..14], payload);
    }

    #[test]
    fn test_jehanne_namespace_binds() {
        let mut ns = JehanneNamespace::new();
        ns.bind_namespace(b"/usr/local", b"/mnt/network/local");

        assert_eq!(ns.binds.len, 1);
        let resolved = ns.resolve_path(b"/usr/local").unwrap();

        let mut res_path = [0u8; 18];
        for i in 0..18 {
            res_path[i] = resolved[i];
        }
        assert_eq!(&res_path, b"/mnt/network/local");
    }

    #[test]
    fn test_distributed_compute_handoffs() {
        let mut compute = DistributedComputeHandoff::new();
        compute.register_compute_node(42, b"node-srv-1.plan9.net", 16);

        assert_eq!(compute.nodes.len, 1);

        // Handoff compiled task bytecode sum math
        let task_bytes = [10, 20, 30]; // sum = 60
        let res = compute.handoff_execution(42, &task_bytes).unwrap();
        assert_eq!(res, 60);
        assert_eq!(compute.total_handoffs.load(Ordering::SeqCst), 1);

        // Handoff to offline/missing node
        assert_eq!(
            compute.handoff_execution(99, &task_bytes).unwrap_err(),
            JehanneError::NodeOffline
        );
    }
}
