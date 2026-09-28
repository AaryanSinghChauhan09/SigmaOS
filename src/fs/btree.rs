// B-tree implementation for Btrfs filesystem
// Inspired by Linux Btrfs B-tree structures

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// B-tree node type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BTreeNodeType {
    Internal,
    Leaf,
}

/// B-tree key
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BTreeKey {
    pub objectid: u64,
    pub type_id: u8,
    pub offset: u64,
}

/// B-tree item
#[derive(Debug, Clone)]
pub struct BTreeItem {
    pub key: BTreeKey,
    pub data: Vec<u8>,
}

/// B-tree node
#[derive(Debug, Clone)]
pub struct BTreeNode {
    pub id: u64,
    pub node_type: BTreeNodeType,
    pub keys: Vec<BTreeKey>,
    pub items: Vec<BTreeItem>,
    pub children: Vec<u64>,
    pub parent: Option<u64>,
}

/// B-tree
pub struct BTree {
    next_node_id: AtomicU64,
    root_id: Option<u64>,
    nodes: HashMap<u64, BTreeNode>,
    max_keys: usize,
    min_keys: usize,
}

impl BTree {
    pub fn new(max_keys: usize) -> Self {
        let min_keys = max_keys / 2;
        
        Self {
            next_node_id: AtomicU64::new(1),
            root_id: None,
            nodes: HashMap::new(),
            max_keys,
            min_keys,
        }
    }

    /// Create a new node
    fn create_node(&self, node_type: BTreeNodeType) -> BTreeNode {
        let id = self.next_node_id.fetch_add(1, Ordering::SeqCst);
        
        BTreeNode {
            id,
            node_type,
            keys: Vec::new(),
            items: Vec::new(),
            children: Vec::new(),
            parent: None,
        }
    }

    /// Insert a key-value pair
    pub fn insert(&mut self, key: BTreeKey, data: Vec<u8>) -> Result<(), &'static str> {
        if self.root_id.is_none() {
            // Create root leaf node
            let root = self.create_node(BTreeNodeType::Leaf);
            let root_id = root.id;
            self.nodes.insert(root_id, root);
            self.root_id = Some(root_id);
        }

