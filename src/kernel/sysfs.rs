// Sysfs - Kernel Parameter Management
// Inspired by Linux sysfs for kernel parameter exposure

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};

/// Sysfs attribute type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysfsAttributeType {
    String,
    Integer,
    Boolean,
    Hex,
}

/// Sysfs attribute
#[derive(Debug, Clone)]
pub struct SysfsAttribute {
    pub name: String,
    pub attr_type: SysfsAttributeType,
    pub value: String,
    pub permissions: u32, // 0444, 0644, etc.
    pub writable: bool,
}

/// Sysfs kobject (kernel object)
#[derive(Debug, Clone)]
pub struct SysfsKobject {
    pub name: String,
    pub parent: Option<u64>,
    pub attributes: HashMap<String, SysfsAttribute>,
    pub children: Vec<u64>,
}

/// Sysfs directory
pub struct Sysfs {
    next_kobject_id: AtomicU64,
    kobjects: HashMap<u64, SysfsKobject>,
    root_kobject: u64,
}

impl Sysfs {
    pub fn new() -> Self {
        let mut sysfs = Self {
            next_kobject_id: AtomicU64::new(1),
            kobjects: HashMap::new(),
            root_kobject: 1,
        };
        
        // Create root kobject
        let root = SysfsKobject {
            name: "sys".to_string(),
            parent: None,
            attributes: HashMap::new(),
            children: Vec::new(),
        };
        
        sysfs.kobjects.insert(1, root);
        sysfs
    }

    /// Create a kobject
    pub fn create_kobject(&mut self, name: String, parent_id: Option<u64>) -> u64 {
        let id = self.next_kobject_id.fetch_add(1, Ordering::SeqCst);
        
        let kobject = SysfsKobject {
            name,
            parent: parent_id,
            attributes: HashMap::new(),
            children: Vec::new(),
        };
        
        if let Some(parent) = parent_id {
            if let Some(parent_kobj) = self.kobjects.get_mut(&parent) {
                parent_kobj.children.push(id);
            }
        }
        
        self.kobjects.insert(id, kobject);
        id
    }

