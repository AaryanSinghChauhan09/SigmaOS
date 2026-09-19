# SigmaOS Sovereign Traffic Control (tc) Qdiscs

## Overview

SigmaOS implements **Linux Traffic Control (tc) queuing disciplines** in 100% safe Rust with zero external dependencies (`src/net/tc_qdisc_sovereign.rs` and `src/network/tc_qdisc_sovereign.rs`).

Queuing disciplines (qdiscs) manage how packets are queued, scheduled, rate-limited, and prioritized before transmission over physical or virtual network interfaces.

## Implemented Qdiscs

### 1. Token Bucket Filter (TBF)
- Strict bandwidth rate limiting (`rate_bps`)
- Burst allowance bucket (`burst_bytes`)
- Token replenishment based on elapsed time
- Queue depth limit with tail-drop accounting

### 2. Strict Priority Scheduler (PRIO)
- Multi-band priority queue (bands 0 to N-1)
- Strict non-preemptive dequeue from highest priority non-empty band
- Prevents high-priority control traffic (DNS, TCP ACKs, SSH) from being starved by bulk transfers

### 3. Hierarchical Token Bucket (HTB)
- Class hierarchy with parent/child relationship
- Guaranteed rate (`rate_bps`) and maximum ceiling rate (`ceil_bps`)
- Token borrowing/lending mechanism when bandwidth is available from parent or ceiling
- Priority-based scheduling between classes

### 4. Fair Queuing Controlled Delay (FQ-CoDel)
- Multi-flow hash-based fair queuing
- Deficit round-robin scheduling across active flows
- Active Queue Management (AQM) for latency reduction
- Explicit Congestion Notification (ECN) marking when queues exceed target delays

## Usage Example

```rust
// Create an HTB qdisc with default class
let mut htb = HtbQdisc::new(1);

// Add a 10 Mbps class with 20 Mbps ceiling
let class = HtbClass::new(1, 0, 10_000_000, 20_000_000, 0);
htb.add_class(class);

// Enqueue packet
let pkt = Packet::new(1500, 0, 1, current_timestamp_ns);
htb.enqueue(pkt, 1);

// Refill tokens and dequeue
htb.tick(current_timestamp_ns);
if let Some(packet) = htb.dequeue() {
    transmit_packet(packet);
}
```

## Linux `tc(8)` Parity

| Linux `tc` Command | SigmaOS Equivalent |
|--------------------|-------------------|
| `tc qdisc add dev eth0 root tbf rate 1mbit burst 32k latency 50ms` | `TbfQdisc::new(125_000, 32_768, 64)` |
| `tc qdisc add dev eth0 root handle 1: prio bands 3` | `PrioQdisc::new(3)` |
| `tc qdisc add dev eth0 root handle 1: htb default 1` | `HtbQdisc::new(1)` |
| `tc class add dev eth0 parent 1: classid 1:1 htb rate 1mbit ceil 2mbit` | `HtbClass::new(1, 0, 125_000, 250_000, 0)` |
| `tc qdisc add dev eth0 root fq_codel` | `FqCodelQdisc::new(1024)` |

## Test Verification

6 standalone unit tests verified in test runner suite `[11]`:
- `test_tbf_rate_limiting`
- `test_tbf_token_refill`
- `test_prio_strict_ordering`
- `test_htb_class_rate_ceiling`
- `test_fq_codel_fair_queuing`
- `test_fq_codel_ecn_marking`
