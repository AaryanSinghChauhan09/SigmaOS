#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! BSD Pledge inspired security mechanism
use std::vec::Vec;
use std::string::String;

#[derive(Debug, Clone, PartialEq)]
pub enum PledgePromise {
    Stdio,
    Rpath,
    Wpath,
    Cpath,
    Dpath,
    Tmppath,
    Inet,
    Mcast,
    Fattr,
    Chown,
    Fattr2,
    Route,
    Wroute,
    Proc,
    Exec,
    Thread,
    Id,
    Pf,
    Pcap,
    Sendfd,
    Recvfd,
}

pub struct PledgeState {
    promises: Vec<PledgePromise>,
    active: bool,
}

impl PledgeState {
    pub fn new() -> Self {
        Self {
            promises: Vec::new(),
            active: false,
        }
    }

    pub fn pledge(&mut self, promises: Vec<PledgePromise>) -> Result<(), &'static str> {
        if self.active {
            // Can only reduce privileges once active
            for promise in &promises {
                if !self.promises.contains(promise) {
                    return Err("Cannot increase privileges after initial pledge");
                }
            }
        }
        self.promises = promises;
        self.active = true;
        Ok(())
    }

    pub fn check(&self, promise: &PledgePromise) -> Result<(), &'static str> {
        if self.active && !self.promises.contains(promise) {
            return Err("Pledge violation");
        }
        Ok(())
    }
}
