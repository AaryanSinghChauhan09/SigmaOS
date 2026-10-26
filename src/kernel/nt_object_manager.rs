// SPDX-License-Identifier: GPL-2.0-or-later
// Principle: HIERARCHICAL NAMESPACE, CAPABILITY HANDLES, SECURITY DESCRIPTORS, OBJECT LIFECYCLE

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

/// NT object type identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Directory,
    SymbolicLink,
    Device,
    Driver,
    File,
    Event,
    Mutex,
    Semaphore,
    Section,
    Key, // Registry key
    Token,
    Process,
    Thread,
    IoCompletion,
}

/// Handle to an object — opaque integer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ObjectHandle(pub u64);

/// Simplified NT security descriptor
#[derive(Debug, Clone)]
pub struct SecurityDescriptor {
    pub owner_sid: u32,
    pub group_sid: u32,
    pub dacl_present: bool,
    pub access_mask: u32,
}

impl SecurityDescriptor {
    pub fn world_readable() -> Self {
        Self {
            owner_sid: 0,
            group_sid: 0,
            dacl_present: true,
            access_mask: 0x00120089,
        }
    }

    pub fn kernel_only() -> Self {
        Self {
            owner_sid: 0,
            group_sid: 0,
            dacl_present: true,
            access_mask: 0x001F01FF,
        }
    }
}

/// NT Object header — metadata for every kernel object
#[derive(Debug, Clone)]
pub struct ObjectHeader {
    pub object_type: ObjectType,
    pub name: String,
    pub ref_count: u32,
    pub handle_count: u32,
    pub security: SecurityDescriptor,
}

impl ObjectHeader {
    pub fn new(object_type: ObjectType, name: String, security: SecurityDescriptor) -> Self {
        Self {
            object_type,
            name,
            ref_count: 1,
            handle_count: 0,
            security,
        }
    }
}

/// A generic NT object (header + opaque body bytes)
#[derive(Debug, Clone)]
pub struct NtObject {
    pub header: ObjectHeader,
    pub body: Vec<u8>,
    pub symlink_target: Option<String>,
}

impl NtObject {
    pub fn new(header: ObjectHeader, body: Vec<u8>) -> Self {
        Self {
            header,
            body,
            symlink_target: None,
        }
    }

    pub fn symlink(name: String, target: String, security: SecurityDescriptor) -> Self {
        let header = ObjectHeader::new(ObjectType::SymbolicLink, name, security);
        Self {
            header,
            body: Vec::new(),
            symlink_target: Some(target),
        }
    }
}

/// Object directory — a node in the NT namespace tree
#[derive(Debug)]
pub struct ObjectDirectory {
    pub name: String,
    pub children: BTreeMap<String, NtObject>,
    pub subdirs: BTreeMap<String, ObjectDirectory>,
    pub security: SecurityDescriptor,
}

impl ObjectDirectory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            children: BTreeMap::new(),
            subdirs: BTreeMap::new(),
            security: SecurityDescriptor::kernel_only(),
        }
    }

    pub fn insert_object(&mut self, obj: NtObject) {
        self.children.insert(obj.header.name.clone(), obj);
    }

    pub fn insert_subdir(&mut self, dir: ObjectDirectory) {
        self.subdirs.insert(dir.name.clone(), dir);
    }

    pub fn lookup(&self, name: &str) -> Option<&NtObject> {
        self.children.get(name)
    }
}

/// NT Object Manager — the root of the kernel namespace
pub struct ObjectManager {
    root: ObjectDirectory,
    handles: BTreeMap<ObjectHandle, (Vec<String>, String)>, // handle -> (path components, object name)
    next_handle: u64,
}

impl ObjectManager {
    pub fn new() -> Self {
        let mut root = ObjectDirectory::new(String::from("\\"));
        // Pre-create standard NT directories
        for dir_name in &[
            "Device",
            "Driver",
            "FileSystem",
            "Registry",
            "KernelObjects",
            "Sessions",
        ] {
            root.insert_subdir(ObjectDirectory::new(String::from(*dir_name)));
        }
        Self {
            root,
            handles: BTreeMap::new(),
            next_handle: 1,
        }
    }

