# Sovereign DNS Resolver

Encrypted, privacy-preserving DNS resolver integrated directly into the
SigmaOS networking shard.

## Features
- **DNS-over-TLS (DoT)** and **DNS-over-HTTPS (DoH)** support
- Local shard-based caching (no external resolver required)
- DNSSEC validation
- Sovereign split-horizon: internal `.sigma` domains resolved locally

## Roadmap
- [ ] DoT client implementation
- [ ] DNSSEC chain validator
- [ ] Local authority for `.sigma` TLD
