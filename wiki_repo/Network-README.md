# SigmaOS Network Stack

This directory contains the custom networking protocols, interfaces, and DNS services of the SigmaOS microkernel.

## Sub-Modules

### 1. Loopback Driver (`loopback.c`)

Standard mock transmission adapter:

- Configures local interface loopback at `127.0.0.1`.

- Feeds outgoing network frames directly back to the RX ingress queues.

- Captures loopback frame counters and data throughput statistics.

### 2. TCP/IP Stack (`tcp_ip.c`)

Freestanding, zero-dependency socket routing engine:

- Registers custom socket handles and maintains connections.

- Simulates POSIX socket routines: `socket()`, `connect()`, `send()`.

- Implements 3-Way Handshake states.

### 3. DNS Resolver (`dns.c`)

Lightweight local DNS mapping:

- Resolves domain addresses to IPv4 integers (e.g. `sigma.nexus`, `google.com`).
