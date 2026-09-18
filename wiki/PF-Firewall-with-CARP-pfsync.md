# PF Firewall with CARP/pfsync

SigmaOS implements PF (Packet Filter) firewall with CARP (Common Address Redundancy Protocol) and pfsync for high-performance stateful firewalling with high availability.

## Overview

PF provides:
- Stateful packet filtering
- Network Address Translation (NAT)
- Traffic shaping with ALTQ
- CARP for high availability
- pfsync for state synchronization
- Tables for address lists
- Packet tagging and policy routing
- Syncookies and SYN proxy

## Architecture

### PF Processing Pipeline
```
Packet → Table Lookup → Rule Evaluation → State Check → Action
                                        ↓
                              Allow / Deny / NAT
```

### CARP Protocol
- **Virtual IP**: Shared IP address across multiple firewalls
- **VHID**: Virtual Host ID (1-255)
- **AdvSkew**: Advertisement skew (priority)
- **Master**: Active firewall handling traffic
- **Backup**: Standby firewall

### pfsync State Synchronization
- **State Table**: Connection state information
- **Sync Protocol**: State synchronization protocol
- **Multicast**: State updates sent via multicast
- **Real-time**: Near-instant state transfer

## Configuration

### PF Configuration
```toml
# /etc/sigmaos/pf.toml
[firewall]
enabled = true
default_policy = "block"

[tables]
# Address tables
blocked_ips = ["192.168.1.100", "10.0.0.50"]
trusted_networks = ["192.168.1.0/24"]

[carp]
# CARP configuration
vhid = 1
virtual_ip = "192.168.1.1"
password = "secret"

[pfsync]
# pfsync configuration
enabled = true
interface = "eth1"
multicast_group = "224.0.0.1"
sync_interval_ms = 200
```

### Runtime Control
```bash
# Enable PF
sigpf enable

# Add rule
sigpf add-rule "pass in on eth0 from 192.168.1.0/24 to any"

# Add table
sigpf add-table blocked_ips 192.168.1.100

# View rules
sigpf list-rules

# View states
sigpf list-states

# Flush states
sigpf flush-states

# Enable CARP
sigcarp enable --vhid 1 --virtual-ip 192.168.1.1

# View CARP status
sigcarp status

# Enable pfsync
sigpfsync enable --interface eth1

# View pfsync statistics
sigpfsync stats
```

---

**[Networking](Category-Networking)** | **[XDP Networking](XDP-Zero-Copy-Networking)** | **[High Availability](High-Availability)**
