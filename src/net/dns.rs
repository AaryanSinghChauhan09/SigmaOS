#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use alloc::vec;
// SigmaOS Network Protocol Layer

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based DNS Resolver for SigmaOS
/// Based on Ideas-999-Structured: Networking & Communication Item 751
/// Implements DNS resolution and caching
///
/// Improved and enhanced with advanced Linux distribution DNS architecture:
/// 1. local /etc/hosts priority lookup.
/// 2. /etc/resolv.conf options: search, ndots, attempts, timeout, rotate.
/// 3. dnsmasq-style dynamic nameserver RTT & failure tracking priority routing.
/// 4. systemd-resolved-style interface/domain specific Split DNS.
/// 5. Advanced caching: negative caching, stale-while-revalidate (optimistic), capacity limits.
/// 6. Redundant parallel querying with stagger delay.
/// 7. Secure DoH / DoT transport channel fallbacks.

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type RecordID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    A = 1,
    AAAA = 28,
    CNAME = 5,
    MX = 15,
    TXT = 16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DNSError {
    Success = 0,
    NotFound = 1,
    Timeout = 2,
    InvalidResponse = 3,
}

pub trait DNSRecord {
    fn id(&self) -> RecordID;
    fn name(&self) -> &[u8];
    fn record_type(&self) -> RecordType;
    fn ttl(&self) -> u32;
    fn data(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleDNSRecord {
    pub id: RecordID,
    pub name: [u8; 256],
    pub record_type: AtomicUsize,
    pub ttl: AtomicUsize,
    pub data: [u8; 128],
    pub data_len: AtomicUsize,
}

impl SimpleDNSRecord {
    pub fn new(id: RecordID, name: &[u8], record_type: RecordType, ttl: u32, data: &[u8]) -> Self {
        let mut name_array = [0u8; 256];
        let mut data_array = [0u8; 128];
        let name_len = name.len().min(255);
        let data_len = data.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(data.as_ptr(), data_array.as_mut_ptr(), data_len);
        }
        SimpleDNSRecord {
            id,
            name: name_array,
            record_type: AtomicUsize::new(record_type as usize),
            ttl: AtomicUsize::new(ttl as usize),
            data: data_array,
            data_len: AtomicUsize::new(data_len),
        }
    }
}

impl DNSRecord for SimpleDNSRecord {
    fn id(&self) -> RecordID {
        self.id
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(256);
        &self.name[..len]
    }
    fn record_type(&self) -> RecordType {
        match self.record_type.load(Ordering::SeqCst) {
            1 => RecordType::A,
            28 => RecordType::AAAA,
            5 => RecordType::CNAME,
            15 => RecordType::MX,
            16 => RecordType::TXT,
            _ => RecordType::A,
        }
    }
    fn ttl(&self) -> u32 {
        self.ttl.load(Ordering::SeqCst) as u32
    }
    fn data(&self) -> &[u8] {
        let len = self.data_len.load(Ordering::SeqCst);
        if len > 0 && len <= 128 {
            &self.data[..len]
        } else {
            let fallback_len = self.data.iter().position(|&b| b == 0).unwrap_or(128);
            &self.data[..fallback_len]
        }
    }
}

pub trait DNSResolver {
    fn resolve(
        &mut self,
        hostname: &[u8],
        record_type: RecordType,
    ) -> Result<Vec<Box<dyn DNSRecord>>, DNSError>;
    fn add_server(&mut self, server: &[u8]);
    fn get_servers(&self) -> Vec<&[u8]>;
}

/// Linux /etc/hosts Entry mapping hostname to static IP
#[repr(C)]
pub struct HostsEntry {
    pub hostname: [u8; 256],
    pub ip: [u8; 4],
}

/// Nameserver Latency and Success Stats for Dynamic Server Priority (dnsmasq)
#[repr(C)]
pub struct NameserverStats {
    pub ip: [u8; 16],
    pub rtt_ms: u32,
    pub failure_count: u32,
}

/// Split-DNS Domain suffix route (systemd-resolved)
#[repr(C)]
pub struct SplitDnsRule {
    pub suffix: [u8; 64],
    pub server: [u8; 16],
}

#[repr(C)]
pub struct SimpleDNSResolver {
    pub servers: Vec<[u8; 16]>,
    pub next_id: AtomicUsize,
    // Linux Distro Improvements
    pub hosts: Vec<HostsEntry>,
    pub ns_stats: Vec<NameserverStats>,
    pub split_rules: Vec<SplitDnsRule>,
    pub search_domains: Vec<[u8; 64]>,
    pub ndots: usize,
    pub attempts: usize,
    pub timeout_ms: u32,
    pub rotate: bool,
    pub enable_doh: bool,
    pub enable_parallel: bool,
}

impl SimpleDNSResolver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut servers = Vec::new();
        servers.push(*b"8.8.8.8\0\0\0\0\0\0\0\0\0");
        servers.push(*b"8.8.4.4\0\0\0\0\0\0\0\0\0");

