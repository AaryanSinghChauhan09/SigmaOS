//! SigmaOS sysfs — Kernel Object Virtual Filesystem
//!
//! Sovereign /sys filesystem exposing kernel objects (kobjects),
//! device attributes, and bus/driver information to userspace.
//!
//! Inspired by Linux sysfs (drivers/base/core.c, fs/sysfs/).
//!
//! # Hierarchy
//! ```
//! /sys/
//! ├── block/          Block devices (sda, nvme0n1...)
//! ├── bus/            Bus types (pci, usb, platform...)
//! ├── class/          Device classes (net, block, tty...)
//! ├── dev/            Devices by major:minor
//! ├── devices/        Device tree
//! ├── firmware/       Firmware interfaces (ACPI, EFI)
//! ├── fs/             Filesystem info
//! ├── kernel/         Kernel parameters
//! │   ├── mm/         Memory management
//! │   └── debug/      Debugging
//! └── module/         Loaded modules and parameters
//! ```

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ============================================================
// KObject — Kernel Object
// ============================================================

/// Unique identifier for a sysfs kobject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KobjId(u64);

/// Type of a sysfs node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysfsNodeType {
    /// Directory (kobject)
    Directory,
    /// Read-only attribute file
    AttrReadOnly,
    /// Read-write attribute file
    AttrReadWrite,
    /// Write-only attribute file
    AttrWriteOnly,
    /// Symbolic link to another kobject
    Symlink,
}

/// A sysfs attribute value.
#[derive(Debug, Clone)]
pub enum AttrValue {
    /// Integer value (decimal)
    Integer(i64),
    /// Unsigned integer
    Unsigned(u64),
    /// Boolean (0 or 1)
    Bool(bool),
    /// String value
    Text(String),
    /// Hex value (printed as 0x...)
    Hex(u64),
}

impl AttrValue {
    /// Render value as string (as read by userspace).
    pub fn to_sysfs_string(&self) -> String {
        match self {
            Self::Integer(n) => format!("{}\n", n),
            Self::Unsigned(n) => format!("{}\n", n),
            Self::Bool(b) => format!("{}\n", if *b { 1 } else { 0 }),
            Self::Text(s) => format!("{}\n", s),
            Self::Hex(n) => format!("0x{:x}\n", n),
        }
    }

    /// Parse a userspace write (string → value).
    pub fn parse_write(s: &str, current: &Self) -> Result<Self, &'static str> {
        let s = s.trim();
        match current {
            Self::Integer(_) => s.parse::<i64>().map(Self::Integer).map_err(|_| "invalid integer"),
            Self::Unsigned(_) => s.parse::<u64>().map(Self::Unsigned).map_err(|_| "invalid integer"),
            Self::Bool(_) => match s { "0" | "false" | "off" => Ok(Self::Bool(false)),
                "1" | "true" | "on" => Ok(Self::Bool(true)), _ => Err("invalid bool") },
            Self::Text(_) => Ok(Self::Text(s.into())),
            Self::Hex(_) => {
                let n = if let Some(h) = s.strip_prefix("0x") {
                    u64::from_str_radix(h, 16)
                } else {
                    s.parse::<u64>()
                };
                n.map(Self::Hex).map_err(|_| "invalid hex")
            }
        }
    }
}

// ============================================================
// SysfsNode
// ============================================================

/// A single node in the sysfs tree.
pub struct SysfsNode {
    id: KobjId,
    name: String,
    node_type: SysfsNodeType,
    parent: Option<KobjId>,
    children: Vec<KobjId>,
    /// Attribute value (for attribute nodes)
    value: Option<AttrValue>,
    /// Symlink target path
    symlink_target: Option<String>,
    /// Kobject subsystem (e.g., "pci", "net")
    subsystem: Option<String>,
}

impl SysfsNode {
    fn new_dir(id: KobjId, name: &str, parent: Option<KobjId>) -> Self {
        Self { id, name: name.into(), node_type: SysfsNodeType::Directory,
            parent, children: Vec::new(), value: None, symlink_target: None, subsystem: None }
    }

    fn new_attr(id: KobjId, name: &str, parent: KobjId, value: AttrValue, rw: bool) -> Self {
        let node_type = if rw { SysfsNodeType::AttrReadWrite } else { SysfsNodeType::AttrReadOnly };
        Self { id, name: name.into(), node_type, parent: Some(parent),
            children: Vec::new(), value: Some(value), symlink_target: None, subsystem: None }
    }

