#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
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
//! Genode-style Component Tree Architecture for SigmaOS
//!
//! Implements recursive component ownership with parent-child resource delegation,
//! inspired by Genode OS framework. Provides hierarchical security isolation and
//! fine-grained privilege scoping through capability-based security.

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;
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

// ============================================================================
// FEDORA-INSPIRED COMPONENT SYSTEM INNOVATIONS
// ============================================================================

/// Fedora `comps.xml` style requirement level for component group inclusion
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompsGroupRequirement {
    Mandatory = 0,   // Must be installed for group functionality
    Default = 1,     // Installed by default unless excluded
    Optional = 2,    // Additional opt-in components
    Conditional = 3, // Dynamically required if condition is met
}

/// AppStream Metadata for component presentation in GUI software catalogs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppStreamMetadata {
    pub component_id: String,
    pub name: String,
    pub summary: String,
    pub icon_name: String,
    pub developer_name: String,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
}

/// Fedora `comps.xml` Component Group definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FedoraComponentGroup {
    pub group_id: String,
    pub name: String,
    pub description: String,
    pub components: BTreeMap<String, CompsGroupRequirement>,
    pub appstream_meta: Option<AppStreamMetadata>,
}

/// Engine managing Fedora `comps.xml` component groups and AppStream metadata
pub struct FedoraCompsComponentGroupEngine {
    pub groups: BTreeMap<String, FedoraComponentGroup>,
}

impl FedoraCompsComponentGroupEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            groups: BTreeMap::new(),
        };
        engine.register_default_comps_groups();
        engine
    }

    fn register_default_comps_groups(&mut self) {
        let mut base_components = BTreeMap::new();
        base_components.insert("systemd".to_string(), CompsGroupRequirement::Mandatory);
        base_components.insert("kernel-core".to_string(), CompsGroupRequirement::Mandatory);
        base_components.insert("glibc".to_string(), CompsGroupRequirement::Mandatory);
        base_components.insert("selinux-policy".to_string(), CompsGroupRequirement::Default);
        base_components.insert(
            "bash-completion".to_string(),
            CompsGroupRequirement::Optional,
        );

        self.groups.insert(
            "core".to_string(),
            FedoraComponentGroup {
                group_id: "core".to_string(),
                name: "Core System".to_string(),
                description: "Essential OS core runtime components".to_string(),
                components: base_components,
                appstream_meta: Some(AppStreamMetadata {
                    component_id: "org.sigmaos.core".to_string(),
                    name: "SigmaOS Core".to_string(),
                    summary: "Core system components".to_string(),
                    icon_name: "system-run".to_string(),
                    developer_name: "SigmaOS Project".to_string(),
                    categories: vec!["System".to_string()],
                    keywords: vec!["core".to_string(), "base".to_string()],
                }),
            },
        );
    }

    pub fn register_group(&mut self, group: FedoraComponentGroup) {
        self.groups.insert(group.group_id.clone(), group);
    }

    pub fn get_mandatory_components(&self, group_id: &str) -> Vec<String> {
        if let Some(group) = self.groups.get(group_id) {
            group
                .components
                .iter()
                .filter(|(_, &req)| req == CompsGroupRequirement::Mandatory)
                .map(|(name, _)| name.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_default_components(&self, group_id: &str) -> Vec<String> {
        if let Some(group) = self.groups.get(group_id) {
            group
                .components
                .iter()
                .filter(|(_, &req)| {
                    req == CompsGroupRequirement::Mandatory || req == CompsGroupRequirement::Default
                })
                .map(|(name, _)| name.clone())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn generate_comps_xml(&self) -> String {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<comps>\n");
        for group in self.groups.values() {
            xml.push_str("  <group>\n");
            xml.push_str(&format!("    <id>{}</id>\n", group.group_id));
            xml.push_str(&format!("    <name>{}</name>\n", group.name));
            xml.push_str(&format!(
                "    <description>{}</description>\n",
                group.description
            ));
            xml.push_str("    <packagelist>\n");
            for (comp, req) in &group.components {
                let req_type = match req {
                    CompsGroupRequirement::Mandatory => "mandatory",
                    CompsGroupRequirement::Default => "default",
                    CompsGroupRequirement::Optional => "optional",
                    CompsGroupRequirement::Conditional => "conditional",
                };
                xml.push_str(&format!(
                    "      <packagereq type=\"{}\">{}</packagereq>\n",
                    req_type, comp
                ));
            }
            xml.push_str("    </packagelist>\n  </group>\n");
        }
        xml.push_str("</comps>");
        xml
    }
}

impl Default for FedoraCompsComponentGroupEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Fedora Modularity (`modulemd.yaml`) stream profile
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulemdProfile {
    pub profile_name: String,
    pub components: Vec<String>,
}

/// Fedora Modularity stream definition
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulemdStream {
    pub module_name: String,
    pub stream_version: String,
    pub summary: String,
    pub profiles: BTreeMap<String, ModulemdProfile>,
    pub api_surface: Vec<String>,
    pub artifact_mapping: BTreeMap<String, String>, // Component -> Store Hash/Path
}

/// Engine managing Fedora Modularity streams, profiles, and API surface filtering
pub struct FedoraModulemdComponentEngine {
    pub streams: BTreeMap<String, BTreeMap<String, ModulemdStream>>, // Module -> (StreamVersion -> Stream)
    pub active_stream_selections: BTreeMap<String, String>, // Module -> Selected StreamVersion
}

impl FedoraModulemdComponentEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            streams: BTreeMap::new(),
            active_stream_selections: BTreeMap::new(),
        };
        engine.register_default_modules();
        engine
    }

    fn register_default_modules(&mut self) {
        let mut node18_profiles = BTreeMap::new();
        node18_profiles.insert(
            "default".to_string(),
            ModulemdProfile {
                profile_name: "default".to_string(),
                components: vec!["nodejs-18.18.0".to_string(), "npm-9.8.1".to_string()],
            },
        );
        node18_profiles.insert(
            "development".to_string(),
            ModulemdProfile {
                profile_name: "development".to_string(),
                components: vec![
                    "nodejs-18.18.0".to_string(),
                    "npm-9.8.1".to_string(),
                    "nodejs-devel-18.18.0".to_string(),
                ],
            },
        );

        let stream18 = ModulemdStream {
            module_name: "nodejs".to_string(),
            stream_version: "18".to_string(),
            summary: "Node.js JavaScript runtime stream 18".to_string(),
            profiles: node18_profiles,
            api_surface: vec!["node".to_string(), "npm".to_string(), "npx".to_string()],
            artifact_mapping: BTreeMap::new(),
        };

        let mut node20_profiles = BTreeMap::new();
        node20_profiles.insert(
            "default".to_string(),
            ModulemdProfile {
                profile_name: "default".to_string(),
                components: vec!["nodejs-20.9.0".to_string(), "npm-10.1.0".to_string()],
            },
        );

        let stream20 = ModulemdStream {
            module_name: "nodejs".to_string(),
            stream_version: "20".to_string(),
            summary: "Node.js JavaScript runtime stream 20".to_string(),
            profiles: node20_profiles,
            api_surface: vec!["node".to_string(), "npm".to_string(), "npx".to_string()],
            artifact_mapping: BTreeMap::new(),
        };

        let mut node_streams = BTreeMap::new();
        node_streams.insert("18".to_string(), stream18);
        node_streams.insert("20".to_string(), stream20);

        self.streams.insert("nodejs".to_string(), node_streams);
        self.active_stream_selections
            .insert("nodejs".to_string(), "20".to_string());
    }

    pub fn register_stream(&mut self, stream: ModulemdStream) {
        self.streams
            .entry(stream.module_name.clone())
            .or_default()
            .insert(stream.stream_version.clone(), stream);
    }

    pub fn select_stream(
        &mut self,
        module_name: &str,
        stream_version: &str,
    ) -> Result<(), &'static str> {
        if let Some(module_streams) = self.streams.get(module_name) {
            if module_streams.contains_key(stream_version) {
                self.active_stream_selections
                    .insert(module_name.to_string(), stream_version.to_string());
                Ok(())
            } else {
                Err("Stream version not found for module")
            }
        } else {
            Err("Module not found")
        }
    }

    pub fn get_active_profile_components(
        &self,
        module_name: &str,
        profile_name: &str,
    ) -> Option<Vec<String>> {
        let active_ver = self.active_stream_selections.get(module_name)?;
        let stream = self.streams.get(module_name)?.get(active_ver)?;
        let profile = stream.profiles.get(profile_name)?;
        Some(profile.components.clone())
    }
}