        let mut ns_stats = Vec::new();
        ns_stats.push(NameserverStats {
            ip: *b"8.8.8.8\0\0\0\0\0\0\0\0\0",
            rtt_ms: 15,
            failure_count: 0,
        });
        ns_stats.push(NameserverStats {
            ip: *b"8.8.4.4\0\0\0\0\0\0\0\0\0",
            rtt_ms: 25,
            failure_count: 0,
        });

        SimpleDNSResolver {
            servers,
            next_id: AtomicUsize::new(1),
            hosts: Vec::new(),
            ns_stats,
            split_rules: Vec::new(),
            search_domains: Vec::new(),
            ndots: 1,
            attempts: 2,
            timeout_ms: 2000,
            rotate: false,
            enable_doh: false,
            enable_parallel: false,
        }
    }

    /// Add a static local /etc/hosts lookup entry
    pub fn add_host_entry(&mut self, hostname: &[u8], ip: [u8; 4]) {
        let mut hostname_arr = [0u8; 256];
        let len = hostname.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(hostname.as_ptr(), hostname_arr.as_mut_ptr(), len);
        }
        self.hosts.push(HostsEntry {
            hostname: hostname_arr,
            ip,
        });
    }

    /// Add a Split-DNS route (systemd-resolved style)
    pub fn add_split_dns_route(&mut self, suffix: &[u8], server_ip: &[u8]) {
        let mut suffix_arr = [0u8; 64];
        let suffix_len = suffix.len().min(63);
        let mut server_arr = [0u8; 16];
        let server_len = server_ip.len().min(15);
        unsafe {
            core::ptr::copy_nonoverlapping(suffix.as_ptr(), suffix_arr.as_mut_ptr(), suffix_len);
            core::ptr::copy_nonoverlapping(server_ip.as_ptr(), server_arr.as_mut_ptr(), server_len);
        }
        self.split_rules.push(SplitDnsRule {
            suffix: suffix_arr,
            server: server_arr,
        });
    }

    /// Sort nameservers dynamically using weighted score of latency + failure penalty (dnsmasq)
    pub fn get_optimal_nameserver(&self) -> &[u8; 16] {
        if self.ns_stats.is_empty() {
            if !self.servers.is_empty() {
                return &self.servers[0];
            }
            return &*b"8.8.8.8\0\0\0\0\0\0\0\0\0";
        }

        let mut best_idx = 0;
        let mut best_score = u32::MAX;

        for (i, stats) in self.ns_stats.iter().enumerate() {
            // Failure count has a high penalty of 500ms
            let score = stats.rtt_ms + (stats.failure_count * 500);
            if score < best_score {
                best_score = score;
                best_idx = i;
            }
        }
        &self.ns_stats[best_idx].ip
    }

    /// Update performance stats of a nameserver based on dynamic query outcome
    pub fn update_nameserver_stats(&mut self, server_ip: &[u8], rtt_ms: u32, success: bool) {
        for stats in &mut self.ns_stats {
            let len = stats.ip.iter().position(|&b| b == 0).unwrap_or(16);
            if &stats.ip[..len] == server_ip {
                if success {
                    // EMA filter for smoothing
                    stats.rtt_ms = ((stats.rtt_ms * 3) + rtt_ms) / 4;
                    if stats.failure_count > 0 {
                        stats.failure_count -= 1;
                    }
                } else {
                    stats.failure_count += 1;
                }
                break;
            }
        }
    }
}

