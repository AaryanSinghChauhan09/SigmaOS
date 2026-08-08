// SigmaOS Network Protocol Layer
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

/// Linux & BSD inspired DNS Resolver for SigmaOS
/// Supporting resolv.conf parsing, nsswitch.conf priority routing, and Unbound-parity dynamic caching.

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RecordID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType { A = 1, AAAA = 28, CNAME = 5, MX = 15, TXT = 16 }

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DNSError { Success = 0, NotFound = 1, Timeout = 2, InvalidResponse = 3 }

pub trait DNSRecord {
    fn id(&self) -> RecordID;
    fn name(&self) -> &[u8];
    fn record_type(&self) -> RecordType;
    fn ttl(&self) -> u32;
    fn data(&self) -> &[u8];
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SimpleDNSRecord {
    pub id: RecordID,
    pub name: [u8; 256],
    pub record_type: RecordType,
    pub ttl: u32,
    pub data: [u8; 128],
}

impl SimpleDNSRecord {
    pub fn new(id: RecordID, name: &[u8], record_type: RecordType, ttl: u32, data: &[u8]) -> Self {
        let mut name_array = [0u8; 256];
        let mut data_array = [0u8; 128];
        let name_len = name.len().min(255);
        let data_len = data.len().min(127);
        for i in 0..name_len {
            name_array[i] = name[i];
        }
        for i in 0..data_len {
            data_array[i] = data[i];
        }
        SimpleDNSRecord {
            id,
            name: name_array,
            record_type,
            ttl,
            data: data_array,
        }
    }
}

impl DNSRecord for SimpleDNSRecord {
    fn id(&self) -> RecordID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(256);
        &self.name[..len]
    }
    fn record_type(&self) -> RecordType { self.record_type }
    fn ttl(&self) -> u32 { self.ttl }
    fn data(&self) -> &[u8] {
        let len = self.data.iter().position(|&b| b == 0).unwrap_or(128);
        &self.data[..len]
    }
}

// =========================================================================
// ResolvConf (/etc/resolv.conf representation)
// =========================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResolvConf {
    pub nameservers: [[u8; 16]; 4],
    pub nameservers_count: usize,
    pub rotate: bool,
    pub timeout_sec: u32,
    pub attempts: u32,
}

impl ResolvConf {
    pub fn new() -> Self {
        let mut conf = ResolvConf {
            nameservers: [[0u8; 16]; 4],
            nameservers_count: 0,
            rotate: false,
            timeout_sec: 5,
            attempts: 2,
        };
        // Fallback nameservers
        conf.add_nameserver("8.8.8.8");
        conf.add_nameserver("1.1.1.1");
        conf
    }

    pub fn add_nameserver(&mut self, ip: &str) {
        if self.nameservers_count < 4 {
            let mut arr = [0u8; 16];
            let bytes = ip.as_bytes();
            for i in 0..bytes.len().min(15) {
                arr[i] = bytes[i];
            }
            self.nameservers[self.nameservers_count] = arr;
            self.nameservers_count += 1;
        }
    }

    pub fn parse_resolv_conf(&mut self, text: &str) {
        // Reset nameservers count for fresh parsing
        self.nameservers_count = 0;
        self.rotate = false;
        self.timeout_sec = 5;
        self.attempts = 2;

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                continue;
            }

            if trimmed.starts_with("nameserver ") {
                let ip = trimmed[11..].trim();
                self.add_nameserver(ip);
            } else if trimmed.starts_with("options ") {
                let opts = trimmed[8..].trim();
                // Check options like "rotate", "timeout:X", "attempts:Y"
                if opts.contains("rotate") {
                    self.rotate = true;
                }
                if let Some(t_idx) = opts.find("timeout:") {
                    let mut num = 0u32;
                    let num_part = &opts[t_idx + 8..];
                    for b in num_part.bytes() {
                        if b >= b'0' && b <= b'9' {
                            num = num * 10 + (b - b'0') as u32;
                        } else {
                            break;
                        }
                    }
                    if num > 0 {
                        self.timeout_sec = num;
                    }
                }
                if let Some(a_idx) = opts.find("attempts:") {
                    let mut num = 0u32;
                    let num_part = &opts[a_idx + 9..];
                    for b in num_part.bytes() {
                        if b >= b'0' && b <= b'9' {
                            num = num * 10 + (b - b'0') as u32;
                        } else {
                            break;
                        }
                    }
                    if num > 0 {
                        self.attempts = num;
                    }
                }
            }
        }
    }
}

// =========================================================================
// HostsDatabase (/etc/hosts representation)
// =========================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HostEntry {
    pub ip: [u8; 4],
    pub hostname: [u8; 64],
}

impl HostEntry {
    pub fn matches(&self, name_str: &str) -> bool {
        let bytes = name_str.as_bytes();
        let mut entry_len = 0;
        while entry_len < 64 && self.hostname[entry_len] != 0 {
            entry_len += 1;
        }
        if entry_len != bytes.len() {
            return false;
        }
        for i in 0..entry_len {
            if self.hostname[i].to_ascii_lowercase() != bytes[i].to_ascii_lowercase() {
                return false;
            }
        }
        true
    }
}

