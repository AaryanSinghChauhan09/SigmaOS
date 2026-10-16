//! # configfs — Userspace-driven Kernel Object Configuration Filesystem
//!
//! `configfs` is an in-memory virtual filesystem for managing kernel objects
//! from user-space. Unlike `sysfs` (where kernel objects are exported by
//! drivers and read/written by user-space), `configfs` inverts the relationship:
//! user-space creates and destroys kernel objects using filesystem operations
//! (`mkdir` and `rmdir`), and configures their properties by writing to
//! attribute files within those directories.
//!
//! Typical mount point: `/sys/kernel/config`
//!
//! Reference: Linux `fs/configfs/`, `Documentation/filesystems/configfs/configfs.rst`.

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use std::sync::{Arc, RwLock};

/// Errors returned by configfs operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigfsError {
    /// Item or directory not found.
    NotFound,
    /// Item already exists in group.
    AlreadyExists,
    /// Permission denied for attribute read/write.
    PermissionDenied,
    /// Item is locked or has active dependent children.
    Busy,
    /// Invalid parameter value supplied.
    InvalidValue,
    /// Operation not supported on this item type.
    NotSupported,
    /// Out of memory or table space.
    OutOfMemory,
}

/// Permissions for an attribute file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttributeMode {
    pub read: bool,
    pub write: bool,
}

impl AttributeMode {
    pub const RO: Self = Self {
        read: true,
        write: false,
    };
    pub const WO: Self = Self {
        read: false,
        write: true,
    };
    pub const RW: Self = Self {
        read: true,
        write: true,
    };
}

/// An attribute file representing a tunable parameter of a config_item.
pub struct ConfigAttribute {
    pub name: String,
    pub mode: AttributeMode,
    pub value: Arc<RwLock<String>>,
}

impl ConfigAttribute {
    pub fn new(name: &str, mode: AttributeMode, initial_value: &str) -> Self {
        Self {
            name: name.to_string(),
            mode,
            value: Arc::new(RwLock::new(initial_value.to_string())),
        }
    }

    pub fn read(&self) -> Result<String, ConfigfsError> {
        if !self.mode.read {
            return Err(ConfigfsError::PermissionDenied);
        }
        Ok(self.value.read().unwrap().clone())
    }

    pub fn write(&self, new_val: &str) -> Result<(), ConfigfsError> {
        if !self.mode.write {
            return Err(ConfigfsError::PermissionDenied);
        }
        *self.value.write().unwrap() = new_val.to_string();
        Ok(())
    }
}

/// An item type defines the schema and allowed attributes for items in a group.
pub struct ConfigItemType {
    pub name: String,
    pub default_attributes: Vec<ConfigAttribute>,
}

/// A kernel object managed through configfs.
pub struct ConfigItem {
    pub name: String,
    pub attributes: BTreeMap<String, ConfigAttribute>,
    pub item_type: Option<Arc<ConfigItemType>>,
}

impl ConfigItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attributes: BTreeMap::new(),
            item_type: None,
        }
    }

    pub fn add_attribute(&mut self, attr: ConfigAttribute) {
        self.attributes.insert(attr.name.clone(), attr);
    }

    pub fn get_attribute(&self, attr_name: &str) -> Option<&ConfigAttribute> {
        self.attributes.get(attr_name)
    }
}

/// A directory node that can contain sub-items or sub-groups.
pub struct ConfigGroup {
    pub item: ConfigItem,
    pub default_groups: BTreeMap<String, Arc<RwLock<ConfigGroup>>>,
    pub dynamic_children: BTreeMap<String, Arc<RwLock<ConfigGroup>>>,
    pub allowed_item_type: Option<Arc<ConfigItemType>>,
}

impl ConfigGroup {
    pub fn new(name: &str) -> Self {
        Self {
            item: ConfigItem::new(name),
            default_groups: BTreeMap::new(),
            dynamic_children: BTreeMap::new(),
            allowed_item_type: None,
        }
    }