        let root_id = self.root_id.unwrap();
        self.insert_into_node(root_id, key, data)
    }

    /// Insert into a node
    fn insert_into_node(&mut self, node_id: u64, key: BTreeKey, data: Vec<u8>) -> Result<(), &'static str> {
        let node = self.nodes.get(&node_id).cloned().ok_or("Node not found")?;
        
        match node.node_type {
            BTreeNodeType::Leaf => {
                self.insert_into_leaf(node_id, key, data)
            }
            BTreeNodeType::Internal => {
                self.insert_into_internal(node_id, key, data)
            }
        }
    }

    /// Insert into leaf node
    fn insert_into_leaf(&mut self, node_id: u64, key: BTreeKey, data: Vec<u8>) -> Result<(), &'static str> {
        let node = self.nodes.get_mut(&node_id).ok_or("Node not found")?;
        
        // Find insertion position
        let pos = node.keys.binary_search(&key).unwrap_or_else(|e| e);
        
        // Insert key and item
        node.keys.insert(pos, key);
        node.items.insert(pos, BTreeItem { key, data });
        
        // Check if node needs to split
        if node.keys.len() > self.max_keys {
            self.split_leaf(node_id);
        }
        
        Ok(())
    }

    /// Insert into internal node
    fn insert_into_internal(&mut self, node_id: u64, key: BTreeKey, data: Vec<u8>) -> Result<(), &'static str> {
        let node = self.nodes.get(&node_id).cloned().ok_or("Node not found")?;
        
        // Find child to descend into
        let pos = node.keys.binary_search(&key).unwrap_or_else(|e| e);
        let child_id = if pos < node.children.len() {
            node.children[pos]
        } else {
            node.children[node.children.len() - 1]
        };
        
        self.insert_into_node(child_id, key, data)
    }

    /// Split a leaf node
    fn split_leaf(&mut self, node_id: u64) {
        let node = self.nodes.get(&node_id).cloned().unwrap();
        let mid = node.keys.len() / 2;
        
        // Create new leaf node
        let mut new_node = self.create_node(BTreeNodeType::Leaf);
        new_node.keys = node.keys[mid..].to_vec();
        new_node.items = node.items[mid..].to_vec();
        new_node.parent = node.parent;
        
        let new_node_id = new_node.id;
        self.nodes.insert(new_node_id, new_node);
        
        // Update original node
        if let Some(original) = self.nodes.get_mut(&node_id) {
            original.keys.truncate(mid);
            original.items.truncate(mid);
        }
        
        // Insert separator key into parent
        let separator_key = node.keys[mid];
        self.insert_separator(node.parent, node_id, new_node_id, separator_key);
    }

    /// Insert separator key into parent
    fn insert_separator(&mut self, parent_id: Option<u64>, left_id: u64, right_id: u64, key: BTreeKey) {
        if let Some(pid) = parent_id {
            if let Some(parent) = self.nodes.get_mut(&pid) {
                let pos = parent.keys.binary_search(&key).unwrap_or_else(|e| e);
                parent.keys.insert(pos, key);
                parent.children.insert(pos + 1, right_id);
                
                // Update parent references
                if let Some(left) = self.nodes.get_mut(&left_id) {
                    left.parent = Some(pid);
                }
                if let Some(right) = self.nodes.get_mut(&right_id) {
                    right.parent = Some(pid);
                }
                
                // Check if parent needs to split
                if parent.keys.len() > self.max_keys {
                    self.split_internal(pid);
                }
            }
        } else {
            // Create new root
            let mut new_root = self.create_node(BTreeNodeType::Internal);
            new_root.keys.push(key);
            new_root.children.push(left_id);
            new_root.children.push(right_id);
            
            let new_root_id = new_root.id;
            self.nodes.insert(new_root_id, new_root);
            self.root_id = Some(new_root_id);
            
            // Update parent references
            if let Some(left) = self.nodes.get_mut(&left_id) {
                left.parent = Some(new_root_id);
            }
            if let Some(right) = self.nodes.get_mut(&right_id) {
                right.parent = Some(new_root_id);
            }
        }
    }

    /// Split internal node
    fn split_internal(&mut self, node_id: u64) {
        let node = self.nodes.get(&node_id).cloned().unwrap();
        let mid = node.keys.len() / 2;
        
        // Create new internal node
        let mut new_node = self.create_node(BTreeNodeType::Internal);
        new_node.keys = node.keys[mid + 1..].to_vec();
        new_node.children = node.children[mid + 1..].to_vec();
        new_node.parent = node.parent;
        
        let new_node_id = new_node.id;
        self.nodes.insert(new_node_id, new_node);
        
        let separator_key = node.keys[mid];
        
        // Update original node
        if let Some(original) = self.nodes.get_mut(&node_id) {
            original.keys.truncate(mid);
            original.children.truncate(mid + 1);
        }
        
        // Insert separator into parent
        self.insert_separator(node.parent, node_id, new_node_id, separator_key);
    }

    /// Search for a key
    pub fn search(&self, key: BTreeKey) -> Option<Vec<u8>> {
        let root_id = self.root_id?;
        self.search_in_node(root_id, key)
    }

    /// Search in a node
    fn search_in_node(&self, node_id: u64, key: BTreeKey) -> Option<Vec<u8>> {
        let node = self.nodes.get(&node_id)?;
        
        match node.node_type {
            BTreeNodeType::Leaf => {
                let pos = node.keys.binary_search(&key).ok()?;
                Some(node.items[pos].data.clone())
            }
            BTreeNodeType::Internal => {
                let pos = node.keys.binary_search(&key).unwrap_or_else(|e| e);
                let child_id = if pos < node.children.len() {
                    node.children[pos]
                } else {
                    node.children[node.children.len() - 1]
                };
                self.search_in_node(child_id, key)
            }
        }
    }

    /// Get node count
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Get root ID
    pub fn root_id(&self) -> Option<u64> {
        self.root_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_insert() {
        let mut btree = BTree::new(4);
        
        let key = BTreeKey {
            objectid: 1,
            type_id: 1,
            offset: 0,
        };
        
        assert!(btree.insert(key, vec![1, 2, 3]).is_ok());
        assert_eq!(btree.node_count(), 1);
    }

    #[test]
    fn test_btree_search() {
        let mut btree = BTree::new(4);
        
        let key = BTreeKey {
            objectid: 1,
            type_id: 1,
            offset: 0,
        };
        
        btree.insert(key, vec![1, 2, 3]).unwrap();
        
        let result = btree.search(key);
        assert_eq!(result, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_btree_split() {
        let mut btree = BTree::new(4);
        
        // Insert enough keys to trigger split
        for i in 0..5 {
            let key = BTreeKey {
                objectid: i,
                type_id: 1,
                offset: 0,
            };
            btree.insert(key, vec![i as u8]).unwrap();
        }
        
        assert!(btree.node_count() > 1);
    }
}
