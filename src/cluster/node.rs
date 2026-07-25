#![no_std]
#![no_main]

/// OOP-based Cluster Node Management for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 231
/// Implements cluster node management and coordination

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type NodeID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NodeState { Offline = 0, Online = 1, Degraded = 2, Maintenance = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClusterError { Success = 0, NotFound = 1, ConnectionFailed = 2 }

pub trait ClusterNode {
    fn id(&self) -> NodeID;
    fn hostname(&self) -> &[u8];
    fn ip_address(&self) -> &[u8];
    fn state(&self) -> NodeState;
    fn set_state(&mut self, state: NodeState);
}

#[repr(C)]
pub struct SimpleClusterNode {
    pub id: NodeID,
    pub hostname: [u8; 64],
    pub ip_address: [u8; 16],
    pub state: AtomicUsize,
}

impl SimpleClusterNode {
    pub fn new(id: NodeID, hostname: &[u8], ip_address: &[u8]) -> Self {
        let mut host_array = [0u8; 64];
        let mut ip_array = [0u8; 16];
        let host_len = hostname.len().min(63);
        let ip_len = ip_address.len().min(15);
        unsafe {
            core::ptr::copy_nonoverlapping(hostname.as_ptr(), host_array.as_mut_ptr(), host_len);
            core::ptr::copy_nonoverlapping(ip_address.as_ptr(), ip_array.as_mut_ptr(), ip_len);
        }
        SimpleClusterNode {
            id,
            hostname: host_array,
            ip_address: ip_array,
            state: AtomicUsize::new(NodeState::Offline as usize),
        }
    }
}

impl ClusterNode for SimpleClusterNode {
    fn id(&self) -> NodeID { self.id }
    fn hostname(&self) -> &[u8] {
        let len = self.hostname.iter().position(|&b| b == 0).unwrap_or(64);
        &self.hostname[..len]
    }
    fn ip_address(&self) -> &[u8] {
        let len = self.ip_address.iter().position(|&b| b == 0).unwrap_or(16);
        &self.ip_address[..len]
    }
    fn state(&self) -> NodeState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

    fn set_state(&mut self, state: NodeState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

pub trait ClusterManager {
    fn add_node(&mut self, node: Box<dyn ClusterNode>) -> Result<NodeID, ClusterError>;
    fn remove_node(&mut self, id: NodeID) -> Result<(), ClusterError>;
    fn get_node(&self, id: NodeID) -> Option<&dyn ClusterNode>;
    fn list_nodes(&self) -> Vec<NodeID>;
    fn elect_leader(&mut self) -> Result<NodeID, ClusterError>;
}

#[repr(C)]
pub struct SimpleClusterManager {
    pub nodes: Vec<Option<Box<dyn ClusterNode>>>,
    pub leader: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleClusterManager {
    pub fn new() -> Self {
        SimpleClusterManager {
            nodes: Vec::new(),
            leader: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ClusterManager for SimpleClusterManager {
    fn add_node(&mut self, node: Box<dyn ClusterNode>) -> Result<NodeID, ClusterError> {
        let id = node.id();
        self.nodes.push(Some(node));
        Ok(id)
    }

    fn remove_node(&mut self, id: NodeID) -> Result<(), ClusterError> {
        for node_option in &mut self.nodes {
            if let Some(ref node) = *node_option {
                if node.id() == id {
                    return Ok(());
                }
            }
        }
        Err(ClusterError::NotFound)
    }

    fn get_node(&self, id: NodeID) -> Option<&dyn ClusterNode> {
        for node_option in &self.nodes {
            if let Some(ref node) = *node_option {
                if node.id() == id { return Some(node.as_ref()); }
            }
        }
        None
    }

    fn list_nodes(&self) -> Vec<NodeID> {
        let mut ids = Vec::new();
        for node_option in &self.nodes {
            if let Some(ref node) = *node_option {
                ids.push(node.id());
            }
        }
        ids
    }

    fn elect_leader(&mut self) -> Result<NodeID, ClusterError> {
        if !self.nodes.is_empty() {
            if let Some(ref node) = *self.nodes[0] {
                self.leader.store(node.id(), Ordering::SeqCst);
                return Ok(node.id());
            }
        }
        Err(ClusterError::NotFound)
    }
}

pub trait Consensus {
    fn propose(&mut self, value: &[u8]) -> Result<(), ClusterError>;
    fn vote(&mut self, proposal_id: usize, accept: bool) -> Result<(), ClusterError>;
    fn get_consensus(&self) -> Option<&[u8]>;
}

#[repr(C)]
pub struct SimpleConsensus {
    pub proposals: Vec<(usize, [u8; 128], Vec<bool>)>,
    pub next_id: AtomicUsize,
}

impl SimpleConsensus {
    pub fn new() -> Self {
        SimpleConsensus {
            proposals: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Consensus for SimpleConsensus {
    fn propose(&mut self, value: &[u8]) -> Result<(), ClusterError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let mut value_array = [0u8; 128];
        let value_len = value.len().min(127);
        for i in 0..value_len {
            value_array[i] = value[i];
        }
        self.proposals.push((id, value_array, Vec::new()));
        Ok(())
    }

    fn vote(&mut self, proposal_id: usize, accept: bool) -> Result<(), ClusterError> {
        for proposal in &mut self.proposals {
            if proposal.0 == proposal_id {
                proposal.2.push(accept);
                return Ok(());
            }
        }
        Err(ClusterError::NotFound)
    }

    fn get_consensus(&self) -> Option<&[u8]> {
        for proposal in &self.proposals {
            let accepts = proposal.2.iter().filter(|&&v| v).count();
            if accepts > proposal.2.len() / 2 {
                let len = proposal.1.iter().position(|&b| b == 0).unwrap_or(128);
                return Some(&proposal.1[..len]);
            }
        }
        None
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn is_empty(&self) -> bool { self.len == 0 }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
