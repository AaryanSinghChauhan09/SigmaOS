# TCP/IP Network Stack

SigmaOS implements a full RFC 793 TCP state machine in `kernel/net/tcp.rs`.

---

## TCP State Machine (RFC 793)

```
               ┌─────────┐
               │  CLOSED │ ◄───────────────────────────────────┐
               └────┬────┘                                      │
          listen()  │  connect()                                 │
                    ▼                                           │
   ┌──────────┐   ┌──────────┐                              LAST_ACK
   │  LISTEN  │   │ SYN_SENT │                                  │
   └────┬─────┘   └────┬─────┘                              FIN+ACK sent
   SYN  │         SYN+ACK│                                      │
        ▼              ▼                                   ┌────┴────┐
   ┌──────────┐   ┌──────────────┐    close()             │CLOSE_WAIT│
   │ SYN_RCVD │   │ ESTABLISHED  │ ─────────────────────► └─────────┘
   └────┬─────┘   └──────┬───────┘
   ACK  │         FIN from│remote
        ▼              ▼
   ┌──────────┐   ┌──────────┐
   │ESTABLISHED│  │FIN_WAIT_1│
               │  └────┬─────┘
               │  ACK  │  FIN+ACK
               │       ▼       ▼
               │  ┌──────────┐ ┌──────────┐
               │  │FIN_WAIT_2│ │ CLOSING  │
               │  └────┬─────┘ └────┬─────┘
               │  FIN  │       ACK  │
               │       ▼            ▼
               │  ┌──────────────────┐
               └─►│   TIME_WAIT      │ ──2MSL──► CLOSED
                  └──────────────────┘
```

---

## Usage

```rust
// Server side
let fd = tcp_socket_create();
tcp_listen(fd, 8080);
// When SYN arrives: state → SYN_RCVD → ESTABLISHED

// Client side
let fd = tcp_socket_create();
tcp_connect(fd, 0xC0A80001, 80); // connect to 192.168.0.1:80
// state: CLOSED → SYN_SENT → ESTABLISHED (after SYN+ACK)

// Data transfer
tcp_send(fd, data.as_ptr(), data.len());
let n = tcp_recv(fd, buf.as_mut_ptr(), buf.len());

// Close
tcp_close(fd); // sends FIN, state → FIN_WAIT_1
```

---

## TCP Checksum

Hand-rolled RFC 793 checksum with pseudo-header:

```
Pseudo header:  src_ip(32) | dst_ip(32) | 0(8) | proto=6(8) | tcp_len(16)
+ TCP header + payload
One's complement sum of all 16-bit words
```

---

## Congestion Control (Phase C)

Planned BBR/CUBIC implementation:
- Slow start
- Congestion avoidance
- Fast retransmit (3 duplicate ACKs)
- Fast recovery

---

## RX/TX Buffers

Each socket has:
- 64 KB RX ring buffer
- 64 KB TX ring buffer
- MSS = 1460 bytes (standard Ethernet - IP - TCP headers)

---

## Integration

TCP receives segments from the IP layer:

```c
// Called by IP layer when a TCP segment arrives
tcp_rx_segment(fd, tcp_hdr, payload, payload_len);

// TCP sends via IP layer
sigma_ip_send(dst_ip, 6/*TCP*/, data, len);
```

---

*Source: `kernel/net/tcp.rs` — 500 lines, RFC 793 compliant, no_std*
