# Security Policy

## Reporting a Vulnerability

**Do not open a public issue.** Please report security vulnerabilities privately to the maintainers at `security@sigmaos.lattice`.

We follow a 90-day disclosure policy. We will acknowledge your report within 48 hours and provide a status update within 7 days.

## Supported Versions

| Version | Supported |
| ------- | --------- |
| Zenith v100.x | ✅ |
| Sovereign v2.x | ✅ |
| Legacy v1.x | ❌ |

## Industrial Hardening Standards

SigmaOS adheres to the following security principles:
1. **Shard Isolation**: No shard can access the matrix without explicit `SovereignEventBus` authorization.
2. **Path Sanitization**: All filesystem requests are strictly validated by the `PathValidator`.
3. **Memory Safety**: The `Heap` primitives include active allocation tracking and overflow protection.
4. **Lifecycle Discipline**: Subscribers must cleanly unmount to prevent event-registry memory leaks.

---
*The Work of Sovereignty is never complete.*