    fn new_symlink(id: KobjId, name: &str, parent: KobjId, target: &str) -> Self {
        Self { id, name: name.into(), node_type: SysfsNodeType::Symlink,
            parent: Some(parent), children: Vec::new(), value: None,
            symlink_target: Some(target.into()), subsystem: None }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn node_type(&self) -> SysfsNodeType { self.node_type }
    pub fn id(&self) -> KobjId { self.id }
    pub fn parent(&self) -> Option<KobjId> { self.parent }
    pub fn children(&self) -> &[KobjId] { &self.children }
    pub fn value(&self) -> Option<&AttrValue> { self.value.as_ref() }
    pub fn symlink_target(&self) -> Option<&str> { self.symlink_target.as_deref() }
    pub fn subsystem(&self) -> Option<&str> { self.subsystem.as_deref() }
}

// ============================================================
// SigmaSysfs — The /sys Filesystem
// ============================================================

/// The SigmaOS sysfs virtual filesystem.
///
/// Manages the tree of kernel objects (kobjects) and their attributes.
/// Provides read/write access to individual attributes.
pub struct SigmaSysfs {
    nodes: BTreeMap<KobjId, SysfsNode>,
    path_index: BTreeMap<String, KobjId>,
    root_id: KobjId,
    next_id: u64,
}

impl SigmaSysfs {
    /// Create a new sysfs with the standard Linux-compatible layout.
    pub fn new() -> Self {
        let mut sysfs = Self {
            nodes: BTreeMap::new(),
            path_index: BTreeMap::new(),
            root_id: KobjId(1),
            next_id: 2,
        };
        sysfs.init_standard_layout();
        sysfs
    }

    fn alloc_id(&mut self) -> KobjId {
        let id = KobjId(self.next_id);
        self.next_id += 1;
        id
    }

    fn init_standard_layout(&mut self) {
        // Root node
        let root = SysfsNode::new_dir(KobjId(1), "", None);
        self.nodes.insert(KobjId(1), root);
        self.path_index.insert("/sys".into(), KobjId(1));
        self.path_index.insert("/".into(), KobjId(1));

        // Create top-level directories
        for dir in &["block", "bus", "class", "dev", "devices", "firmware", "fs", "kernel", "module"] {
            self.mkdir_at(KobjId(1), dir).ok();
        }

        // Populate /sys/kernel/
        let kernel_id = *self.path_index.get("/sys/kernel").unwrap_or(&KobjId(1));
        for dir in &["mm", "debug", "security", "tracing"] {
            self.mkdir_at(kernel_id, dir).ok();
        }

        // Add /sys/kernel/mm/ attributes (memory management)
        let mm_id = self.mkdir_at(kernel_id, "mm").unwrap_or(kernel_id);
        self.add_attr_rw(mm_id, "overcommit_memory", AttrValue::Integer(0));
        self.add_attr_rw(mm_id, "overcommit_ratio", AttrValue::Integer(50));
        self.add_attr_rw(mm_id, "swappiness", AttrValue::Integer(60));
        self.add_attr_rw(mm_id, "dirty_ratio", AttrValue::Integer(20));
        self.add_attr_rw(mm_id, "dirty_background_ratio", AttrValue::Integer(10));

        // Add /sys/kernel/debug/ attributes
        let dbg_id = self.mkdir_at(kernel_id, "debug").unwrap_or(kernel_id);
        self.add_attr_ro(dbg_id, "kprobes_enabled", AttrValue::Bool(true));
        self.add_attr_rw(dbg_id, "tracing_enabled", AttrValue::Bool(false));

        // /sys/class/net/ for network interfaces
        let class_id = self.find_path("/sys/class").unwrap_or(KobjId(1));
        let net_id = self.mkdir_at(class_id, "net").unwrap_or(class_id);
        let lo_id = self.mkdir_at(net_id, "lo").unwrap_or(net_id);
        self.add_attr_ro(lo_id, "address", AttrValue::Text("00:00:00:00:00:00".into()));
        self.add_attr_ro(lo_id, "mtu", AttrValue::Integer(65536));
        self.add_attr_rw(lo_id, "flags", AttrValue::Hex(0x49));
        self.add_attr_ro(lo_id, "speed", AttrValue::Integer(-1));
        self.add_attr_ro(lo_id, "operstate", AttrValue::Text("unknown".into()));

        let eth0_id = self.mkdir_at(net_id, "eth0").unwrap_or(net_id);
        self.add_attr_ro(eth0_id, "address", AttrValue::Text("02:42:ac:11:00:02".into()));
        self.add_attr_ro(eth0_id, "mtu", AttrValue::Integer(1500));
        self.add_attr_rw(eth0_id, "flags", AttrValue::Hex(0x1043));
        self.add_attr_ro(eth0_id, "speed", AttrValue::Integer(1000));
        self.add_attr_ro(eth0_id, "operstate", AttrValue::Text("up".into()));
        self.add_attr_rw(eth0_id, "tx_queue_len", AttrValue::Integer(1000));
    }

