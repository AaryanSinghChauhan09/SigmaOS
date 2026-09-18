use std::string::{String, ToString};
use std::vec::Vec;
use std::format;
// SigmaOS Legacy Network Protocols Adaptation Layer (LegacyProtocolAdapter)
// Designed for ancient serial/modem encapsulation protocols (SLIP, PPP, and legacy IPv4-only stack)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyProtocol {
    Slip, // Serial Line Internet Protocol
    Ppp,  // Point-to-Point Protocol
    Ipv4Only,
}

pub struct LegacyProtocolAdapter {
    pub protocol: LegacyProtocol,
    pub is_connected: bool,
    pub frame_counter: u64,
}

impl LegacyProtocolAdapter {
    pub fn new(proto: LegacyProtocol) -> Self {
        LegacyProtocolAdapter {
            protocol: proto,
            is_connected: false,
            frame_counter: 0,
        }
    }

    pub fn connect_link(&mut self) {
        self.is_connected = true;
    }

    pub fn disconnect_link(&mut self) {
        self.is_connected = false;
    }

    pub fn encapsulate_packet(&mut self, payload: &[u8]) -> Result<Vec<u8>, ()> {
        if !self.is_connected {
            return Err(());
        }
        self.frame_counter += 1;
        let mut framed = Vec::new();
        match self.protocol {
            LegacyProtocol::Slip => {
                framed.push(0xC0); // SLIP END character
                for &b in payload {
                    if b == 0xC0 {
                        framed.push(0xDB);
                        framed.push(0xDC);
                    } else if b == 0xDB {
                        framed.push(0xDB);
                        framed.push(0xDD);
                    } else {
                        framed.push(b);
                    }
                }
                framed.push(0xC0); // SLIP END character
            }
            LegacyProtocol::Ppp => {
                framed.push(0x7E); // PPP Flag Sequence
                framed.push(0xFF); // Address Field
                framed.push(0x03); // Control Field
                for &b in payload {
                    framed.push(b);
                }
                framed.push(0x7E);
            }
            LegacyProtocol::Ipv4Only => {
                // Return simple IPv4 header + payload mock
                framed.push(0x45); // Version 4, Header length 5
                for &b in payload {
                    framed.push(b);
                }
            }
        }
        Ok(framed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legacy_protocol_slip() {
        let mut adapter = LegacyProtocolAdapter::new(LegacyProtocol::Slip);
        assert!(adapter.encapsulate_packet(b"test").is_err());

        adapter.connect_link();
        assert!(adapter.is_connected);

        let framed = adapter.encapsulate_packet(b"OK").unwrap();
        assert_eq!(framed[0], 0xC0);
        assert_eq!(framed[1], b'O');
        assert_eq!(framed[2], b'K');
        assert_eq!(framed[3], 0xC0);
        assert_eq!(adapter.frame_counter, 1);
    }

    #[test]
    fn test_legacy_protocol_ppp() {
        let mut adapter = LegacyProtocolAdapter::new(LegacyProtocol::Ppp);
        adapter.connect_link();
        let framed = adapter.encapsulate_packet(b"PPP").unwrap();
        assert_eq!(framed[0], 0x7E);
        assert_eq!(framed[1], 0xFF);
        assert_eq!(framed[2], 0x03);
    }
}
