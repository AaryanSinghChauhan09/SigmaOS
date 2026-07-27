# OSS Absorption: NetworkManager

## Overview

NetworkManager is a daemon that provides dynamic network configuration and connections on Linux, widely used across almost all mainstream distributions. It handles wired, wireless, VPN, and mobile broadband connections.

## Key Principles Absorbed

### Dynamic DHCP Leasing

- NetworkManager relies on internal or external DHCP clients (like `dhclient`).
- SigmaOS absorbs this via `sigma_net::dhcp::DhcpClient`, a native Rust implementation for discovering and requesting IPv4 address leases natively on port 68.

### Integrated DNS Stub Resolver

- Linux distributions usually defer DNS to `systemd-resolved` or `dnsmasq`.
- SigmaOS implements a native stub resolver directly in the network stack: `sigma_net::dns::DnsResolver`.
- This ensures DNS resolution is tightly coupled with interface states and immune to cache poisoning via memory safety.

### Integrated NTP Client

- Network time synchronization (usually handled by `chrony` or `systemd-timesyncd`).
- SigmaOS implements SNTP (Simple Network Time Protocol) natively via `sigma_net::ntp::NtpClient`.

## Displaced Technologies

| Technology | SigmaOS Replacement |
| --- | --- |
| NetworkManager | `sigma_net` core state machine |
| dhclient | `sigma_net::dhcp::DhcpClient` |
| systemd-resolved | `sigma_net::dns::DnsResolver` |
| systemd-timesyncd | `sigma_net::ntp::NtpClient` |

## Status

**Core Absorbed** — The core struct definitions and API designs for native DHCP, DNS, and NTP have been integrated directly into `userland/sigma_net/src/`.
