// SigmaOS Custom Network Library
// Reduces dependency on std::net by providing custom implementations

/// Custom IPv4 address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ipv4Addr {
    pub octets: [u8; 4],
}

impl Ipv4Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Ipv4Addr { octets: [a, b, c, d] }
    }

    pub fn octets(&self) -> [u8; 4] {
        self.octets
    }

    pub fn localhost() -> Self {
        Ipv4Addr::new(127, 0, 0, 1)
    }

    pub fn unspecified() -> Self {
        Ipv4Addr::new(0, 0, 0, 0)
    }
}

/// Custom IPv6 address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ipv6Addr {
    pub segments: [u16; 8],
}

impl Ipv6Addr {
    pub fn new(segments: [u16; 8]) -> Self {
        Ipv6Addr { segments }
    }

    pub fn octets(&self) -> [u8; 16] {
        let mut bytes = [0u8; 16];
        for (i, &seg) in self.segments.iter().enumerate() {
            bytes[i * 2] = (seg >> 8) as u8;
            bytes[i * 2 + 1] = (seg & 0xFF) as u8;
        }
        bytes
    }

    pub fn localhost() -> Self {
        Ipv6Addr::new([0, 0, 0, 0, 0, 0, 0, 1])
    }

    pub fn unspecified() -> Self {
        Ipv6Addr::new([0, 0, 0, 0, 0, 0, 0, 0])
    }
}

/// Custom IP address enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddr {
    pub fn v4(a: u8, b: u8, c: u8, d: u8) -> Self {
        IpAddr::V4(Ipv4Addr::new(a, b, c, d))
    }

    pub fn v6(segments: [u16; 8]) -> Self {
        IpAddr::V6(Ipv6Addr::new(segments))
    }

    pub fn localhost() -> Self {
        IpAddr::V4(Ipv4Addr::localhost())
    }

    pub fn unspecified() -> Self {
        IpAddr::V4(Ipv4Addr::unspecified())
    }
}

impl core::fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.octets[0], self.octets[1], self.octets[2], self.octets[3])
    }
}

impl core::fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
            self.segments[0], self.segments[1], self.segments[2], self.segments[3],
            self.segments[4], self.segments[5], self.segments[6], self.segments[7]
        )
    }
}

impl core::fmt::Display for IpAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IpAddr::V4(v4) => write!(f, "{}", v4),
            IpAddr::V6(v6) => write!(f, "{}", v6),
        }
    }
}