    /// Create a directory kobject.
    pub fn mkdir_at(&mut self, parent: KobjId, name: &str) -> Result<KobjId, &'static str> {
        // Build path
        let parent_path = self.id_to_path(parent);
        let path = format!("{}/{}", parent_path.trim_end_matches('/'), name);

        if self.path_index.contains_key(&path) {
            return self.path_index.get(&path).copied().ok_or("already exists");
        }

        let id = self.alloc_id();
        let node = SysfsNode::new_dir(id, name, Some(parent));
        self.nodes.insert(id, node);
        self.path_index.insert(path, id);
        if let Some(p) = self.nodes.get_mut(&parent) {
            p.children.push(id);
        }
        Ok(id)
    }

    /// Add a read-only attribute.
    pub fn add_attr_ro(&mut self, parent: KobjId, name: &str, value: AttrValue) -> KobjId {
        let id = self.alloc_id();
        let node = SysfsNode::new_attr(id, name, parent, value, false);
        let parent_path = self.id_to_path(parent);
        let path = format!("{}/{}", parent_path.trim_end_matches('/'), name);
        self.path_index.insert(path, id);
        self.nodes.insert(id, node);
        if let Some(p) = self.nodes.get_mut(&parent) {
            p.children.push(id);
        }
        id
    }

    /// Add a read-write attribute.
    pub fn add_attr_rw(&mut self, parent: KobjId, name: &str, value: AttrValue) -> KobjId {
        let id = self.alloc_id();
        let node = SysfsNode::new_attr(id, name, parent, value, true);
        let parent_path = self.id_to_path(parent);
        let path = format!("{}/{}", parent_path.trim_end_matches('/'), name);
        self.path_index.insert(path, id);
        self.nodes.insert(id, node);
        if let Some(p) = self.nodes.get_mut(&parent) {
            p.children.push(id);
        }
        id
    }

    /// Add a symbolic link.
    pub fn add_symlink(&mut self, parent: KobjId, name: &str, target: &str) -> KobjId {
        let id = self.alloc_id();
        let node = SysfsNode::new_symlink(id, name, parent, target);
        self.nodes.insert(id, node);
        if let Some(p) = self.nodes.get_mut(&parent) {
            p.children.push(id);
        }
        id
    }

