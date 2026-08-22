// SigmaOS Windows/Linux/BSD-Inspired Advanced Object Manager (Obp)
// Implements advanced Object Manager namespaces, symbolic link translation,
// driver entry contexts, dynamic unloading, and Non-Paged Pool memory tracking.

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::collections::HashMap;

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

    pub fn load_driver(&mut self, unload: fn(usize) -> Result<(), ObjectError>) -> Result<(), ObjectError> {
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
        self.symbolic_links.insert(alias.to_string(), real_path.to_string());
    }

    pub fn resolve_path(&self, path: &str) -> String {
        if let Some(real_path) = self.symbolic_links.get(path) {
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

#[cfg(test)]
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
