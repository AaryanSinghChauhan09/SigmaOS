# SigmaOS Sovereign XDP Zero-Copy Networking

## Overview

SigmaOS implements **Linux XDP (eXpress Data Path) + io_uring-style zero-copy networking** in 100% safe Rust (`src/network/zero_copy_networking.rs`). This replaces traditional socket copy paths with direct NIC-to-userspace memory sharing.

## Inspiration

| Technology | Origin | Innovation |
|-----------|--------|------------|
| **AF_XDP** | Linux 4.18 (2018) | UMEM ring-based zero-copy receive/send |
| **XDP programs** | Linux 4.8 (2016) | eBPF hooks at NIC driver level — before skb alloc |
| **io_uring** | Linux 5.1 (2019) | Async I/O via shared ring buffers |
| **FreeBSD sendfile(2)** | FreeBSD 3.1 (1999) | Original zero-copy send |
| **FreeBSD netmap** | FreeBSD 9 (2011) | Userspace NIC ring access |

## Zero-Copy Architecture

```
NIC DMA → UMEM region (shared memory)
               ↓
          Fill Ring (kernel → user: "here's a free buffer")
               ↓
           RX Ring (kernel → user: "packet arrived at chunk N")
               ↓
         XDP Action: PASS / DROP / TX / REDIRECT
               ↓
           TX Ring (user → kernel: "send chunk N")
               ↓
       Completion Ring (kernel → user: "chunk N sent, reuse it")
```

No data is copied. The same physical memory chunk moves from fill → rx → tx → completion.

## Key Types

### `UmemPool` — Unified Memory Region

```rust
let mut pool = UmemPool::new(
    512,   // number of chunks
    4096   // chunk size in bytes (one page)
);

let idx = pool.alloc_chunk().unwrap(); // Allocate from pool
pool.free_chunk(idx);                  // Return to pool
println!("{}% utilized", pool.utilization_pct());
```

### `XdpRing` — Packet Ring Buffer

```rust
let mut ring = XdpRing::new(128); // 128-entry ring
ring.enqueue(PacketRingDescriptor { chunk_idx: 0, data_offset: 256, data_len: 1500, flags: 0 });
let desc = ring.dequeue().unwrap();
println!("processed: {}", ring.packets_processed);
println!("dropped:   {}", ring.drops);
```

### `SovereignZeroCopySocket`

```rust
let mut sock = SovereignZeroCopySocket::new(
    "eth0",  // interface name
    0,       // queue ID
    512,     // UMEM chunks
    128      // ring size
);

// Receive path
let chunk = sock.rx_packet(1500).unwrap();
let action = sock.process_rx().unwrap();
match action {
    XdpAction::Pass     => { /* pass to network stack */ }
    XdpAction::Drop     => { /* already freed */ }
    XdpAction::Tx       => { /* reflect back */ }
    XdpAction::Redirect => { /* send to other queue */ }
    XdpAction::Aborted  => { /* XDP program error */ }
}

// Transmit path
let chunk = sock.umem.alloc_chunk().unwrap();
sock.tx_packet(chunk, 64); // zero-copy send

// Drain completions
while let Some(cqe) = sock.cq.consume() {
    println!("sent {} bytes (user_data={})", cqe.result, cqe.user_data);
}
```

### `IoCompletionQueue` — io_uring-style CQ

```rust
let mut cq = IoCompletionQueue::new(256);
cq.post_completion(user_data_token, bytes_transferred);
cq.post_completion(other_token, -11); // -EAGAIN
let cqe = cq.consume().unwrap();
```

## XDP Actions

| Action | Value | Effect |
|--------|-------|--------|
| `XDP_PASS` | 2 | Forward packet up the stack |
| `XDP_DROP` | 1 | Drop packet at driver level (fastest) |
| `XDP_TX` | 3 | Reflect packet back out same NIC |
| `XDP_REDIRECT` | 4 | Redirect to different NIC queue or CPU |
| `XDP_ABORTED` | 0 | Error — packet dropped with trace event |

## Performance vs Traditional Sockets

| Metric | Traditional `recv()` | AF_XDP / SigmaOS |
|--------|---------------------|-------------------|
| Copies per packet | 2 (NIC→kernel→user) | 0 |
| Alloc per packet | 1 `sk_buff` (~256B) | 0 (pre-allocated UMEM) |
| Max PPS (Mpps) | ~1–3 Mpps | ~10–40 Mpps |
| Latency | ~5–20µs | ~1–3µs |

## Tests

6 unit tests + 1 CQ test, all passing:

- `test_umem_pool_alloc_free` — alloc/free, double-free prevention
- `test_xdp_ring_enqueue_dequeue` — ring full drops, FIFO ordering
- `test_zero_copy_socket_rx` — full receive path, chunk allocation
- `test_xdp_drop_small_packets` — packets < 14 bytes (no Ethernet header) dropped
- `test_zero_copy_tx` — zero-copy transmit with CQE
- `test_io_completion_queue` — CQ post/consume, error results

## Source

[`src/network/zero_copy_networking.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/network/zero_copy_networking.rs)
