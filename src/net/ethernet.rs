/// SigmaOS Ethernet Frame Processing (Phase 2 Networking)
/// Inspired by Linux's net/ethernet/eth.c

pub const ETH_ALEN: usize = 6;
pub const ETH_HLEN: usize = 14;

#[derive(Debug, Clone, Copy)]
pub struct EthernetHeader {
    pub dest: [u8; ETH_ALEN],
    pub source: [u8; ETH_ALEN],
    pub ethertype: u16,
}

pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const ETHERTYPE_ARP: u16 = 0x0806;
pub const ETHERTYPE_IPV6: u16 = 0x86DD;

impl EthernetHeader {
    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < ETH_HLEN {
            return Err("Frame too short for Ethernet header");
        }
        let mut dest = [0u8; ETH_ALEN];
        let mut source = [0u8; ETH_ALEN];
        dest.copy_from_slice(&data[0..6]);
        source.copy_from_slice(&data[6..12]);
        let ethertype = ((data[12] as u16) << 8) | data[13] as u16;
        Ok(Self { dest, source, ethertype })
    }

    pub fn serialize(&self) -> [u8; ETH_HLEN] {
        let mut buf = [0u8; ETH_HLEN];
        buf[0..6].copy_from_slice(&self.dest);
        buf[6..12].copy_from_slice(&self.source);
        buf[12] = (self.ethertype >> 8) as u8;
        buf[13] = (self.ethertype & 0xFF) as u8;
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethernet_roundtrip() {
        let hdr = EthernetHeader {
            dest: [0xFF; 6],
            source: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
            ethertype: ETHERTYPE_IPV4,
        };
        let serialized = hdr.serialize();
        let parsed = EthernetHeader::parse(&serialized).unwrap();
        assert_eq!(parsed.ethertype, ETHERTYPE_IPV4);
        assert_eq!(parsed.source, hdr.source);
    }

    #[test]
    fn test_short_frame_rejected() {
        assert!(EthernetHeader::parse(&[0u8; 5]).is_err());
    }
}