impl Default for FedoraModulemdComponentEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Fedora Silverblue / CoreOS / rpm-ostree Atomic Layer Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtomicLayerType {
    BaseSystem, // Core immutable OS image base
    Overlay,    // Layered rpm-ostree component overlay
    HotSwap,    // Transient runtime component hot-swap
    Ephemeral,  // Temporary tmpfs development layer
}

/// Fedora Atomic Component Layer representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicComponentLayer {
    pub layer_id: String,
    pub layer_type: AtomicLayerType,
    pub component_names: Vec<String>,
    pub checksum: String,
    pub mount_target: String,
}

/// Snapshot deployment commit for Fedora Atomic updates
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtomicDeploymentSnapshot {
    pub deployment_hash: String,
    pub timestamp_sec: u64,
    pub layers: Vec<AtomicComponentLayer>,
    pub is_active: bool,
}

/// Engine managing Fedora Silverblue / rpm-ostree atomic component layering and snapshots
pub struct FedoraAtomicComponentLayerEngine {
    pub deployments: Vec<AtomicDeploymentSnapshot>,
    pub active_deployment_hash: Option<String>,
}

impl FedoraAtomicComponentLayerEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            deployments: Vec::new(),
            active_deployment_hash: None,
        };
        engine.register_base_deployment();
        engine
    }

    fn register_base_deployment(&mut self) {
        let base_layer = AtomicComponentLayer {
            layer_id: "layer-base-0".to_string(),
            layer_type: AtomicLayerType::BaseSystem,
            component_names: vec![
                "kernel".to_string(),
                "systemd".to_string(),
                "glibc".to_string(),
            ],
            checksum: "sha256:base000000000000000000000000000000000000000000000000000000000000"
                .to_string(),
            mount_target: "/sysroot/ostree/deploy/base".to_string(),
        };

        let snapshot = AtomicDeploymentSnapshot {
            deployment_hash: "fedora-silverblue-base-hash-1".to_string(),
            timestamp_sec: 1700000000,
            layers: vec![base_layer],
            is_active: true,
        };

        self.deployments.push(snapshot);
        self.active_deployment_hash = Some("fedora-silverblue-base-hash-1".to_string());
    }

    pub fn apply_component_overlay(&mut self, overlay_id: &str, components: &[String]) -> String {
        let current_layers = if let Some(active_hash) = &self.active_deployment_hash {
            self.deployments
                .iter()
                .find(|d| d.deployment_hash == *active_hash)
                .map(|d| d.layers.clone())
                .unwrap_or_default()
        } else {
            Vec::new()
        };

        let mut new_layers = current_layers;
        let overlay_layer = AtomicComponentLayer {
            layer_id: overlay_id.to_string(),
            layer_type: AtomicLayerType::Overlay,
            component_names: components.to_vec(),
            checksum: format!("sha256:overlay_{}", overlay_id),
            mount_target: format!("/sysroot/ostree/deploy/overlay/{}", overlay_id),
        };
        new_layers.push(overlay_layer);

        // Deactivate previous
        for dep in &mut self.deployments {
            dep.is_active = false;
        }

        let new_hash = format!("deployment-hash-{}", self.deployments.len() + 1);
        let new_snapshot = AtomicDeploymentSnapshot {
            deployment_hash: new_hash.clone(),
            timestamp_sec: 1700000050,
            layers: new_layers,
            is_active: true,
        };

        self.deployments.push(new_snapshot);
        self.active_deployment_hash = Some(new_hash.clone());
        new_hash
    }

    pub fn rollback_deployment(&mut self) -> Result<String, &'static str> {
        if self.deployments.len() <= 1 {
            return Err("No prior atomic deployment snapshot available for rollback");
        }

        self.deployments.pop(); // Remove active
        if let Some(prev) = self.deployments.last_mut() {
            prev.is_active = true;
            let prev_hash = prev.deployment_hash.clone();
            self.active_deployment_hash = Some(prev_hash.clone());
            Ok(prev_hash)
        } else {
            Err("Failed to activate previous deployment")
        }
    }
}

impl Default for FedoraAtomicComponentLayerEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Fedora Toolbx / Podman socket passthrough configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolboxSocketPassthrough {
    pub display_socket: bool, // X11 / Wayland display socket passthrough
    pub audio_socket: bool,   // PulseAudio / PipeWire socket passthrough
    pub dbus_session: bool,   // DBus session bus socket passthrough
    pub ssh_agent: bool,      // SSH agent socket passthrough
    pub host_udev: bool,      // Device node access
}

/// Fedora Toolbx containerized component sandbox configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolboxComponentSandbox {
    pub container_name: String,
    pub base_image: String,
    pub host_workdir_mount: String,
    pub passthrough: ToolboxSocketPassthrough,
    pub installed_tools: Vec<String>,
    pub is_running: bool,
}

/// Engine managing Fedora Toolbx / Podman development component sandboxes
pub struct FedoraToolboxComponentSandboxEngine {
    pub sandboxes: BTreeMap<String, ToolboxComponentSandbox>,
}

impl FedoraToolboxComponentSandboxEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            sandboxes: BTreeMap::new(),
        };
        engine.create_default_sandbox(
            "fedora-toolbox-39",
            "registry.fedoraproject.org/fedora-toolbox:39",
        );
        engine
    }

    pub fn create_default_sandbox(&mut self, name: &str, image: &str) {
        let sandbox = ToolboxComponentSandbox {
            container_name: name.to_string(),
            base_image: image.to_string(),
            host_workdir_mount: "/var/home/user/workspace".to_string(),
            passthrough: ToolboxSocketPassthrough {
                display_socket: true,
                audio_socket: true,
                dbus_session: true,
                ssh_agent: true,
                host_udev: false,
            },
            installed_tools: vec![
                "gcc".to_string(),
                "gdb".to_string(),
                "make".to_string(),
                "git".to_string(),
            ],
            is_running: false,
        };
        self.sandboxes.insert(name.to_string(), sandbox);
    }

    pub fn start_sandbox(&mut self, name: &str) -> Result<(), &'static str> {
        if let Some(sandbox) = self.sandboxes.get_mut(name) {
            sandbox.is_running = true;
            Ok(())
        } else {
            Err("Toolbox component sandbox not found")
        }
    }

    pub fn install_tool_in_sandbox(&mut self, name: &str, tool: &str) -> Result<(), &'static str> {
        if let Some(sandbox) = self.sandboxes.get_mut(name) {
            if !sandbox.installed_tools.contains(&tool.to_string()) {
                sandbox.installed_tools.push(tool.to_string());
            }
            Ok(())
        } else {
            Err("Toolbox component sandbox not found")
        }
    }
}

impl Default for FedoraToolboxComponentSandboxEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Component Health and Compliance Audit Report
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentHealthReport {
    pub component_id: usize,
    pub name: String,
    pub selinux_context: String,
    pub license_spdx: String,
    pub security_score: f32, // 0.0 to 10.0
    pub is_compliant: bool,
    pub audit_issues: Vec<String>,
}

/// Engine managing Fedora component security scoring, SELinux context verification, and license compliance auditing
pub struct FedoraComponentHealthAuditorEngine;

impl FedoraComponentHealthAuditorEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn audit_component(
        &self,
        component_id: usize,
        name: &str,
        selinux_context: &str,
        license_spdx: &str,
    ) -> ComponentHealthReport {
        let mut audit_issues = Vec::new();
        let mut score = 10.0f32;

        // Verify SELinux context format (e.g. system_u:object_r:daemon_exec_t:s0)
        if !selinux_context.contains(":object_r:") && !selinux_context.contains(":system_r:") {
            audit_issues.push("Invalid or non-standard SELinux security context".to_string());
            score -= 3.0;
        }

        // Verify Open Source license compliance
        let is_open_source = license_spdx.contains("GPL")
            || license_spdx.contains("MIT")
            || license_spdx.contains("Apache")
            || license_spdx.contains("BSD");

        if !is_open_source {
            audit_issues.push("Component uses proprietary or non-standard license".to_string());
            score -= 2.5;
        }

        let is_compliant = score >= 7.0 && audit_issues.is_empty();

        ComponentHealthReport {
            component_id,
            name: name.to_string(),
            selinux_context: selinux_context.to_string(),
            license_spdx: license_spdx.to_string(),
            security_score: score.max(0.0),
            is_compliant,
            audit_issues,
        }
    }
}

