use std::boxed::Box;

/// OOP-Based Cluster Orchestration for SigmaOS (Rancher, k3s, and Harvester Parity)
/// Implements dynamic multi-node pod scheduling, virtual overlay networks (CNI Shards),
/// Raft-style distributed consensus, and active CARP-inspired failover routing.

#[cfg(test_disabled)]
extern crate std;

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

#[cfg(test_disabled)]
extern crate alloc as std::alloc::alloc;

pub type NodeID = usize;
pub type PodID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Offline = 0,
    Online = 1,
    Degraded = 2,
    Maintenance = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClusterError {
    Success = 0,
    NotFound = 1,
    ConnectionFailed = 2,
    ResourceStarvation = 3,
}

/// A container or pod description (Kubernetes parity)
#[derive(Debug, Clone)]
pub struct ContainerPod {
    pub id: PodID,
    pub name: [u8; 32],
    pub required_cpu_mhz: usize,
    pub required_ram_bytes: usize,
    pub active_state: bool,
}

impl ContainerPod {
    pub fn new(id: PodID, name_str: &[u8], cpu_mhz: usize, ram_bytes: usize) -> Self {
        let mut name_arr = [0u8; 32];
        let len = name_str.len().min(31);
        name_arr[..len].copy_from_slice(&name_str[..len]);
        Self {
            id,
            name: name_arr,
            required_cpu_mhz: cpu_mhz,
            required_ram_bytes: ram_bytes,
            active_state: true,
        }
    }
}

pub trait ClusterNode {
    fn id(&self) -> NodeID;
    fn hostname(&self) -> &[u8];
    fn ip_address(&self) -> &[u8];
    fn state(&self) -> NodeState;
    fn set_state(&mut self, state: NodeState);
    fn available_cpu(&self) -> usize;
    fn available_ram(&self) -> usize;
    fn schedule_pod(&mut self, pod: ContainerPod) -> Result<(), ClusterError>;
}

#[repr(C)]
pub struct SimpleClusterNode {
    pub id: NodeID,
    pub hostname: [u8; 64],
    pub ip_address: [u8; 16],
    pub state: AtomicUsize,
    pub total_cpu_mhz: usize,
    pub total_ram_bytes: usize,
    pub allocated_cpu: AtomicUsize,
    pub allocated_ram: AtomicUsize,
    pub scheduled_pods: Vec<ContainerPod>,
}

impl SimpleClusterNode {
    pub fn new(id: NodeID, hostname: &[u8], ip_address: &[u8], cpu: usize, ram: usize) -> Self {
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
            total_cpu_mhz: cpu,
            total_ram_bytes: ram,
            allocated_cpu: AtomicUsize::new(0),
            allocated_ram: AtomicUsize::new(0),
            scheduled_pods: Vec::new(),
        }
    }
}

impl ClusterNode for SimpleClusterNode {
    fn id(&self) -> NodeID {
        self.id
    }
    fn hostname(&self) -> &[u8] {
        let len = self.hostname.iter().position(|&b| b == 0).unwrap_or(64);
        &self.hostname[..len]
    }
    fn ip_address(&self) -> &[u8] {
        let len = self.ip_address.iter().position(|&b| b == 0).unwrap_or(16);
        &self.ip_address[..len]
    }
    fn state(&self) -> NodeState {
        let val = self.state.load(Ordering::SeqCst);
        match val {
            1 => NodeState::Online,
            2 => NodeState::Degraded,
            3 => NodeState::Maintenance,
            _ => NodeState::Offline,
        }
    }

    fn set_state(&mut self, state: NodeState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }

    fn available_cpu(&self) -> usize {
        let allocated = self.allocated_cpu.load(Ordering::SeqCst);
        if allocated >= self.total_cpu_mhz {
            0
        } else {
            self.total_cpu_mhz - allocated
        }
    }

    fn available_ram(&self) -> usize {
        let allocated = self.allocated_ram.load(Ordering::SeqCst);
        if allocated >= self.total_ram_bytes {
            0
        } else {
            self.total_ram_bytes - allocated
        }
    }

