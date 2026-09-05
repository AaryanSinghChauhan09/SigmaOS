#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// NUMA (Non-Uniform Memory Access) Allocator
// Linux-style NUMA-aware memory allocation for multi-socket systems

// (no_std only applicable at crate root - removed)

use std::collections::BTreeMap;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeState {
    Online,
    Offline,
    MemoryPressure,
}

#[derive(Debug, Clone)]
pub struct NumaNode {
    pub node_id: u32,
    pub state: NodeState,
    pub distance: Vec<u32>, // Distance to other nodes
    pub memory_size: u64,   // Total memory in bytes
    pub free_memory: u64,   // Free memory in bytes
    pub cpus: Vec<u32>,     // CPUs attached to this node
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationPolicy {
    Local,          // Allocate from local node
    Interleave,     // Interleave across nodes
    Preferred(u32), // Preferred node
    Bind(u32),      // Bind to specific node
}

pub struct NumaAllocator {
    nodes: BTreeMap<u32, NumaNode>,
    default_policy: AllocationPolicy,
    current_node: u32,
}

impl NumaAllocator {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            default_policy: AllocationPolicy::Local,
            current_node: 0,
        }
    }

    /// Add a NUMA node to the system
    pub fn add_node(&mut self, node: NumaNode) -> Result<(), &'static str> {
        if self.nodes.contains_key(&node.node_id) {
            return Err("Node already exists");
        }
        self.nodes.insert(node.node_id, node);
        Ok(())
    }

    /// Get a node by ID
    pub fn get_node(&self, node_id: u32) -> Option<&NumaNode> {
        self.nodes.get(&node_id)
    }

    /// Get the current node
    pub fn current_node(&self) -> u32 {
        self.current_node
    }

    /// Set the current node
    pub fn set_current_node(&mut self, node_id: u32) -> Result<(), &'static str> {
        if !self.nodes.contains_key(&node_id) {
            return Err("Node not found");
        }
        self.current_node = node_id;
        Ok(())
    }

    /// Set the default allocation policy
    pub fn set_policy(&mut self, policy: AllocationPolicy) {
        self.default_policy = policy;
    }

    /// Get the default allocation policy
    pub fn policy(&self) -> AllocationPolicy {
        self.default_policy
    }

    /// Select the best node for allocation based on policy
    pub fn select_node(&self, policy: Option<AllocationPolicy>) -> Option<u32> {
        let policy = policy.unwrap_or(self.default_policy);

        match policy {
            AllocationPolicy::Local => {
                // Return current node if it has free memory
                if let Some(node) = self.nodes.get(&self.current_node) {
                    if node.free_memory > 0 && node.state == NodeState::Online {
                        return Some(self.current_node);
                    }
                }
                // Fallback to any online node with free memory
                self.nodes
                    .iter()
                    .find(|(_, n)| n.free_memory > 0 && n.state == NodeState::Online)
                    .map(|(id, _)| *id)
            }
            AllocationPolicy::Interleave => {
                // Round-robin across nodes
                let node_ids: Vec<u32> = self.nodes.keys().copied().collect();
                if node_ids.is_empty() {
                    None
                } else {
                    let idx = (self.current_node as usize) % node_ids.len();
                    node_ids.get(idx).copied()
                }
            }
            AllocationPolicy::Preferred(node_id) => {
                if let Some(node) = self.nodes.get(&node_id) {
                    if node.free_memory > 0 && node.state == NodeState::Online {
                        return Some(node_id);
                    }
                }
                None
            }
            AllocationPolicy::Bind(node_id) => {
                if self.nodes.contains_key(&node_id) {
                    Some(node_id)
                } else {
                    None
                }
            }
        }
    }

    /// Allocate memory from a specific node
    pub fn allocate(&mut self, size: u64, node_id: Option<u32>) -> Result<*mut u8, &'static str> {
        let target_node =
            node_id.unwrap_or_else(|| self.select_node(None).unwrap_or(self.current_node));

        let node = self.nodes.get_mut(&target_node).ok_or("Node not found")?;

        if node.free_memory < size {
            return Err("Insufficient memory on node");
        }

        if node.state != NodeState::Online {
            return Err("Node not online");
        }

        node.free_memory -= size;

        // In a real implementation, this would return actual memory
        // For now, return a dummy pointer
        Ok(0x1000 as *mut u8)
    }

    /// Free memory to a specific node
    pub fn free(&mut self, size: u64, node_id: u32) -> Result<(), &'static str> {
        let node = self.nodes.get_mut(&node_id).ok_or("Node not found")?;

        node.free_memory += size;
        Ok(())
    }

    /// Get total memory across all nodes
    pub fn total_memory(&self) -> u64 {
        self.nodes.values().map(|n| n.memory_size).sum()
    }

    /// Get total free memory across all nodes
    pub fn total_free_memory(&self) -> u64 {
        self.nodes.values().map(|n| n.free_memory).sum()
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for NumaAllocator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut allocator = NumaAllocator::new();

        let node = NumaNode {
            node_id: 0,
            state: NodeState::Online,
            distance: vec![10, 20],
            memory_size: 1024 * 1024 * 1024, // 1GB
            free_memory: 1024 * 1024 * 1024,
            cpus: vec![0, 1, 2, 3],
        };

        allocator.add_node(node).unwrap();
        assert_eq!(allocator.node_count(), 1);
    }

    #[test]
    fn test_local_policy() {
        let mut allocator = NumaAllocator::new();

        let node = NumaNode {
            node_id: 0,
            state: NodeState::Online,
            distance: vec![10],
            memory_size: 1024,
            free_memory: 512,
            cpus: vec![0],
        };

        allocator.add_node(node).unwrap();
        let _ = allocator.set_current_node(0);

        let selected = allocator.select_node(Some(AllocationPolicy::Local));
        assert_eq!(selected, Some(0));
    }

    #[test]
    fn test_allocate() {
        let mut allocator = NumaAllocator::new();

        let node = NumaNode {
            node_id: 0,
            state: NodeState::Online,
            distance: vec![10],
            memory_size: 1024,
            free_memory: 512,
            cpus: vec![0],
        };

        allocator.add_node(node).unwrap();

        let result = allocator.allocate(256, Some(0));
        assert!(result.is_ok());

        let node = allocator.get_node(0).unwrap();
        assert_eq!(node.free_memory, 256);
    }

    #[test]
    fn test_free() {
        let mut allocator = NumaAllocator::new();

        let node = NumaNode {
            node_id: 0,
            state: NodeState::Online,
            distance: vec![10],
            memory_size: 1024,
            free_memory: 256,
            cpus: vec![0],
        };

        allocator.add_node(node).unwrap();

        allocator.free(256, 0).unwrap();

        let node = allocator.get_node(0).unwrap();
        assert_eq!(node.free_memory, 512);
    }

    #[test]
    fn test_total_memory() {
        let mut allocator = NumaAllocator::new();

        let node1 = NumaNode {
            node_id: 0,
            state: NodeState::Online,
            distance: vec![10, 20],
            memory_size: 1024,
            free_memory: 512,
            cpus: vec![0],
        };

        let node2 = NumaNode {
            node_id: 1,
            state: NodeState::Online,
            distance: vec![20, 10],
            memory_size: 2048,
            free_memory: 1024,
            cpus: vec![1],
        };

        allocator.add_node(node1).unwrap();
        allocator.add_node(node2).unwrap();

        assert_eq!(allocator.total_memory(), 3072);
        assert_eq!(allocator.total_free_memory(), 1536);
    }
}
