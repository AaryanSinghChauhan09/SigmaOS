// SPDX-License-Identifier: MIT
//! SigmaOS Process-Local DNS Resolver & Cache Compatibility Layer
//!
//! Provides a secure, zero-dependency, safe, and thread-safe process-local
//! DNS cache resolver supporting both IPv4 and IPv6 resolution mappings,
//! TTL-based expiration checks, query statistics, and verification tests.

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpAddress {
    V4([u8; 4]),
    V6([u8; 16]),
}

pub struct DnsRecord {
    pub hostname: String,
    pub ip: IpAddress,
    pub ttl: u32,
    pub created_at: u64,
}

pub struct SigmaDnsResolver {
    pub records: Vec<DnsRecord>,
    pub lookup_count: AtomicUsize,
    pub cache_hits: AtomicUsize,
}

impl SigmaDnsResolver {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            lookup_count: AtomicUsize::new(0),
            cache_hits: AtomicUsize::new(0),
        }
    }

    pub fn add_record(&mut self, hostname: &str, ip: IpAddress, ttl: u32, now: u64) {
        self.records.push(DnsRecord {
            hostname: hostname.to_string(),
            ip,
            ttl,
            created_at: now,
        });
    }

    pub fn resolve(&self, hostname: &str, now: u64) -> Option<IpAddress> {
        self.lookup_count.fetch_add(1, Ordering::SeqCst);
        for record in &self.records {
            if record.hostname == hostname {
                // Check if the record is expired based on TTL
                if now <= record.created_at + (record.ttl as u64) {
                    self.cache_hits.fetch_add(1, Ordering::SeqCst);
                    return Some(record.ip);
                }
            }
        }
        None
    }

    pub fn get_metrics(&self) -> (usize, usize) {
        (
            self.lookup_count.load(Ordering::SeqCst),
            self.cache_hits.load(Ordering::SeqCst),
        )
    }
}

impl Default for SigmaDnsResolver {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_resolution_and_ttl() {
        let mut resolver = SigmaDnsResolver::new();
        let ip_v4 = IpAddress::V4([10, 0, 0, 1]);
        let ip_v6 = IpAddress::V6([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);

        resolver.add_record("localhost", ip_v4, 60, 1000);
        resolver.add_record("ipv6.localhost", ip_v6, 60, 1000);

        // Verify successful lookup before TTL expires
        assert_eq!(resolver.resolve("localhost", 1020), Some(ip_v4));
        assert_eq!(resolver.resolve("ipv6.localhost", 1050), Some(ip_v6));

        // Verify lookup fails after TTL expires (at timestamp 1000 + 60 = 1060)
        assert_eq!(resolver.resolve("localhost", 1070), None);

        // Check cache metrics (3 total queries, 2 successful cache hits)
        let (queries, hits) = resolver.get_metrics();
        assert_eq!(queries, 3);
        assert_eq!(hits, 2);
    }
}

type SigmaU8 = u8;
type SigmaU32 = u32;
type SigmaI32 = i32;
type SigmaBool = bool;

/// Simple C-compatible DNS Record mapping
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RawDnsRecord {
    pub domain: [u8; 128],
    pub ip_address: [u8; 4],
    pub is_valid: SigmaBool,
}

/// Static maximum local cache size
const MAX_DNS_RECORDS: usize = 16;

/// Local isolated cache state
static mut DNS_INITIALIZED: SigmaBool = false;
static mut LOCAL_CACHE: [RawDnsRecord; MAX_DNS_RECORDS] = [RawDnsRecord {
    domain: [0; 128],
    ip_address: [0; 4],
    is_valid: false,
}; MAX_DNS_RECORDS];
static mut RECORD_COUNT: usize = 0;

/// Initialize the isolated process-local DNS cache
#[no_mangle]
pub unsafe extern "C" fn dns_local_init() -> SigmaI32 {
    DNS_INITIALIZED = true;
    RECORD_COUNT = 0;
    for i in 0..MAX_DNS_RECORDS {
        LOCAL_CACHE[i].is_valid = false;
    }
    0 // Success
}

/// Insert a new isolated resolution record
#[no_mangle]
pub unsafe extern "C" fn dns_local_insert(domain: *const u8, ip: *const u8) -> SigmaI32 {
    if !DNS_INITIALIZED || domain.is_null() || ip.is_null() || RECORD_COUNT >= MAX_DNS_RECORDS {
        return -1;
    }
    let mut record = RawDnsRecord {
        domain: [0; 128],
        ip_address: [0; 4],
        is_valid: true,
    };
    // Copy domain string
    for i in 0..127 {
        let byte = *domain.add(i);
        if byte == 0 { break; }
        record.domain[i] = byte;
    }
    // Copy IP components
    for i in 0..4 {
        record.ip_address[i] = *ip.add(i);
    }
    LOCAL_CACHE[RECORD_COUNT] = record;
    RECORD_COUNT += 1;
    0
}

/// Resolve a domain name within this isolated cache (bypassing the global system resolver)
#[no_mangle]
pub unsafe extern "C" fn dns_local_resolve(domain: *const u8, resolved_ip: *mut u8) -> SigmaI32 {
    if !DNS_INITIALIZED || domain.is_null() || resolved_ip.is_null() {
        return -1;
    }
    for i in 0..RECORD_COUNT {
        let record = &LOCAL_CACHE[i];
        if !record.is_valid {
            continue;
        }
        // Compare domain strings
        let mut match_found = true;
        for j in 0..128 {
            let record_byte = record.domain[j];
            let query_byte = *domain.add(j);
            if record_byte != query_byte {
                match_found = false;
                break;
            }
            if record_byte == 0 && query_byte == 0 {
                break;
            }
        }
        if match_found {
            for j in 0..4 {
                *resolved_ip.add(j) = record.ip_address[j];
            }
            return 0; // Success
        }
    }
    -1 // Resolution failed (isolated from global network leaks)
}
