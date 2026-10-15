# 802.11 Wi-Fi Networking & WPA3 Component Agent

## Component Overview
Governs 802.11 a/b/g/n/ac/ax station mode, NL80211 command interface, WPA2/WPA3-SAE cryptographic handshakes, dynamic roaming, and power saving.

## Linux Inspiration
Linux cfg80211 and mac80211 subsystems, iwd (Intel Wireless Daemon), and wpa_supplicant.

## BSD Inspiration
FreeBSD net80211 framework, OpenBSD ifconfig wireless scanning and wpa key exchange.

## Current SigmaOS Status
802.11 beacon frame parser, basic scanning probe engine, and VirtIO-Net WiFi adapter emulation in place. WPA3 SAE Dragonfly handshake requires completion.

## Critical Missing Features
Complete WPA3-Personal SAE (Simultaneous Authentication of Equals) handshake, background BSSID roaming, and 802.11ax Wi-Fi 6 OFDMA channel management.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/wireless/wifi.rs, src/wireless/wpa3.rs, src/net/ethernet.rs

## Testing Strategy
WPA2/WPA3 synthetic handshake verification, roaming signal-strength hysteresis test, frame packet injection checks.

## Dependencies
NETWORKING_STACK_AGENT, CRYPTOGRAPHY_AGENT, HARDWARE_HAL_DRIVERS_AGENT

## Success Criteria
Sub-1.5s initial Wi-Fi connection from cold boot, 0 packet loss during AP roaming, full WPA3 security validation.

## Open Source Competitors Analysis
Combines the lean speed of iwd with the robust multi-standard coverage of FreeBSD net80211, written in safe Rust.

## Future Enhancements
Wi-Fi 7 (802.11be) Multi-Link Operation (MLO) aggregating 5GHz and 6GHz bands simultaneously.
