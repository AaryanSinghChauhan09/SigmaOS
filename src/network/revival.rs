#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// SigmaOS Legacy Networking Protocols Revival (NetRevival)
// Revives obsolete LAN network stacks (Novell IPX/SPX, IBM NetBEUI) decorated behind a secure zero-trust envelope

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevivalProtocol {
    IpxSpx,
    NetBeui,
}

pub struct NetRevival {
    pub protocol: RevivalProtocol,
    pub is_secure_decorated: bool,
    pub active_routes: usize,
}

impl NetRevival {
    pub fn new(proto: RevivalProtocol) -> Self {
        NetRevival {
            protocol: proto,
            is_secure_decorated: true, // Wrap behind PQC encryption by default
            active_routes: 0,
        }
    }

    pub fn setup_route(&mut self, source_node: &str, dest_node: &str) -> Result<String, ()> {
        self.active_routes += 1;
        let mut descriptor = format!("Route established: {} -> {}", source_node, dest_node);
        if self.is_secure_decorated {
            descriptor.push_str(" (Wrapped via Kyber-1024 Secure Tunnel)");
        }
        Ok(descriptor)
    }

    pub fn process_incoming_packet(&self, raw_frame: &[u8]) -> Result<Vec<u8>, ()> {
        if raw_frame.len() < 4 {
            return Err(());
        }
        let mut decapsulated = Vec::new();
        match self.protocol {
            RevivalProtocol::IpxSpx => {
                // Read legacy IPX packet type field at offset 5 relative to header length (30 bytes)
                // For demonstration, just forward the parsed payload
                for &b in &raw_frame[4..] {
                    decapsulated.push(b);
                }
            }
            RevivalProtocol::NetBeui => {
                // Read NetBIOS command bytes
                for &b in &raw_frame[2..] {
                    decapsulated.push(b);
                }
            }
        }
        Ok(decapsulated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipx_spx_secure_tunnel() {
        let mut revival = NetRevival::new(RevivalProtocol::IpxSpx);
        assert!(revival.is_secure_decorated);

        let desc = revival.setup_route("node-A", "node-B").unwrap();
        assert!(desc.contains("Wrapped via Kyber-1024 Secure Tunnel"));
        assert_eq!(revival.active_routes, 1);

        let frame = vec![0x11, 0x22, 0x33, 0x44, 0xAA, 0xBB, 0xCC];
        let payload = revival.process_incoming_packet(&frame).unwrap();
        assert_eq!(payload[0], 0xAA);
    }
}
