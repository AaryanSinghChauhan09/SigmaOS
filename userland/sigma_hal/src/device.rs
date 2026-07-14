use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum UeventAction {
    Add,
    Remove,
    Change,
}

#[derive(Debug, Clone)]
pub struct Uevent {
    pub action: UeventAction,
    pub devpath: String,
    pub subsystem: String,
    pub devname: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeviceNode {
    pub path: String,
    pub major: u32,
    pub minor: u32,
    pub selinux_context: String, // Enforced by sigma_security
}

/// DeviceManager listens to kernel netlink uevents and dynamically creates
/// or removes device nodes in the `devtmpfs` equivalent.
pub struct DeviceManager {
    nodes: HashMap<String, DeviceNode>,
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Process a kernel uevent (e.g., from a netlink socket).
    pub fn handle_uevent(&mut self, event: Uevent) -> Result<(), String> {
        match event.action {
            UeventAction::Add => {
                if let Some(name) = event.devname {
                    let path = format!("/dev/{}", name);
                    // In a real system, we'd use `mknod` here.
                    let node = DeviceNode {
                        path: path.clone(),
                        major: 8, // Stub
                        minor: 0,
                        selinux_context: format!("system_u:object_r:{}_device_t:s0", event.subsystem),
                    };
                    self.nodes.insert(path, node);
                }
            }
            UeventAction::Remove => {
                if let Some(name) = event.devname {
                    let path = format!("/dev/{}", name);
                    self.nodes.remove(&path);
                }
            }
            UeventAction::Change => {
                // Update rules
            }
        }
        Ok(())
    }
}
