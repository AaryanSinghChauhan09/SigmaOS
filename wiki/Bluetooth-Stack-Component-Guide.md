# Bluetooth Subsystem & Wireless Audio Component Guide

## Component Overview
Governs USB/UART HCI controller initialization, L2CAP protocol multiplexing, RFCOMM serial profiles, BLE (Bluetooth Low Energy) GATT client/server, and A2DP high-definition audio routing.

## Linux Inspiration
Linux BlueZ Bluetooth stack, btmon HCI sniffer, and PipeWire SPA Bluetooth audio plugin.

## BSD Inspiration
FreeBSD netgraph Bluetooth subsystem (ng_btsocket, ng_l2cap, ng_hci), OpenBSD bthcid daemon.

## Current SigmaOS Status
HCI command packet serialization and basic USB Bluetooth dongle probe functional. L2CAP state machine and A2DP audio sink streaming require completion.

## Critical Missing Features
L2CAP channel negotiation, BLE GATT advertising and service discovery, LDAC and aptX audio codec integration, and automated pairing agent.

## Implementation Priority
MEDIUM (Phase 2)

## Key Files to Create/Improve
src/wireless/bluetooth.rs, src/drivers/bluetooth_usb.rs, src/audio/graph.rs

## Testing Strategy
HCI packet round-trip latency tests in QEMU virtual USB dongle, synthetic A2DP audio frame transmission.

## Dependencies
HARDWARE_HAL_DRIVERS_AGENT, AUDIO_SUBSYSTEM_AGENT, SECURITY_AGENT

## Success Criteria
Sub-20ms wireless audio latency, zero unhandled HCI disconnect panics, seamless peripheral reconnects upon wake.

## Open Source Competitors Analysis
Replaces the notoriously complex and crash-prone BlueZ daemon with a memory-safe, typed Rust stack.

## Future Enhancements
Spatial audio head-tracking orientation processing integrated directly into Bluetooth audio streams.
