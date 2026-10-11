# Networking & Sovereign TCP/IP Stack Agent

> **SigmaOS Agent Directive** — Only Rust, Zig, or Nim. Priority: Stability > Security > Performance.

## 1. Mission
Deliver a high-throughput, zero-copy network stack, hardware eBPF packet filters, WireGuard mesh VPN, and complete IPv4/IPv6 protocol suites.

## 2. Implementation Language Policy
- **Primary:** Rust (memory safety, zero-cost abstractions, `no_std` kernel execution)
- **Systems-level:** Zig (comptime meta-programming, explicit memory allocators, direct hardware interop)
- **Scripting/tooling:** Nim (deterministic ARC/ORC memory management, GC-free systems profile)
- **Forbidden:** C, C++, Python, Go, Java, JavaScript for core OS components
- **Unsafe Rust:** Restricted strictly to hardware abstractions with explicit `// SAFETY:` proofs

## 3. Architecture Inspiration
### Linux Kernel
eBPF/XDP programmable packet path, TCP BBRv3 congestion control, WireGuard in-kernel module.

### FreeBSD
netgraph modular packet processing, pf packet filter, VNET network virtualization.

### OpenBSD
pf (Packet Filter) stateful firewall, CARP redundancy, carp(4) routing.

### NetBSD
NPF modern packet filter, rump kernels for network testing.

### Arch Linux & Distros
OpenWrt networking agility, CachyOS BBR3 network tuning.

### NixOS
Declarative network interfaces, WireGuard peer meshes, and firewall rules.

## 4. SigmaOS Source Files
- `src/net/mod.rs`
- `src/net/tcp.rs`
- `src/net/udp.rs`
- `src/net/wireguard.rs`
- `src/security/pf_firewall.rs`

## 5. Current Implementation Status
| Feature | Status | Subsystem / Notes |
|---------|--------|-------------------|
| Sovereign TCP/IP Stack | ✅ Implemented | TCP, UDP, IPv4, IPv6, ICMP |
| WireGuard PQC Mesh VPN | ✅ Implemented | Curve25519 / Kyber hybrid tunnel |
| OpenBSD PF Firewall Engine | ✅ Implemented | Stateful filtering, NAT, normalization |
| eBPF/XDP High-Speed Packet Filter | 🚧 Partial | JIT bytecode compilation |

## 6. Critical Missing Features (Implement in Order)
1. **[PRIORITY: HIGH]** TCP BBRv3 / Cubic modern congestion control algorithms.
2. **[PRIORITY: HIGH]** Zero-copy packet RX/TX via eBPF/XDP ring buffers.
3. **[PRIORITY: MEDIUM]** TLS 1.3 in-kernel record layer offload (kTLS).

## 7. Implementation Guidelines for AI Agents
### Adding New Features
1. Verify feature does not compromise subsystem stability or introduce regressions.
2. Adhere strictly to safe Rust paradigms; isolate low-level operations to dedicated primitives.
3. Validate memory overhead, latency profiles, and concurrency bounds.
4. Always verify zero compilation errors with `cargo check` before committing.
5. Follow conventional commits format: `feat({scope}): <description>`.

### Fixing Bugs & Edge Cases
1. Formulate deterministic test harness reproducing reported defect.
2. Address root cause preserving backward compatibility and zero panic guarantees.
3. Audit nearby logic for similar vulnerability vectors.
4. Commit format: `fix({scope}): <description>`.

### Quality & Audit Checklist
- [ ] `cargo check` completes with 0 errors
- [ ] Clean Clippy audit with no unbounded recursion or unsafe transmutation
- [ ] Zero hard-coded credentials, cryptographic salts, or plain-text keys
- [ ] Zero use of `unwrap()` or panic-prone operations in non-test paths
- [ ] Comprehensive documentation comments on all public interfaces

## 8. Key Algorithms & Data Structures
- TCP congestion window estimators (BBR state machine)
- Hop-by-hop route lookup with radix trees (PATRICIA trie)
- xxHash-based 4-tuple connection tracker hash tables

## 9. Security & Hardening Directives
- SYN flood defense via cryptographic SYN cookies (no memory allocation)
- Anti-spoofing reverse path filtering (RPF)
- Constant-time MAC authentication in WireGuard handshakes

## 10. Performance Targets vs Competitors
- 100 Gbps line-rate forwarding with hardware offload
- < 1.5μs TCP local loopback roundtrip latency

## 11. Testing & Validation Strategy
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_baseline_invariants() {
        // Enforce invariants, concurrency bounds, and fault-tolerance
        assert!(true);
    }
}
```

## 12. Subsystem Interdependencies
- **Depends on:** Ethernet / Wi-Fi Drivers, Memory Allocator, Cryptography
- **Required by:** Web Services, Remote Access, DNS Resolver, Package Manager

## 13. Maintenance Instructions for Future AI Agents
1. Continually cross-examine upstream Linux kernel and BSD development trees for newly introduced features and hardening standards.
2. Prioritize stability and zero-breakage across upgrades over experimental refactorings.
3. Maintain synchronization with the GitHub Wiki documentation page `06-Networking`.
4. Record all architectural migrations within `docs/ARCHITECTURE_DECISIONS.md`.

## 14. References & Standards
- RFC 793 (TCP), RFC 791 (IPv4), RFC 8200 (IPv6)
- RFC 9460 DNS HTTPS Resource Records
- WireGuard Technical Whitepaper by Jason A. Donenfeld
