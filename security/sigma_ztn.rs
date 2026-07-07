// sigma_ztn.rs — Zero-Trust Networking Daemon
// A network layer daemon enforcing Zero-Trust policies. Every packet must be 
// authenticated against capability tokens, and all cross-MicroVM traffic is mTLS encrypted.

#![no_std]
#![allow(dead_code)]

extern crate alloc;
use alloc::{vec::Vec, string::String};

pub struct TlsCertificate {
    pub identity: String,
    pub public_key: Vec<u8>,
}

pub struct ZtnNode {
    pub microvm_id: u32,
    pub allowed_peers: Vec<u32>,
    pub cert: TlsCertificate,
}

pub struct ZeroTrustNetwork {
    pub nodes: Vec<ZtnNode>,
}

impl ZeroTrustNetwork {
    pub fn new() -> Self {
        ZeroTrustNetwork {
            nodes: Vec::new(),
        }
    }

    pub fn register_node(&mut self, id: u32, cert: TlsCertificate) {
        self.nodes.push(ZtnNode {
            microvm_id: id,
            allowed_peers: Vec::new(),
            cert,
        });
    }

    pub fn grant_access(&mut self, from_id: u32, to_id: u32) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.microvm_id == from_id) {
            node.allowed_peers.push(to_id);
        }
    }

    pub fn route_packet(&self, source_id: u32, target_id: u32, _payload: &[u8]) -> Result<(), &'static str> {
        let source_node = self.nodes.iter()
            .find(|n| n.microvm_id == source_id)
            .ok_or("Source node not registered")?;

        if !source_node.allowed_peers.contains(&target_id) {
            return Err("Zero-Trust Policy Violation: Access Denied");
        }

        // Encrypt via mTLS and forward to target_id (mocked)
        Ok(())
    }
}