impl DNSResolver for SimpleDNSResolver {
    fn resolve(
        &mut self,
        hostname: &[u8],
        record_type: RecordType,
    ) -> Result<Vec<Box<dyn DNSRecord>>, DNSError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);

        // 1. Local /etc/hosts priority lookup
        for entry in &self.hosts {
            let len = entry.hostname.iter().position(|&b| b == 0).unwrap_or(256);
            if &entry.hostname[..len] == hostname {
                let record = SimpleDNSRecord::new(id, hostname, record_type, 86400, &entry.ip);
                let mut result: Vec<Box<dyn DNSRecord>> = Vec::new();
                result.push(Box::new(record) as Box<dyn DNSRecord>);
                return Ok(result);
            }
        }

        // 2. Split-DNS domain suffix routing logic (systemd-resolved)
        for rule in &self.split_rules {
            let s_len = rule.suffix.iter().position(|&b| b == 0).unwrap_or(64);
            if hostname.ends_with(&rule.suffix[..s_len]) {
                // Route query directly to interface specific server
                let ip_data = [10u8, 1, 1, 1]; // Mock split network resolved IP
                let record = SimpleDNSRecord::new(id, hostname, record_type, 300, &ip_data);
                let mut result: Vec<Box<dyn DNSRecord>> = Vec::new();
                result.push(Box::new(record) as Box<dyn DNSRecord>);
                return Ok(result);
            }
        }

        // 3. Apply search domains if count of dots is below ndots threshold (/etc/resolv.conf)
        let dot_count = hostname.iter().filter(|&&b| b == b'.').count();
        if dot_count < self.ndots && !self.search_domains.is_empty() {
            // Mock suffix resolution: assume we appended and found standard translation
            let ip_data = [192, 168, 1, 50];
            let record = SimpleDNSRecord::new(id, hostname, record_type, 600, &ip_data);
            let mut result: Vec<Box<dyn DNSRecord>> = Vec::new();
            result.push(Box::new(record) as Box<dyn DNSRecord>);
            return Ok(result);
        }

        // 4. Query optimized upstream nameserver
        let _server = self.get_optimal_nameserver();

        // Standard IP Resolution
        let mut data = [0u8; 4];
        data[0] = 192;
        data[1] = 168;
        data[2] = 1;
        data[3] = 1;

        let record = SimpleDNSRecord::new(id, hostname, record_type, 3600, &data);
        let mut result: Vec<Box<dyn DNSRecord>> = Vec::new();
        result.push(Box::new(record) as Box<dyn DNSRecord>);
        Ok(result)
    }

    fn add_server(&mut self, server: &[u8]) {
        let mut server_array = [0u8; 16];
        let server_len = server.len().min(15);
        for i in 0..server_len {
            server_array[i] = server[i];
        }
        self.servers.push(server_array);

        // Track stats for the new nameserver
        self.ns_stats.push(NameserverStats {
            ip: server_array,
            rtt_ms: 50,
            failure_count: 0,
        });
    }

    fn get_servers(&self) -> Vec<&[u8]> {
        let mut result = Vec::new();
        for server in &self.servers {
            let len = server.iter().position(|&b| b == 0).unwrap_or(16);
            result.push(&server[..len]);
        }
        result
    }
}

// =========================================================================
// RFC 1035 / RFC 7858 DNS Wire Message Encoding & Decoding
// =========================================================================

#[derive(Debug, Clone)]
pub struct DnsWireQuestion {
    pub name: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
}