impl Default for FedoraComponentHealthAuditorEngine {
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

    #[test]
    fn test_fedora_comps_component_group_engine() {
        let comps_engine = FedoraCompsComponentGroupEngine::new();
        let mandatory = comps_engine.get_mandatory_components("core");
        assert!(mandatory.contains(&"systemd".to_string()));
        assert!(mandatory.contains(&"kernel-core".to_string()));

        let defaults = comps_engine.get_default_components("core");
        assert!(defaults.contains(&"selinux-policy".to_string()));

        let xml = comps_engine.generate_comps_xml();
        assert!(xml.contains("<comps>"));
        assert!(xml.contains("<id>core</id>"));
        assert!(xml.contains("<packagereq type=\"mandatory\">systemd</packagereq>"));
    }

    #[test]
    fn test_fedora_modulemd_component_engine() {
        let mut module_engine = FedoraModulemdComponentEngine::new();
        let node20_comps = module_engine
            .get_active_profile_components("nodejs", "default")
            .unwrap();
        assert!(node20_comps.contains(&"nodejs-20.9.0".to_string()));

        // Switch stream to Node.js 18
        assert!(module_engine.select_stream("nodejs", "18").is_ok());
        let node18_dev_comps = module_engine
            .get_active_profile_components("nodejs", "development")
            .unwrap();
        assert!(node18_dev_comps.contains(&"nodejs-devel-18.18.0".to_string()));
    }

    #[test]
    fn test_fedora_atomic_component_layer_engine() {
        let mut atomic_engine = FedoraAtomicComponentLayerEngine::new();
        assert_eq!(atomic_engine.deployments.len(), 1);

        let new_hash = atomic_engine
            .apply_component_overlay("dev-tools", &["gcc".to_string(), "gdb".to_string()]);
        assert_eq!(atomic_engine.deployments.len(), 2);
        assert_eq!(
            atomic_engine.active_deployment_hash.as_deref(),
            Some(new_hash.as_str())
        );

        // Rollback atomic deployment
        let prev_hash = atomic_engine.rollback_deployment().unwrap();
        assert_eq!(atomic_engine.deployments.len(), 1);
        assert_eq!(prev_hash, "fedora-silverblue-base-hash-1");
    }

    #[test]
    fn test_fedora_toolbox_component_sandbox_engine() {
        let mut toolbox_engine = FedoraToolboxComponentSandboxEngine::new();
        assert!(toolbox_engine.start_sandbox("fedora-toolbox-39").is_ok());

        assert!(toolbox_engine
            .install_tool_in_sandbox("fedora-toolbox-39", "clang")
            .is_ok());
        let sandbox = toolbox_engine.sandboxes.get("fedora-toolbox-39").unwrap();
        assert!(sandbox.is_running);
        assert!(sandbox.installed_tools.contains(&"clang".to_string()));
        assert!(sandbox.passthrough.display_socket);
    }

    #[test]
    fn test_fedora_component_health_auditor_engine() {
        let auditor = FedoraComponentHealthAuditorEngine::new();
        let report = auditor.audit_component(
            1,
            "kernel-core",
            "system_u:object_r:kernel_t:s0",
            "GPL-2.0-only",
        );

        assert!(report.is_compliant);
        assert_eq!(report.security_score, 10.0);
        assert!(report.audit_issues.is_empty());

        let bad_report =
            auditor.audit_component(2, "proprietary-driver", "invalid_context", "Proprietary");

        assert!(!bad_report.is_compliant);
        assert!(bad_report.security_score < 7.0);
        assert_eq!(bad_report.audit_issues.len(), 2);
    }
}
