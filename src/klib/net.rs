// SigmaOS Custom Network Library
// Reduces dependency on std::net by providing custom implementations

/// Custom IPv4 address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv4Addr {
    pub octets: [u8; 4],
}

impl Ipv4Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Ipv4Addr { octets: [a, b, c, d] }
    }

    pub fn localhost() -> Self {
        Ipv4Addr::new(127, 0, 0, 1)
    }

    pub fn unspecified() -> Self {
        Ipv4Addr::new(0, 0, 0, 0)
    }
}

/// Custom IPv6 address
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ipv6Addr {
    pub segments: [u16; 8],
}

impl Ipv6Addr {
    pub fn new(segments: [u16; 8]) -> Self {
        Ipv6Addr { segments }
    }

    pub fn localhost() -> Self {
        Ipv6Addr::new([0, 0, 0, 0, 0, 0, 0, 1])
    }

    pub fn unspecified() -> Self {
        Ipv6Addr::new([0, 0, 0, 0, 0, 0, 0, 0])
    }
}

/// Custom IP address enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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