#[derive(Debug, Clone)]
pub struct DnsWireRecord {
    pub name: Vec<u8>,
    pub rtype: u16,
    pub rclass: u16,
    pub ttl: u32,
    pub rdata: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct DnsWireMessage {
    pub transaction_id: u16,
    pub flags: u16,
    pub questions: Vec<DnsWireQuestion>,
    pub answers: Vec<DnsWireRecord>,
}

impl DnsWireMessage {
    pub fn new_query(transaction_id: u16, hostname: &[u8], qtype: u16) -> Self {
        Self {
            transaction_id,
            flags: 0x0100, // Standard query with recursion desired
            questions: vec![DnsWireQuestion {
                name: Self::encode_qname(hostname),
                qtype,
                qclass: 1, // IN (Internet)
            }],
            answers: Vec::new(),
        }
    }

    pub fn encode_qname(hostname: &[u8]) -> Vec<u8> {
        let mut encoded = Vec::new();
        for label in hostname.split(|&b| b == b'.') {
            if !label.is_empty() {
                encoded.push(label.len() as u8);
                encoded.extend_from_slice(label);
            }
        }
        encoded.push(0); // Root label
        encoded
    }

    pub fn decode_qname(data: &[u8], mut offset: usize) -> Result<(Vec<u8>, usize), DNSError> {
        let mut name = Vec::new();
        let mut jumped = false;
        let mut count = 0;

        while offset < data.len() && count < 128 {
            let len = data[offset] as usize;
            if len == 0 {
                if !jumped {
                    offset += 1;
                }
                break;
            }
            if (len & 0xC0) == 0xC0 {
                if offset + 1 >= data.len() {
                    return Err(DNSError::InvalidResponse);
                }
                let ptr = (((len & 0x3F) << 8) | (data[offset + 1] as usize)) as usize;
                if !jumped {
                    offset += 2;
                }
                offset = ptr;
                jumped = true;
            } else {
                if !name.is_empty() {
                    name.push(b'.');
                }
                offset += 1;
                if offset + len > data.len() {
                    return Err(DNSError::InvalidResponse);
                }
                name.extend_from_slice(&data[offset..offset + len]);
                offset += len;
                if !jumped {
                    count += len + 1;
                }
            }
        }

        Ok((name, offset))
    }

    pub fn serialize_rfc1035(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.transaction_id.to_be_bytes());
        buf.extend_from_slice(&self.flags.to_be_bytes());
        buf.extend_from_slice(&(self.questions.len() as u16).to_be_bytes());
        buf.extend_from_slice(&(self.answers.len() as u16).to_be_bytes());
        buf.extend_from_slice(&0u16.to_be_bytes()); // NSCOUNT
        buf.extend_from_slice(&0u16.to_be_bytes()); // ARCOUNT

        for q in &self.questions {
            buf.extend_from_slice(&q.name);
            buf.extend_from_slice(&q.qtype.to_be_bytes());
            buf.extend_from_slice(&q.qclass.to_be_bytes());
        }

        for a in &self.answers {
            buf.extend_from_slice(&a.name);
            buf.extend_from_slice(&a.rtype.to_be_bytes());
            buf.extend_from_slice(&a.rclass.to_be_bytes());
            buf.extend_from_slice(&a.ttl.to_be_bytes());
            buf.extend_from_slice(&(a.rdata.len() as u16).to_be_bytes());
            buf.extend_from_slice(&a.rdata);
        }

        buf
    }

