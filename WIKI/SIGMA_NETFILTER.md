# SigmaOS Netfilter

## Overview

`src/net/sigma_netfilter.rs` is the SigmaOS-native packet filtering and NAT
framework, inspired by Linux netfilter/nftables and BSD `pf` (Packet Filter).

---

## Hook Architecture

```
Incoming packet
     │
     ▼
[PREROUTING]  ── DNAT, connection tracking
     │
     ├──(local)──▶ [INPUT] ──▶ local socket
     │
     └──(forward)──▶ [FORWARD] ──▶ [POSTROUTING] ──▶ egress

Outgoing (locally-generated) packet
     │
     ▼
[OUTPUT] ──▶ [POSTROUTING] ──▶ egress
```

---

## Tables

| Table | Chains | Purpose |
|-------|--------|---------|
| `filter` | INPUT, FORWARD, OUTPUT | Accept/Drop decisions |
| `nat` | PREROUTING, OUTPUT, POSTROUTING | Address/port translation |
| `mangle` | All five | Packet marking and modification |

---

## Chains and Rules

### Chain

A chain is an ordered list of `NetfilterRule`s plus a **default policy**
(`Accept` or `Drop`) applied when no rule matches.

```rust
let mut chain = Chain::new("INPUT", PacketVerdict::Drop);
chain.add_rule(NetfilterRule::new(
    vec![MatchCriteria::DstPort(80), MatchCriteria::Proto(IpProto::Tcp)],
    PacketVerdict::Accept,
));
```

### NetfilterRule

A rule fires when **all** its `MatchCriteria` hold.  First-match wins.

```rust
NetfilterRule::new(criteria_vec, PacketVerdict::Drop)
    .with_comment("block SSH brute force");
```

---

## Match Criteria

| Criterion | Description |
|-----------|-------------|
| `SrcIp(addr)` | Exact source IPv4 address |
| `SrcIpCidr { addr, prefix }` | Source address within CIDR |
| `DstIp(addr)` | Exact destination IPv4 address |
| `DstIpCidr { addr, prefix }` | Destination address within CIDR |
| `SrcPort(p)` | Source port |
| `DstPort(p)` | Destination port |
| `DstPortRange(lo, hi)` | Destination port in range \[lo, hi\] |
| `Proto(p)` | IP protocol (TCP / UDP / ICMP / Other) |
| `InIface(name)` | Incoming interface name |
| `OutIface(name)` | Outgoing interface name |
| `Mark(m)` | Packet fwmark value |
| `Not(box crit)` | Negation of any criterion |

---

## Verdicts

| Verdict | Effect |
|---------|--------|
| `Accept` | Allow packet; continue processing |
| `Drop` | Silently discard packet |
| `Reject` | Discard + send ICMP unreachable |
| `Queue(q)` | Send to userspace queue `q` |
| `Return` | Return from current chain to caller |
| `Jump(chain)` | Evaluate named user-defined chain |
| `Log(msg)` | Log match; continue evaluation |

---

## User-Defined Chains

User chains enable rule reuse and hierarchy:

```rust
// Define a reusable chain.
let mut allowed_ports = Chain::new("allowed-ports", PacketVerdict::Return);
allowed_ports.add_rule(NetfilterRule::new(
    vec![MatchCriteria::DstPort(80)], PacketVerdict::Accept));
allowed_ports.add_rule(NetfilterRule::new(
    vec![MatchCriteria::DstPort(443)], PacketVerdict::Accept));
table.add_user_chain(allowed_ports);

// Jump to it from INPUT.
table.add_rule(ChainHook::Input,
    NetfilterRule::new(vec![], PacketVerdict::Jump("allowed-ports".into())));
```

---

## NAT

`NatTable` provides SNAT/masquerade support:

```rust
let mut nat = NatTable::default();

// Static SNAT: remap a specific source address.
nat.add_snat(NatEntry {
    orig_src: Ipv4Addr::new(192, 168, 1, 100),
    orig_sport: 54321,
    translated_src: Ipv4Addr::new(203, 0, 113, 1),
    translated_sport: 54321,
});

// Dynamic MASQUERADE: rewrite all sources to interface address.
nat.enable_masquerade(Ipv4Addr::new(203, 0, 113, 1));

let mut pkt = Packet::tcp(src, dst, sport, dport);
nat.apply_snat(&mut pkt);
```

Full connection-tracking (to allow DNAT reverse mapping) is planned for a
future milestone.

---

## Rule Hit Counters

Each `NetfilterRule` has a `hit_count: u64` field incremented on every match.
Use `table.stats()` to print a diagnostic summary:

```
Table: filter
  Chain INPUT (policy: Accept)
    rule[0]: Drop hit_count=42
    rule[1]: Accept hit_count=1337
```

---

## Comparison

### vs iptables

| Feature | iptables | SigmaOS Netfilter |
|---------|----------|-------------------|
| Rule language | CLI flags | Rust structs |
| Tables | filter, nat, mangle, raw | filter, nat (mangle stub) |
| User chains | Yes | Yes |
| Hit counters | Yes | Yes |
| Connection tracking | `conntrack` module | Planned |
| IPv6 (ip6tables) | Separate binary | Planned |

### vs nftables

| Feature | nftables | SigmaOS Netfilter |
|---------|----------|-------------------|
| Rule language | nft DSL | Rust API |
| Sets / maps | Yes (`nft set`) | Planned (`MatchSet`) |
| Expressions / math | Yes | Partial (bitmasking) |
| Stateful NAT | Yes | Stub |
| Netlink API | Yes | N/A |

### vs BSD pf

| Feature | BSD pf | SigmaOS Netfilter |
|---------|--------|-------------------|
| Config language | `pf.conf` | Rust API |
| Anchor tables | Yes | User chains |
| State tables | Yes | Planned |
| Altq (QoS) | Yes | Planned |
| Scrub/normalization | Yes | Planned |

---

## Quick-Start Example

```rust
// Build a basic stateless firewall.
let mut table = NetfilterTable::new_filter();

// Drop incoming SSH from the internet.
table.add_rule(ChainHook::Input, NetfilterRule::new(
    vec![
        MatchCriteria::DstPort(22),
        MatchCriteria::Proto(IpProto::Tcp),
        MatchCriteria::Not(Box::new(MatchCriteria::SrcIpCidr {
            addr: Ipv4Addr::new(10, 0, 0, 0),
            prefix: 8,
        })),
    ],
    PacketVerdict::Drop,
));

// Evaluate a packet.
let pkt = Packet::tcp(
    Ipv4Addr::new(1, 2, 3, 4),
    Ipv4Addr::new(10, 0, 0, 1),
    55000, 22,
);
assert_eq!(table.evaluate_packet(ChainHook::Input, &pkt), PacketVerdict::Drop);
```

---

## Source Location

`src/net/sigma_netfilter.rs`
