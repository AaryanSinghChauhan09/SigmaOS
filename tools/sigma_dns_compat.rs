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
// =========================================================================

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
