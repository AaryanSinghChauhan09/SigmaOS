/// SigmaOS UDP Stack (Phase 2 Networking)
/// Inspired by Linux's net/ipv4/udp.c

#[derive(Debug, Clone, Copy)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl UdpHeader {
    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 8 {
            return Err("Packet too short for UDP header");
        }
        Ok(Self {
            src_port: ((data[0] as u16) << 8) | data[1] as u16,
            dst_port: ((data[2] as u16) << 8) | data[3] as u16,
            length: ((data[4] as u16) << 8) | data[5] as u16,
            checksum: ((data[6] as u16) << 8) | data[7] as u16,
        })
    }

    pub fn serialize(&self) -> [u8; 8] {
        [
            (self.src_port >> 8) as u8, (self.src_port & 0xFF) as u8,
            (self.dst_port >> 8) as u8, (self.dst_port & 0xFF) as u8,
            (self.length >> 8) as u8, (self.length & 0xFF) as u8,
            (self.checksum >> 8) as u8, (self.checksum & 0xFF) as u8,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udp_roundtrip() {
        let hdr = UdpHeader { src_port: 12345, dst_port: 53, length: 20, checksum: 0 };
        let serialized = hdr.serialize();
        let parsed = UdpHeader::parse(&serialized).unwrap();
        assert_eq!(parsed.src_port, 12345);
        assert_eq!(parsed.dst_port, 53);
    }
}
