#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
//! Genode-style Component Tree Architecture for SigmaOS
//!
//! Implements recursive component ownership with parent-child resource delegation,
//! inspired by Genode OS framework. Provides hierarchical security isolation and
//! fine-grained privilege scoping through capability-based security.


extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Component identifier (Genode-style capability-based)
pub type ComponentId = usize;

/// Capability rights (Fuchsia/Genode-inspired)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityRights {
    pub can_read: bool,
    pub can_write: bool,
    pub can_execute: bool,
    pub can_delegate: bool,
    pub can_create_child: bool,
}

impl CapabilityRights {
    pub fn none() -> Self {
        CapabilityRights {
            can_read: false,
            can_write: false,
            can_execute: false,
            can_delegate: false,
            can_create_child: false,
        }
    }

    pub fn full() -> Self {
        CapabilityRights {
            can_read: true,
            can_write: true,
            can_execute: true,
            can_delegate: true,
            can_create_child: true,
        }
    }

    pub fn with_read(mut self) -> Self {
        self.can_read = true;
        self
    }

    pub fn with_write(mut self) -> Self {
        self.can_write = true;
        self
    }

    pub fn with_execute(mut self) -> Self {
        self.can_execute = true;
        self
    }

    pub fn with_delegate(mut self) -> Self {
        self.can_delegate = true;
        self
    }

    pub fn with_create_child(mut self) -> Self {
        self.can_create_child = true;
        self
    }
}

/// Capability handle (32-bit handle like Fuchsia)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityHandle {
    pub id: u32,
    pub rights: CapabilityRights,
}

impl CapabilityHandle {
    pub fn new(id: u32, rights: CapabilityRights) -> Self {
        CapabilityHandle { id, rights }
    }

    pub fn null() -> Self {
        CapabilityHandle {
            id: 0,
            rights: CapabilityRights::none(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// Component state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentState {
    Created = 0,
    Running = 1,
    Suspended = 2,
    Destroyed = 3,
}

/// Resource type for component delegation
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Memory = 0,
    Cpu = 1,
    IoPort = 2,
    Irq = 3,
    Capability = 4,
    Dma = 5,
}

/// Resource allocation for component
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ResourceAllocation {
    pub resource_type: ResourceType,
    pub amount: usize,
    pub start: usize,
    pub end: usize,
}

impl ResourceAllocation {
    pub fn new(resource_type: ResourceType, amount: usize, start: usize, end: usize) -> Self {
        ResourceAllocation {
            resource_type,
            amount,
            start,
            end,
        }
    }
}

/// Component in the tree (Genode-style)
#[repr(C)]
pub struct Component {
    pub id: ComponentId,
    pub name: String,
    pub state: ComponentState,
    pub parent: Option<ComponentId>,
    pub children: Vec<ComponentId>,
    pub capabilities: BTreeMap<u32, CapabilityRights>,
    pub resources: Vec<ResourceAllocation>,
    pub capability_space: BTreeMap<u32, CapabilityHandle>,
    pub next_capability_id: AtomicUsize,
}

impl Component {
    pub fn new(id: ComponentId, name: &str, parent: Option<ComponentId>) -> Self {
        let mut capabilities = BTreeMap::new();
        capabilities.insert(0, CapabilityRights::full());
        Component {
            id,
            name: String::from(name),
            state: ComponentState::Created,
            parent,
            children: Vec::new(),
            capabilities,
            resources: Vec::new(),
            capability_space: BTreeMap::new(),
            next_capability_id: AtomicUsize::new(1),
        }
    }

    /// Add a child component
    pub fn add_child(&mut self, child_id: ComponentId) {
        self.children.push(child_id);
    }

    /// Remove a child component
    pub fn remove_child(&mut self, child_id: ComponentId) {
        self.children.retain(|&id| id != child_id);
    }

    /// Allocate a capability to this component
    pub fn allocate_capability(&mut self, rights: CapabilityRights) -> CapabilityHandle {
        let id = self.next_capability_id.fetch_add(1, Ordering::SeqCst) as u32;
        let handle = CapabilityHandle::new(id, rights);
        self.capability_space.insert(id, handle);
        self.capabilities.insert(id, rights);
        handle
    }

    /// Delegate a capability to this component
    pub fn delegate_capability(&mut self, handle: CapabilityHandle) -> Result<(), ComponentError> {
        if !handle.rights.can_delegate {
            return Err(ComponentError::PermissionDenied);
        }
        self.capability_space.insert(handle.id, handle);
        self.capabilities.insert(handle.id, handle.rights);
        Ok(())
    }

    /// Revoke a capability from this component
    pub fn revoke_capability(&mut self, handle_id: u32) -> Result<(), ComponentError> {
        self.capability_space.remove(&handle_id);
        self.capabilities.remove(&handle_id);
        Ok(())
    }

    /// Check if component has specific capability rights
    pub fn has_capability_rights(&self, handle_id: u32, required_rights: CapabilityRights) -> bool {
        if let Some(&rights) = self.capabilities.get(&handle_id) {
            (!required_rights.can_read || rights.can_read)
                && (!required_rights.can_write || rights.can_write)
                && (!required_rights.can_execute || rights.can_execute)
                && (!required_rights.can_delegate || rights.can_delegate)
                && (!required_rights.can_create_child || rights.can_create_child)
        } else {
            false
        }
    }

    /// Allocate resource to component
    pub fn allocate_resource(
        &mut self,
        resource: ResourceAllocation,
    ) -> Result<(), ComponentError> {
        self.resources.push(resource);
        Ok(())
    }

    /// Get total resource usage
    pub fn get_resource_usage(&self, resource_type: ResourceType) -> usize {
        self.resources
            .iter()
            .filter(|r| r.resource_type == resource_type)
            .map(|r| r.amount)
            .sum()
    }
}

/// Component error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentError {
    Success = 0,
    NotFound = 1,
    PermissionDenied = 2,
    InvalidCapability = 3,
    ResourceExhausted = 4,
    ParentNotFound = 5,
    ChildNotFound = 6,
    AlreadyExists = 7,
}

/// Component tree manager (Genode-style hierarchical component system)
#[repr(C)]
pub struct ComponentTree {
    pub components: BTreeMap<ComponentId, Component>,
    pub root_id: ComponentId,
    pub next_component_id: AtomicUsize,
}

impl ComponentTree {
    pub fn new() -> Self {
        let root_id = 0;
        let mut components = BTreeMap::new();

        // Create root component
        let root = Component::new(root_id, "root", None);
        components.insert(root_id, root);

        ComponentTree {
            components,
            root_id,
            next_component_id: AtomicUsize::new(1),
        }
    }