#[repr(C)]
pub struct HostsDatabase {
    pub entries: Vec<HostEntry>,
}

impl HostsDatabase {
    pub fn new() -> Self {
        HostsDatabase {
            entries: Vec::new(),
        }
    }

    pub fn add_host(&mut self, hostname: &str, ip: [u8; 4]) {
        let mut entry = HostEntry {
            ip,
            hostname: [0u8; 64],
        };
        let bytes = hostname.as_bytes();
        for i in 0..bytes.len().min(63) {
            entry.hostname[i] = bytes[i];
        }
        self.entries.push(entry);
    }

    pub fn parse_hosts_file(&mut self, text: &str) {
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            // Parse: "127.0.0.1 localhost"
            let mut parts = trimmed.split_whitespace();
            if let Some(ip_str) = parts.next() {
                if let Some(host_str) = parts.next() {
                    // Parse ip_str to [u8; 4]
                    let mut ip_arr = [0u8; 4];
                    let mut octet_idx = 0;
                    let mut current_val = 0u8;
                    for b in ip_str.bytes() {
                        if b == b'.' {
                            if octet_idx < 4 {
                                ip_arr[octet_idx] = current_val;
                                octet_idx += 1;
                                current_val = 0;
                            }
                        } else if b >= b'0' && b <= b'9' {
                            current_val = current_val.saturating_mul(10).saturating_add(b - b'0');
                        }
                    }
                    if octet_idx < 4 {
                        ip_arr[octet_idx] = current_val;
                    }
                    self.add_host(host_str, ip_arr);
                }
            }
        }
    }
}

// =========================================================================
// NSSwitch configuration order
// =========================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NssHostsOrder {
    FilesFirst = 0,
    DnsFirst = 1,
}

// =========================================================================
// Unbound-parity Cache
// =========================================================================

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NegativeCacheEntry {
    pub hostname: [u8; 64],
    pub record_type: RecordType,
    pub ttl: u32, // Remaining ticks until expiration
}

impl NegativeCacheEntry {
    pub fn matches(&self, hostname_str: &str, record_type: RecordType) -> bool {
        if self.record_type != record_type {
            return false;
        }
        let bytes = hostname_str.as_bytes();
        let mut entry_len = 0;
        while entry_len < 64 && self.hostname[entry_len] != 0 {
            entry_len += 1;
        }
        if entry_len != bytes.len() {
            return false;
        }
        for i in 0..entry_len {
            if self.hostname[i].to_ascii_lowercase() != bytes[i].to_ascii_lowercase() {
                return false;
            }
        }
        true
    }
}

#[repr(C)]
pub struct UnboundCache {
    pub positive_records: Vec<SimpleDNSRecord>,
    pub negative_records: Vec<NegativeCacheEntry>,
}

impl UnboundCache {
    pub fn new() -> Self {
        UnboundCache {
            positive_records: Vec::new(),
            negative_records: Vec::new(),
        }
    }

    pub fn insert_positive(&mut self, record: SimpleDNSRecord) {
        self.positive_records.push(record);
    }

    pub fn insert_negative(&mut self, hostname: &str, record_type: RecordType, ttl: u32) {
        let mut entry = NegativeCacheEntry {
            hostname: [0u8; 64],
            record_type,
            ttl,
        };
        let bytes = hostname.as_bytes();
        for i in 0..bytes.len().min(63) {
            entry.hostname[i] = bytes[i];
        }
        self.negative_records.push(entry);
    }

    pub fn lookup_positive(&self, hostname_str: &str, record_type: RecordType) -> Option<SimpleDNSRecord> {
        let bytes = hostname_str.as_bytes();
        for i in 0..self.positive_records.len {
            let rec = unsafe { &*self.positive_records.data.add(i) };
            if rec.record_type == record_type {
                // Perform name matches
                let mut matches = true;
                let mut name_len = 0;
                while name_len < 256 && rec.name[name_len] != 0 {
                    name_len += 1;
                }
                if name_len != bytes.len() {
                    matches = false;
                } else {
                    for j in 0..name_len {
                        if rec.name[j].to_ascii_lowercase() != bytes[j].to_ascii_lowercase() {
                            matches = false;
                            break;
                        }
                    }
                }
                if matches {
                    return Some(*rec);
                }
            }
        }
        None
    }

    pub fn lookup_negative(&self, hostname_str: &str, record_type: RecordType) -> bool {
        for i in 0..self.negative_records.len {
            let entry = unsafe { &*self.negative_records.data.add(i) };
            if entry.matches(hostname_str, record_type) && entry.ttl > 0 {
                return true;
            }
        }
        false
    }

    pub fn tick_down_ttl(&mut self, elapsed_ticks: u32) {
        // Expire positive
        let mut i = 0;
        while i < self.positive_records.len {
            let rec = unsafe { &mut *self.positive_records.data.add(i) };
            if rec.ttl <= elapsed_ticks {
                self.positive_records.remove(i);
            } else {
                rec.ttl -= elapsed_ticks;
                i += 1;
            }
        }
        // Expire negative
        let mut i = 0;
        while i < self.negative_records.len {
            let entry = unsafe { &mut *self.negative_records.data.add(i) };
            if entry.ttl <= elapsed_ticks {
                self.negative_records.remove(i);
            } else {
                entry.ttl -= elapsed_ticks;
                i += 1;
            }
        }
    }
}

