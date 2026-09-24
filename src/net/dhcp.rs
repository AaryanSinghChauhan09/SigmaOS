/// SigmaOS DHCP Client (Phase 2 Networking)
/// Inspired by Linux Mint's network-manager DHCP integration

use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum DhcpState {
    Init,
    Selecting,
    Requesting,
    Bound,
    Renewing,
    Rebinding,
}

pub struct DhcpLease {
    pub client_ip: [u8; 4],
    pub server_ip: [u8; 4],
    pub gateway: [u8; 4],
    pub subnet_mask: [u8; 4],
    pub dns_servers: Vec<[u8; 4]>,
    pub lease_time_secs: u32,
}

pub struct DhcpClient {
    pub state: DhcpState,
    pub mac: [u8; 6],
    pub lease: Option<DhcpLease>,
    pub transaction_id: u32,
}

impl DhcpClient {
    pub fn new(mac: [u8; 6]) -> Self {
        Self {
            state: DhcpState::Init,
            mac,
            lease: None,
            transaction_id: 0x12345678,
        }
    }

    pub fn send_discover(&mut self) -> Vec<u8> {
        self.state = DhcpState::Selecting;
        // BOOTP/DHCP discover message (simplified)
        let mut pkt = vec![1u8; 300]; // op=BOOTREQUEST
        pkt[0] = 1; // BOOTREQUEST
        pkt[1] = 1; // HW type ethernet
        pkt[2] = 6; // HW addr len
        pkt[4..8].copy_from_slice(&self.transaction_id.to_be_bytes());
        pkt[28..34].copy_from_slice(&self.mac);
        pkt
    }

    pub fn handle_offer(&mut self, offered_ip: [u8; 4], server_ip: [u8; 4]) {
        if self.state == DhcpState::Selecting {
            self.state = DhcpState::Requesting;
            self.lease = Some(DhcpLease {
                client_ip: offered_ip,
                server_ip,
                gateway: [0; 4],
                subnet_mask: [255, 255, 255, 0],
                dns_servers: vec![[8, 8, 8, 8]],
                lease_time_secs: 86400,
            });
        }
    }

    pub fn handle_ack(&mut self) {
        if self.state == DhcpState::Requesting {
            self.state = DhcpState::Bound;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dhcp_lifecycle() {
        let mut client = DhcpClient::new([0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
        assert_eq!(client.state, DhcpState::Init);
        let _discover = client.send_discover();
        assert_eq!(client.state, DhcpState::Selecting);
        client.handle_offer([192, 168, 1, 100], [192, 168, 1, 1]);
        assert_eq!(client.state, DhcpState::Requesting);
        client.handle_ack();
        assert_eq!(client.state, DhcpState::Bound);
        let lease = client.lease.as_ref().unwrap();
        assert_eq!(lease.client_ip, [192, 168, 1, 100]);
    }
}
