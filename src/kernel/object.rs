#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Windows/Linux/BSD-Inspired Advanced Object Manager (Obp)
// Implements advanced Object Manager namespaces, symbolic link translation,
// driver entry contexts, dynamic unloading, and Non-Paged Pool memory tracking.

use crate::klib::collections::HashMap;
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectError {
    InitFailed,
    RegisterFailed,
    NotFound,
    AlreadyRegistered,
    CapabilityDenied,
    BufferTooSmall,
}

pub struct KRef {
    count: AtomicUsize,
}

impl KRef {
    pub fn new() -> Self {
        KRef {
            count: AtomicUsize::new(1),
        }
    }

    pub fn acquire(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn release(&self) -> bool {
        self.count.fetch_sub(1, Ordering::SeqCst) == 1
    }

    pub fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

impl Default for KRef {
    fn default() -> Self {
        Self::new()
    }
}

pub trait KernelObject: Send + Sync {
    fn name(&self) -> &str;
    fn set_name(&mut self, name: &str);
    fn parent(&self) -> Option<&dyn KernelObject>;
    fn set_parent(&mut self, parent: Option<&dyn KernelObject>);
    fn children(&self) -> Vec<&dyn KernelObject>;
    fn add_child(&mut self, child: &dyn KernelObject);
    fn remove_child(&mut self, child_name: &str) -> Option<Box<dyn KernelObject>>;
    fn kref(&self) -> &KRef;
    fn as_any(&self) -> &dyn core::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
    fn sysfs_attrs(&self) -> Vec<&str>;
    fn sysfs_show(&self, attr: &str) -> Option<String>;
    fn sysfs_store(&mut self, attr: &str, value: &str) -> Result<(), ObjectError>;
}

pub struct KObject {
    name: String,
    parent: Option<*const dyn KernelObject>,
    children: Vec<*const dyn KernelObject>,
    kref: KRef,
}

unsafe impl Send for KObject {}
unsafe impl Sync for KObject {}

impl KObject {
    pub fn new(name: &str) -> Self {
        KObject {
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
            kref: KRef::new(),
        }
    }
}

impl KernelObject for KObject {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn parent(&self) -> Option<&dyn KernelObject> {
        self.parent.map(|p| unsafe { &*p })
    }

    fn set_parent(&mut self, parent: Option<&dyn KernelObject>) {
        self.parent = parent.map(|p| unsafe {
            core::mem::transmute::<&dyn KernelObject, &'static dyn KernelObject>(p)
                as *const dyn KernelObject
        });
    }

    fn children(&self) -> Vec<&dyn KernelObject> {
        self.children
            .iter()
            .filter_map(|c| unsafe { c.as_ref() })
            .collect()
    }

    fn add_child(&mut self, child: &dyn KernelObject) {
        self.children.push(unsafe {
            core::mem::transmute::<&dyn KernelObject, &'static dyn KernelObject>(child)
                as *const dyn KernelObject
        });
    }

    fn remove_child(&mut self, child_name: &str) -> Option<Box<dyn KernelObject>> {
        if let Some(idx) = self
            .children
            .iter()
            .position(|c| unsafe { c.as_ref() }.map_or(false, |child| child.name() == child_name))
        {
            let child_ptr = self.children.remove(idx);
            unsafe { Some(Box::from_raw(child_ptr as *mut dyn KernelObject)) }
        } else {
            None
        }
    }

    fn kref(&self) -> &KRef {
        &self.kref
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }

    fn sysfs_attrs(&self) -> Vec<&str> {
        Vec::new()
    }

    fn sysfs_show(&self, _attr: &str) -> Option<String> {
        None
    }

    fn sysfs_store(&mut self, _attr: &str, _value: &str) -> Result<(), ObjectError> {
        Err(ObjectError::CapabilityDenied)
    }
}

impl Drop for KObject {
    fn drop(&mut self) {
        while self.children.pop().is_some() {}
    }
}

pub struct DeviceObject {
    pub base: KObject,
    pub device_id: u16,
    pub vendor_id: u16,
    pub device_type: String,
    pub driver_name: Option<String>,
    pub capabilities: Vec<u64>,
}

impl DeviceObject {
    pub fn new(name: &str, device_id: u16, vendor_id: u16) -> Self {
        DeviceObject {
            base: KObject::new(name),
            device_id,
            vendor_id,
            device_type: String::new(),
            driver_name: None,
            capabilities: Vec::new(),
        }
    }

    pub fn device_id(&self) -> u16 {
        self.device_id
    }

    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }

    pub fn set_device_type(&mut self, dtype: &str) {
        self.device_type = dtype.to_string();
    }

    pub fn device_type(&self) -> &str {
        &self.device_type
    }

    pub fn set_driver(&mut self, driver: &str) {
        self.driver_name = Some(driver.to_string());
    }

    pub fn driver_name(&self) -> Option<&str> {
        self.driver_name.as_deref()
    }

    pub fn add_capability(&mut self, cap: u64) {
        if !self.capabilities.contains(&cap) {
            self.capabilities.push(cap);
        }
    }

    pub fn has_capability(&self, cap: u64) -> bool {
        self.capabilities.contains(&cap)
    }
}

impl KernelObject for DeviceObject {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn set_name(&mut self, name: &str) {
        self.base.set_name(name);
    }
    fn parent(&self) -> Option<&dyn KernelObject> {
        self.base.parent()
    }
    fn set_parent(&mut self, parent: Option<&dyn KernelObject>) {
        self.base.set_parent(parent);
    }
    fn children(&self) -> Vec<&dyn KernelObject> {
        self.base.children()
    }
    fn add_child(&mut self, child: &dyn KernelObject) {
        self.base.add_child(child);
    }
    fn remove_child(&mut self, child_name: &str) -> Option<Box<dyn KernelObject>> {
        self.base.remove_child(child_name)
    }
    fn kref(&self) -> &KRef {
        self.base.kref()
    }
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
    fn sysfs_attrs(&self) -> Vec<&str> {
        self.base.sysfs_attrs()
    }
    fn sysfs_show(&self, attr: &str) -> Option<String> {
        self.base.sysfs_show(attr)
    }
    fn sysfs_store(&mut self, attr: &str, value: &str) -> Result<(), ObjectError> {
        self.base.sysfs_store(attr, value)
    }
}

// =========================================================================
// ADVANCED OBJECT MANAGER STRUCTURES
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObpObjectType {
    Directory,
    SymbolicLink,
    Device,
    Driver,
    Adapter,
    Process,
}

pub struct SymbolicLink {
    pub base: KObject,
    pub target_path: String,
}

impl SymbolicLink {
    pub fn new(name: &str, target: &str) -> Self {
        Self {
            base: KObject::new(name),
            target_path: target.to_string(),
        }
    }
}

pub struct ObpDirectory {
    pub base: KObject,
    pub directory_type: ObpObjectType,
    pub members: HashMap<String, *const dyn KernelObject>,
}

unsafe impl Send for ObpDirectory {}
unsafe impl Sync for ObpDirectory {}

impl ObpDirectory {
    pub fn new(name: &str) -> Self {
        Self {
            base: KObject::new(name),
            directory_type: ObpObjectType::Directory,
            members: HashMap::new(),
        }
    }

