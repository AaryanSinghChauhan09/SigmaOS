#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

/// Provides sovereign enterprise network features including IPv6 addressing, VPN encrypted tunneling, and SSL/TLS.
/// 
/// SECURITY WARNING: This module contains mock cryptographic implementations for testing purposes only.
/// In production, use:
/// - `crate::security::crypto_utils::SecureRandom` for key generation
/// - Proper cryptographic libraries (RustCrypto, OpenSSL, etc.) for encryption
/// - Never use hard-coded keys or weak cryptographic primitives

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnterpriseNetworkError {
    Success = 0,
    InvalidAddress = 1,
    TunnelNotEstablished = 2,
    EncryptionFailed = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IPv6Address {
    pub segments: [u16; 8],
}

impl IPv6Address {
    pub const fn new(segments: [u16; 8]) -> Self {
        IPv6Address { segments }
    }

    /// Parse an IPv6 address string (e.g. fe80::1 or 2001:db8:0:0:0:0:0:1)
    pub fn parse(address: &[u8]) -> Result<Self, EnterpriseNetworkError> {
        let mut segments = [0u16; 8];
        let mut current_segment = 0;
        let mut current_val = 0u32;
        let mut has_val = false;

        let mut idx = 0;
        while idx < address.len() {
            let b = address[idx];
            if b == b':' {
                if has_val {
                    if current_segment >= 8 {
                        return Err(EnterpriseNetworkError::InvalidAddress);
                    }
                    segments[current_segment] = current_val as u16;
                    current_segment += 1;
                    current_val = 0;
                    has_val = false;
                }

                // Handle double colon "::"
                if idx + 1 < address.len() && address[idx + 1] == b':' {
                    // Quick mock representation of double colon
                    idx += 1;
                }
            } else {
                let digit = match b {
                    b'0'..=b'9' => (b - b'0') as u32,
                    b'a'..=b'f' => (b - b'a' + 10) as u32,
                    b'A'..=b'F' => (b - b'A' + 10) as u32,
                    _ => return Err(EnterpriseNetworkError::InvalidAddress),
                };
                current_val = (current_val << 4) | digit;
                if current_val > 0xFFFF {
                    return Err(EnterpriseNetworkError::InvalidAddress);
                }
                has_val = true;
            }
            idx += 1;
        }

        if has_val {
            if current_segment >= 8 {
                return Err(EnterpriseNetworkError::InvalidAddress);
            }
            segments[current_segment] = current_val as u16;
        }

        Ok(IPv6Address { segments })
    }
}

/// IPv6 Standard Header
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IPv6Header {
    pub version_traffic_class_flow_label: u32,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub source: IPv6Address,
    pub destination: IPv6Address,
}

impl IPv6Header {
    pub fn new(source: IPv6Address, destination: IPv6Address, next_header: u8, payload_len: u16) -> Self {
        Self {
            version_traffic_class_flow_label: 0x6000_0000, // Version 6
            payload_length: payload_len,
            next_header,
            hop_limit: 64,
            source,
            destination,
        }
    }
}

/// Stateless Address Autoconfiguration (SLAAC)
pub struct SlaacAutoconfig;

impl SlaacAutoconfig {
    /// Generates IPv6 link-local address based on EUI-64 of a 48-bit MAC address
    pub fn generate_link_local(mac: &[u8; 6]) -> IPv6Address {
        let mut segments = [0u16; 8];
        segments[0] = 0xfe80;
        segments[4] = (((mac[0] ^ 0x02) as u16) << 8) | (mac[1] as u16);
        segments[5] = ((mac[2] as u16) << 8) | 0xff;
        segments[6] = 0xfe00 | (mac[3] as u16);
        segments[7] = ((mac[4] as u16) << 8) | (mac[5] as u16);
        IPv6Address { segments }
    }

    /// Generates IPv6 global address using prefix and EUI-64 of MAC address
    pub fn generate_global(prefix: &[u16; 4], mac: &[u8; 6]) -> IPv6Address {
        let mut segments = [0u16; 8];
        segments[0..4].copy_from_slice(prefix);
        segments[4] = (((mac[0] ^ 0x02) as u16) << 8) | (mac[1] as u16);
        segments[5] = ((mac[2] as u16) << 8) | 0xff;
        segments[6] = 0xfe00 | (mac[3] as u16);
        segments[7] = ((mac[4] as u16) << 8) | (mac[5] as u16);
        IPv6Address { segments }
    }
}

/// IPv6 CIDR Prefix matching route
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IPv6Route {
    pub prefix: IPv6Address,
    pub prefix_len: u8,
    pub gateway: Option<IPv6Address>,
}

#[cfg(not(test))]
pub type RouteVec = crate::klib::Vec<IPv6Route>;
#[cfg(test)]
pub type RouteVec = std::vec::Vec<IPv6Route>;

/// Longest Prefix Match (LPM) IPv6 routing table
pub struct IPv6RoutingTable {
    pub routes: RouteVec,
}

impl IPv6RoutingTable {
    pub fn new() -> Self {
        Self {
            routes: RouteVec::new(),
        }
    }

    pub fn add_route(&mut self, prefix: IPv6Address, prefix_len: u8, gateway: Option<IPv6Address>) {
        self.routes.push(IPv6Route {
            prefix,
            prefix_len,
            gateway,
        });
    }

    /// Longest Prefix Match (LPM) lookup for destination address
    pub fn lookup(&self, destination: &IPv6Address) -> Option<IPv6Route> {
        let mut best_match: Option<IPv6Route> = None;
        for i in 0..self.routes.len() {
            let route = self.routes[i];
            if Self::matches(destination, &route.prefix, route.prefix_len) {
                match best_match {
                    None => best_match = Some(route),
                    Some(best) => {
                        if route.prefix_len > best.prefix_len {
                            best_match = Some(route);
                        }
                    }
                }
            }
        }
        best_match
    }

    fn matches(addr: &IPv6Address, prefix: &IPv6Address, prefix_len: u8) -> bool {
        let mut remaining_bits = prefix_len;
        for i in 0..8 {
            if remaining_bits == 0 {
                return true;
            }
            let bits_to_compare = core::cmp::min(remaining_bits, 16);
            let mask = if bits_to_compare == 16 {
                0xFFFF
            } else {
                0xFFFF_u16 << (16 - bits_to_compare)
            };
            if (addr.segments[i] & mask) != (prefix.segments[i] & mask) {
                return false;
            }
            remaining_bits -= bits_to_compare;
        }
        true
    }
}

/// Secure VPN Tunnel (WireGuard-inspired endpoint-to-endpoint encryptor)
#[derive(Clone)]
pub struct SecureVpnTunnel {
    pub preshared_key: [u8; 32],
    pub established: bool,
}

impl SecureVpnTunnel {
    pub fn new(preshared_key: &[u8; 32]) -> Self {
        SecureVpnTunnel {
            preshared_key: *preshared_key,
            established: false,
        }
    }

    /// Perform secure handshake
    pub fn handshake(&mut self, peer_public_key: &[u8; 32]) -> Result<(), EnterpriseNetworkError> {
        let mut valid = false;
        for i in 0..32 {
            if peer_public_key[i] != 0 {
                valid = true;
            }
        }

        if !valid {
            return Err(EnterpriseNetworkError::TunnelNotEstablished);
        }

        self.established = true;
        Ok(())
    }

    /// Encapsulate and encrypt a data packet using the preshared key channel
    pub fn encrypt_packet(
        &self,
        payload: &[u8],
        encrypted_buffer: &mut [u8],
    ) -> Result<usize, EnterpriseNetworkError> {
        if !self.established {
            return Err(EnterpriseNetworkError::TunnelNotEstablished);
        }

        if payload.len() > encrypted_buffer.len() {
            return Err(EnterpriseNetworkError::EncryptionFailed);
        }

        // Optimized by Bolt ⚡: Single-pass iterator chain eliminates integer modulo division (% 32)
        // and index bounds-checking for every byte, facilitating compiler auto-vectorization (SIMD).
        for ((out_byte, &in_byte), &mask) in encrypted_buffer.iter_mut().zip(payload.iter()).zip(self.preshared_key.iter().cycle()) {
            *out_byte = in_byte ^ mask;
        }

        Ok(payload.len())
    }

    /// Decrypt packet payload
    pub fn decrypt_packet(
        &self,
        encrypted_payload: &[u8],
        decrypted_buffer: &mut [u8],
    ) -> Result<usize, EnterpriseNetworkError> {
        if !self.established {
            return Err(EnterpriseNetworkError::TunnelNotEstablished);
        }

        if encrypted_payload.len() > decrypted_buffer.len() {
            return Err(EnterpriseNetworkError::EncryptionFailed);
        }

        // Optimized by Bolt ⚡: Single-pass iterator chain eliminates integer modulo division (% 32)
        // and index bounds-checking for every byte, facilitating compiler auto-vectorization (SIMD).
        for ((out_byte, &in_byte), &mask) in decrypted_buffer.iter_mut().zip(encrypted_payload.iter()).zip(self.preshared_key.iter().cycle()) {
            *out_byte = in_byte ^ mask;
        }

        Ok(encrypted_payload.len())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_prefix_match() {
        let mut table = IPv6RoutingTable::new();
        let prefix1 = IPv6Address::parse(b"2001:db8:1::").unwrap();
        let prefix2 = IPv6Address::parse(b"2001:db8:1:2::").unwrap();

        table.add_route(prefix1, 48, Some(IPv6Address::parse(b"fe80:0:0:0:0:0:0:1").unwrap()));
        table.add_route(prefix2, 64, Some(IPv6Address::parse(b"fe80:0:0:0:0:0:0:2").unwrap()));

        let dst = IPv6Address::parse(b"2001:db8:1:2:3:4:5:6").unwrap();
        let matched = table.lookup(&dst).unwrap();
        assert_eq!(matched.prefix_len, 64);
        assert_eq!(matched.gateway.unwrap().segments[7], 2);
    }

    #[test]
    fn test_vpn_packet_encryption_optimization() {
        let key = [0x42u8; 32];
        let mut tunnel = SecureVpnTunnel::new(&key);
        tunnel.handshake(&[0x01u8; 32]).unwrap();

        let payload = b"Bolt lightning fast VPN packet encryption performance test payload!";
        let mut enc_buf = [0u8; 128];
        let mut dec_buf = [0u8; 128];

        let enc_len = tunnel.encrypt_packet(payload, &mut enc_buf).unwrap();
        assert_eq!(enc_len, payload.len());
        assert_ne!(&enc_buf[..enc_len], payload);

        let dec_len = tunnel.decrypt_packet(&enc_buf[..enc_len], &mut dec_buf).unwrap();
        assert_eq!(dec_len, payload.len());
        assert_eq!(&dec_buf[..dec_len], payload);
    }
}
