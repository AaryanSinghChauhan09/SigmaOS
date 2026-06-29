# Networking Stack

SigmaOS ships a custom network stack with no external dependencies. All four layers — TLS, DNS, DHCP, and WiFi — are implemented from scratch and integrated into the sigma_net unified interface.

---

## Stack Overview

```
Application (navigator.sigmaos.process.spawn / sigma_net_connect_secure)
         │
         ▼
sigma_net — unified interface (sigma_net.h)
         │
    ┌────┴──────────────────┐
    │   TLS 1.3 + Kyber     │  sigma_tls.h
    │   DNS-over-HTTPS      │  sigma_dns.h
    │   DHCP client         │  sigma_dhcp.h
    │   WPA3/SAE            │  sigma_wpa3.h
    └────┬──────────────────┘
         │
         ▼
kernel/net/ — TCP/IP state machine, conntrack, sigma-shield firewall
```

---

## TLS 1.3 with Post-Quantum Hybrid Key Exchange

Source: `net/tls/sigma_tls.h`

**Key exchange**: X25519 + Kyber-1024 hybrid (`TLS_GROUP_X25519_KYBER1024 = 0xFE01`). Both algorithms must agree on the shared secret — breaking either one is not sufficient.

**Certificate signatures**: Dilithium5 (ML-DSA-87, NIST FIPS 204). Note: Kyber-1024 is used *only* for key encapsulation, never for signing. See [Security Model](Security-Model) for the Kyber misuse bug fixed in the hypervisor.

**Key derivation**: HKDF (RFC 5869) label expansion, exactly as specified in RFC 8446 §7.1.

**AEAD**: AES-256-GCM or ChaCha20-Poly1305, negotiated via cipher suite.

```c
sigma_tls_config_t* cfg = sigma_tls_config_new();
sigma_tls_config_enable_pqc(cfg, true);    // X25519 + Kyber-1024 hybrid

sigma_tls_session_t* sess = sigma_tls_session_new(cfg);
sigma_tls_connect(sess, "api.example.com", send_cb, recv_cb, ctx);
// Handshake: ClientHello with hybrid key_share → ServerHello → Finished
sigma_tls_write(sess, data, len);
```

---

## DNS Resolver with DoH + DNSSEC

Source: `net/dns/sigma_dns.h`

Three transport modes:
- `DNS_TRANSPORT_UDP` — classic RFC 1035, port 53
- `DNS_TRANSPORT_HTTPS` — DNS-over-HTTPS (RFC 8484), HTTPS POST with `application/dns-message`
- `DNS_TRANSPORT_TLS` — DNS-over-TLS (DoT), port 853

DNSSEC validation uses the root KSK 2017 as trust anchor (`SIGMA_DNS_ROOT_KSK_2017`). Responses with `AD=1` (Authenticated Data) and valid RRSIG chains are marked `validated=true`. Broken chains set `bogus=true` and the resolver returns `SERVFAIL`.

```c
sigma_dns_resolver_t* r = sigma_dns_resolver_new();
sigma_dns_config_set_doh(&r->config, "https://dns.sigma.os/dns-query", true);

sigma_dns_response_t* resp;
sigma_dns_resolve(r, "api.example.com", DNS_TYPE_A, &resp);
// resp->answers[0].data.a.addr = IPv4 address
// resp->answers[0].validated   = true (DNSSEC OK)

uint8_t ip[4]; uint32_t ttl;
sigma_dns_get_address_a(resp, ip, &ttl);
sigma_dns_response_free(resp);
```

Cache: up to `DNS_CACHE_MAX_ENTRIES` (1024) entries, respects TTL, configurable min/max bounds.

**DNS Sinkhole** (source: `kernel/net/sigma_dns_sinkhole.h`): Blocks malware domains by returning NXDOMAIN or a sinkhole IP. Block lists are signed `.sinkhole` packages installed via sigma-pkg. Subdomain matching is included: blocking `malware.com` also blocks `evil.malware.com`.

---

## DHCP Client

Source: `net/dhcp/sigma_dhcp.h`

Full RFC 2131 / RFC 2132 implementation:

1. `SIGMA_ACQ_PENDING` → `sigma_dhcp_discover()` → broadcast DISCOVER
2. Receive OFFER → `sigma_dhcp_request(ip, server_id)` → send REQUEST
3. Receive ACK → `DHCP_LEASE_STATE_BOUND` — IP assigned
4. T1 timer fires → `SIGMA_LEASE_STATE_RENEWING` → unicast REQUEST to original server
5. T2 timer fires → `SIGMA_LEASE_STATE_REBINDING` → broadcast REQUEST
6. Lease expires → `SIGMA_LEASE_STATE_EXPIRED` → restart discovery

Lease information (IP, subnet, router, DNS, TFTP, hostname, domain) is stored in `sigma_dhcp_lease_t` and published to sigma-ds at `sigma.net.lease.<interface>`.

---

## WPA3/SAE Authentication

Source: `net/wifi/sigma_wpa3.h`

Implements Simultaneous Authentication of Equals (SAE, RFC 7664):
- Hunting-and-pecking algorithm to derive a password element from the shared passphrase + peer MACs
- Commit → Confirm exchange using P-256 EC operations
- Session keys derived via HKDF from the SAE PMK

Also supports OWE (Opportunistic Wireless Encryption) for open networks that still want encryption without a passphrase.

---

## Unified Network Stack API

Source: `net/sigma_net.h`

```c
// Default init — DHCP + DoH + WPA3
sigma_net_stack_t* net = sigma_net_init();

// Wait for network to come up (DHCP bound + DNS reachable)
sigma_net_wait_available(net, 30000 /*30s timeout*/);

// Resolve + establish TLS connection in one call
sigma_tls_session_t* sess;
sigma_net_connect_secure(net, "pkg.sigma.os", 443, &sess);

// Use the connection
sigma_tls_write(sess, request, req_len);
```

Three built-in configuration presets:

| Preset | Use case |
|---|---|
| `sigma_net_config_default()` | Sane defaults — DHCP, DoH, no DNSSEC mandatory |
| `sigma_net_config_secure()` | Maximum security — DNSSEC required, PQC enabled, WPA3 only |
| `sigma_net_config_fast()` | IoT / constrained — caching, no DNSSEC validation |

---

## Firewall (sigma-shield)

The `sigma_shield` packet filter evaluates rules against **actual packet 5-tuples** — not mocked data (Bug #10 fix). Rules are evaluated in O(1) via a hash table keyed on `(src_ip, dst_ip, src_port, dst_port, proto)`.

**Conntrack**: The counter is decremented when connections close (Bug #19 fix). A configurable maximum (`sigma-sysctl net.firewall.conntrack_max`) rejects new entries when full.

**eBPF integration**: `kernel/security/sigma_ebpf.h` — custom BPF programs can attach to `SIGMA_EBPF_HOOK_PACKET_RX` and `SIGMA_EBPF_HOOK_PACKET_TX` for programmable packet processing without recompiling the kernel.

---

## Network Namespace Isolation

Every process spawned via `sigmad-process` gets its own network namespace (`CLONE_NEWNET`). By default only loopback is available. Adding `net:host` capability to the app manifest adds a veth pair to the host network.

For maximum isolation, `sigma_netgw_config_t` (`kernel/virt/sigma_netgw.h`) allows running a Whonix-style network gateway VM that forces all traffic through Tor and prevents any direct clearnet connections from the workload VM.

---

*See also: [Security Model](Security-Model) · [Architecture Overview](Architecture-Overview) · [Building from Source](Building-from-Source)*
