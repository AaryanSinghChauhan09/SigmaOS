// SPDX-License-Identifier: MIT
// SigmaOS Kobject Subsystem
// Kernel object management inspired by Linux kobject and sysfs

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};

/// Kobject type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KObjectType {
    Directory,
    File,
    Symlink,
    Device,
    Attribute,
}

/// Kobject state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KObjectState {
    Initialized,
    Added,
    Removed,
}

/// Kobject ID
pub type KObjectId = u64;

/// Kobject attribute
#[derive(Debug, Clone)]
pub struct KObjectAttribute {
    pub name: String,
    pub value: String,
    pub mode: u32, // File mode bits
}

impl KObjectAttribute {
    pub fn new(name: String, value: String, mode: u32) -> Self {
        KObjectAttribute {
            name,
            value,
            mode,
        }
    }
}

/// Kobject
#[derive(Debug)]
pub struct KObject {
    pub id: KObjectId,
    pub name: String,
    pub obj_type: KObjectType,
    pub state: AtomicU32, // KObjectState as u32
    pub parent_id: Option<KObjectId>,
    pub children: Vec<KObjectId>,
    pub attributes: BTreeMap<String, KObjectAttribute>,
    pub ref_count: AtomicU32,
}

impl KObject {
    pub fn new(id: KObjectId, name: String, obj_type: KObjectType, parent_id: Option<KObjectId>) -> Self {
        KObject {
            id,
            name,
            obj_type,
            state: AtomicU32::new(KObjectState::Initialized as u32),
            parent_id,
            children: Vec::new(),
            attributes: BTreeMap::new(),
            ref_count: AtomicU32::new(1),
        }
    }

    pub fn get_state(&self) -> KObjectState {
        match self.state.load(Ordering::SeqCst) {
            0 => KObjectState::Initialized,
            1 => KObjectState::Added,
            2 => KObjectState::Removed,
            _ => KObjectState::Initialized,
        }
    }

    pub fn set_state(&self, state: KObjectState) {
        self.state.store(state as u32, Ordering::SeqCst);
    }

    pub fn add_attribute(&mut self, attr: KObjectAttribute) {
        self.attributes.insert(attr.name.clone(), attr);
    }

    pub fn get_attribute(&self, name: &str) -> Option<&KObjectAttribute> {
        self.attributes.get(name)
    }

    pub fn set_attribute(&mut self, name: String, value: String, mode: u32) {
        let attr = KObjectAttribute::new(name.clone(), value, mode);
        self.attributes.insert(name, attr);
    }

    pub fn add_child(&mut self, child_id: KObjectId) {
        self.children.push(child_id);
    }

    pub fn remove_child(&mut self, child_id: KObjectId) {
        self.children.retain(|&id| id != child_id);
    }

    pub fn get_refcount(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    pub fn increment_refcount(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn decrement_refcount(&self) -> u32 {
        self.ref_count.fetch_sub(1, Ordering::SeqCst) - 1
    }
}

/// Kobject subsystem
#[derive(Debug)]
pub struct KObjectSubsystem {
    kobjects: BTreeMap<KObjectId, KObject>,
    next_kobject_id: AtomicU64,
    root_kobject_id: KObjectId,
}

impl KObjectSubsystem {
    pub fn new() -> Self {
        let root_id = 1;
        let mut kobjects = BTreeMap::new();
        
        let root = KObject::new(root_id, "/".to_string(), KObjectType::Directory, None);
        root.set_state(KObjectState::Added);
        kobjects.insert(root_id, root);

        KObjectSubsystem {
            kobjects,
            next_kobject_id: AtomicU64::new(2),
            root_kobject_id: root_id,
        }
    }

    /// Create a new kobject
    pub fn create_kobject(&mut self, name: String, obj_type: KObjectType, parent_id: Option<KObjectId>) -> Result<KObjectId, &'static str> {
        let parent = match parent_id {
            Some(id) => self.kobjects.get(&id).ok_or("Parent kobject not found")?,
            None => self.kobjects.get(&self.root_kobject_id).ok_or("Root kobject not found")?,
        };

        let id = self.next_kobject_id.fetch_add(1, Ordering::SeqCst);
        let kobject = KObject::new(id, name, obj_type, parent_id.or(Some(self.root_kobject_id)));
        
        if let Some(pid) = parent_id {
            if let Some(parent) = self.kobjects.get_mut(&pid) {
                parent.add_child(id);
            }
        }

        self.kobjects.insert(id, kobject);
        Ok(id)
    }

    /// Get kobject by ID
    pub fn get_kobject(&self, id: KObjectId) -> Option<&KObject> {
        self.kobjects.get(&id)
    }

    /// Get mutable kobject by ID
    pub fn get_kobject_mut(&mut self, id: KObjectId) -> Option<&mut KObject> {
        self.kobjects.get_mut(&id)
    }

    /// Add kobject to hierarchy
    pub fn add_kobject(&mut self, id: KObjectId) -> Result<(), &'static str> {
        let kobject = self.kobjects.get_mut(&id).ok_or("Kobject not found")?;
        kobject.set_state(KObjectState::Added);
        Ok(())
    }