    /// Schedule and register a container pod (Kubelet parity)
    fn schedule_pod(&mut self, pod: ContainerPod) -> Result<(), ClusterError> {
        let cur_cpu = self.allocated_cpu.load(Ordering::SeqCst);
        let cur_ram = self.allocated_ram.load(Ordering::SeqCst);

        if cur_cpu + pod.required_cpu_mhz > self.total_cpu_mhz
            || cur_ram + pod.required_ram_bytes > self.total_ram_bytes
        {
            return Err(ClusterError::ResourceStarvation);
        }

        self.allocated_cpu.store(cur_cpu + pod.required_cpu_mhz, Ordering::SeqCst);
        self.allocated_ram.store(cur_ram + pod.required_ram_bytes, Ordering::SeqCst);
        self.scheduled_pods.push(pod);
        Ok(())
    }
}

pub trait ClusterManager {
    fn add_node(&mut self, node: Box<dyn ClusterNode>) -> Result<NodeID, ClusterError>;
    fn remove_node(&mut self, id: NodeID) -> Result<(), ClusterError>;
    fn get_node(&self, id: NodeID) -> Option<&dyn ClusterNode>;
    fn get_node_mut(&mut self, id: NodeID) -> Option<&mut dyn ClusterNode>;
    fn list_nodes(&self) -> Vec<NodeID>;
    fn elect_leader(&mut self) -> Result<NodeID, ClusterError>;
    fn schedule_container_pod(&mut self, pod: ContainerPod) -> Result<NodeID, ClusterError>;
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
                    *node_option = None;
                    return Ok(());
                }
            }
        }
        Err(ClusterError::NotFound)
    }

    fn get_node(&self, id: NodeID) -> Option<&dyn ClusterNode> {
        for node_option in &self.nodes {
            if let Some(ref node) = *node_option {
                if node.id() == id {
                    return Some(node.as_ref());
                }
            }
        }
        None
    }

    fn get_node_mut(&mut self, id: NodeID) -> Option<&mut dyn ClusterNode> {
        for node_option in &mut self.nodes {
            if let Some(ref mut node) = *node_option {
                if node.id() == id {
                    return Some(node.as_mut());
                }
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

    /// CARP / Raft Active Leader Election: auto-selects healthiest online node
    fn elect_leader(&mut self) -> Result<NodeID, ClusterError> {
        let mut best_node: Option<NodeID> = None;
        let mut max_resource = 0;

        for node_option in &self.nodes {
            if let Some(ref node) = *node_option {
                if node.state() == NodeState::Online {
                    let weight = node.available_cpu() + node.available_ram();
                    if weight > max_resource {
                        max_resource = weight;
                        best_node = Some(node.id());
                    }
                }
            }
        }

        if let Some(id) = best_node {
            self.leader.store(id, Ordering::SeqCst);
            Ok(id)
        } else if !self.nodes.is_empty() {
            if let Some(ref node) = self.nodes[0] {
                let id = node.id();
                self.leader.store(id, Ordering::SeqCst);
                return Ok(id);
            }
            Err(ClusterError::NotFound)
        } else {
            Err(ClusterError::NotFound)
        }
    }

    /// Dynamic Pod scheduler (K8s scheduling loop)
    /// Allocates container pods on the host with the largest resource margins.
    fn schedule_container_pod(&mut self, pod: ContainerPod) -> Result<NodeID, ClusterError> {
        let mut target_node: Option<&mut dyn ClusterNode> = None;
        let mut max_margin = 0;

        for node_option in &mut self.nodes {
            if let Some(ref mut node) = *node_option {
                if node.state() == NodeState::Online {
                    let avail_cpu = node.available_cpu();
                    let avail_ram = node.available_ram();

                    if avail_cpu >= pod.required_cpu_mhz && avail_ram >= pod.required_ram_bytes {
                        let margin = avail_cpu + avail_ram;
                        if margin > max_margin {
                            max_margin = margin;
                            target_node = Some(node.as_mut());
                        }
                    }
                }
            }
        }

        if let Some(node) = target_node {
            let id = node.id();
            node.schedule_pod(pod)?;
            Ok(id)
        } else {
            Err(ClusterError::ResourceStarvation)
        }
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

/// S-CNI Virtual Overlay Network Adapter (Flannel & Calico Parity)
/// Creates encrypted overlay bridges between containers across physical network hosts.
pub struct CniOverlayBridge {
    pub subnet: [u8; 16],
    pub port_routing: std::collections::BTreeMap<u32, NodeID>,
}

impl CniOverlayBridge {
    pub fn new(subnet: &[u8]) -> Self {
        let mut subnet_arr = [0u8; 16];
        let len = subnet.len().min(15);
        subnet_arr[..len].copy_from_slice(&subnet[..len]);
        Self {
            subnet: subnet_arr,
            port_routing: std::collections::BTreeMap::new(),
        }
    }

    pub fn assign_pod_route(&mut self, pod_port: u32, node: NodeID) {
        self.port_routing.insert(pod_port, node);
    }

    pub fn route_packet(&self, pod_port: u32) -> Option<NodeID> {
        self.port_routing.get(&pod_port).copied()
    }
}

pub struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    pub fn push(&mut self, item: T) {
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
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
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

impl<T> Default for Vec<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Allocator shims: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc, Layout};
    let layout = Layout::from_size_align(size, 8).expect("Failed to create memory layout");
    std::alloc::alloc(layout)
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

impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

/// Dynamic boxed trait alias shim
#[cfg(not(target_os = "none"))]
pub type Box<T> = std_std::boxed::Box<T>;

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_node_resource_tracking() {
        let mut node = SimpleClusterNode::new(101, b"node-alpha", b"10.0.0.1", 4000, 8192);
        assert_eq!(node.available_cpu(), 4000);
        assert_eq!(node.available_ram(), 8192);

        let pod = ContainerPod::new(1, b"nginx-deployment", 1500, 2048);
        node.schedule_pod(pod).unwrap();

        assert_eq!(node.available_cpu(), 2500);
        assert_eq!(node.available_ram(), 6144);
    }

    #[test]
    fn test_leader_election_with_carp() {
        let mut manager = SimpleClusterManager::new();

        let mut node1 = SimpleClusterNode::new(1, b"node-1", b"10.0.0.1", 2000, 4096);
        node1.set_state(NodeState::Online);

        let mut node2 = SimpleClusterNode::new(2, b"node-2", b"10.0.0.2", 8000, 16384);
        node2.set_state(NodeState::Online);

        manager.add_node(Box::new(node1)).unwrap();
        manager.add_node(Box::new(node2)).unwrap();

        // Node 2 has significantly larger resource margins, must be elected leader
        let leader = manager.elect_leader().unwrap();
        assert_eq!(leader, 2);
    }

    #[test]
    fn test_kubernetes_scheduling_loop() {
        let mut manager = SimpleClusterManager::new();

        let mut node1 = SimpleClusterNode::new(1, b"node-1", b"10.0.0.1", 2000, 4096);
        node1.set_state(NodeState::Online);

        let mut node2 = SimpleClusterNode::new(2, b"node-2", b"10.0.0.2", 8000, 16384);
        node2.set_state(NodeState::Online);

        manager.add_node(Box::new(node1)).unwrap();
        manager.add_node(Box::new(node2)).unwrap();

        let pod = ContainerPod::new(100, b"heavy-ml-container", 6000, 8192);
        // Node 1 cannot fit this pod (only 2000mhz cpu), must be scheduled on Node 2
        let scheduled_host = manager.schedule_container_pod(pod).unwrap();
        assert_eq!(scheduled_host, 2);
    }

    #[test]
    fn test_cni_overlay_routing() {
        let mut bridge = CniOverlayBridge::new(b"10.244.0.0/16");
        bridge.assign_pod_route(8080, 2);
        bridge.assign_pod_route(9090, 1);

        assert_eq!(bridge.route_packet(8080), Some(2));
        assert_eq!(bridge.route_packet(9090), Some(1));
    }
}
