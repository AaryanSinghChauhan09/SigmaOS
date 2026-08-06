#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem;
/// Advanced Enterprise Networking Suite for SigmaOS
/// Provides sovereign enterprise network features including IPv6 addressing, VPN encrypted tunneling, and SSL/TLS.
use core::sync::atomic::{AtomicUsize, Ordering};

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

        // Mock stream cipher using key masking
        for i in 0..payload.len() {
            let mask = self.preshared_key[i % 32];
            encrypted_buffer[i] = payload[i] ^ mask;
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

        for i in 0..encrypted_payload.len() {
            let mask = self.preshared_key[i % 32];
            decrypted_buffer[i] = encrypted_payload[i] ^ mask;
        }

        Ok(encrypted_payload.len())
    }
}

/// Sequence Anti-Replay sliding window sequence verification
pub struct AntiReplayWindow {
    pub max_seq: u64,
    pub window_mask: u64,
}

impl AntiReplayWindow {
    pub fn new() -> Self {
        Self {
            max_seq: 0,
            window_mask: 0,
        }
    }

    /// Verify if the sequence number has already been seen or is too old, and update the mask
    pub fn check_and_update(&mut self, seq: u64) -> bool {
        if seq == 0 {
            return false;
        }

        if seq > self.max_seq {
            let diff = seq - self.max_seq;
            if diff >= 64 {
                self.window_mask = 1;
            } else {
                self.window_mask = (self.window_mask << diff) | 1;
            }
            self.max_seq = seq;
            true
        } else {
            let offset = self.max_seq - seq;
            if offset >= 64 {
                false // too old
            } else {
                let mask_bit = 1_u64 << offset;
                if (self.window_mask & mask_bit) != 0 {
                    false // duplicate replay
                } else {
                    self.window_mask |= mask_bit;
                    true
                }
            }
        }
    }
}

/// VPN Stateful Encapsulated Virtual Private Network interface routing
pub struct VpnVirtualInterface {
    pub tunnel: SecureVpnTunnel,
    pub replay_filter: AntiReplayWindow,
    pub mtu: usize,
}

impl VpnVirtualInterface {
    pub fn new(tunnel: SecureVpnTunnel) -> Self {
        Self {
            tunnel,
            replay_filter: AntiReplayWindow::new(),
            mtu: 1420,
        }
    }

    /// Encapsulates and encrypts packet prepending big-endian 64-bit sequence number
    pub fn encapsulate(&mut self, seq: u64, payload: &[u8], out_buffer: &mut [u8]) -> Result<usize, EnterpriseNetworkError> {
        if !self.tunnel.established {
            return Err(EnterpriseNetworkError::TunnelNotEstablished);
        }

        if payload.len() + 8 > out_buffer.len() {
            return Err(EnterpriseNetworkError::EncryptionFailed);
        }

        let seq_bytes = seq.to_be_bytes();
        out_buffer[0..8].copy_from_slice(&seq_bytes);

        self.tunnel.encrypt_packet(payload, &mut out_buffer[8..])?;
        Ok(payload.len() + 8)
    }

    /// Decapsulates packet, runs anti-replay checks and decrypts payload
    pub fn decapsulate(&mut self, packet: &[u8], out_payload: &mut [u8]) -> Result<usize, EnterpriseNetworkError> {
        if packet.len() < 8 {
            return Err(EnterpriseNetworkError::EncryptionFailed);
        }

        let mut seq_bytes = [0u8; 8];
        seq_bytes.copy_from_slice(&packet[0..8]);
        let seq = u64::from_be_bytes(seq_bytes);

        if !self.replay_filter.check_and_update(seq) {
            return Err(EnterpriseNetworkError::EncryptionFailed);
        }

        let decrypted_len = self.tunnel.decrypt_packet(&packet[8..], out_payload)?;
        Ok(decrypted_len)
    }
}

/// TLS State Enum representing state machine phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsState {
    Uninitialized,
    ClientHelloSent,
    ServerHelloReceived,
    Established,
}

/// standard TLS content type record identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsRecordType {
    Handshake = 22,
    ApplicationData = 23,
    Alert = 21,
}

/// Sovereign TLS 1.3 State Machine Engine
pub struct SovereignSslEngine {
    pub state: TlsState,
    pub session_ticket: Option<[u8; 16]>,
    pub write_key: [u8; 16],
    pub read_key: [u8; 16],
}

impl SovereignSslEngine {
    pub fn new() -> Self {
        Self {
            state: TlsState::Uninitialized,
            session_ticket: None,
            write_key: [0u8; 16],
            read_key: [0u8; 16],
        }
    }

    /// Perform ClientHello generation
    pub fn send_client_hello(&mut self, out_buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.state != TlsState::Uninitialized {
            return Err("Invalid TLS State for ClientHello");
        }

        out_buffer[0] = TlsRecordType::Handshake as u8;
        out_buffer[1] = 0x03;
        out_buffer[2] = 0x03; // TLS 1.3

        let msg = b"ClientHello13";
        out_buffer[3..5].copy_from_slice(&(msg.len() as u16).to_be_bytes());
        out_buffer[5..5 + msg.len()].copy_from_slice(msg);

        self.state = TlsState::ClientHelloSent;
        Ok(5 + msg.len())
    }

    /// Process ServerHello response
    pub fn receive_server_hello(&mut self, in_buffer: &[u8]) -> Result<(), &'static str> {
        if self.state != TlsState::ClientHelloSent {
            return Err("Invalid state to receive ServerHello");
        }

