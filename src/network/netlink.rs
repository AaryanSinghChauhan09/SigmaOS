// SigmaOS Linux Netlink Socket Subsystem
// Reference: Linux <linux/netlink.h>, <linux/rtnetlink.h>

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub const NETLINK_ROUTE: i32 = 0;
pub const NETLINK_FIREWALL: i32 = 3;
pub const NETLINK_INET_DIAG: i32 = 4;
pub const NETLINK_NFLOG: i32 = 5;
pub const NETLINK_XFRM: i32 = 6;
pub const NETLINK_SELINUX: i32 = 7;
pub const NETLINK_AUDIT: i32 = 9;
pub const NETLINK_FIB_LOOKUP: i32 = 10;
pub const NETLINK_CONNECTOR: i32 = 11;
pub const NETLINK_NETFILTER: i32 = 12;
pub const NETLINK_KOBJECT_UEVENT: i32 = 15;
pub const NETLINK_GENERIC: i32 = 16;
pub const NETLINK_CRYPTO: i32 = 21;

// Netlink Message Flags
pub const NLM_F_REQUEST: u16 = 0x01;
pub const NLM_F_MULTI: u16 = 0x02;
pub const NLM_F_ACK: u16 = 0x04;
pub const NLM_F_ECHO: u16 = 0x08;
pub const NLM_F_ROOT: u16 = 0x100;
pub const NLM_F_MATCH: u16 = 0x200;
pub const NLM_F_ATOMIC: u16 = 0x400;
pub const NLM_F_DUMP: u16 = NLM_F_ROOT | NLM_F_MATCH;

// Routing Netlink types
pub const RTM_NEWLINK: u16 = 16;
pub const RTM_DELLINK: u16 = 17;
pub const RTM_GETLINK: u16 = 18;
pub const RTM_NEWADDR: u16 = 20;
pub const RTM_DELADDR: u16 = 21;
pub const RTM_GETADDR: u16 = 22;
pub const RTM_NEWROUTE: u16 = 24;
pub const RTM_DELROUTE: u16 = 25;
pub const RTM_GETROUTE: u16 = 26;

/// Standard Netlink Message Header (16 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NlMsgHdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

impl NlMsgHdr {
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() < 16 {
            return Err("Buffer too short for Netlink header");
        }
        let nlmsg_len = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        let nlmsg_type = u16::from_le_bytes(bytes[4..6].try_into().unwrap());
        let nlmsg_flags = u16::from_le_bytes(bytes[6..8].try_into().unwrap());
        let nlmsg_seq = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let nlmsg_pid = u32::from_le_bytes(bytes[12..16].try_into().unwrap());

        Ok(Self {
            nlmsg_len,
            nlmsg_type,
            nlmsg_flags,
            nlmsg_seq,
            nlmsg_pid,
        })
    }

    pub fn to_bytes(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0..4].copy_from_slice(&self.nlmsg_len.to_le_bytes());
        buf[4..6].copy_from_slice(&self.nlmsg_type.to_le_bytes());
        buf[6..8].copy_from_slice(&self.nlmsg_flags.to_le_bytes());
        buf[8..12].copy_from_slice(&self.nlmsg_seq.to_le_bytes());
        buf[12..16].copy_from_slice(&self.nlmsg_pid.to_le_bytes());
        buf
    }
}

/// Interface information message (ifinfomsg)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IfInfoMsg {
    pub ifi_family: u8,
    pub ifi_type: u16,
    pub ifi_index: i32,
    pub ifi_flags: u32,
    pub ifi_change: u32,
}

/// Netlink Socket Bus Router
pub struct NetlinkSocketRouter {
    protocol: i32,
    subscribers: BTreeMap<u32, u32>, // pid -> multicast groups mask
    message_queue: Vec<(NlMsgHdr, Vec<u8>)>,
}

impl NetlinkSocketRouter {
    pub fn new(protocol: i32) -> Self {
        Self {
            protocol,
            subscribers: BTreeMap::new(),
            message_queue: Vec::new(),
        }
    }

    /// Bind a userspace PID to this netlink socket with multicast groups
    pub fn bind(&mut self, pid: u32, groups: u32) {
        self.subscribers.insert(pid, groups);
    }

    /// Post a kernel netlink message
    pub fn send_message(&mut self, msg_type: u16, flags: u16, seq: u32, pid: u32, payload: &[u8]) {
        let len = (16 + payload.len()) as u32;
        let hdr = NlMsgHdr {
            nlmsg_len: len,
            nlmsg_type: msg_type,
            nlmsg_flags: flags,
            nlmsg_seq: seq,
            nlmsg_pid: pid,
        };
        self.message_queue.push((hdr, payload.to_vec()));
    }

    /// Receive pending messages for a given pid
    pub fn receive(&mut self, pid: u32) -> Vec<(NlMsgHdr, Vec<u8>)> {
        let mut matching = Vec::new();
        self.message_queue.retain(|(hdr, payload)| {
            if hdr.nlmsg_pid == pid || hdr.nlmsg_pid == 0 {
                matching.push((*hdr, payload.clone()));
                false
            } else {
                true
            }
        });
        matching
    }

    pub fn queue_depth(&self) -> usize {
        self.message_queue.len()
    }
}
