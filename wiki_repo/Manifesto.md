# SigmaOS Manifesto

## Target Audience
- Sovereign, privacy-focused organizations (government, telecom, defense)
- Developers and users who prioritize security, performance, and auditability
- High-security cloud providers and edge deployments

## Unique Value Propositions
1. **Minimal TCB (Trusted Computing Base)**: Small, verifiable kernel and trusted components
2. **Capability-based Security**: Fine-grained, least-privilege access control
3. **Deterministic Builds & Reproducibility**: Every build produces an identical, verifiable binary
4. **Secure Boot & Attestation**: Measured boot with remote attestation for full chain of trust
5. **Sovereign-first Design**: No foreign dependencies, fully auditable supply chain

## Compatibility Promises
- **Linux Container Runtime**: Run OCI/Docker containers without modification
- **Linux ABI Shim**: Compatibility layer for common Linux userland apps
- **POSIX Compatibility**: Support for standard POSIX interfaces
