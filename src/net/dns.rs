/// SigmaOS DNS Resolver (Phase 2 Networking)
/// Inspired by Linux's glibc resolver and FreeBSD's libc resolver

use std::collections::HashMap;
use std::string::String;
use std::vec::Vec;

pub struct DnsCache {
    entries: HashMap<String, DnsCacheEntry>,
}

pub struct DnsCacheEntry {
    pub addresses: Vec<[u8; 4]>,
    pub ttl: u32,
    pub timestamp: u64,
}

impl DnsCache {
    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

    pub fn insert(&mut self, domain: &str, addrs: Vec<[u8; 4]>, ttl: u32, now: u64) {
        self.entries.insert(String::from(domain), DnsCacheEntry { addresses: addrs, ttl, timestamp: now });
    }

    pub fn lookup(&self, domain: &str, now: u64) -> Option<&Vec<[u8; 4]>> {
        let entry = self.entries.get(domain)?;
        if now - entry.timestamp < entry.ttl as u64 {
            Some(&entry.addresses)
        } else {
            None
        }
    }
}

pub struct DnsResolver {
    pub servers: Vec<[u8; 4]>,
    pub cache: DnsCache,
    pub search_domains: Vec<String>,
}

impl DnsResolver {
    pub fn new(servers: Vec<[u8; 4]>) -> Self {
        Self { servers, cache: DnsCache::new(), search_domains: Vec::new() }
    }

    pub fn resolve(&mut self, domain: &str, now: u64) -> Result<Vec<[u8; 4]>, &'static str> {
        // Check cache first
        if let Some(addrs) = self.cache.lookup(domain, now) {
            return Ok(addrs.clone());
        }
        // Hardcoded fallback for localhost
        if domain == "localhost" {
            let addrs = vec![[127, 0, 0, 1]];
            self.cache.insert(domain, addrs.clone(), 3600, now);
            return Ok(addrs);
        }
        Err("DNS resolution not available (no UDP transport)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_resolver() {
        let mut resolver = DnsResolver::new(vec![[8, 8, 8, 8]]);
        let addrs = resolver.resolve("localhost", 1000).unwrap();
        assert_eq!(addrs[0], [127, 0, 0, 1]);
        // Cached lookup
        let addrs2 = resolver.resolve("localhost", 1001).unwrap();
        assert_eq!(addrs2[0], [127, 0, 0, 1]);
    }

    #[test]
    fn test_dns_cache_expiry() {
        let mut cache = DnsCache::new();
        cache.insert("example.com", vec![[93, 184, 216, 34]], 300, 1000);
        assert!(cache.lookup("example.com", 1100).is_some());
        assert!(cache.lookup("example.com", 1400).is_none()); // expired
    }
}