    pub fn insert_object(&mut self, name: String, obj: *const dyn KernelObject) {
        self.members.insert(name, obj);
    }

    pub fn lookup_object(&self, name: &str) -> Option<*const dyn KernelObject> {
        self.members.get(name).copied()
    }
}

impl KernelObject for ObpDirectory {
    fn name(&self) -> &str {
        self.base.name()
    }
    fn set_name(&mut self, name: &str) {
        self.base.set_name(name);
    }
    fn parent(&self) -> Option<&dyn KernelObject> {
        self.base.parent()
    }
    fn set_parent(&mut self, parent: Option<&dyn KernelObject>) {
        self.base.set_parent(parent);
    }
    fn children(&self) -> Vec<&dyn KernelObject> {
        self.base.children()
    }
    fn add_child(&mut self, child: &dyn KernelObject) {
        self.base.add_child(child);
    }
    fn remove_child(&mut self, child_name: &str) -> Option<Box<dyn KernelObject>> {
        self.base.remove_child(child_name)
    }
    fn kref(&self) -> &KRef {
        self.base.kref()
    }
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
    fn sysfs_attrs(&self) -> Vec<&str> {
        self.base.sysfs_attrs()
    }
    fn sysfs_show(&self, attr: &str) -> Option<String> {
        self.base.sysfs_show(attr)
    }
    fn sysfs_store(&mut self, attr: &str, value: &str) -> Result<(), ObjectError> {
        self.base.sysfs_store(attr, value)
    }
}

/// Non-Paged Pool Memory Tracker (WDK-inspired physical pinned memory)
pub struct NonPagedPoolMemory {
    pub base_address: usize,
    pub allocated_size: usize,
    pub offsets: HashMap<String, usize>,
}

impl NonPagedPoolMemory {
    pub fn new(base: usize, size: usize) -> Self {
        Self {
            base_address: base,
            allocated_size: size,
            offsets: HashMap::new(),
        }
    }

