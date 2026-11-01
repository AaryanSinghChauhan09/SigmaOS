// SigmaOS Devtmpfs & Udev Device Node Auto-population Subsystem
// Reference: Linux drivers/base/devtmpfs.c, systemd-udevd

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// Kernel uevent action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UeventAction {
    Add,
    Remove,
    Change,
    Move,
    Online,
    Offline,
    Bind,
    Unbind,
}

impl UeventAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Remove => "remove",
            Self::Change => "change",
            Self::Move => "move",
            Self::Online => "online",
            Self::Offline => "offline",
            Self::Bind => "bind",
            Self::Unbind => "unbind",
        }
    }
}

/// Kernel uevent structure sent to udev netlink socket
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelUevent {
    pub action: UeventAction,
    pub devpath: String,
    pub subsystem: String,
    pub devname: String,
    pub major: u32,
    pub minor: u32,
    pub seqnum: u64,
}

/// Device Node representation in /dev
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevNode {
    pub name: String,
    pub major: u32,
    pub minor: u32,
    pub is_block: bool,
    pub mode: u32, // standard UNIX file permissions (e.g. 0o666, 0o660)
    pub uid: u32,
    pub gid: u32,
    pub symlinks: Vec<String>,
}

/// Devtmpfs Filesystem Engine
pub struct DevTmpFs {
    nodes: BTreeMap<String, DevNode>,
    seqnum_counter: u64,
    uevent_log: Vec<KernelUevent>,
}

impl DevTmpFs {
    pub fn new() -> Self {
        let mut dev = Self {
            nodes: BTreeMap::new(),
            seqnum_counter: 1,
            uevent_log: Vec::new(),
        };

        // Populate base standard POSIX/Linux devices
        dev.create_node("null", 1, 3, false, 0o666, 0, 0, "mem");
        dev.create_node("zero", 1, 5, false, 0o666, 0, 0, "mem");
        dev.create_node("full", 1, 7, false, 0o666, 0, 0, "mem");
        dev.create_node("random", 1, 8, false, 0o666, 0, 0, "mem");
        dev.create_node("urandom", 1, 9, false, 0o666, 0, 0, "mem");
        dev.create_node("kmsg", 1, 11, false, 0o644, 0, 0, "mem");
        dev.create_node("tty", 5, 0, false, 0o666, 0, 5, "tty");
        dev.create_node("console", 5, 1, false, 0o600, 0, 0, "tty");
        dev.create_node("ptmx", 5, 2, false, 0o666, 0, 5, "tty");
        dev.create_node("fuse", 10, 229, false, 0o666, 0, 0, "misc");
        dev.create_node("rtc0", 254, 0, false, 0o664, 0, 0, "rtc");

        dev
    }

    /// Creates a device node and broadcasts an 'add' uevent
    pub fn create_node(
        &mut self,
        name: &str,
        major: u32,
        minor: u32,
        is_block: bool,
        mode: u32,
        uid: u32,
        gid: u32,
        subsystem: &str,
    ) {
        let node = DevNode {
            name: String::from(name),
            major,
            minor,
            is_block,
            mode,
            uid,
            gid,
            symlinks: Vec::new(),
        };

        self.nodes.insert(String::from(name), node);

        let seq = self.seqnum_counter;
        self.seqnum_counter += 1;

        let uevent = KernelUevent {
            action: UeventAction::Add,
            devpath: alloc::format!("/sys/devices/virtual/{}/{}", subsystem, name),
            subsystem: String::from(subsystem),
            devname: String::from(name),
            major,
            minor,
            seqnum: seq,
        };

        self.uevent_log.push(uevent);
    }

    /// Removes a device node and broadcasts a 'remove' uevent
    pub fn remove_node(&mut self, name: &str, subsystem: &str) -> bool {
        if let Some(removed) = self.nodes.remove(name) {
            let seq = self.seqnum_counter;
            self.seqnum_counter += 1;

            let uevent = KernelUevent {
                action: UeventAction::Remove,
                devpath: alloc::format!("/sys/devices/virtual/{}/{}", subsystem, name),
                subsystem: String::from(subsystem),
                devname: String::from(name),
                major: removed.major,
                minor: removed.minor,
                seqnum: seq,
            };

            self.uevent_log.push(uevent);
            true
        } else {
            false
        }
    }

    pub fn get_node(&self, name: &str) -> Option<&DevNode> {
        self.nodes.get(name)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn recent_events(&self) -> &[KernelUevent] {
        &self.uevent_log
    }
}