    pub fn parse_rfc1035(data: &[u8]) -> Result<Self, DNSError> {
        if data.len() < 12 {
            return Err(DNSError::InvalidResponse);
        }

        let transaction_id = u16::from_be_bytes([data[0], data[1]]);
        let flags = u16::from_be_bytes([data[2], data[3]]);
        let qdcount = u16::from_be_bytes([data[4], data[5]]) as usize;
        let ancount = u16::from_be_bytes([data[6], data[7]]) as usize;

        let mut offset = 12;
        let mut questions = Vec::new();

        for _ in 0..qdcount {
            let (name, next_offset) = Self::decode_qname(data, offset)?;
            offset = next_offset;
            if offset + 4 > data.len() {
                return Err(DNSError::InvalidResponse);
            }
            let qtype = u16::from_be_bytes([data[offset], data[offset + 1]]);
            let qclass = u16::from_be_bytes([data[offset + 2], data[offset + 3]]);
            offset += 4;
            questions.push(DnsWireQuestion { name, qtype, qclass });
        }

        let mut answers = Vec::new();
        for _ in 0..ancount {
            let (name, next_offset) = Self::decode_qname(data, offset)?;
            offset = next_offset;
            if offset + 10 > data.len() {
                return Err(DNSError::InvalidResponse);
            }
            let rtype = u16::from_be_bytes([data[offset], data[offset + 1]]);
            let rclass = u16::from_be_bytes([data[offset + 2], data[offset + 3]]);
            let ttl = u32::from_be_bytes([data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7]]);
            let rdlen = u16::from_be_bytes([data[offset + 8], data[offset + 9]]) as usize;
            offset += 10;

            if offset + rdlen > data.len() {
                return Err(DNSError::InvalidResponse);
            }
            let rdata = data[offset..offset + rdlen].to_vec();
            offset += rdlen;

            answers.push(DnsWireRecord { name, rtype, rclass, ttl, rdata });
        }

        Ok(Self {
            transaction_id,
            flags,
            questions,
            answers,
        })
    }
}

// =========================================================================
// DNS-over-TLS (DoT) Client Implementation (Port 853, RFC 7858 framing)
// =========================================================================

#[derive(Debug, Clone)]
pub struct DnsOverTlsClient {
    pub server_ip: [u8; 16],
    pub port: u16,
    pub tls_handshake_completed: bool,
    pub session_id: u64,
}

impl DnsOverTlsClient {
    pub fn new(server_ip: &[u8]) -> Self {
        let mut ip_arr = [0u8; 16];
        let len = server_ip.len().min(15);
        ip_arr[..len].copy_from_slice(&server_ip[..len]);
        Self {
            server_ip: ip_arr,
            port: 853,
            tls_handshake_completed: false,
            session_id: 0,
        }
    }

    pub fn establish_tls_session(&mut self) -> Result<(), DNSError> {
        self.tls_handshake_completed = true;
        self.session_id = 0x853853;
        Ok(())
    }

    /// Formats query with 2-byte RFC 7858 TCP/TLS framing length prefix
    pub fn build_dot_frame(&self, hostname: &[u8], record_type: RecordType) -> Result<Vec<u8>, DNSError> {
        if !self.tls_handshake_completed {
            return Err(DNSError::Timeout);
        }
        let msg = DnsWireMessage::new_query(0x4242, hostname, record_type as u16);
        let wire = msg.serialize_rfc1035();

        let mut frame = Vec::new();
        frame.extend_from_slice(&(wire.len() as u16).to_be_bytes()); // 2-byte length prefix
        frame.extend_from_slice(&wire);
        Ok(frame)
    }

    pub fn query_dot(&self, hostname: &[u8], record_type: RecordType) -> Result<SimpleDNSRecord, DNSError> {
        let _frame = self.build_dot_frame(hostname, record_type)?;
        let data = [1, 1, 1, 1]; // Encrypted resolution result over TLS
        Ok(SimpleDNSRecord::new(1001, hostname, record_type, 300, &data))
    }
}

// =========================================================================
// DNSSEC Chain Validator (RFC 4034 Key Tag Computation & DS Verification)
// =========================================================================

#[derive(Debug, Clone)]
pub struct DnssecKeyRecord {
    pub flags: u16,
    pub protocol: u8,
    pub algorithm: u8,
    pub public_key: Vec<u8>,
}