    /// User creates a new subsystem item via `mkdir <name>`.
    pub fn mkdir(&mut self, name: &str) -> Result<Arc<RwLock<ConfigGroup>>, ConfigfsError> {
        if self.dynamic_children.contains_key(name) || self.default_groups.contains_key(name) {
            return Err(ConfigfsError::AlreadyExists);
        }

        let mut child = ConfigGroup::new(name);
        if let Some(ref itype) = self.allowed_item_type {
            for attr in &itype.default_attributes {
                child.item.add_attribute(ConfigAttribute::new(
                    &attr.name,
                    attr.mode,
                    &attr.value.read().unwrap(),
                ));
            }
            child.item.item_type = Some(itype.clone());
        }

        let child_arc = Arc::new(RwLock::new(child));
        self.dynamic_children
            .insert(name.to_string(), child_arc.clone());
        Ok(child_arc)
    }

    /// User destroys a subsystem item via `rmdir <name>`.
    pub fn rmdir(&mut self, name: &str) -> Result<(), ConfigfsError> {
        if let Some(child) = self.dynamic_children.get(name) {
            let child_lock = child.read().unwrap();
            if !child_lock.dynamic_children.is_empty() {
                return Err(ConfigfsError::Busy);
            }
        } else {
            return Err(ConfigfsError::NotFound);
        }

        self.dynamic_children.remove(name);
        Ok(())
    }
}

/// The configfs subsystem root mount.
pub struct ConfigfsSubsystem {
    pub root: Arc<RwLock<ConfigGroup>>,
}

impl ConfigfsSubsystem {
    pub fn new() -> Self {
        let root = Arc::new(RwLock::new(ConfigGroup::new("config")));
        Self { root }
    }

    /// Register a top-level subsystem group (e.g. "target", "usb_gadget", "rdma").
    pub fn register_subsystem(
        &self,
        name: &str,
        item_type: Option<Arc<ConfigItemType>>,
    ) -> Result<(), ConfigfsError> {
        let mut root_guard = self.root.write().unwrap();
        if root_guard.default_groups.contains_key(name) {
            return Err(ConfigfsError::AlreadyExists);
        }

        let mut group = ConfigGroup::new(name);
        group.allowed_item_type = item_type;
        root_guard
            .default_groups
            .insert(name.to_string(), Arc::new(RwLock::new(group)));
        Ok(())
    }

    /// Read attribute at path `/subsystem/item/attribute`.
    pub fn read_attribute(&self, path: &str) -> Result<String, ConfigfsError> {
        let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
        if parts.len() < 2 {
            return Err(ConfigfsError::NotFound);
        }

        let root_guard = self.root.read().unwrap();
        let sub_name = parts[0];
        let sub_group = root_guard
            .default_groups
            .get(sub_name)
            .ok_or(ConfigfsError::NotFound)?;
        let sub_lock = sub_group.read().unwrap();

        if parts.len() == 2 {
            // Attribute directly on subsystem
            let attr_name = parts[1];
            let attr = sub_lock
                .item
                .get_attribute(attr_name)
                .ok_or(ConfigfsError::NotFound)?;
            return attr.read();
        }

        // Deeper path
        let item_name = parts[1];
        let item_group = sub_lock
            .dynamic_children
            .get(item_name)
            .ok_or(ConfigfsError::NotFound)?;
        let item_lock = item_group.read().unwrap();
        let attr_name = parts[2];
        let attr = item_lock
            .item
            .get_attribute(attr_name)
            .ok_or(ConfigfsError::NotFound)?;
        attr.read()
    }

    /// Write attribute at path `/subsystem/item/attribute`.
    pub fn write_attribute(&self, path: &str, val: &str) -> Result<(), ConfigfsError> {
        let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
        if parts.len() < 2 {
            return Err(ConfigfsError::NotFound);
        }

        let root_guard = self.root.write().unwrap();
        let sub_name = parts[0];
        let sub_group = root_guard
            .default_groups
            .get(sub_name)
            .ok_or(ConfigfsError::NotFound)?;
        let sub_lock = sub_group.read().unwrap();

        if parts.len() == 2 {
            let attr_name = parts[1];
            let attr = sub_lock
                .item
                .get_attribute(attr_name)
                .ok_or(ConfigfsError::NotFound)?;
            return attr.write(val);
        }

        let item_name = parts[1];
        let item_group = sub_lock
            .dynamic_children
            .get(item_name)
            .ok_or(ConfigfsError::NotFound)?;
        let item_lock = item_group.read().unwrap();
        let attr_name = parts[2];
        let attr = item_lock
            .item
            .get_attribute(attr_name)
            .ok_or(ConfigfsError::NotFound)?;
        attr.write(val)
    }
}

impl Default for ConfigfsSubsystem {
    fn default() -> Self {
        Self::new()
    }
}
