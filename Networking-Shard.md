# 🌐 Networking Shard

> "Sovereign connectivity, from the physical layer up to the transport layer, without importing a single line of libc sockets."

## 1. Network Interface Cards (NIC)
SigmaOS currently supports the Intel Gigabit Ethernet (`e1000`) and Realtek `RTL8139`. These drivers interface directly with the physical hardware, reading MAC addresses from the EEPROM and setting up TX/RX ring buffers.

## 2. Address Resolution Protocol (ARP)
Before IPv4 can route to a local destination, the MAC address must be resolved. SigmaOS maintains a lightweight ARP cache. If an IP is unknown, an ARP broadcast is issued.

## 3. IPv4 Implementation
All incoming packets from the NICs are routed through the IPv4 demultiplexer. 
- The IPv4 header is validated using `sovereign_checksum`.
- Based on the `protocol` field, the payload is forwarded:
  - Protocol 1: ICMP (Ping)
  - Protocol 6: TCP
  - Protocol 17: UDP

## 4. Transport Layer (TCP/UDP)
- **UDP:** A connectionless, stateless implementation for fast packet broadcast/multicast.
- **TCP:** A robust state machine (WIP). Currently handles header parsing (SYN, ACK, FIN flags) preparing for a full Sovereign Pseudo-Socket layer in userland.