    /// Create a new component as child of parent
    pub fn create_component(
        &mut self,
        parent_id: ComponentId,
        name: &str,
    ) -> Result<ComponentId, ComponentError> {
        // Check if parent exists
        if !self.components.contains_key(&parent_id) {
            return Err(ComponentError::ParentNotFound);
        }

        // Check parent has create_child permission
        let parent = self.components.get(&parent_id).unwrap();
        let parent_has_permission = parent
            .capabilities
            .values()
            .any(|&rights| rights.can_create_child);
        if parent_id != self.root_id && !parent_has_permission {
            return Err(ComponentError::PermissionDenied);
        }

        // Create new component
        let new_id = self.next_component_id.fetch_add(1, Ordering::SeqCst);
        let mut new_component = Component::new(new_id, name, Some(parent_id));
        new_component.allocate_capability(CapabilityRights::full());

        // Inherit some capabilities from parent (basic rights)
        if let Some(parent_component) = self.components.get(&parent_id) {
            for (&cap_id, &rights) in &parent_component.capabilities {
                if rights.can_delegate {
                    let inherited_rights = CapabilityRights {
                        can_read: rights.can_read,
                        can_write: false, // Don't inherit write by default
                        can_execute: rights.can_execute,
                        can_delegate: false, // Can't re-delegate inherited
                        can_create_child: false,
                    };
                    new_component
                        .delegate_capability(CapabilityHandle::new(cap_id, inherited_rights))
                        .ok();
                }
            }
        }

        // Add to parent's children
        if let Some(parent_component) = self.components.get_mut(&parent_id) {
            parent_component.add_child(new_id);
        }

        self.components.insert(new_id, new_component);
        Ok(new_id)
    }

    /// Destroy a component and all its children
    pub fn destroy_component(&mut self, component_id: ComponentId) -> Result<(), ComponentError> {
        if component_id == self.root_id {
            return Err(ComponentError::PermissionDenied); // Can't destroy root
        }

        let (children_to_destroy, parent_id) = {
            let component = self
                .components
                .get(&component_id)
                .ok_or(ComponentError::NotFound)?;
            (component.children.clone(), component.parent)
        };

        // Recursively destroy children
        for child_id in children_to_destroy {
            self.destroy_component(child_id).ok();
        }

        // Remove from parent's children
        if let Some(parent_id) = parent_id {
            if let Some(parent_component) = self.components.get_mut(&parent_id) {
                parent_component.remove_child(component_id);
            }
        }

        // Remove component
        self.components.remove(&component_id);
        Ok(())
    }

    /// Get component by ID
    pub fn get_component(&self, component_id: ComponentId) -> Result<&Component, ComponentError> {
        self.components
            .get(&component_id)
            .ok_or(ComponentError::NotFound)
    }

    /// Get mutable component by ID
    pub fn get_component_mut(
        &mut self,
        component_id: ComponentId,
    ) -> Result<&mut Component, ComponentError> {
        self.components
            .get_mut(&component_id)
            .ok_or(ComponentError::NotFound)
    }

