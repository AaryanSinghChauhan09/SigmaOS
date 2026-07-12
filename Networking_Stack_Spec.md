# SigmaOS Networking Stack Specification

## Layer Model

```
Application (sigma-curl, sigma-ssh, sigma-pkg)
  │  POSIX-like socket API (sigma_connect, sigma_bind, sigma_send, sigma_recv)
  ▼
sigma-net (userland networking shard)
  │  typed sigma-bus messages: NetMsg::Connect, NetMsg::Send, NetMsg::Recv
  ▼
smoltcp TCP/IP stack (kernel/net/)
  │  Device trait → SigmaSmoltcpDevice adapter
  ▼
HAL NIC driver (kernel/drivers/net/)
  │  DMA ring buffer (TX/RX descriptor rings)
  ▼
Physical / virtio NIC
```

---

## Zero-Copy Path: DMA Ring → smoltcp → App Buffer

The zero-copy path eliminates intermediate copies for large data transfers:

1. **NIC DMA** fills an RX descriptor ring slot directly into a kernel-mapped physical page.

2. **SigmaSmoltcpDevice::receive()** returns the physical page pointer — no copy.

3. **smoltcp** processes IP/TCP headers in-place.

4. **sigma-net** maps the page into the receiving app's address space using `sigma_map_shared`.

5. App reads data directly from the shared page.

6. App signals completion; kernel releases the page back to the DMA ring.

Memory copies: **zero** between NIC DMA and application read.

---

## TLS 1.3 + Kyber-1024 Hybrid: Handshake

```
Client                                 Server
  │                                       │
  │── ClientHello ──────────────────────► │
  │   supported_groups: [X25519Kyber1024] │
  │                                       │
  │◄─ ServerHello ────────────────────── │
  │   key_share: X25519 part              │
  │◄─ ServerHello Kyber part ──────────── │
  │   key_share: Kyber1024 ciphertext     │
  │                                       │
  │   Client decapsulates Kyber ct        │
  │   Both sides: hybrid_ss =             │
  │   HKDF(x25519_ss || kyber_ss)         │
  │                                       │
  │◄─ EncryptedExtensions ─────────────── │
  │◄─ Certificate (Dilithium-5) ─────────│
  │◄─ CertificateVerify ─────────────── │
  │◄─ Finished ────────────────────────── │
  │── Finished ───────────────────────► │
  │  [Handshake complete — AES-256-GCM] │
```

---

## DNS / DoH Pipeline

```
App: resolve("registry.sigmaos.dev")
  │
  ▼
sigma-dns-resolver (userland shard)
  │  check /etc/sigma/hosts.toml cache (in-memory, TTL-based)
  │  if miss:
  │    send DoH query: HTTPS POST https://dns.sigmaos.dev/dns-query
  │    using sigma-curl + rustls TLS 1.3
  │    parse DNS-over-HTTPS JSON response (RFC 8427)
  ▼
sigma-net returns IpAddr to app
```

DNSSEC validation is performed by sigma-dns-resolver for all non-DoH fallback queries.

---

## Firewall Rule Engine: BPF-Equivalent Filter Language

SigmaOS uses an **aya eBPF XDP program** as its firewall engine. Rules are expressed in a simple TOML DSL and compiled to XDP programs via the sigma-firewall tool.

### Rule DSL Example (`/etc/sigma/firewall.toml`)

```toml
[[rules]]
name    = "block-malicious"
action  = "drop"
src_ip  = "203.0.113.0/24"
proto   = "any"
priority = 100

[[rules]]
name    = "allow-ssh"
action  = "pass"
dst_port = 22
proto   = "tcp"
priority = 200

[[rules]]
name    = "default-drop"
action  = "drop"
src_ip  = "0.0.0.0/0"
priority = 65535
```

### Compilation

```bash
sigma-firewall compile /etc/sigma/firewall.toml --output /etc/sigma/fw.bpf
sigma-firewall load /etc/sigma/fw.bpf --interface eth0
```

The compiled XDP program is loaded via aya and attached to the NIC's XDP hook, processing packets before they reach smoltcp.

---

## Benchmark Targets

| Metric | Target |
|---|---|
| TCP throughput (loopback) | > 10 Gbps |
| TCP latency (loopback, p99) | < 50 µs |
| TLS 1.3 handshake time | < 5 ms |
| DNS resolution (DoH, cached) | < 1 ms |
| XDP firewall throughput | > 10 Mpps |
