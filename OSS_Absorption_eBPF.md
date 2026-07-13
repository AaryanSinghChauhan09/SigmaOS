# OSS Absorption: eBPF — Kernel Programmability Layer

> **Status**: 🔄 Active | **Source Projects**: Linux eBPF, BCC, bpftrace, Cilium | **Target Shard**: `SigmaOS Kernel Observability & Policy Layer`

---

## 1. Executive Summary

eBPF (extended Berkeley Packet Filter) is the most significant Linux kernel innovation of the last decade. It allows safe, sandboxed programs to run inside the kernel without modifying kernel source or loading kernel modules. eBPF programs are JIT-compiled, formally verified by the kernel verifier, and can:

- **Observe** any kernel event (syscalls, network packets, scheduler events, memory allocations) with zero overhead when inactive
- **Enforce** network policy at line rate (replacing iptables — 10x faster)
- **Profile** CPU flamegraphs, lock contention, memory leaks in production without code changes

SigmaOS integrates eBPF as the backbone of `sigma-observe` (observability), `sigma-net-policy` (network firewall), and `sigma-perf` (continuous performance profiling).

---

## 2. Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                    SIGMA eBPF LAYER                              │
│                                                                  │
│  Userspace Tools:                                                │
│  sigma-observe │ sigma-perf │ sigma-net-policy │ sigma-trace     │
│       │                │               │              │          │
│       └────────────────┴───────────────┴──────────────┘          │
│                                │ BPF syscall                     │
│  ┌─────────────────────────────▼──────────────────────────────┐  │
│  │                  eBPF VERIFIER (kernel)                    │  │
│  │  • Bounds checking                                         │  │
│  │  • No unbounded loops                                      │  │
│  │  • Type safety (BTF — BPF Type Format)                    │  │
│  │  • Max 1M instructions                                     │  │
│  └─────────────────────────────┬──────────────────────────────┘  │
│                                │ JIT compile → native x86/ARM    │
│  ┌─────────────────────────────▼──────────────────────────────┐  │
│  │              ATTACHMENT POINTS                              │  │
│  │  kprobe  │ tracepoint │ XDP (network) │ cgroup │ LSM hook  │  │
│  └─────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 3. Key Features

### 3.1 `sigma-observe` — Zero-Overhead Tracing (inspired by bpftrace)

```bash
# Trace all open() syscalls system-wide
$ sigma observe syscall open
Σ [OBSERVE] Tracing open() syscalls... (Ctrl+C to stop)
  [firefox:12345]  open("/etc/ssl/certs/ca-bundle.crt", O_RDONLY)   → fd 14
  [code:67890]     open("/home/alice/project/src/main.rs", O_RDONLY) → fd 7
  [sshd:1001]      open("/etc/ssh/sshd_config", O_RDONLY)           → fd 4

# Trace slow disk I/O (>10ms)
$ sigma observe disk --slower-than 10ms
  [rustc:9999]  read  /home/alice/target/debug/... (45ms, 2.1MB)
  [firefox:12345] read /sigma/store/firefox/.../libxul.so (12ms, 8MB)

# CPU flamegraph (profile all processes for 10s)
$ sigma perf flamegraph --duration 10s --output flamegraph.svg
Σ [PERF] Profiling all CPUs for 10 seconds...
  Output: flamegraph.svg (open in browser)
```

### 3.2 `sigma-net-policy` — eBPF Network Policy (XDP, replacing iptables)

XDP (eXpress Data Path) runs eBPF programs at the network driver layer — before sk_buff allocation — achieving 10–25 Mpps on a single core vs ~1Mpps for iptables.

```rust
// kernel/net/xdp_policy.rs — compiled to eBPF bytecode
// SPDX-License-Identifier: GPL-2.0

#[xdp_program]
pub fn sigma_packet_filter(ctx: XdpContext) -> XdpAction {
    let eth = ctx.load::<EthHdr>(0)?;
    if eth.ether_type != EtherType::Ipv4 {
        return XdpAction::Pass;
    }

    let ip = ctx.load::<Iphdr>(ETH_HDR_LEN)?;
    let tcp = ctx.load::<TcphdrFixed>(ETH_HDR_LEN + ip.ihl() * 4)?;

    // Drop SYN packets to port 23 (Telnet) at driver layer
    if tcp.dest == 23 && tcp.syn() == 1 {
        return XdpAction::Drop;   // Kernel never sees this packet
    }

    // Rate-limit: drop if source IP sent >10k pps in last second
    if RATE_TABLE.get_pps(ip.src) > 10_000 {
        return XdpAction::Drop;
    }

    XdpAction::Pass
}
```

```bash
# Declarative firewall rules — compiled to eBPF/XDP at apply time
$ sigma firewall add-rule "deny tcp dport 23"
$ sigma firewall add-rule "allow tcp dport 22 from 10.0.0.0/8"
$ sigma firewall add-rule "rate-limit udp 1000pps"
$ sigma firewall apply
Σ [FIREWALL] Compiled 3 rules to eBPF/XDP — loaded at driver level
  Performance: ~15Mpps throughput, <100ns latency per rule check
```

### 3.3 `sigma-perf` — Continuous Production Profiling

Inspired by Google's continuous profiling (`parca`), SigmaOS runs a low-overhead eBPF profiler as a background shard:

```bash
# Show live CPU hotspots
$ sigma perf top
Σ [PERF] CPU Profile (sampled at 99Hz using eBPF perf events):

  Symbol                                        CPU%  Samples
  ─────────────────────────────────────────────────────────────
  rustc::codegen_llvm::compile_fn               38.2%  3,821
  firefox::gfx::layers::LayerManagerComposite   12.1%  1,210
  kernel::mm::page_fault_handler                 5.4%    541
  [idle]                                        31.2%  3,120

# Show lock contention
$ sigma perf locks --top 5
Σ [PERF] Top Lock Contention (last 60s):
  sigma-ipc::MessageQueue::lock    2.3ms avg wait  18,000 contentions
  kernel::mm::mmap_lock            0.8ms avg wait   4,200 contentions
```

### 3.4 `sigma-cilium` — Container Network Policy

For the container/Kubernetes profile, SigmaOS integrates Cilium's eBPF-based CNI:

```yaml
# Container network policy (enforced by eBPF, not iptables)
apiVersion: sigma.io/v1
kind: NetworkPolicy
metadata:
  name: frontend-policy
spec:
  selector: app=frontend
  ingress:
    - from: app=api-gateway
      ports: [8080]
  egress:
    - to: app=backend
      ports: [5432]
    - to: 0.0.0.0/0
      ports: [443]   # Allow HTTPS to internet
```

---

## 4. Performance vs Traditional Approaches

| Mechanism | Throughput | Latency per Rule | Overhead |
|:---------|:-----------|:-----------------|:---------|
| iptables (traditional) | ~1 Mpps | ~5µs | High (sk_buff copy) |
| nftables | ~2 Mpps | ~3µs | Medium |
| eBPF/XDP (sigma) | ~25 Mpps | ~80ns | Near-zero |

---

## 5. References & Standards

- Linux eBPF documentation — `kernel.org/doc/html/latest/bpf/`
- BCC (BPF Compiler Collection) — `github.com/iovisor/bcc` (Apache-2.0)
- bpftrace — `github.com/iovisor/bpftrace` (Apache-2.0)
- Cilium — `cilium.io` (Apache-2.0)
- XDP — `prototype-kernel.readthedocs.io/en/latest/networking/XDP/`
- Aya (Rust eBPF library) — `aya-rs.dev` (MIT / Apache-2.0)