    /// Remove kobject from hierarchy
    pub fn remove_kobject(&mut self, id: KObjectId) -> Result<(), &'static str> {
        if id == self.root_kobject_id {
            return Err("Cannot remove root kobject");
        }

        let kobject = self.kobjects.get(&id).ok_or("Kobject not found")?;
        
        if !kobject.children.is_empty() {
            return Err("Cannot remove kobject with children");
        }

        if kobject.get_refcount() > 1 {
            return Err("Cannot remove kobject with active references");
        }

        if let Some(parent_id) = kobject.parent_id {
            if let Some(parent) = self.kobjects.get_mut(&parent_id) {
                parent.remove_child(id);
            }
        }

        if let Some(kobject) = self.kobjects.get_mut(&id) {
            kobject.set_state(KObjectState::Removed);
        }

        self.kobjects.remove(&id);
        Ok(())
    }

    /// Find kobject by path
    pub fn find_by_path(&self, path: &str) -> Option<KObjectId> {
        if path == "/" {
            return Some(self.root_kobject_id);
        }

        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current_id = self.root_kobject_id;

        for part in parts {
            let current = self.kobjects.get(&current_id)?;
            let mut found = None;
            
            for &child_id in &current.children {
                if let Some(child) = self.kobjects.get(&child_id) {
                    if child.name == part {
                        found = Some(child_id);
                        break;
                    }
                }
            }
            
            current_id = found?;
        }

        Some(current_id)
    }

    /// Get kobject count
    pub fn kobject_count(&self) -> usize {
        self.kobjects.len()
    }

    /// Get root kobject ID
    pub fn root_kobject_id(&self) -> KObjectId {
        self.root_kobject_id
    }
}

impl Default for KObjectSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kobject_creation() {
        let mut subsystem = KObjectSubsystem::new();
        
        let id = subsystem.create_kobject("test".to_string(), KObjectType::Directory, None).unwrap();
        assert!(id > 1);
        assert_eq!(subsystem.kobject_count(), 2);
    }

    #[test]
    fn test_kobject_hierarchy() {
        let mut subsystem = KObjectSubsystem::new();
        
        let parent_id = subsystem.create_kobject("parent".to_string(), KObjectType::Directory, None).unwrap();
        let child_id = subsystem.create_kobject("child".to_string(), KObjectType::File, Some(parent_id)).unwrap();
        
        let parent = subsystem.get_kobject(parent_id).unwrap();
        assert!(parent.children.contains(&child_id));
    }

    #[test]
    fn test_kobject_attributes() {
        let mut subsystem = KObjectSubsystem::new();
        
        let id = subsystem.create_kobject("test".to_string(), KObjectType::File, None).unwrap();
        let kobject = subsystem.get_kobject_mut(id).unwrap();
        
        kobject.add_attribute(KObjectAttribute::new("attr1".to_string(), "value1".to_string(), 0o644));
        assert!(kobject.get_attribute("attr1").is_some());
    }

    #[test]
    fn test_kobject_refcount() {
        let mut subsystem = KObjectSubsystem::new();
        
        let id = subsystem.create_kobject("test".to_string(), KObjectType::File, None).unwrap();
        let kobject = subsystem.get_kobject(id).unwrap();
        
        assert_eq!(kobject.get_refcount(), 1);
        kobject.increment_refcount();
        assert_eq!(kobject.get_refcount(), 2);
    }

    #[test]
    fn test_kobject_path_lookup() {
        let mut subsystem = KObjectSubsystem::new();
        
        let parent_id = subsystem.create_kobject("parent".to_string(), KObjectType::Directory, None).unwrap();
        subsystem.add_kobject(parent_id).unwrap();
        let child_id = subsystem.create_kobject("child".to_string(), KObjectType::File, Some(parent_id)).unwrap();
        subsystem.add_kobject(child_id).unwrap();
        
        let found_id = subsystem.find_by_path("/parent/child");
        assert_eq!(found_id, Some(child_id));
    }

    #[test]
    fn test_kobject_removal() {
        let mut subsystem = KObjectSubsystem::new();
        
        let id = subsystem.create_kobject("test".to_string(), KObjectType::File, None).unwrap();
        subsystem.remove_kobject(id).unwrap();
        
        assert_eq!(subsystem.kobject_count(), 1);
    }
}
