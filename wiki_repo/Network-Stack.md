# Sovereign Network Stack

SigmaOS includes a custom, zero-dependency networking stack built from the ground up for minimal overhead and high security.

## Supported Protocols
- **L2**: Ethernet (MAC addressing, ARP)
- **L3**: IPv4
- **L4**: TCP and UDP

## Socket API

The network stack provides a familiar, socket-based API internally:
- `socket_create()`
- `socket_bind()`
- `socket_listen()`
- `socket_accept()`
- `socket_connect()`
- `socket_send()`
- `socket_recv()`
- `socket_close()`

## TCP State Machine

The stack implements a rigorous TCP state machine, carefully tracking the transition between `LISTEN`, `SYN_SENT`, `ESTABLISHED`, and connection teardown states to defend against SYN flood and RST injection attacks.
