# Sovereign Networking Shard (S-NET)

The Networking Shard is a modular, hot-swappable TCP/IP stack implemented independently from the monolithic kernel core. It provides secure sockets and strict network isolation for SigmaOS.

## Architecture Diagram

```mermaid
graph TD
    A[Userland App] --> | Z-SYSCALL | B(S-NET Socket API)
    B --> C{PQC Engine}
    C --> | Encrypted | D[TCP/IP Stack]
    C --> | Unencrypted | D
    D --> E[Sovereign HAL]
    E --> F[Hardware NIC

 **TCP/IP Stack**: Full IPv4 (and future IPv6) implementation.

<<<<<<< HEAD:docs/Networking-Shard.md
- **TCP/IP Stack**: Full IPv4 (and future IPv6) implementation.
- **Secure Sockets**: Built-in integration with the Post-Quantum Cryptography

  (PQC) engine for default-encrypted packet transmission.

- **Hot-swappable**: The network driver and stack can be restarted or updated

  without rebooting the kernel.
=======
- **Secure Sockets**: Built-in integration with the Post-Quantum Cryptography (PQC) engine for default-encrypted packet transmission.

- **Hot-swappable**: The network driver and stack can be restarted or updated without rebooting the kernel.
>>>>>>> wiki/master:Networking-Shard.md

## API Examples

### Creating a Socke

c
int fd;
sigma_status status = SovereignNetworkShard::getInstance().socket_create(AF_INET, SOCK_STREAM, 0, &fd);
if (status == SIGMA_OK) {
    sigma_log("Socket successfully created.");

### Binding to Por

<<<<<<< HEAD:docs/Networking-Shard.md
```c
sigma_net_enable_pqe(fd, &my_kyber_keypair);
```

## Roadmap

- [x] TCP/IP stack (`tcp.rs`, `tcpip.rs`)
- [x] PQC engine stub (`pqfs.rs`)
- [x] Socket API (`socket.rs`)
- [ ] IPv6 SLAAC / NDP
- [ ] DHCPv6 client
- [ ] WireGuard-inspired VPN tunnel
=======
c
SovereignNetworkShard::getInstance().socket_bind(fd, 0x7F000001, 8080)
>>>>>>> wiki/master:Networking-Shard.md
