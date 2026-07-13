# Essential Shards Specification

This specification defines the security, networking, and basic hardware drivers representing the essential operating environment of the SigmaOS Sovereign Lattice.

---

## 🛡️ Security & Verification Shards

These modules enforce cryptographic trust chains, integrity validation, and access policy control.

| Shard Path | System Component | Functional Purpose |
| :--- | :--- | :--- |
| `kernel/core/security/SovereignPQC.cpp` | Cryptographic Engine | Validates Dilithium5 package signatures and handles Kyber exchanges |
| `kernel/core/security/SovereignAudit.cpp` | Audit Trail | Writes immutable, signed events to the physical system log |
| `kernel/core/security/SovereignIMA.cpp` | Integrity Measurement| Measures files before execution, enforcing state immutability |

---

## 🌐 Network Protocol Shards

The network layer provides robust stateful firewall protections and full IPv6 socket abstractions.

| Shard Path | Component Class | Core Features |
| :--- | :--- | :--- |
| `kernel/core/network/SovereignTCPIP.cpp` | TCP/IP Core | Zero-copy packet parsing, socket buffers, window scaling |
| `kernel/core/network/SovereignIPv6.cpp` | IPv6 Node | Stateless Address Autoconfiguration (SLAAC), NDP, path MTU |
| `kernel/core/network/SovereignFirewall.cpp`| sigma-shield Hook | Hooks into ingress/egress queues to apply connection rules |

---

## 🔌 Layer 2: Core Device Driver Shards

Essential block storage and standard input/output device drivers.

| Shard Path | Device Class | Interface Abstraction |
| :--- | :--- | :--- |
| `kernel/core/drivers/SovereignPS2.cpp` | Keyboard/Mouse | PS/2 controller status loops, scancode mapping |
| `kernel/core/drivers/SovereignATA.cpp` | IDE Storage | Disk sector I/O, PIO/DMA transfers, master/slave selection |
| `kernel/core/drivers/SovereignSATA.cpp` | AHCI Controller | Command lists, FIS structures, native command queuing (NCQ) |
| `kernel/core/drivers/SovereignE1000.cpp` | Ethernet Driver | Intel 8254x NIC driver, DMA rings, descriptor structures |
