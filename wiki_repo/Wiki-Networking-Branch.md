# Networking Branch Wiki (TCP/IP & Sovereign Comms)

This knowledge base governs the development of the SigmaOS networking stack within the `networking` branch.

## Focus Areas

* **TCP/IP Stack Design**: Sovereign, dependency-free implementation of IPv4/IPv6, TCP, UDP, and ICMP protocols.
* **Sovereign Firewall (S-FIREWALL)**: Ring-0 rule enforcement and packet filtering.
* **Shard-Aware Networking**: Secure IPC networking via loopback and external NIC abstractions.
* **Implementation Plan**: Start with basic packet parsing, move to loopback, and integrate specific NIC drivers.
