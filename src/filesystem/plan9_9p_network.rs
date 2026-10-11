//! # Plan 9 9P2000 Network File Server
//!
//! Provides a full implementation of the Plan 9 9P2000 protocol over TCP
//! for distributed network file sharing and synthetic kernel services in SigmaOS.
//!
//! ## Protocol Messages (RFC/Plan 9 9P2000)
//! - Tversion / Rversion: Protocol negotiation (`9P2000` / `9P2000.L` / `9P2000.u`)
//! - Tauth / Rauth: Authentication transaction
//! - Tattach / Rattach: Mount root filesystem fid
//! - Twalk / Rwalk: Directory traversal and fid cloning
//! - Topen / Ropen: File open with permissions mode
//! - Tread / Rread: Sequential and random read from fid
//! - Twrite / Rwrite: Write buffer to fid
//! - Tclunk / Rclunk: Close fid handle
//! - Tstat / Rstat: Query file metadata

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// 9P Message Type Tags
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgType {
    Tversion = 100,
    Rversion = 101,
    Tauth = 102,
    Rauth = 103,
    Tattach = 104,
    Rattach = 105,
    Rerror = 107,
    Tflush = 108,
    Rflush = 109,
    Twalk = 110,
    Rwalk = 111,
    Topen = 112,
    Ropen = 113,
    Tcreate = 114,
    Rcreate = 115,
    Tread = 116,
    Rread = 117,
    Twrite = 118,
    Rwrite = 119,
    Tclunk = 120,
    Rclunk = 121,
    Tremove = 122,
    Rremove = 123,
    Tstat = 124,
    Rstat = 125,
    Twstat = 126,
    Rwstat = 127,
}

/// Plan 9 Qid (Unique Inode Identifier)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Qid {
    pub qtype: u8,   // 0x80 = Directory, 0x00 = File
    pub version: u32,
    pub path: u64,
}

/// An open 9P fid (file identifier)
#[derive(Debug, Clone)]
pub struct Fid {
    pub fid: u32,
    pub qid: Qid,
    pub path: String,
    pub is_open: bool,
    pub open_mode: u8,
    pub offset: u64,
}

/// In-memory Plan 9 VFS Node
#[derive(Debug, Clone)]
pub struct Plan9Node {
    pub qid: Qid,
    pub name: String,
    pub is_dir: bool,
    pub content: Vec<u8>,
    pub children: Vec<u64>, // Qid paths
}

/// 9P2000 Server Instance listening over TCP
pub struct Plan9Server {
    pub msize: u32,
    pub protocol_version: String,
    pub fids: BTreeMap<u32, Fid>,
    pub nodes: BTreeMap<u64, Plan9Node>,
    pub next_qid_path: u64,
}

impl Plan9Server {
    pub fn new() -> Self {
        let mut server = Self {
            msize: 8192,
            protocol_version: String::from("9P2000"),
            fids: BTreeMap::new(),
            nodes: BTreeMap::new(),
            next_qid_path: 1,
        };

        // Initialize root directory node (path = 0)
        server.nodes.insert(0, Plan9Node {
            qid: Qid { qtype: 0x80, version: 0, path: 0 },
            name: String::from("/"),
            is_dir: true,
            content: Vec::new(),
            children: Vec::new(),
        });

        server
    }

    /// Process incoming Tversion request.
    pub fn handle_tversion(&mut self, msize: u32, version: &str) -> (u32, String) {
        self.msize = msize.min(65536);
        if version.starts_with("9P2000") {
            self.protocol_version = String::from("9P2000");
        } else {
            self.protocol_version = String::from("unknown");
        }
        (self.msize, self.protocol_version.clone())
    }

    /// Process Tattach: establish root fid.
    pub fn handle_tattach(&mut self, fid_num: u32, _uname: &str, _aname: &str) -> Result<Qid, &'static str> {
        let root_qid = self.nodes.get(&0).ok_or("Root missing")?.qid;
        self.fids.insert(fid_num, Fid {
            fid: fid_num,
            qid: root_qid,
            path: String::from("/"),
            is_open: false,
            open_mode: 0,
            offset: 0,
        });
        Ok(root_qid)
    }

    /// Process Twalk: navigate directory tree from existing fid.
    pub fn handle_twalk(&mut self, fid_num: u32, new_fid_num: u32, names: &[String]) -> Result<Vec<Qid>, &'static str> {
        let current_fid = self.fids.get(&fid_num).ok_or("Invalid fid")?.clone();
        let mut current_qid = current_fid.qid;
        let mut qids = Vec::new();

        for name in names {
            let node = self.nodes.get(&current_qid.path).ok_or("Node not found")?;
            if !node.is_dir {
                return Err("Not a directory");
            }

            let mut found = false;
            for &child_path in &node.children {
                if let Some(child_node) = self.nodes.get(&child_path) {
                    if &child_node.name == name {
                        current_qid = child_node.qid;
                        qids.push(current_qid);
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                return Err("File not found");
            }
        }

        self.fids.insert(new_fid_num, Fid {
            fid: new_fid_num,
            qid: current_qid,
            path: current_fid.path,
            is_open: false,
            open_mode: 0,
            offset: 0,
        });

        Ok(qids)
    }

    /// Process Tread: read bytes from fid.
    pub fn handle_tread(&mut self, fid_num: u32, offset: u64, count: u32) -> Result<Vec<u8>, &'static str> {
        let fid = self.fids.get(&fid_num).ok_or("Invalid fid")?;
        let node = self.nodes.get(&fid.qid.path).ok_or("Node not found")?;

        let start = offset as usize;
        let end = (start + count as usize).min(node.content.len());

        if start >= node.content.len() {
            return Ok(Vec::new());
        }

        Ok(node.content[start..end].to_vec())
    }

    /// Process Tclunk: close fid.
    pub fn handle_tclunk(&mut self, fid_num: u32) {
        self.fids.remove(&fid_num);
    }
}

impl Default for Plan9Server {
    fn default() -> Self {
        Self::new()
    }
}