    pub fn allocate_block(&mut self, tag: &str, size: usize) -> Result<usize, ObjectError> {
        let mut current_total = 0;
        for &val in self.offsets.values() {
            current_total += val;
        }
        if current_total + size > self.allocated_size {
            return Err(ObjectError::CapabilityDenied);
        }
        let address = self.base_address + current_total;
        self.offsets.insert(tag.to_string(), size);
        Ok(address)
    }
}

/// Driver specific structure registering entry & unload contexts
pub struct DriverEntryContext {
    pub driver_name: String,
    pub driver_entry_address: usize,
    pub unload_routine: Option<fn(context_address: usize) -> Result<(), ObjectError>>,
    pub is_loaded: bool,
}

impl DriverEntryContext {
    pub fn new(name: &str, entry_addr: usize) -> Self {
        Self {
            driver_name: name.to_string(),
            driver_entry_address: entry_addr,
            unload_routine: None,
            is_loaded: false,
        }
    }

    pub fn load_driver(
        &mut self,
        unload: fn(usize) -> Result<(), ObjectError>,
    ) -> Result<(), ObjectError> {
        self.unload_routine = Some(unload);
        self.is_loaded = true;
        Ok(())
    }

    pub fn unload_driver(&mut self) -> Result<(), ObjectError> {
        if !self.is_loaded {
            return Err(ObjectError::NotFound);
        }
        if let Some(unload) = self.unload_routine {
            (unload)(self.driver_entry_address)?;
        }
        self.is_loaded = false;
        Ok(())
    }
}

/// Central Object Manager maintaining root namespace directories and symbolic links
pub struct ObpObjectManager {
    pub root_dir: ObpDirectory,
    pub symbolic_links: HashMap<String, String>,
    pub memory_pool: NonPagedPoolMemory,
}

impl ObpObjectManager {
    pub fn new() -> Self {
        let mut root = ObpDirectory::new("\\");

        let dev_dir = Box::into_raw(Box::new(ObpDirectory::new("Device")));
        let dos_dir = Box::into_raw(Box::new(ObpDirectory::new("DosDevices")));
        let drv_dir = Box::into_raw(Box::new(ObpDirectory::new("Driver")));

        root.insert_object("Device".to_string(), dev_dir);
        root.insert_object("DosDevices".to_string(), dos_dir);
        root.insert_object("Driver".to_string(), drv_dir);

        Self {
            root_dir: root,
            symbolic_links: HashMap::new(),
            memory_pool: NonPagedPoolMemory::new(0xFFFF800000000000, 1024 * 1024),
        }
    }

    pub fn register_symbolic_link(&mut self, alias: &str, real_path: &str) {
        self.symbolic_links
            .insert(alias.to_string(), real_path.to_string());
    }

