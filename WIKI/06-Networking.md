# Networking

SigmaOS contains networking models and prototype components. The commands and configuration examples below describe intended interfaces; use them only where the corresponding executable, service, or backend is implemented and enabled.

## Implementation Status: Networking Models

The `src/network/zero_copy_networking.rs` and `src/network/tc_qdisc_sovereign.rs` components are in-process models. UMEM chunks, packet/completion queues, and traffic-control queues are ordinary Rust data structures; they do not map NIC DMA memory, open AF_XDP sockets, call `io_uring`, attach qdiscs to a host interface, or transmit packets through an operating-system network backend. Do not treat them as production datapaths, lock-free queues, or security boundaries.

Before describing these components as operational backends, implement and review the hardware/OS integration, explicit buffer ownership and completion lifecycle, queue synchronization, and resource limits. Keep their model status clear in code and docs until those pieces exist. AI agents maintaining them must preserve descriptor bounds, FIFO ordering, queue capacity invariants, token-refill precision, and overflow-safe accounting, and run the focused networking checks when changes are made.

## Network Configuration

### Network Interfaces

View network interfaces:

```bash
# List interfaces
sigif list

# Show interface details
sigif show eth0

# Configure interface
sigif configure eth0
```

### Wired Networking

Configure wired connection:

```toml
# /etc/sigmaos/network.toml
[interface.eth0]
type = "wired"
method = "dhcp"
```

Or use command line:

```bash
# Enable DHCP
sigif eth0 dhcp on

# Set static IP
sigif eth0 address 192.168.1.100/24
sigif eth0 gateway 192.168.1.1
sigif eth0 dns 8.8.8.8
```

### Wireless Networking

Configure wireless connection:

```bash
# Scan for networks
sigif wlan0 scan

# Connect to network
sigif wlan0 connect network-name password

# View connection status
sigif wlan0 status
```

## Network Services

### SSH Server

Enable and configure SSH:

```bash
# Enable SSH server
sigma-systemctl enable sshd

# Start SSH server
sigma-systemctl start sshd

# Configure SSH
edit /etc/ssh/sshd_config
```

### Firewall

Configure firewall rules:

```bash
# Enable firewall
sigma-firewall enable

# Allow SSH
sigma-firewall allow 22/tcp

# Allow HTTP
sigma-firewall allow 80/tcp

# Deny port
sigma-firewall deny 53/udp
```

## Network Diagnostics

### Connectivity Testing

Test network connectivity:

```bash
# Ping host
ping example.com

# Trace route
traceroute example.com

# DNS lookup
nslookup example.com
```

### Network Monitoring

Monitor network traffic:

```bash
# Show network statistics
sigif stats

# Show connections
sigif connections

# Show bandwidth usage
sigif bandwidth
```

## Advanced Networking

### WireGuard VPN

Configure WireGuard VPN:

```bash
# Generate keys
wg genkey > privatekey
wg pubkey < privatekey > publickey

# Create interface
wg genkey | tee privatekey | wg pubkey > publickey
```

### Network Bonding

Configure network bonding:

```bash
# Create bond interface
sigif bond0 mode balance-rr eth0 eth1

# Show bond status
sigif bond0 status
```

## Next Steps

- [Security](07-Security.md) - Security and network hardening
- [Desktop](08-Desktop.md) - Desktop and GUI configuration
- [Kernel](04-Kernel.md) - Kernel network stack
