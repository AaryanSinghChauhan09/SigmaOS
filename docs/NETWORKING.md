# SigmaOS Network Stack Architecture

SigmaOS utilizes a modern, forward-looking custom network stack built from scratch, prioritizing performance, distribution, and zero-trust security.

## 1. QUIC Protocol (`sigma_quic`)

SigmaOS prioritizes QUIC over TCP for modern transport.

- **0-RTT Connection**: Reduces latency for known hosts.

- **Stream Multiplexing**: Eliminates head-of-line blocking.

- **TLS 1.3 Integration**: Encrypted by default using `sigma_aes256_gcm`.

## 2. IPv6 First (`sigma_ipv6`)

Built for modern address spaces.

- **NDP**: Neighbor Discovery Protocol for local routing.

- **SLAAC**: Stateless Address Autoconfiguration allows nodes to self-assign IPs without DHCP.

## 3. Distributed Mesh (`sigma_mesh_protocol`)

Designed for decentralized deployments.

- **Gossip Heartbeats**: Nodes announce their presence to peers.

- **DHT Routing**: Kademlia-style XOR distance metric for finding optimal routes.

- **Self-Healing**: Automatically reroutes around failed nodes.

## 4. WireGuard-inspired VPN (`sigma_wireguard`)

Kernel-level secure tunneling.

- Implements Noise Protocol concepts for key exchange.

- Uses AES-GCM (eventually ChaCha20-Poly1305) for high-speed transport encryption.

- Seamlessly encapsulates standard IP traffic between authorized peers.
