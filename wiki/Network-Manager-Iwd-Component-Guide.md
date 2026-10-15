# Network Interface Manager & IWD Fast Daemon Component Guide

## Component Overview
Governs wired Ethernet link auto-negotiation, dynamic DHCPv4/DHCPv6 lease acquisition, SLAAC IPv6 configuration, Wi-Fi profile management via IWD, and WireGuard VPN tunnels.

## Linux Inspiration
iwd (Intel Wireless Daemon), systemd-networkd, and NetworkManager.

## BSD Inspiration
OpenBSD dhcpleased and ifconfig, FreeBSD dhclient and wpa_supplicant.

## Current SigmaOS Status
Ethernet link carrier detection, DHCP state machine, and static route tables implemented in networking stack.

## Critical Missing Features
Declarative network connection profiles with priority-based multi-interface failover (switching Wi-Fi to Ethernet automatically).

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/net/ethernet.rs, src/net/ipv4.rs, src/wireless/wifi.rs

## Testing Strategy
Ethernet cable disconnect-reconnect failover timing tests (<500ms lease renewal), Wi-Fi roaming handover tests.

## Dependencies
NETWORKING_STACK_AGENT, WIRELESS_WIFI_AGENT, SECURITY_AGENT

## Success Criteria
Sub-second network lease acquisition, seamless carrier failover without dropping active TCP connections.

## Open Source Competitors Analysis
Bypasses the bloated D-Bus/GLib stack of NetworkManager in favor of an ultra-compact ~1,500-line safe Rust daemon.

## Future Enhancements
Multipath TCP (MPTCP) aggregating Wi-Fi and 5G cellular connections for combined throughput and zero-packet-drop failover.