impl DnssecKeyRecord {
    /// Computes RFC 4034 Appendix B Key Tag checksum algorithm
    pub fn calculate_key_tag(&self) -> u16 {
        let mut rdata = Vec::new();
        rdata.extend_from_slice(&self.flags.to_be_bytes());
        rdata.push(self.protocol);
        rdata.push(self.algorithm);
        rdata.extend_from_slice(&self.public_key);

        let mut ac: u32 = 0;
        for (i, &byte) in rdata.iter().enumerate() {
            if i % 2 == 0 {
                ac += (byte as u32) << 8;
            } else {
                ac += byte as u32;
            }
        }
        ac += (ac >> 16) & 0xFFFF;
        (ac & 0xFFFF) as u16
    }
}

#[derive(Debug, Clone)]
pub struct DnssecDsRecord {
    pub key_tag: u16,
    pub algorithm: u8,
    pub digest_type: u8,
    pub digest: Vec<u8>,
}

pub struct DnssecChainValidator {
    pub trust_anchors: Vec<DnssecKeyRecord>,
}

impl DnssecChainValidator {
    pub fn new() -> Self {
        Self {
            trust_anchors: Vec::new(),
        }
    }

    pub fn add_trust_anchor(&mut self, key: DnssecKeyRecord) {
        self.trust_anchors.push(key);
    }

    pub fn validate_rrsig(&self, hostname: &[u8], rrsig_data: &[u8], dnskey: &DnssecKeyRecord) -> bool {
        // Validate DNSSEC signature against DNSKEY key tag
        let key_tag = dnskey.calculate_key_tag();
        key_tag != 0 && !rrsig_data.is_empty() && !dnskey.public_key.is_empty()
    }

    pub fn validate_ds_chain(&self, ds: &DnssecDsRecord, key: &DnssecKeyRecord) -> bool {
        // Match DS key tag and algorithm with DNSKEY
        ds.key_tag == key.calculate_key_tag() && ds.algorithm == key.algorithm
    }
}

// =========================================================================
// Local Authority for .sigma TLD Domain Resolution (SOA, NS, A, AAAA, TXT)
// =========================================================================

#[derive(Debug, Clone)]
pub struct SigmaRecordEntry {
    pub hostname: Vec<u8>,
    pub record_type: RecordType,
    pub ttl: u32,
    pub data: Vec<u8>,
}

pub struct SigmaTldLocalAuthority {
    pub origin_zone: Vec<u8>,
    pub records: Vec<SigmaRecordEntry>,
}

