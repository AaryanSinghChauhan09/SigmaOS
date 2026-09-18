# Networking Support Matrix

SigmaOS provides comprehensive networking support with advanced features including IPv6, bonding, VLANs, VPNs, and network namespaces for enterprise-grade networking.

## Overview

Networking support includes:
- IPv4 and IPv6 support with dual-stack configuration
- Network bonding and teaming for high availability
- VLAN tagging and trunking
- Network namespaces for container isolation
- VPN support (WireGuard, OpenVPN, IPsec)
- Network bridge management
- Network filtering with nftables
- Network monitoring and diagnostics
- Network performance tuning

## Supported Network Features

### IPv4 and IPv6

#### IPv4
- **Status**: Fully supported
- **Features**: DHCP, static addressing, routing, NAT
- **Use cases**: General networking, legacy compatibility
- **Configuration**: `/etc/network/interfaces`, `ip addr`, `ip route`

#### IPv6
- **Status**: Fully supported
- **Features**: SLAAC, DHCPv6, static addressing, routing
- **Use cases**: Modern networking, future-proofing
- **Configuration**: `/etc/network/interfaces`, `ip -6 addr`, `ip -6 route`

### Network Bonding

#### Bonding Modes
- **Mode 0 (balance-rr)**: Round-robin load balancing
- **Mode 1 (active-backup)**: Active-backup failover
- **Mode 2 (balance-xor)**: XOR load balancing
- **Mode 3 (broadcast)**: Broadcast load balancing
- **Mode 4 (802.3ad)**: LACP dynamic bonding
- **Mode 5 (balance-tlb)**: Adaptive transmit load balancing
- **Mode 6 (balance-alb)**: Adaptive load balancing

### VLANs

#### VLAN Tagging
- **Status**: Fully supported
- **Features**: 802.1Q tagging, trunk ports, access ports
- **Use cases**: Network segmentation, security
- **Configuration**: `ip link add link eth0 type vlan id 10`

### Network Namespaces

#### Namespaces
- **Status**: Fully supported
- **Features**: Process isolation, routing tables, firewall rules
- **Use cases**: Container networking, multi-tenancy
- **Configuration**: `ip netns`, `ip netns exec`

### VPN Support

#### WireGuard
- **Status**: Fully supported
- **Features**: Modern VPN, UDP-based, minimal code
- **Use cases**: Site-to-site VPN, road warrior
- **Configuration**: `/etc/wireguard/wg0.conf`, `wg-quick`

#### OpenVPN
- **Status**: Fully supported
- **Features**: TCP/UDP, TLS encryption, client/server
- **Use cases**: Remote access, site-to-site VPN
- **Configuration**: `/etc/openvpn/client.conf`, `/etc/openvpn/server.conf`

#### IPsec
- **Status**: Fully supported
- **Features**: IKEv2, ESP, AH, X.509 certificates
- **Use cases**: Site-to-site VPN, road warrior
- **Configuration**: `/etc/ipsec.conf`, `/etc/ipsec.secrets`

### Network Bridging

#### Bridge
- **Status**: Fully supported
- **Features**: Layer 2 bridging, STP, filtering
- **Use cases**: Container networking, virtualization
- **Configuration**: `brctl`, `ip link add br0 type bridge`

### Network Filtering

#### nftables
- **Status**: Fully supported
- **Features**: Packet filtering, NAT, connection tracking
- **Use cases**: Firewall, NAT, traffic shaping
- **Configuration**: `nft`, `/etc/nftables.conf`

### Network Monitoring

#### Tools
- **iproute2**: Network configuration and monitoring
- **ss**: Socket statistics
- **tcpdump**: Packet capture
- **wireshark**: Packet analysis
- **netstat**: Network statistics

## Configuration

### Network Configuration
```toml
# /etc/sigmaos/networking.toml
[ipv4]
# IPv4 settings
enabled = true
dhcp = true
static_ip = "192.168.1.100"
netmask = "255.255.255.0"
gateway = "192.168.1.1"

[ipv6]
# IPv6 settings
enabled = true
autoconf = true
static_ip = "2001:db8::1"
prefix = "64"

[bonding]
# Bonding settings
enabled = false
mode = 4
interfaces = ["eth0", "eth1"]

[vlan]
# VLAN settings
enabled = false
interface = "eth0"
vlan_id = 10

[wireguard]
# WireGuard settings
enabled = false
config = "/etc/wireguard/wg0.conf"
```

### Runtime Control
```bash
# Show network interfaces
signet show-interfaces

# Configure IP address
signet set-ip eth0 192.168.1.100/24

# Configure route
signet add-route default 192.168.1.1

# Enable bonding
signet enable-bonding bond0 eth0 eth1

# Add VLAN
signet add-vlan eth0 10

# Create bridge
signet create-bridge br0

# Add interface to bridge
signet add-to-bridge br0 eth0

# Enable WireGuard
signet enable-wireguard wg0

# Show routes
signet show-routes

# Show sockets
signet show-sockets
```

## Performance Optimization

### Network Tuning
Optimize network performance:
```bash
# Increase TCP buffer sizes
signet set-tcp-buffer-size 8388608

# Enable TCP fast open
signet enable-tcp-fast-open

# Enable TCP window scaling
signet enable-tcp-window-scaling

# Enable TCP selective ACKs
signet enable-tcp-sack

# Increase receive buffer
signet set-rmem-max 12582912

# Increase send buffer
signet set-wmem-max 12582912
```

### Bonding Optimization
Optimize bonding for performance:
```bash
# Set LACP rate
signet set-lacp-rate fast

# Set transmit hash policy
signet set-xmit-hash-policy layer3+4

# Enable MII monitoring
signet enable-mii-monitoring
```

### VLAN Optimization
Optimize VLAN for performance:
```bash
# Increase VLAN filtering
signet set-vlan-filtering on

# Enable VLAN hardware offload
signet enable-vlan-offload

# Set MTU
signet set-mtu eth0 9000
```

## Troubleshooting

### No Network Connectivity
If no network connectivity:
1. Check interface status: `signet show-interfaces`
2. Check cable connection
3. Check IP configuration: `signet show-ip`
4. Check routing: `signet show-routes`
5. Check DNS: `signet test-dns`

### Slow Network Performance
If network performance is slow:
1. Check for errors: `signet show-errors`
2. Check duplex mismatch
3. Check MTU size
4. Enable offloading
5. Check for congestion

### VPN Connection Fails
If VPN connection fails:
1. Check VPN configuration
2. Check firewall rules
3. Check for NAT issues
4. Check authentication
5. Check for routing issues

### Bridge Not Working
If bridge not working:
1. Check bridge status: `signet show-bridges`
2. Check interface membership
3. Check STP status
4. Check for firewall rules
5. Check for VLAN issues

---

**[Networking](Category-Networking)** | **[Filesystems](Category-Filesystems)** | **[Network Configuration](Category-Network-Config)**