    /// Navigate to a subdirectory, creating it if necessary
    pub fn create_directory(&mut self, path: &str) -> Result<(), &'static str> {
        let parts = Self::split_path(path);
        let mut cur = &mut self.root;
        for part in parts {
            if part.is_empty() {
                continue;
            }
            if !cur.subdirs.contains_key(part.as_str()) {
                cur.subdirs
                    .insert(part.clone(), ObjectDirectory::new(part.clone()));
            }
            cur = cur
                .subdirs
                .get_mut(part.as_str())
                .ok_or("navigation error")?;
        }
        Ok(())
    }

    /// Insert an object at the given directory path
    pub fn insert_object(&mut self, dir_path: &str, obj: NtObject) -> Result<(), &'static str> {
        let parts = Self::split_path(dir_path);
        let mut cur = &mut self.root;
        for part in &parts {
            if part.is_empty() {
                continue;
            }
            cur = cur
                .subdirs
                .get_mut(part.as_str())
                .ok_or("directory not found")?;
        }
        cur.insert_object(obj);
        Ok(())
    }

    /// Open an object by full NT path, returning an ObjectHandle
    pub fn open_object(&mut self, full_path: &str) -> Result<ObjectHandle, &'static str> {
        let mut parts = Self::split_path(full_path);
        let obj_name = parts.pop().ok_or("empty path")?;
        let dir_parts = parts;

        // Verify the path exists
        let cur = self.navigate(&dir_parts)?;
        if cur.lookup(obj_name.as_str()).is_none() {
            return Err("object not found");
        }

        let handle = ObjectHandle(self.next_handle);
        self.next_handle += 1;
        self.handles.insert(handle, (dir_parts, obj_name));
        Ok(handle)
    }

    pub fn close_handle(&mut self, handle: ObjectHandle) -> Result<(), &'static str> {
        self.handles.remove(&handle).ok_or("invalid handle")?;
        Ok(())
    }

    /// Resolve a symbolic link to its target path
    pub fn resolve_symlink(&self, path: &str) -> Result<String, &'static str> {
        let mut parts = Self::split_path(path);
        let obj_name = parts.pop().ok_or("empty path")?;
        let cur = self.navigate_ref(&parts)?;
        match cur.lookup(obj_name.as_str()) {
            Some(obj) if obj.header.object_type == ObjectType::SymbolicLink => {
                obj.symlink_target.clone().ok_or("symlink has no target")
            }
            Some(_) => Err("not a symbolic link"),
            None => Err("object not found"),
        }
    }

    fn navigate_ref<'a>(&'a self, parts: &[String]) -> Result<&'a ObjectDirectory, &'static str> {
        let mut cur = &self.root;
        for part in parts {
            if part.is_empty() {
                continue;
            }
            cur = cur
                .subdirs
                .get(part.as_str())
                .ok_or("directory not found")?;
        }
        Ok(cur)
    }

    fn navigate(&mut self, parts: &[String]) -> Result<&mut ObjectDirectory, &'static str> {
        let mut cur = &mut self.root;
        for part in parts {
            if part.is_empty() {
                continue;
            }
            cur = cur
                .subdirs
                .get_mut(part.as_str())
                .ok_or("directory not found")?;
        }
        Ok(cur)
    }

    fn split_path(path: &str) -> Vec<String> {
        path.split('\\').map(|s| String::from(s)).collect()
    }

    pub fn handle_count(&self) -> usize {
        self.handles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_device_obj(name: &str) -> NtObject {
        NtObject::new(
            ObjectHeader::new(
                ObjectType::Device,
                String::from(name),
                SecurityDescriptor::kernel_only(),
            ),
            alloc::vec![0xDE, 0xAD],
        )
    }

    #[test]
    fn test_insert_and_open_object() {
        let mut om = ObjectManager::new();
        om.insert_object("\\Device", make_device_obj("Disk0"))
            .unwrap();
        let h = om.open_object("\\Device\\Disk0").unwrap();
        assert!(h.0 >= 1);
        om.close_handle(h).unwrap();
        assert_eq!(om.handle_count(), 0);
    }

    #[test]
    fn test_create_nested_directory() {
        let mut om = ObjectManager::new();
        om.create_directory("\\FileSystem\\Ntfs").unwrap();
        let r = om.create_directory("\\FileSystem\\Ntfs\\Instances");
        assert!(r.is_ok());
    }

    #[test]
    fn test_symlink_resolution() {
        let mut om = ObjectManager::new();
        let link = NtObject::symlink(
            String::from("C:"),
            String::from("\\Device\\HarddiskVolume1"),
            SecurityDescriptor::world_readable(),
        );
        om.create_directory("\\GLOBAL??").unwrap();
        om.insert_object("\\GLOBAL??", link).unwrap();
        let target = om.resolve_symlink("\\GLOBAL??\\C:").unwrap();
        assert_eq!(target, "\\Device\\HarddiskVolume1");
    }

    #[test]
    fn test_close_invalid_handle_fails() {
        let mut om = ObjectManager::new();
        let result = om.close_handle(ObjectHandle(9999));
        assert!(result.is_err());
    }

    #[test]
    fn test_open_nonexistent_fails() {
        let mut om = ObjectManager::new();
        let result = om.open_object("\\Device\\Nonexistent");
        assert!(result.is_err());
    }
}
