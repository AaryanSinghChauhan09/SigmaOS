//! DNS resolution and mDNS announcement stub tests.
//!
//! Tests follow FIRST principles: fast, isolated (no real network),
//! repeatable, self-validating, timely.

use sigmaos_network::{
    DnsRecord, DnsResolver, MdnsAnnouncer, MdnsService, NetworkError,
};

// ---------------------------------------------------------------------------
// DnsResolver tests
// ---------------------------------------------------------------------------

#[test]
fn test_resolve_with_server_returns_stub_addr() {
    // Arrange — one DNS server configured
    let mut resolver = DnsResolver::new(vec![[8, 8, 8, 8]]);

    // Act
    let addr = resolver.resolve("example.com").expect("resolve failed");

    // Assert — stub returns loopback for any hostname
    assert_eq!(addr, [127, 0, 0, 1]);
}

#[test]
fn test_resolve_without_servers_fails() {
    // Arrange — no DNS servers
    let mut resolver = DnsResolver::new(vec![]);

    // Act
    let result = resolver.resolve("example.com");

    // Assert
    assert!(matches!(result, Err(NetworkError::DnsResolutionFailed(_))));
}

#[test]
fn test_resolve_caches_result() {
    // Arrange
    let mut resolver = DnsResolver::new(vec![[1, 1, 1, 1]]);
    resolver.resolve("cached.example.com").unwrap();

    // Act — second resolution should hit cache (cache length increases by 1,
    // then stays at 1 because the record is already there)
    assert_eq!(resolver.cache_len(), 1);
    resolver.resolve("cached.example.com").unwrap();

    // Assert — no duplicate inserted
    assert_eq!(resolver.cache_len(), 1);
}

#[test]
fn test_seed_prepopulates_cache() {
    // Arrange
    let mut resolver = DnsResolver::new(vec![]);
    let record = DnsRecord {
        name: "local.dev".to_string(),
        addr: [192, 168, 1, 10],
        ttl: 300,
    };

    // Act
    resolver.seed(record);

    // Assert
    assert_eq!(resolver.cache_len(), 1);
    let addr = resolver.resolve("local.dev").expect("seeded resolve failed");
    assert_eq!(addr, [192, 168, 1, 10]);
}

#[test]
fn test_multiple_seeds_independent() {
    // Arrange
    let mut resolver = DnsResolver::new(vec![]);
    resolver.seed(DnsRecord {
        name: "a.local".to_string(),
        addr: [10, 0, 0, 1],
        ttl: 60,
    });
    resolver.seed(DnsRecord {
        name: "b.local".to_string(),
        addr: [10, 0, 0, 2],
        ttl: 60,
    });

    // Act + Assert
    assert_eq!(resolver.resolve("a.local").unwrap(), [10, 0, 0, 1]);
    assert_eq!(resolver.resolve("b.local").unwrap(), [10, 0, 0, 2]);
}

// ---------------------------------------------------------------------------
// MdnsAnnouncer tests
// ---------------------------------------------------------------------------

#[test]
fn test_announcer_starts_after_register() {
    // Arrange
    let mut ann = MdnsAnnouncer::new();
    ann.register(MdnsService::new("SigmaOS Files", "_smb._tcp", 445));

    // Act
    ann.start().expect("start failed");

    // Assert
    assert!(ann.is_running());
}

#[test]
fn test_announcer_start_without_services_fails() {
    // Arrange — no services registered
    let mut ann = MdnsAnnouncer::new();

    // Act
    let result = ann.start();

    // Assert
    assert!(matches!(result, Err(NetworkError::MdnsError(_))));
    assert!(!ann.is_running());
}

#[test]
fn test_announcer_stop() {
    // Arrange
    let mut ann = MdnsAnnouncer::new();
    ann.register(MdnsService::new("SigmaOS HTTP", "_http._tcp", 80));
    ann.start().unwrap();

    // Act
    ann.stop();

    // Assert
    assert!(!ann.is_running());
}

#[test]
fn test_announcer_services_list() {
    // Arrange
    let mut ann = MdnsAnnouncer::new();
    let svc1 = MdnsService::new("SigmaOS SSH", "_ssh._tcp", 22);
    let svc2 = MdnsService::new("SigmaOS HTTP", "_http._tcp", 80);

    // Act
    ann.register(svc1.clone());
    ann.register(svc2.clone());

    // Assert
    assert_eq!(ann.services().len(), 2);
    assert_eq!(ann.services()[0], svc1);
    assert_eq!(ann.services()[1], svc2);
}

#[test]
fn test_announcer_default() {
    // Arrange + Act — Default impl should behave same as new()
    let ann = MdnsAnnouncer::default();

    // Assert
    assert!(!ann.is_running());
    assert!(ann.services().is_empty());
}