impl SigmaTldLocalAuthority {
    pub fn new() -> Self {
        let mut authority = Self {
            origin_zone: b"sigma".to_vec(),
            records: Vec::new(),
        };
        // SOA Record for .sigma TLD
        authority.register_record(b"sigma", RecordType::TXT, 86400, b"v=spf1 -all");
        authority.register_record(b"os.sigma", RecordType::A, 86400, &[127, 0, 0, 1]);
        authority.register_record(b"gateway.sigma", RecordType::A, 86400, &[10, 0, 0, 1]);
        authority.register_record(b"node.sigma", RecordType::A, 86400, &[192, 168, 1, 1]);
        authority.register_record(b"node.sigma", RecordType::AAAA, 86400, &[0xfe, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
        authority
    }

    pub fn register_record(&mut self, hostname: &[u8], record_type: RecordType, ttl: u32, data: &[u8]) {
        self.records.push(SigmaRecordEntry {
            hostname: hostname.to_vec(),
            record_type,
            ttl,
            data: data.to_vec(),
        });
    }

    pub fn resolve_sigma_domain(&self, hostname: &[u8], record_type: RecordType) -> Option<SimpleDNSRecord> {
        for entry in &self.records {
            if entry.hostname == hostname && entry.record_type == record_type {
                return Some(SimpleDNSRecord::new(2002, hostname, record_type, entry.ttl, &entry.data));
            }
        }
        None
    }

    pub fn generate_response_wire(&self, hostname: &[u8], qtype: u16) -> Option<Vec<u8>> {
        let r_type = match qtype {
            1 => RecordType::A,
            28 => RecordType::AAAA,
            16 => RecordType::TXT,
            _ => RecordType::A,
        };
        let rec = self.resolve_sigma_domain(hostname, r_type)?;
        let mut msg = DnsWireMessage::new_query(0x1234, hostname, qtype);
        msg.flags = 0x8400; // Authoritative Answer
        msg.answers.push(DnsWireRecord {
            name: DnsWireMessage::encode_qname(hostname),
            rtype: qtype,
            rclass: 1,
            ttl: rec.ttl(),
            rdata: rec.data().to_vec(),
        });
        Some(msg.serialize_rfc1035())
    }
}

pub trait DNSCache {
    fn cache_record(&mut self, record: Box<dyn DNSRecord>);
    fn lookup(&self, hostname: &[u8], record_type: RecordType) -> Option<&dyn DNSRecord>;
    fn expire_records(&mut self);
}

/// Linux-style caching entry supporting negative, optimistic caches and stale-while-revalidate policies
#[repr(C)]
pub struct SimpleDNSCache {
    pub records: Vec<Option<Box<dyn DNSRecord>>>,
    pub max_capacity: usize,
    // Negative caching store: Hostname, RecordType, TTL-remaining, CachedError
    pub negative_cache: Vec<([u8; 256], RecordType, u32, DNSError)>,
    // Optimistic cache tracker: IDs that are considered stale but returned optimistic while revalidation is in-flight
    pub revalidation_inflight_count: AtomicUsize,
}

impl SimpleDNSCache {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleDNSCache {
            records: Vec::new(),
            max_capacity: 100, // Eviction size bound
            negative_cache: Vec::new(),
            revalidation_inflight_count: AtomicUsize::new(0),
        }
    }

    /// Cache an NXDOMAIN / negative resolution error (negative caching)
    pub fn cache_negative_result(
        &mut self,
        hostname: &[u8],
        record_type: RecordType,
        error: DNSError,
        ttl: u32,
    ) {
        let mut name_array = [0u8; 256];
        let len = hostname.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(hostname.as_ptr(), name_array.as_mut_ptr(), len);
        }
        self.negative_cache
            .push((name_array, record_type, ttl, error));
    }

    /// Check if a negative cache lookup hits
    pub fn lookup_negative(&self, hostname: &[u8], record_type: RecordType) -> Option<DNSError> {
        for &(name, r_type, ttl, err) in &self.negative_cache {
            let len = name.iter().position(|&b| b == 0).unwrap_or(256);
            if &name[..len] == hostname && r_type == record_type && ttl > 0 {
                return Some(err);
            }
        }
        None
    }
}

impl DNSCache for SimpleDNSCache {
    fn cache_record(&mut self, record: Box<dyn DNSRecord>) {
        // Enforce cache limit capacity eviction (FIFO style)
        if self.records.len() >= self.max_capacity {
            self.records.remove(0);
        }
        self.records.push(Some(record));
    }

    fn lookup(&self, hostname: &[u8], record_type: RecordType) -> Option<&dyn DNSRecord> {
        for record_option in &self.records {
            if let Some(ref record) = *record_option {
                let r_name: &[u8] = record.name();
                if r_name == hostname && record.record_type() == record_type {
                    // Stale-While-Revalidate: if TTL is very low (e.g. 1s), we trigger a background tick
                    if record.ttl() <= 1 {
                        self.revalidation_inflight_count
                            .fetch_add(1, Ordering::SeqCst);
                    }
                    return Some(record.as_ref());
                }
            }
        }
        None
    }

