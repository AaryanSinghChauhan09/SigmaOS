# Netfilter / nftables Agent

| Field | Value |
|---|---|
| **Portal** | Networking |
| **Maintainer Agent** | NETFILTER_NFTABLES_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_nftables_engine.rs` |
| **Language** | Rust (primary), Nim (controller) |

---

## Table of Contents
1. [Overview](#overview)
2. [Technical Specifications](#technical-specifications)
3. [Implementation](#implementation)
4. [Standard Library Authorization](#standard-library-authorization)
5. [Configuration](#configuration)
6. [Diagnostics](#diagnostics)
7. [See Also](#see-also)

---

## 1. Overview
Implements the nftables packet filtering framework as the modern successor to iptables. Manages hook registration in the Netfilter framework (prerouting, input, forward, output, postrouting), ruleset compilation, NAT masquerade, stateful conntrack, and set/map data structures.

## 2. Technical Specifications
- Hooks: NF_INET_PRE_ROUTING, NF_INET_LOCAL_IN, NF_INET_FORWARD, NF_INET_LOCAL_OUT, NF_INET_POST_ROUTING
- Netlink family: NFNL_SUBSYS_NFTABLES
- Conntrack: nf_conntrack, ct state machine
- Sets: hash, rbtree, bitmap, interval
- NAT: SNAT, DNAT, masquerade, redirect

## 3. Implementation
Engine: [`sigma_nftables_engine.rs`](../src/kernel/sigma_nftables_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized.

## 5. Configuration
```toml
[nftables]
default_policy = "drop"
enable_conntrack = true
masquerade_interface = "wlan0"
ruleset_path = "/etc/nftables.conf"
```

## 6. Diagnostics
```bash
nft list ruleset
nft monitor
conntrack -L
sigma-diag firewall --nft --stats
```

## 7. See Also
- [DNSMASQ_DHCP_AGENT](./DNSMASQ_DHCP_AGENT.md) 
- [WIREGUARD_VPN_AGENT](./WIREGUARD_VPN_AGENT.md)
