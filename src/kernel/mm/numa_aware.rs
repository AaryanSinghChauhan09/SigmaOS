/// SigmaOS NUMA (Non-Uniform Memory Access) Topology manager
/// Tracks NUMA nodes and handles node-local memory allocation preferences

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NumaNode {
    pub id: u32,
    pub total_memory: u64,
    pub free_memory: u64,
    pub cpu_cores: Vec<u32>,
}

pub struct NumaTopologyManager {
    nodes: HashMap<u32, NumaNode>,
    cpu_to_node: HashMap<u32, u32>,
}

impl NumaTopologyManager {
    pub fn new() -> Self {
        NumaTopologyManager {
            nodes: HashMap::new(),
            cpu_to_node: HashMap::new(),
        }
    }

    pub fn register_node(&mut self, node: NumaNode) {
        for &cpu in &node.cpu_cores {
            self.cpu_to_node.insert(cpu, node.id);
        }
        self.nodes.insert(node.id, node);
    }

    pub fn get_node_for_cpu(&self, cpu: u32) -> Option<u32> {
        self.cpu_to_node.get(&cpu).copied()
    }

    pub fn allocate_node_local(&mut self, cpu: u32, size: u64) -> Result<u32, &'static str> {
        let node_id = self.get_node_for_cpu(cpu).unwrap_or(0);
        if let Some(node) = self.nodes.get_mut(&node_id) {
            if node.free_memory >= size {
                node.free_memory -= size;
                return Ok(node_id);
            }
        }

        // Fallback: Allocate from another node with free memory
        for node in self.nodes.values_mut() {
            if node.free_memory >= size {
                node.free_memory -= size;
                return Ok(node.id);
            }
        }

        Err("OOM: No NUMA node has sufficient memory")
    }

    pub fn free_node_memory(&mut self, node_id: u32, size: u64) {
        if let Some(node) = self.nodes.get_mut(&node_id) {
            node.free_memory += size;
        }
    }
}

impl Default for NumaTopologyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numa_topology() {
        let mut ntm = NumaTopologyManager::new();
        ntm.register_node(NumaNode {
            id: 0,
            total_memory: 1024 * 1024 * 1024,
            free_memory: 1024 * 1024 * 1024,
            cpu_cores: vec![0, 1],
        });
        ntm.register_node(NumaNode {
            id: 1,
            total_memory: 1024 * 1024 * 1024,
            free_memory: 1024 * 1024 * 1024,
            cpu_cores: vec![2, 3],
        });

        assert_eq!(ntm.get_node_for_cpu(2), Some(1));
        
        let alloc_node = ntm.allocate_node_local(2, 512).unwrap();
        assert_eq!(alloc_node, 1);
    }
}