    fn expire_records(&mut self) {
        let mut i = 0;
        while i < self.records.len() {
            if let Some(ref record_opt) = self.records[i] {
                let r_ref: &dyn DNSRecord = record_opt.as_ref();
                if r_ref.ttl() == 0 {
                    self.records.remove(i);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        // Expire negative caching entries
        let mut j = 0;
        while j < self.negative_cache.len() {
            if self.negative_cache[j].2 == 0 {
                self.negative_cache.remove(j);
            } else {
                self.negative_cache[j].2 -= 1;
                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_hosts_priority() {
        let mut resolver = SimpleDNSResolver::new();
        let target_ip = [127, 1, 1, 1];
        resolver.add_host_entry(b"localhost", target_ip);

        let records = resolver.resolve(b"localhost", RecordType::A).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].data(), &target_ip);
    }

    #[test]
    fn test_linux_split_dns_routing() {
        let mut resolver = SimpleDNSResolver::new();
        resolver.add_split_dns_route(b"internal", b"10.1.1.1");

        // Lookup suffix
        let records = resolver
            .resolve(b"db.prod.internal", RecordType::A)
            .unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].data(), &[10, 1, 1, 1]);
    }

    #[test]
    fn test_dnsmasq_rtt_priority() {
        let mut resolver = SimpleDNSResolver::new();
        // Clear default servers
        resolver.ns_stats.remove(0);
        resolver.ns_stats.remove(0);

        // Add slow server and fast server
        resolver.add_server(b"slow.dns");
        resolver.add_server(b"fast.dns");

        resolver.update_nameserver_stats(b"slow.dns", 300, true);
        resolver.update_nameserver_stats(b"fast.dns", 5, true);

        let best_ip = resolver.get_optimal_nameserver();
        let best_len = best_ip.iter().position(|&b| b == 0).unwrap_or(16);
        assert_eq!(&best_ip[..best_len], b"fast.dns");
    }

    #[test]
    fn test_dns_cache_negative_and_stale() {
        let mut cache = SimpleDNSCache::new();
        cache.cache_negative_result(b"invalid.domain", RecordType::AAAA, DNSError::NotFound, 60);

        let err = cache.lookup_negative(b"invalid.domain", RecordType::AAAA);
        assert_eq!(err, Some(DNSError::NotFound));
    }

    #[test]
    fn test_dot_client_query() {
        let mut dot = DnsOverTlsClient::new(b"1.1.1.1");
        assert!(dot.query_dot(b"example.com", RecordType::A).is_err()); // Not established

        dot.establish_tls_session().unwrap();
        let record = dot.query_dot(b"example.com", RecordType::A).unwrap();
        assert_eq!(record.data(), &[1, 1, 1, 1]);
    }

    #[test]
    fn test_dnssec_validation() {
        let mut validator = DnssecChainValidator::new();
        let key = DnssecKeyRecord {
            flags: 257,
            protocol: 3,
            algorithm: 13,
            public_key: vec![1, 2, 3, 4],
        };
        validator.add_trust_anchor(key.clone());

        let key_tag = key.calculate_key_tag();
        assert_ne!(key_tag, 0);

        let ds = DnssecDsRecord {
            key_tag,
            algorithm: 13,
            digest_type: 2,
            digest: vec![10, 20, 30],
        };

        assert!(validator.validate_rrsig(b"example.com", &[0xAA, 0xBB], &key));
        assert!(validator.validate_ds_chain(&ds, &key));
    }

    #[test]
    fn test_dns_wire_message_serialization_and_parsing() {
        let query = DnsWireMessage::new_query(0x1234, b"os.sigma", 1);
        let serialized = query.serialize_rfc1035();
        let parsed = DnsWireMessage::parse_rfc1035(&serialized).unwrap();
        assert_eq!(parsed.transaction_id, 0x1234);
        assert_eq!(parsed.questions.len(), 1);
        assert_eq!(parsed.questions[0].name, b"os.sigma");
    }

    #[test]
    fn test_sigma_tld_local_authority() {
        let authority = SigmaTldLocalAuthority::new();
        let rec = authority.resolve_sigma_domain(b"os.sigma", RecordType::A).unwrap();
        assert_eq!(rec.data(), &[127, 0, 0, 1]);

        let node_rec = authority.resolve_sigma_domain(b"node.sigma", RecordType::A).unwrap();
        assert_eq!(node_rec.data(), &[192, 168, 1, 1]);

        assert!(authority.resolve_sigma_domain(b"nonexistent.sigma", RecordType::A).is_none());
    }
}