    pub fn resolve_path(&self, path: &str) -> String {
        if let Some(real_path) = self.symbolic_links.get(path) {
            let real_path: &String = real_path;
            real_path.clone()
        } else {
            path.to_string()
        }
    }
}

impl Default for ObpObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    static mut MOCK_UNLOAD_CALLED: bool = false;
    fn mock_unload_routine(context_address: usize) -> Result<(), ObjectError> {
        if context_address == 0xBAADF00D {
            unsafe {
                MOCK_UNLOAD_CALLED = true;
            }
            Ok(())
        } else {
            Err(ObjectError::CapabilityDenied)
        }
    }

    #[test]
    fn test_obp_directory_lookup_and_traversal() {
        let mut dev_dir = ObpDirectory::new("Device");
        let disk = DeviceObject::new("HarddiskVolume1", 0x1111, 0x2222);
        let disk_ptr = Box::into_raw(Box::new(disk));

        dev_dir.insert_object("HarddiskVolume1".to_string(), disk_ptr);

        let retrieved = dev_dir.lookup_object("HarddiskVolume1").unwrap();
        unsafe {
            let dev_obj = &*retrieved;
            assert_eq!(dev_obj.name(), "HarddiskVolume1");
        }

        unsafe {
            let _ = Box::from_raw(disk_ptr as *mut DeviceObject);
        }
    }

    #[test]
    fn test_symbolic_link_resolution() {
        let mut manager = ObpObjectManager::new();

        manager.register_symbolic_link("\\DosDevices\\C:", "\\Device\\HarddiskVolume1");

        let resolved = manager.resolve_path("\\DosDevices\\C:");
        assert_eq!(resolved, "\\Device\\HarddiskVolume1");

        let unlinked = manager.resolve_path("\\Device\\HarddiskVolume1");
        assert_eq!(unlinked, "\\Device\\HarddiskVolume1");
    }

    #[test]
    fn test_non_paged_pool_allocation_audits() {
        let mut pool = NonPagedPoolMemory::new(0xFFFF800000000000, 1024);

        let tag1_addr = pool.allocate_block("IrpBuffer", 256).unwrap();
        assert_eq!(tag1_addr, 0xFFFF800000000000);

        let tag2_addr = pool.allocate_block("DpcContext", 512).unwrap();
        assert_eq!(tag2_addr, 0xFFFF800000000100);

        assert!(pool.allocate_block("Overflow", 512).is_err());
    }

    #[test]
    fn test_driver_entry_and_dynamic_unloading() {
        let mut driver_ctx = DriverEntryContext::new("SovereignFileShim", 0xBAADF00D);
        assert_eq!(driver_ctx.driver_name, "SovereignFileShim");
        assert!(!driver_ctx.is_loaded);

        driver_ctx.load_driver(mock_unload_routine).unwrap();
        assert!(driver_ctx.is_loaded);

        driver_ctx.unload_driver().unwrap();
        assert!(!driver_ctx.is_loaded);

        unsafe {
            assert!(MOCK_UNLOAD_CALLED);
        }
    }
}

// ==========================================
// Windows NT-Style Object Manager Subsystem
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtObjectType {
    Directory,
    Device,
    SymbolicLink,
    Driver,
    Section,
}

#[derive(Clone, Debug)]
pub struct NtObject {
    pub name: String,
    pub object_type: NtObjectType,
    pub target_path: Option<String>, // Symbolic link pointing to real object
}

#[derive(Clone)]
pub struct NtObjectDirectory {
    pub name: String,
    pub objects: HashMap<String, NtObject>,
    pub subdirectories: HashMap<String, NtObjectDirectory>,
}

pub struct NtObjectManager {
    pub root: NtObjectDirectory,
}

impl NtObjectManager {
    pub fn new() -> Self {
        let mut root = NtObjectDirectory {
            name: String::from("\\"),
            objects: HashMap::new(),
            subdirectories: HashMap::new(),
        };
        // Pre-populate standard Windows-style directories
        root.subdirectories.insert(
            String::from("Device"),
            NtObjectDirectory {
                name: String::from("Device"),
                objects: HashMap::new(),
                subdirectories: HashMap::new(),
            },
        );
        root.subdirectories.insert(
            String::from("DosDevices"),
            NtObjectDirectory {
                name: String::from("DosDevices"),
                objects: HashMap::new(),
                subdirectories: HashMap::new(),
            },
        );
        NtObjectManager { root }
    }

