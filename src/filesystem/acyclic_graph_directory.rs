// SigmaOS Sovereign Directed Acyclic Graph (DAG) Directory Engine
// Inspired by Linux filesystem DAG traversals (fs/dcache.c), NixOS content-addressable store DAGs,
// Plan 9 bind mount graph namespaces, and DragonFly BSD directory firmlinks.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DagNodeType {
    Directory,
    RegularFile,
    Symlink,
    BindMount,
}

#[derive(Debug, Clone)]
pub struct DagNode {
    pub node_id: u64,
    pub name: String,
    pub node_type: DagNodeType,
    pub content_hash: String,
    pub reference_count: u32,
}

#[derive(Debug, Clone)]
pub struct DagEdge {
    pub parent_id: u64,
    pub child_id: u64,
    pub edge_label: String,
}

#[derive(Debug, Clone, Default)]
pub struct SovereignAcyclicGraphDirectoryEngine {
    pub nodes: BTreeMap<u64, DagNode>,
    pub parent_to_children: BTreeMap<u64, Vec<u64>>,
    pub child_to_parents: BTreeMap<u64, Vec<u64>>,
    pub edge_labels: BTreeMap<(u64, u64), String>,
    next_node_id: u64,
}

impl SovereignAcyclicGraphDirectoryEngine {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            parent_to_children: BTreeMap::new(),
            child_to_parents: BTreeMap::new(),
            edge_labels: BTreeMap::new(),
            next_node_id: 1,
        }
    }

    /// Adds a new directory or file node to the graph
    pub fn create_node(&mut self, name: &str, node_type: DagNodeType, content_hash: &str) -> u64 {
        let node_id = self.next_node_id;
        self.next_node_id += 1;

        let node = DagNode {
            node_id,
            name: name.to_string(),
            node_type,
            content_hash: content_hash.to_string(),
            reference_count: 0,
        };

        self.nodes.insert(node_id, node);
        self.parent_to_children.insert(node_id, Vec::new());
        self.child_to_parents.insert(node_id, Vec::new());

        node_id
    }

    /// Connects parent directory node to child node with cycle detection
    pub fn link_child(&mut self, parent_id: u64, child_id: u64, label: &str) -> Result<(), String> {
        if !self.nodes.contains_key(&parent_id) {
            return Err(format!("ENOENT: Parent node {} not found", parent_id));
        }
        if !self.nodes.contains_key(&child_id) {
            return Err(format!("ENOENT: Child node {} not found", child_id));
        }

        // Cycle Detection: DFS check if adding parent -> child creates a cycle
        if self.path_exists(child_id, parent_id) {
            return Err(format!(
                "ELOOP: Adding edge {} -> {} would create a directory cycle in the DAG",
                parent_id, child_id
            ));
        }

        // Add parent -> child mapping
        if let Some(children) = self.parent_to_children.get_mut(&parent_id) {
            if !children.contains(&child_id) {
                children.push(child_id);
            }
        }

        // Add child -> parent mapping (Multi-parent support for DAG)
        if let Some(parents) = self.child_to_parents.get_mut(&child_id) {
            if !parents.contains(&parent_id) {
                parents.push(parent_id);
            }
        }

        self.edge_labels
            .insert((parent_id, child_id), label.to_string());

        // Increment child reference count
        if let Some(child_node) = self.nodes.get_mut(&child_id) {
            child_node.reference_count += 1;
        }

        Ok(())
    }

    /// DFS helper to check if a directed path exists from start to target
    pub fn path_exists(&self, start_id: u64, target_id: u64) -> bool {
        if start_id == target_id {
            return true;
        }

        let mut visited = BTreeSet::new();
        let mut stack = vec![start_id];

        while let Some(current) = stack.pop() {
            if current == target_id {
                return true;
            }

            if visited.insert(current) {
                if let Some(children) = self.parent_to_children.get(&current) {
                    for &child in children {
                        if !visited.contains(&child) {
                            stack.push(child);
                        }
                    }
                }
            }
        }

        false
    }

    /// Computes Topological Sort of the directory DAG (Kahn's Algorithm)
    pub fn topological_sort(&self) -> Result<Vec<u64>, String> {
        let mut in_degrees: BTreeMap<u64, u32> = BTreeMap::new();
        for &node_id in self.nodes.keys() {
            let parent_count = self.child_to_parents.get(&node_id).map_or(0, |p| p.len() as u32);
            in_degrees.insert(node_id, parent_count);
        }

        let mut queue = VecDeque::new();
        for (&node_id, &degree) in &in_degrees {
            if degree == 0 {
                queue.push_back(node_id);
            }
        }

        let mut order = Vec::new();

        while let Some(node_id) = queue.pop_front() {
            order.push(node_id);

            if let Some(children) = self.parent_to_children.get(&node_id) {
                for &child_id in children {
                    if let Some(degree) = in_degrees.get_mut(&child_id) {
                        *degree = degree.saturating_sub(1);
                        if *degree == 0 {
                            queue.push_back(child_id);
                        }
                    }
                }
            }
        }

        if order.len() != self.nodes.len() {
            return Err("ELOOP: Cycle detected during topological sort".to_string());
        }

        Ok(order)
    }

    /// Returns all distinct paths from root to target node in the DAG
    pub fn find_all_paths(&self, root_id: u64, target_id: u64) -> Vec<Vec<String>> {
        let mut all_paths = Vec::new();
        let mut current_path = Vec::new();

        if let Some(root_node) = self.nodes.get(&root_id) {
            current_path.push(root_node.name.clone());
            self.dfs_find_paths(root_id, target_id, &mut current_path, &mut all_paths);
        }

        all_paths
    }

    fn dfs_find_paths(
        &self,
        current_id: u64,
        target_id: u64,
        current_path: &mut Vec<String>,
        all_paths: &mut Vec<Vec<String>>,
    ) {
        if current_id == target_id {
            all_paths.push(current_path.clone());
            return;
        }

        if let Some(children) = self.parent_to_children.get(&current_id) {
            for &child_id in children {
                if let Some(label) = self.edge_labels.get(&(current_id, child_id)) {
                    current_path.push(label.clone());
                    self.dfs_find_paths(child_id, target_id, current_path, all_paths);
                    current_path.pop();
                }
            }
        }
    }

    /// Unlinks child from parent, updating reference count
    pub fn unlink_child(&mut self, parent_id: u64, child_id: u64) -> Result<(), String> {
        if let Some(children) = self.parent_to_children.get_mut(&parent_id) {
            children.retain(|&id| id != child_id);
        }

        if let Some(parents) = self.child_to_parents.get_mut(&child_id) {
            parents.retain(|&id| id != parent_id);
        }

        self.edge_labels.remove(&(parent_id, child_id));

        if let Some(child_node) = self.nodes.get_mut(&child_id) {
            child_node.reference_count = child_node.reference_count.saturating_sub(1);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acyclic_graph_directory_creation_and_cycle_prevention() {
        let mut dag = SovereignAcyclicGraphDirectoryEngine::new();

        let root = dag.create_node("/", DagNodeType::Directory, "hash_root");
        let usr = dag.create_node("usr", DagNodeType::Directory, "hash_usr");
        let bin = dag.create_node("bin", DagNodeType::Directory, "hash_bin");

        // Link root -> usr and usr -> bin
        assert!(dag.link_child(root, usr, "usr").is_ok());
        assert!(dag.link_child(usr, bin, "bin").is_ok());

        // Attempt cycle: bin -> root MUST fail with ELOOP
        let cycle_res = dag.link_child(bin, root, "root_loop");
        assert!(cycle_res.is_err());
        assert!(cycle_res.unwrap_err().contains("ELOOP"));
    }

    #[test]
    fn test_multi_parent_dag_and_path_finding() {
        let mut dag = SovereignAcyclicGraphDirectoryEngine::new();

        let root = dag.create_node("/", DagNodeType::Directory, "hash_root");
        let opt = dag.create_node("opt", DagNodeType::Directory, "hash_opt");
        let shared_lib = dag.create_node("libssl.so", DagNodeType::RegularFile, "hash_ssl");

        // Root -> opt -> libssl.so AND Root -> libssl.so (Multi-parent DAG node)
        dag.link_child(root, opt, "opt").unwrap();
        dag.link_child(opt, shared_lib, "libssl.so").unwrap();
        dag.link_child(root, shared_lib, "libssl.so").unwrap();

        assert_eq!(dag.nodes.get(&shared_lib).unwrap().reference_count, 2);

        let paths = dag.find_all_paths(root, shared_lib);
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_topological_sort() {
        let mut dag = SovereignAcyclicGraphDirectoryEngine::new();

        let n1 = dag.create_node("node1", DagNodeType::Directory, "h1");
        let n2 = dag.create_node("node2", DagNodeType::Directory, "h2");
        let n3 = dag.create_node("node3", DagNodeType::Directory, "h3");

        dag.link_child(n1, n2, "link12").unwrap();
        dag.link_child(n2, n3, "link23").unwrap();

        let order = dag.topological_sort().unwrap();
        assert_eq!(order, vec![n1, n2, n3]);
    }
}