// =========================================================================
// SovereignDNSResolver (Standard DNSResolver parity)
// =========================================================================

pub trait DNSResolver {
    fn resolve(&mut self, hostname: &str, record_type: RecordType) -> Result<Vec<SimpleDNSRecord>, DNSError>;
    fn resolve_with_failover(&mut self, hostname: &str, record_type: RecordType, simulate_timeout: bool) -> Result<Vec<SimpleDNSRecord>, DNSError>;
}

#[repr(C)]
pub struct SovereignDNSResolver {
    pub resolv_conf: ResolvConf,
    pub hosts_db: HostsDatabase,
    pub nss_order: NssHostsOrder,
    pub cache: UnboundCache,
    pub current_nameserver_idx: usize,
    pub next_record_id: usize,
}

impl SovereignDNSResolver {
    pub fn new() -> Self {
        SovereignDNSResolver {
            resolv_conf: ResolvConf::new(),
            hosts_db: HostsDatabase::new(),
            nss_order: NssHostsOrder::FilesFirst,
            cache: UnboundCache::new(),
            current_nameserver_idx: 0,
            next_record_id: 1,
        }
    }

    fn query_nameserver_simulated(&mut self, hostname: &str, record_type: RecordType, simulate_timeout: bool) -> Result<SimpleDNSRecord, DNSError> {
        if simulate_timeout {
            return Err(DNSError::Timeout);
        }

        // Return a simulated ip address based on the queried name
        let id = self.next_record_id;
        self.next_record_id += 1;

        let mut data = [0u8; 4];
        if hostname.contains("example") {
            data = [93, 184, 216, 34];
        } else {
            data = [192, 168, 5, 5];
        }

        Ok(SimpleDNSRecord::new(id, hostname.as_bytes(), record_type, 300, &data))
    }
}

impl DNSResolver for SovereignDNSResolver {
    fn resolve(&mut self, hostname: &str, record_type: RecordType) -> Result<Vec<SimpleDNSRecord>, DNSError> {
        self.resolve_with_failover(hostname, record_type, false)
    }

    fn resolve_with_failover(&mut self, hostname: &str, record_type: RecordType, simulate_timeout: bool) -> Result<Vec<SimpleDNSRecord>, DNSError> {
        // 1. Check NSSwitch order precedence
        if self.nss_order == NssHostsOrder::FilesFirst {
            for i in 0..self.hosts_db.entries.len {
                let host = unsafe { &*self.hosts_db.entries.data.add(i) };
                if host.matches(hostname) {
                    let id = self.next_record_id;
                    self.next_record_id += 1;
                    let rec = SimpleDNSRecord::new(id, hostname.as_bytes(), record_type, 86400, &host.ip);
                    let mut res = Vec::new();
                    res.push(rec);
                    return Ok(res);
                }
            }
        }

        // 2. Check Positive / Negative Caching (Unbound parity)
        if self.cache.lookup_negative(hostname, record_type) {
            return Err(DNSError::NotFound);
        }
        if let Some(cached_rec) = self.cache.lookup_positive(hostname, record_type) {
            let mut res = Vec::new();
            res.push(cached_rec);
            return Ok(res);
        }

        // 3. Fallback Nameservers Failover & Load-balancing (rotate option)
        if self.resolv_conf.nameservers_count == 0 {
            return Err(DNSError::NotFound);
        }

        let mut last_err = DNSError::Timeout;
        let mut attempts_remaining = self.resolv_conf.attempts;

        while attempts_remaining > 0 {
            // Determine nameserver index (support options rotate)
            let ns_idx = if self.resolv_conf.rotate {
                let idx = self.current_nameserver_idx;
                self.current_nameserver_idx = (self.current_nameserver_idx + 1) % self.resolv_conf.nameservers_count;
                idx
            } else {
                0
            };

            // Query
            match self.query_nameserver_simulated(hostname, record_type, simulate_timeout) {
                Ok(rec) => {
                    // Cache the hit (Positive Caching)
                    self.cache.insert_positive(rec);
                    let mut res = Vec::new();
                    res.push(rec);
                    return Ok(res);
                }
                Err(err) => {
                    last_err = err;
                    attempts_remaining -= 1;
                }
            }
        }

        // If not found, write a Negative Cache Entry (Negative Caching to shield nameservers)
        if last_err == DNSError::NotFound || last_err == DNSError::Timeout {
            self.cache.insert_negative(hostname, record_type, 60); // 60-ticks negative caching
        }

        Err(last_err)
    }
}

// =========================================================================
// OOP heap allocation-free/custom-heap Vec implementation
// =========================================================================

pub struct Vec<T> { pub data: *mut T, pub len: usize, pub capacity: usize }

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    if let Ok(layout) = Layout::from_size_align(size, 8) {
        std_alloc(layout)
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}