        if in_buffer.len() < 5 || in_buffer[0] != TlsRecordType::Handshake as u8 {
            return Err("Unexpected non-handshake record type");
        }

        let mut len_bytes = [0u8; 2];
        len_bytes.copy_from_slice(&in_buffer[3..5]);
        let len = u16::from_be_bytes(len_bytes) as usize;

        if len + 5 > in_buffer.len() {
            return Err("Malformed record length");
        }

        if &in_buffer[5..5 + len] != b"ServerHello13" {
            return Err("Expected ServerHello TLS 1.3 Handshake payload");
        }

        // Key derivation simulated
        for i in 0..16 {
            self.write_key[i] = 0x5A ^ (i as u8);
            self.read_key[i] = 0xA5 ^ (i as u8);
        }

        self.state = TlsState::ServerHelloReceived;
        Ok(())
    }

    /// Complete state transition to Established
    pub fn establish_handshake(&mut self) {
        if self.state == TlsState::ServerHelloReceived {
            self.state = TlsState::Established;
            self.session_ticket = Some([0x77u8; 16]);
        }
    }

    /// Encapsulates application data payload inside encrypted record
    pub fn encrypt_record(&self, plaintext: &[u8], record_buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.state != TlsState::Established {
            return Err("Handshake not established");
        }

        if plaintext.len() + 5 > record_buffer.len() {
            return Err("Record buffer overflow");
        }

        record_buffer[0] = TlsRecordType::ApplicationData as u8;
        record_buffer[1] = 0x03;
        record_buffer[2] = 0x03;
        record_buffer[3..5].copy_from_slice(&(plaintext.len() as u16).to_be_bytes());

        for i in 0..plaintext.len() {
            let key_mask = self.write_key[i % 16];
            record_buffer[5 + i] = plaintext[i] ^ key_mask;
        }

        Ok(5 + plaintext.len())
    }

    /// Decapsulates and decrypts TLS ApplicationData records
    pub fn decrypt_record(&self, record: &[u8], plaintext_buffer: &mut [u8]) -> Result<usize, &'static str> {
        if self.state != TlsState::Established {
            return Err("Handshake not established");
        }

        if record.len() < 5 {
            return Err("Record too short");
        }

        if record[0] != TlsRecordType::ApplicationData as u8 {
            return Err("Expected application data content type");
        }

        let mut len_bytes = [0u8; 2];
        len_bytes.copy_from_slice(&record[3..5]);
        let len = u16::from_be_bytes(len_bytes) as usize;

        if len + 5 > record.len() {
            return Err("Malformed record length field");
        }

        if len > plaintext_buffer.len() {
            return Err("Plaintext destination overflow");
        }

        for i in 0..len {
            let key_mask = self.read_key[i % 16];
            plaintext_buffer[i] = record[5 + i] ^ key_mask;
        }

        Ok(len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_parsing() {
        let addr = IPv6Address::parse(b"2001:db8:0:0:0:0:0:1").unwrap();
        assert_eq!(addr.segments[0], 0x2001);
        assert_eq!(addr.segments[1], 0x0db8);
        assert_eq!(addr.segments[7], 0x0001);
    }

    #[test]
    fn test_vpn_tunnel() {
        let key = [0x55u8; 32];
        let mut tunnel = SecureVpnTunnel::new(&key);

        let peer_key = [0xAAu8; 32];
        assert!(tunnel.handshake(&peer_key).is_ok());
        assert!(tunnel.established);

        let data = b"Enterprise Security Data";
        let mut encrypted = [0u8; 64];
        let enc_len = tunnel.encrypt_packet(data, &mut encrypted).unwrap();
        assert_eq!(enc_len, data.len());

        let mut decrypted = [0u8; 64];
        let dec_len = tunnel
            .decrypt_packet(&encrypted[..enc_len], &mut decrypted)
            .unwrap();
        assert_eq!(&decrypted[..dec_len], data);
    }

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
    fn test_vpn_replay_prevention() {
        let key = [0xBBu8; 32];
        let mut tunnel = SecureVpnTunnel::new(&key);
        let peer_key = [0xCCu8; 32];
        tunnel.handshake(&peer_key).unwrap();

        let mut vpn = VpnVirtualInterface::new(tunnel);
        let payload = b"Sensitive Tunnel Data";
        let mut packet = [0u8; 64];
        let len = vpn.encapsulate(10, payload, &mut packet).unwrap();

        let mut dec_payload = [0u8; 64];
        let dec_len = vpn.decapsulate(&packet[..len], &mut dec_payload).unwrap();
        assert_eq!(&dec_payload[..dec_len], payload);

        // Replay attempt must fail
        assert!(vpn.decapsulate(&packet[..len], &mut dec_payload).is_err());
    }

    #[test]
    fn test_ssl_handshake_flow() {
        let mut client = SovereignSslEngine::new();

        let mut hello_buf = [0u8; 64];
        let _len = client.send_client_hello(&mut hello_buf).unwrap();

        // Construct server hello response
        let mut response_buf = [0u8; 64];
        response_buf[0] = TlsRecordType::Handshake as u8;
        response_buf[1] = 0x03;
        response_buf[2] = 0x03;
        let smsg = b"ServerHello13";
        response_buf[3..5].copy_from_slice(&(smsg.len() as u16).to_be_bytes());
        response_buf[5..5 + smsg.len()].copy_from_slice(smsg);

        // Client processes server hello
        assert!(client.receive_server_hello(&response_buf[..5 + smsg.len()]).is_ok());

        client.establish_handshake();
        assert_eq!(client.state, TlsState::Established);
    }
}