    /// Read an attribute by path.
    pub fn read(&self, path: &str) -> Result<Vec<u8>, &'static str> {
        let id = self.find_path(path).ok_or("no such attribute")?;
        let node = self.nodes.get(&id).ok_or("node not found")?;
        match &node.node_type {
            SysfsNodeType::AttrReadOnly | SysfsNodeType::AttrReadWrite => {
                node.value.as_ref()
                    .map(|v| v.to_sysfs_string().into_bytes())
                    .ok_or("no value")
            }
            SysfsNodeType::Symlink => {
                node.symlink_target.as_ref()
                    .map(|t| t.as_bytes().to_vec())
                    .ok_or("no target")
            }
            _ => Err("is a directory"),
        }
    }

    /// Write an attribute by path.
    pub fn write(&mut self, path: &str, data: &str) -> Result<(), &'static str> {
        let id = self.find_path(path).ok_or("no such attribute")?;
        let node = self.nodes.get_mut(&id).ok_or("node not found")?;
        match node.node_type {
            SysfsNodeType::AttrWriteOnly | SysfsNodeType::AttrReadWrite => {
                let new_val = AttrValue::parse_write(data,
                    node.value.as_ref().ok_or("no current value")?)?;
                node.value = Some(new_val);
                Ok(())
            }
            SysfsNodeType::AttrReadOnly => Err("read-only attribute"),
            _ => Err("not an attribute"),
        }
    }

    /// List directory contents.
    pub fn readdir(&self, path: &str) -> Result<Vec<String>, &'static str> {
        let id = self.find_path(path).ok_or("no such directory")?;
        let node = self.nodes.get(&id).ok_or("node not found")?;
        if node.node_type != SysfsNodeType::Directory {
            return Err("not a directory");
        }
        Ok(node.children.iter()
            .filter_map(|c| self.nodes.get(c))
            .map(|n| n.name.clone())
            .collect())
    }

    /// Find a node ID by path.
    pub fn find_path(&self, path: &str) -> Option<KobjId> {
        // Normalise path
        let p = if path.starts_with("/sys") { path.into() }
                else { format!("/sys/{}", path.trim_start_matches('/')) };
        self.path_index.get(&p).copied()
            .or_else(|| self.path_index.get(path).copied())
    }

    fn id_to_path(&self, id: KobjId) -> String {
        for (path, &nid) in &self.path_index {
            if nid == id { return path.clone(); }
        }
        "/sys".into()
    }

    /// Register a PCI device kobject.
    pub fn register_pci_device(&mut self, bus: u8, dev: u8, func: u8,
                                vendor: u16, device: u16, class: u32, name: &str) -> KobjId {
        let devices_id = self.find_path("/sys/devices").unwrap_or(self.root_id);
        let pci_path = format!("0000:{:02x}:{:02x}.{}", bus, dev, func);
        let pci_id = self.mkdir_at(devices_id, &pci_path).unwrap_or(devices_id);
        self.add_attr_ro(pci_id, "vendor", AttrValue::Hex(vendor as u64));
        self.add_attr_ro(pci_id, "device", AttrValue::Hex(device as u64));
        self.add_attr_ro(pci_id, "class", AttrValue::Hex(class as u64));
        self.add_attr_ro(pci_id, "subsystem_vendor", AttrValue::Hex(0));
        self.add_attr_ro(pci_id, "subsystem_device", AttrValue::Hex(0));
        self.add_attr_ro(pci_id, "irq", AttrValue::Integer(16));
        self.add_attr_ro(pci_id, "local_cpus", AttrValue::Text("ffffffff".into()));
        self.add_attr_rw(pci_id, "enable", AttrValue::Bool(true));
        self.add_attr_rw(pci_id, "power/control", AttrValue::Text("auto".into()));
        pci_id
    }

    pub fn root_id(&self) -> KobjId { self.root_id }
    pub fn node_count(&self) -> usize { self.nodes.len() }
    pub fn get_node(&self, id: KobjId) -> Option<&SysfsNode> { self.nodes.get(&id) }
}

impl Default for SigmaSysfs {
    fn default() -> Self { Self::new() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_layout() {
        let sysfs = SigmaSysfs::new();
        assert!(sysfs.find_path("/sys/kernel").is_some());
        assert!(sysfs.find_path("/sys/class").is_some());
        assert!(sysfs.find_path("/sys/devices").is_some());
        assert!(sysfs.find_path("/sys/bus").is_some());
    }

    #[test]
    fn test_read_attr() {
        let sysfs = SigmaSysfs::new();
        let content = String::from_utf8(
            sysfs.read("/sys/kernel/mm/swappiness").unwrap()
        ).unwrap();
        assert!(content.trim() == "60");
    }

    #[test]
    fn test_write_attr() {
        let mut sysfs = SigmaSysfs::new();
        sysfs.write("/sys/kernel/mm/swappiness", "10").unwrap();
        let content = String::from_utf8(
            sysfs.read("/sys/kernel/mm/swappiness").unwrap()
        ).unwrap();
        assert!(content.trim() == "10");
    }

    #[test]
    fn test_write_readonly_fails() {
        let mut sysfs = SigmaSysfs::new();
        assert!(sysfs.write("/sys/class/net/eth0/address", "ff:ff:ff:ff:ff:ff").is_err());
    }

    #[test]
    fn test_readdir() {
        let sysfs = SigmaSysfs::new();
        let entries = sysfs.readdir("/sys/class/net").unwrap();
        assert!(entries.contains(&"eth0".to_string()));
        assert!(entries.contains(&"lo".to_string()));
    }

    #[test]
    fn test_pci_device() {
        let mut sysfs = SigmaSysfs::new();
        let id = sysfs.register_pci_device(0, 2, 0, 0x8086, 0x1234, 0x030200, "vga");
        let node = sysfs.get_node(id).unwrap();
        assert_eq!(node.name(), "0000:00:02.0");
    }
}
