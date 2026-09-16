# Network Stack, eBPF/XDP & PQC Security Rules for AI Agents

## Kernel Bypass eBPF/XDP Processing
- Use zero-copy DMA ring buffers (process_xdp_zero_copy_packet)
- Ensure XDP actions explicitly return XDP_PASS, XDP_DROP, XDP_TX, or XDP_REDIRECT
- Implement eBPF verifier-compliant code

## Post-Quantum Cryptography (PQC)
- Implement Dilithium-5 and Kyber-1024 for key exchange
- Use PQC for package manifest signatures
- Implement hybrid classical/PQC key exchange

## Network Security
- Implement eBPF firewall with deny-by-default
- Use XDP for DDoS mitigation at line rate
- Implement network flow filtering at kernel level

## Design Patterns
- **Decorator**: Layer TLS/crypto on existing sockets
- **Strategy**: Plugable crypto algorithms
- **Proxy**: Network filtering middleware
