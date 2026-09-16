# SigmaOS Networking

## Network Stack Architecture

```
Application (BSD sockets API)
         ↓
  Socket Layer (src/network/socket.rs)
         ↓
  Protocol Layer
  ┌──────┬──────┬──────┐
  │ TCP  │ UDP  │ ICMP │
  └──────┴──────┴──────┘
         ↓
  IP Layer (IPv4 + IPv6)
         ↓
  Ethernet / L2
         ↓
  NIC Driver (e1000e, RTL8139, VirtIO)
```

## Supported Protocols

| Layer | Protocol | Status |
|-------|----------|--------|
| L2 | Ethernet II | ✅ |
| L2 | 802.1Q VLAN | ✅ |
| L3 | IPv4 | ✅ |
| L3 | IPv6 | ✅ |
| L3 | ARP | ✅ |
| L3 | ICMPv4/v6 | ✅ |
| L4 | TCP | ✅ |
| L4 | UDP | ✅ |
| L7 | DNS | ✅ |
| L7 | DHCP | ✅ |
| L7 | mDNS/DNS-SD | ✅ |
| L7 | TLS 1.3 | ✅ |
| L7 | HTTP/1.1 | ✅ |
| L7 | HTTP/2 | ⬜ |
| L7 | QUIC/HTTP3 | ⬜ |

## Network Configuration

```toml
# /etc/sigma/network.toml
[[interface]]
name = "eth0"
dhcp4 = true
dhcp6 = true

[[interface]]
name = "eth1"
address4 = "192.168.1.10/24"
gateway4 = "192.168.1.1"
dns = ["8.8.8.8", "1.1.1.1"]
mtu = 9000  # Jumbo frames

[[interface]]
name = "wlan0"
dhcp4 = true
wifi_ssid = "MyNetwork"
wifi_security = "wpa3-personal"
```

## Socket API

```rust
use sigma::network::socket::{Socket, SocketFamily, SocketType};

// TCP client
let sock = Socket::new(SocketFamily::Inet, SocketType::Stream)?;
sock.connect("93.184.216.34:80")?;
sock.write(b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")?;
let mut buf = [0u8; 4096];
let n = sock.read(&mut buf)?;

// UDP socket
let sock = Socket::new(SocketFamily::Inet, SocketType::Dgram)?;
sock.bind("0.0.0.0:53")?;
sock.sendto(b"query", "8.8.8.8:53")?;

// Unix domain socket
let sock = Socket::new(SocketFamily::Unix, SocketType::Stream)?;
sock.connect("/var/run/sigma.sock")?;
```

## TLS/mTLS

```rust
use sigma::crypto::tls::{TlsConfig, TlsStream};

let config = TlsConfig::new()
    .with_cert("/etc/sigma/tls/server.crt")
    .with_key("/etc/sigma/tls/server.key")
    .with_ca("/etc/sigma/tls/ca.crt")
    .require_client_cert(true);  // mTLS

let tls_stream = TlsStream::connect("example.com:443", config)?;
```

## Network Namespace Support

```bash
# Create network namespace (container isolation)
sigma-net ns create myns

# Run command in namespace
sigma-net ns exec myns -- my-service

# Set up veth pair
sigma-net veth create veth0 veth1
sigma-net veth move veth1 myns
```

## Firewall (eBPF-based)

```rust
// Load eBPF firewall program
let firewall = EbpfFirewall::load("/etc/sigma/firewall.bpf")?;
firewall.attach("eth0", AttachPoint::Ingress)?;

// Add rule: drop incoming TCP to port 23 (telnet)
firewall.add_rule(Rule::drop().tcp().dst_port(23))?;
```