    /// Add an attribute to a kobject
    pub fn add_attribute(&mut self, kobject_id: u64, attr: SysfsAttribute) -> Result<(), &'static str> {
        if let Some(kobject) = self.kobjects.get_mut(&kobject_id) {
            kobject.attributes.insert(attr.name.clone(), attr);
            Ok(())
        } else {
            Err("Kobject not found")
        }
    }

    /// Read an attribute
    pub fn read_attribute(&self, kobject_id: u64, attr_name: &str) -> Result<String, &'static str> {
        let kobject = self.kobjects.get(&kobject_id).ok_or("Kobject not found")?;
        let attr = kobject.attributes.get(attr_name).ok_or("Attribute not found")?;
        Ok(attr.value.clone())
    }

    /// Write an attribute
    pub fn write_attribute(&mut self, kobject_id: u64, attr_name: &str, value: String) -> Result<(), &'static str> {
        let kobject = self.kobjects.get_mut(&kobject_id).ok_or("Kobject not found")?;
        let attr = kobject.attributes.get_mut(attr_name).ok_or("Attribute not found")?;
        
        if !attr.writable {
            return Err("Attribute is read-only");
        }
        
        attr.value = value;
        Ok(())
    }

    /// Lookup kobject by path
    pub fn lookup(&self, path: &str) -> Option<u64> {
        if path == "/" || path == "" {
            return Some(self.root_kobject);
        }
        
        let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let mut current_id = self.root_kobject;
        
        for component in components {
            let kobject = self.kobjects.get(&current_id)?;
            
            let child_id = kobject.children.iter()
                .find(|&&id| {
                    if let Some(child) = self.kobjects.get(&id) {
                        child.name == component
                    } else {
                        false
                    }
                })
                .copied()?;
            
            current_id = child_id;
        }
        
        Some(current_id)
    }

    /// Get kobject children
    pub fn get_children(&self, kobject_id: u64) -> Vec<&SysfsKobject> {
        let kobject = self.kobjects.get(&kobject);
        
        match kobject {
            Some(kobj) => {
                kobj.children.iter()
                    .filter_map(|&id| self.kobjects.get(&id))
                    .collect()
            }
            None => Vec::new(),
        }
    }

    /// Get kobject attributes
    pub fn get_attributes(&self, kobject_id: u64) -> Vec<&SysfsAttribute> {
        let kobject = self.kobjects.get(&kobject_id);
        
        match kobject {
            Some(kobj) => {
                kobj.attributes.values().collect()
            }
            None => Vec::new(),
        }
    }

    /// Create standard kernel parameters
    pub fn create_kernel_params(&mut self) {
        // Create kernel directory
        let kernel_id = self.create_kobject("kernel".to_string(), Some(self.root_kobject));
        
        // Add hostname attribute
        self.add_attribute(kernel_id, SysfsAttribute {
            name: "hostname".to_string(),
            attr_type: SysfsAttributeType::String,
            value: "sigmaos".to_string(),
            permissions: 0o644,
            writable: true,
        }).unwrap();
        
        // Add osrelease attribute
        self.add_attribute(kernel_id, SysfsAttribute {
            name: "osrelease".to_string(),
            attr_type: SysfsAttributeType::String,
            value: "1.0.0".to_string(),
            permissions: 0o444,
            writable: false,
        }).unwrap();
        
        // Add version attribute
        self.add_attribute(kernel_id, SysfsAttribute {
            name: "version".to_string(),
            attr_type: SysfsAttributeType::String,
            value: "#1 SMP".to_string(),
            permissions: 0o444,
            writable: false,
        }).unwrap();
        
        // Create vm directory
        let vm_id = self.create_kobject("vm".to_string(), Some(self.root_kobject));
        
        // Add swappiness attribute
        self.add_attribute(vm_id, SysfsAttribute {
            name: "swappiness".to_string(),
            attr_type: SysfsAttributeType::Integer,
            value: "60".to_string(),
            permissions: 0o644,
            writable: true,
        }).unwrap();
        
        // Add dirty_ratio attribute
        self.add_attribute(vm_id, SysfsAttribute {
            name: "dirty_ratio".to_string(),
            attr_type: SysfsAttributeType::Integer,
            value: "20".to_string(),
            permissions: 0o644,
            writable: true,
        }).unwrap();
        
        // Create net directory
        let net_id = self.create_kobject("net".to_string(), Some(self.root_kobject));
        
        // Add ipv4 directory
        let ipv4_id = self.create_kobject("ipv4".to_string(), Some(net_id));
        
        // Add ip_forward attribute
        self.add_attribute(ipv4_id, SysfsAttribute {
            name: "ip_forward".to_string(),
            attr_type: SysfsAttributeType::Integer,
            value: "0".to_string(),
            permissions: 0o644,
            writable: true,
        }).unwrap();
    }

    /// Get kobject count
    pub fn kobject_count(&self) -> usize {
        self.kobjects.len()
    }

    /// Get root kobject
    pub fn root_kobject(&self) -> u64 {
        self.root_kobject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_kobject() {
        let mut sysfs = Sysfs::new();
        
        let id = sysfs.create_kobject("test".to_string(), Some(1));
        assert_eq!(id, 2);
        assert_eq!(sysfs.kobject_count(), 2);
    }

    #[test]
    fn test_add_attribute() {
        let mut sysfs = Sysfs::new();
        
        let id = sysfs.create_kobject("test".to_string(), Some(1));
        
        let attr = SysfsAttribute {
            name: "attr1".to_string(),
            attr_type: SysfsAttributeType::String,
            value: "value1".to_string(),
            permissions: 0o644,
            writable: true,
        };
        
        assert!(sysfs.add_attribute(id, attr).is_ok());
    }

    #[test]
    fn test_read_write_attribute() {
        let mut sysfs = Sysfs::new();
        
        let id = sysfs.create_kobject("test".to_string(), Some(1));
        
        let attr = SysfsAttribute {
            name: "attr1".to_string(),
            attr_type: SysfsAttributeType::String,
            value: "value1".to_string(),
            permissions: 0o644,
            writable: true,
        };
        
        sysfs.add_attribute(id, attr).unwrap();
        
        let value = sysfs.read_attribute(id, "attr1").unwrap();
        assert_eq!(value, "value1");
        
        sysfs.write_attribute(id, "attr1", "value2".to_string()).unwrap();
        
        let value = sysfs.read_attribute(id, "attr1").unwrap();
        assert_eq!(value, "value2");
    }

    #[test]
    fn test_lookup() {
        let mut sysfs = Sysfs::new();
        
        let id = sysfs.create_kobject("test".to_string(), Some(1));
        
        let found_id = sysfs.lookup("/test").unwrap();
        assert_eq!(found_id, id);
    }

    #[test]
    fn test_kernel_params() {
        let mut sysfs = Sysfs::new();
        
        sysfs.create_kernel_params();
        
        let kernel_id = sysfs.lookup("/kernel").unwrap();
        let hostname = sysfs.read_attribute(kernel_id, "hostname").unwrap();
        assert_eq!(hostname, "sigmaos");
    }

    #[test]
    fn test_readonly_attribute() {
        let mut sysfs = Sysfs::new();
        
        let id = sysfs.create_kobject("test".to_string(), Some(1));
        
        let attr = SysfsAttribute {
            name: "attr1".to_string(),
            attr_type: SysfsAttributeType::String,
            value: "value1".to_string(),
            permissions: 0o444,
            writable: false,
        };
        
        sysfs.add_attribute(id, attr).unwrap();
        
        assert!(sysfs.write_attribute(id, "attr1", "value2".to_string()).is_err());
    }
}
