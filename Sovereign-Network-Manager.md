# Sovereign Network Manager (`sigma-net`)

The **Sovereign Network Manager** (inspired by Fedora's NetworkManager combined with NixOS's declarative configuration model) is SigmaOS's native tool for secure, reproducible network provisioning and state management.

It forms the critical pipeline required to establish secure remote ledger connections, allowing the system to verify signatures and retrieve signed packages or drivers.

## 🛠️ Key Architectural Subsystems

### 1. TCP/IP Interface Manager
Extends core SigmaOS networking to register and bring up physical/virtual interfaces (`eth0`), configure stable MAC address generation, and handle mock DHCP address leases.
- **Path:** `net/sovereign_tcp_ip.cpp`

### 2. Post-Quantum Secure DNS Resolver
Mandates DNS-over-HTTPS (DoH) to eliminate cleartext sniffing. Employs cryptographic response verification to prevent domain hijacking or DNS spoofing.
- **Path:** `net/sovereign_dns.cpp`

### 3. Declarative Profile Manager & Rollback Engine
Parses and applies declarative Profiles (such as `HOME`, `ENTERPRISE`, and `CLOUD`). Before committing a change, it creates an atomic network snapshot. In the event of a connectivity loss, it can trigger an instant rollback to restore known-good configurations.
- **Path:** `tools/net_manager/profile_manager.cpp`

### 4. Default-Deny MAC Firewall
A strict mandatory access control (MAC) integrated packet filter. All traffic is blocked by default unless explicitly permitted by high-clearance capability rules.
- **Path:** `net/sovereign_firewall.cpp`

---

## 💻 Command Line Tool Reference (`sigma-net`)

Operators manage network states using the high-performance `sigma-net` CLI:

```bash
# Display active interfaces, allocated IPs, and DNS posture
sigma-net status

# Apply a declarative network profile (e.g., home or enterprise)
sigma-net connect <profile_name>

# Resolve a hostname securely using the post-quantum DoH stack
sigma-net dns <hostname>

# Force an emergency rollback to the last known-good state
sigma-net rollback
```

---

## 🧪 Comprehensive Verification

The stack is covered by comprehensive Vitest units validating edge cases:
- Dynamic DHCP lease acquisition transitions.
- Rejection of forged or unsigned DNS spoofing responses.
- Lattice-level firewall block of unauthorized ports.
- Perfect profile snapshot state consistency.

Run the test suite using:
```bash
npx vitest run tests/sovereign_network.test.js
```
All 9 core network test suites are verified with **100% success**.
