# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| v1.2.5-BEYOND (main) | ✅ Active |
| v1.1.0-APEX-SINGULARITY | ⚠️ Critical fixes only |
| < v1.0 | ❌ No longer supported |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities via public GitHub Issues.**

To report a security vulnerability in SigmaOS:


1. **Email**: [Sovereign-OS0305@gmail.com](mailto:Sovereign-OS0305@gmail.com)
2. **Or**: Open a private [Security Advisory](https://github.com/Sovereign-OS/SigmaOS/security/advisories/new) on GitHub

3. **Include**:

   - A description of the vulnerability
   - Steps to reproduce (PoC if possible)

   - The affected shard(s) or component
   - Potential impact assessment

4. **Response time**: We aim to acknowledge reports within **72 hours** and provide a fix timeline within **7 days**

## Security Architecture

SigmaOS enforces sovereign security through:


- **Capability-Based IPC (S-Zircon)**: All inter-shard communication uses unforgeable capability handles
- **Hardware-Backed Isolation**: Each suite runs in isolated shard domains with hardware compartmentalization

- **Neural Firewall (S08_Security)**: AI-driven anomaly detection at the kernel level
- **Zero-Trust Network (S07_Network)**: All mesh communications use HMAC-authenticated gossip

- **Privacy Shard (S31_Privacy)**: Native Tor routing and amnesic memory for sensitive operations
- **Quantum-Resistant Design**: Post-quantum cryptography planned for Phase 7 (CRYSTALS-Kyber)

## Scope

The following are **in scope** for security reports:

- Kernel privilege escalation via shard boundaries
- Memory corruption in `suites/S01_Genesis/` (core kernel)

- Bypass of capability-based access controls
- Cryptographic weaknesses in Lattice Mesh (S33) HMAC implementation

- Buffer overflows in any `sigma_*` LibC function

The following are **out of scope**:

- Issues in userland shards that don't affect kernel integrity
- Performance degradation without security impact

- Issues requiring physical hardware access
