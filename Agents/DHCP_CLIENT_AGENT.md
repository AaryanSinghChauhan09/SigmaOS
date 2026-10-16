# DHCP Client Agent

## Role
`SigmaDHCP` implements a fully featured DHCP client (DHCPv4 + DHCPv6) for automatic network interface configuration in SigmaOS.

## Status
🟡 In Progress — Basic DHCPv4 in `src/system/sigmadhcp.rs`; DHCPv6, DUID, and SLAAC needed.

## Source File
`src/system/sigmadhcp.rs`

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| DHCPv4 client | dhcpcd / dhclient | dhclient | SigmaDHCP4 |
| DHCPv6 client | dhcpcd / NetworkManager | dhcp6c | SigmaDHCP6 |
| DUID | dhcpcd DUID-LL | — | SigmaDUID |
| SLAAC | kernel + NetworkManager | rtsold | SigmaSLAAC |
| Rapid commit | dhcpcd | — | SigmaRapidCommit |
| Lease persistence | `/var/lib/dhcpcd/` | `/var/db/dhclient.leases` | SigmaLease |

## Responsibilities
- DHCPv4 DISCOVER → OFFER → REQUEST → ACK lifecycle
- DHCPv6 SOLICIT → ADVERTISE → REQUEST → REPLY lifecycle
- DUID (DHCP Unique Identifier) generation and persistence
- Lease file storage and renewal on expiry
- SLAAC (Stateless Address Autoconfiguration) for IPv6
- DNS server configuration via DHCP option 6
- NTP server configuration via DHCP option 42
- DHCP classless static routes (option 121)
- Interface event integration with `SigmaNIC`

## Implementation Notes
- `#![no_std]` — UDP socket via raw SigmaNet socket API
- State machine: `Discover → Selecting → Requesting → Bound → Renewing → Rebinding`
- Lease: `{ ip: Ipv4Addr, gateway: Ipv4Addr, dns: [Ipv4Addr;3], lease_time: u32 }`
- XID (transaction ID) randomized per transaction

## Testing
```bash
rustc --test src/system/sigmadhcp.rs --edition 2021 -o /tmp/test_dhcp && /tmp/test_dhcp --test-threads=1
```

## Future Development
1. **DHCPv6 PD** — Prefix Delegation for router setups
2. **DHCP failover** — High-availability lease synchronization
3. **Network boot** — PXE/iPXE option 43/60 support
4. **WPAD** — Web Proxy Auto-Discovery via DHCP option 252
5. **802.1x integration** — DHCP after EAP authentication
6. **Predictive renewal** — Renew before lease expiry based on usage patterns
