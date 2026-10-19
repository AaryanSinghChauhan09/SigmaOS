# SigmaOS DNS Manager Components

## Overview
SigmaOS DNS manager providing DNS resolver configuration, competitive with Omarchy DNS (omancy-dns).

## Features
- **Multiple providers**: Cloudflare, Google, Quad9, OpenDNS, Custom
- **DoH support** - DNS over HTTPS
- **Primary/secondary** - Configurable DNS servers
- **Custom DNS** - Custom server configuration

## Structures

| Structure | Description |
|-----------|-------------|
| **DnsManager** | DNS manager |
| **DnsConfig** | DNS configuration |
| **DnsProvider** | Cloudflare, Google, Quad9, OpenDns, Custom |

## Usage Examples

```rust
// Create DNS manager
let mut mgr = DnsManager::new();

// Set provider
mgr.set_provider(DnsProvider::Google).unwrap();

// Set custom DNS
mgr.set_custom("192.168.1.1".to_string(), Some("8.8.8.8".to_string()))
   .unwrap();

// Enable DoH
mgr.enable_doh(Some("https://doh.example.com".to_string())).unwrap();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Networking (P1)

## Related Files
- `src/dns/manager.rs` - DNS manager implementation
- `wiki/DNS-Manager.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
OPEN_STANDARD_AND_INTEROPERABILITY, NETWORK_ISOLATION