/// SigmaOS IPv4 Stack (Phase 2 Networking)
/// Inspired by Linux's net/ipv4/ip_input.c and FreeBSD's netinet/ip_input.c

#[derive(Debug, Clone, Copy)]
pub struct Ipv4Header {
    pub version_ihl: u8,
    pub tos: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags_fragment: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub checksum: u16,
    pub src_addr: [u8; 4],
    pub dst_addr: [u8; 4],
}

pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;
pub const IPPROTO_ICMP: u8 = 1;

impl Ipv4Header {
    pub fn parse(data: &[u8]) -> Result<Self, &'static str> {
        if data.len() < 20 {
            return Err("Packet too short for IPv4 header");
        }
        let version_ihl = data[0];
        if (version_ihl >> 4) != 4 {
            return Err("Not an IPv4 packet");
        }
        let mut src_addr = [0u8; 4];
        let mut dst_addr = [0u8; 4];
        src_addr.copy_from_slice(&data[12..16]);
        dst_addr.copy_from_slice(&data[16..20]);

        Ok(Self {
            version_ihl,
            tos: data[1],
            total_length: ((data[2] as u16) << 8) | data[3] as u16,
            identification: ((data[4] as u16) << 8) | data[5] as u16,
            flags_fragment: ((data[6] as u16) << 8) | data[7] as u16,
            ttl: data[8],
            protocol: data[9],
            checksum: ((data[10] as u16) << 8) | data[11] as u16,
            src_addr,
            dst_addr,
        })
    }

    pub fn header_length(&self) -> usize {
        ((self.version_ihl & 0x0F) as usize) * 4
    }

    pub fn compute_checksum(data: &[u8]) -> u16 {
        let mut sum: u32 = 0;
        let mut i = 0;
        while i < data.len() - 1 {
            sum += ((data[i] as u32) << 8) | data[i + 1] as u32;
            i += 2;
        }
        if data.len() % 2 != 0 {
            sum += (data[data.len() - 1] as u32) << 8;
        }
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }
        !(sum as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_parse() {
        let mut pkt = [0u8; 20];
        pkt[0] = 0x45; // version=4, ihl=5
        pkt[8] = 64; // TTL
        pkt[9] = IPPROTO_TCP;
        pkt[12..16].copy_from_slice(&[192, 168, 1, 100]);
        pkt[16..20].copy_from_slice(&[10, 0, 0, 1]);
        let hdr = Ipv4Header::parse(&pkt).unwrap();
        assert_eq!(hdr.ttl, 64);
        assert_eq!(hdr.protocol, IPPROTO_TCP);
        assert_eq!(hdr.src_addr, [192, 168, 1, 100]);
        assert_eq!(hdr.header_length(), 20);
    }
}
