# Networking

SigmaOS provides a complete networking stack for connectivity and communication.

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

## AI Agent Maintenance Instructions

- **Bolt ⚡**: Ensure documentation of any new zero-allocation optimizations or performance improvements are added concisely without marketing fluff.
- **Palette 🎨**: Maintain Arch Linux wiki style: clear, factual, one page per topic, using appropriate markdown formatting and tables where necessary.
- **Sentinel 🛡️**: Verify that no hardcoded credentials or unvetted cryptographic algorithms are documented as production-ready. Ensure security limitations are accurately stated.
- **General**: Keep pages up-to-date with current repository capabilities. Remove redundant files when consolidating information.

## Reference projects and future roadmap

Study Linux's packet path, namespaces, nftables and WireGuard; OpenBSD PF and conservative policy defaults; FreeBSD VNET/Capsicum; and NetBSD NPF/rump testing. Configuration structures and protocol names do not establish that traffic is processed or filtered by these mechanisms.

1. Trace ingress through parsing, routing, policy, sockets, and egress; identify actual enforcement hooks.
2. Separate firewall rule storage from packet-path enforcement and define rule precedence.
3. Specify network namespace ownership, teardown, and capability inheritance.
4. Fuzz protocol and policy parsers with bounded input and resource use.
5. Integrate cryptography only through an audited provider with explicit entropy and key handling.

**Completion evidence:** deny rules are shown to block traffic at runtime; invalid packets and policies fail safely; isolation is tested at each enforcement boundary; provider and tested device/emulator are documented.