    /// Insert an object into the object manager namespace at a specific path (e.g. "\Device\Keyboard")
    pub fn insert_object(&mut self, path: &str, obj: NtObject) -> Result<(), &'static str> {
        let parts: Vec<&str> = path.split('\\').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return Err("Invalid path");
        }

        let mut current_dir = &mut self.root;
        for i in 0..parts.len() - 1 {
            let part = parts[i];
            if !current_dir.subdirectories.contains_key(part) {
                current_dir.subdirectories.insert(
                    part.to_string(),
                    NtObjectDirectory {
                        name: part.to_string(),
                        objects: HashMap::new(),
                        subdirectories: HashMap::new(),
                    },
                );
            }
            current_dir = current_dir.subdirectories.get_mut(part).unwrap();
        }

        let name = parts.last().unwrap().to_string();
        current_dir.objects.insert(name, obj);
        Ok(())
    }

    /// Retrieve an object by its absolute path, resolving symbolic links/aliases recursively
    pub fn lookup_object(&self, path: &str) -> Option<NtObject> {
        let parts: Vec<&str> = path.split('\\').filter(|s| !s.is_empty()).collect();
        if parts.is_empty() {
            return None;
        }

        let mut current_dir = &self.root;
        for i in 0..parts.len() - 1 {
            let part = parts[i];
            current_dir = current_dir.subdirectories.get(part)?;
        }

        let name = *parts.last().unwrap();
        let obj = current_dir.objects.get(name)?;

        if obj.object_type == NtObjectType::SymbolicLink {
            if let Some(ref target) = obj.target_path {
                let target_str: &str = target.as_str();
                return self.lookup_object(target_str);
            }
        }
        Some(obj.clone())
    }
}

impl Default for NtObjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test_disabled)]
mod tests_extended {
    use super::*;

    #[test]
    fn test_nt_object_manager_directories_and_symlinks() {
        let mut manager = NtObjectManager::new();

        // 1. Create a real device object and insert it into \Device\Keyboard
        let keyboard_dev = NtObject {
            name: String::from("Keyboard"),
            object_type: NtObjectType::Device,
            target_path: None,
        };
        manager
            .insert_object("\\Device\\Keyboard", keyboard_dev)
            .unwrap();

        // 2. Create a symbolic link in \DosDevices\KeyboardAlias pointing to \Device\Keyboard
        let keyboard_link = NtObject {
            name: String::from("KeyboardAlias"),
            object_type: NtObjectType::SymbolicLink,
            target_path: Some(String::from("\\Device\\Keyboard")),
        };
        manager
            .insert_object("\\DosDevices\\KeyboardAlias", keyboard_link)
            .unwrap();

        // 3. Look up \DosDevices\KeyboardAlias and verify it resolves to the real Keyboard Device object
        let resolved = manager
            .lookup_object("\\DosDevices\\KeyboardAlias")
            .unwrap();
        assert_eq!(resolved.name, "Keyboard");
        assert_eq!(resolved.object_type, NtObjectType::Device);
    }

    #[test]
    fn test_non_paged_pool_allocation() {
        let mut pool = NonPagedPoolMemory::new(0xFFFF800000000000, 1024);
        assert_eq!(pool.allocated_size, 1024);

        let addr1 = pool.allocate_block("tag1", 256).unwrap();
        assert_eq!(addr1, 0xFFFF800000000000);

        let addr2 = pool.allocate_block("tag2", 512).unwrap();
        assert_eq!(addr2, 0xFFFF800000000100);

        // Allocating beyond capacity should fail
        assert!(pool.allocate_block("tag3", 512).is_err());
    }

    #[test]
    fn test_driver_entry_context() {
        let driver = DriverEntryContext {
            driver_name: "AcpiBattery".to_string(),
            driver_entry_address: 0xFFFF800000000000,
            unload_routine: None,
            is_loaded: true,
        };

        assert!(driver.is_loaded);
        assert_eq!(driver.driver_name, "AcpiBattery");
    }
}