    /// Allocate capability to component
    pub fn allocate_capability(
        &mut self,
        component_id: ComponentId,
        rights: CapabilityRights,
    ) -> Result<CapabilityHandle, ComponentError> {
        let component = self
            .components
            .get_mut(&component_id)
            .ok_or(ComponentError::NotFound)?;
        Ok(component.allocate_capability(rights))
    }

    /// Delegate capability from parent to child
    pub fn delegate_capability(
        &mut self,
        parent_id: ComponentId,
        child_id: ComponentId,
        handle: CapabilityHandle,
    ) -> Result<(), ComponentError> {
        let parent = self
            .components
            .get(&parent_id)
            .ok_or(ComponentError::ParentNotFound)?;

        // Check parent has capability in capability_space and has delegation rights
        if !parent.has_capability_rights(handle.id, CapabilityRights::none().with_delegate()) {
            return Err(ComponentError::PermissionDenied);
        }

        let child = self
            .components
            .get_mut(&child_id)
            .ok_or(ComponentError::ChildNotFound)?;
        child.delegate_capability(handle)
    }

    /// Find path from root to component
    pub fn get_component_path(
        &self,
        component_id: ComponentId,
    ) -> Result<Vec<String>, ComponentError> {
        let mut path = Vec::new();
        let mut current_id = component_id;

        loop {
            let component = self
                .components
                .get(&current_id)
                .ok_or(ComponentError::NotFound)?;
            path.insert(0, component.name.clone());

            match component.parent {
                Some(parent_id) => current_id = parent_id,
                None => break, // Reached root
            }
        }

        Ok(path)
    }

    /// Get all descendants of a component
    pub fn get_descendants(
        &self,
        component_id: ComponentId,
    ) -> Result<Vec<ComponentId>, ComponentError> {
        let component = self
            .components
            .get(&component_id)
            .ok_or(ComponentError::NotFound)?;
        let mut descendants = Vec::new();

        for &child_id in &component.children {
            descendants.push(child_id);
            if let Ok(child_descendants) = self.get_descendants(child_id) {
                descendants.extend(child_descendants);
            }
        }

        Ok(descendants)
    }

    /// Check resource availability in component tree
    pub fn check_resource_availability(
        &self,
        component_id: ComponentId,
        resource_type: ResourceType,
        required: usize,
    ) -> bool {
        if let Ok(component) = self.get_component(component_id) {
            let available = component.get_resource_usage(resource_type);
            available >= required
        } else {
            false
        }
    }

    /// Propagate resource limits from parent to children
    pub fn propagate_resource_limits(
        &mut self,
        parent_id: ComponentId,
    ) -> Result<(), ComponentError> {
        let (parent_resources, parent_children) = {
            let parent = self
                .components
                .get(&parent_id)
                .ok_or(ComponentError::NotFound)?;
            (parent.resources.clone(), parent.children.clone())
        };
        let children_count = parent_children.len().max(1);

        for child_id in parent_children {
            if let Some(child) = self.components.get_mut(&child_id) {
                // Distribute parent resources among children
                for resource in &parent_resources {
                    let child_allocation = ResourceAllocation::new(
                        resource.resource_type,
                        resource.amount / children_count,
                        resource.start,
                        resource.end,
                    );
                    child.allocate_resource(child_allocation).ok();
                }
            }
            self.propagate_resource_limits(child_id).ok();
        }

        Ok(())
    }
}

impl Default for ComponentTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_creation() {
        let mut tree = ComponentTree::new();
        let child_id = tree.create_component(0, "test_child").unwrap();
        assert!(tree.get_component(child_id).is_ok());
    }

    #[test]
    fn test_capability_delegation() {
        let mut tree = ComponentTree::new();
        let parent_id = tree.create_component(0, "parent").unwrap();
        tree.allocate_capability(parent_id, CapabilityRights::full())
            .unwrap();
        let child_id = tree.create_component(parent_id, "child").unwrap();

        let rights = CapabilityRights::full();
        let handle = tree.allocate_capability(parent_id, rights).unwrap();

        let result = tree.delegate_capability(parent_id, child_id, handle);
        assert!(result.is_ok());
    }

    #[test]
    fn test_resource_propagation() {
        let mut tree = ComponentTree::new();
        let parent_id = tree.create_component(0, "parent").unwrap();
        tree.allocate_capability(parent_id, CapabilityRights::full())
            .unwrap();
        let child_id = tree.create_component(parent_id, "child").unwrap();

        let resource = ResourceAllocation::new(ResourceType::Memory, 1024, 0, 1024);
        tree.get_component_mut(parent_id)
            .unwrap()
            .allocate_resource(resource)
            .unwrap();

        tree.propagate_resource_limits(parent_id).unwrap();

        let child = tree.get_component(child_id).unwrap();
        assert!(child.get_resource_usage(ResourceType::Memory) > 0);
    }
}
