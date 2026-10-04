# Security

**Capability state: Prototype.** Security models and APIs do not establish runtime enforcement. Status terms are defined in the [shared vocabulary](14-Future-Development.md#work-status-vocabulary).

## Current capability

The repository contains security-related interfaces and models, but the booted-kernel enforcement path has not been verified. Do not rely on capability checks, pledge/unveil, W^X, encryption, secure boot, package signatures, or audit logging unless a tested runtime implementation and supported configuration are documented. Cryptographic interfaces are not production-ready solely because an algorithm or API exists.

## Design references

OpenBSD pledge/unveil and W^X, FreeBSD Capsicum and signed catalogs, Linux namespaces and seccomp, and seL4 capability boundaries are references for specific threat models. Arch's transparent configuration and Mint's recovery guidance are useful operational models.

## Roadmap

1. Define protected assets, privilege boundaries, and failure behavior for the initial boot profile.
2. Enforce one narrow permission boundary and add denial/bypass tests.
3. Integrate audited cryptography for authenticated metadata; fail closed when verification is unavailable.
4. Test update rollback, recovery, logging, and security boundaries through actual runtime paths.

**Completion evidence:** documented threat model, repeatable adversarial tests, audited providers, and a recovery process that preserves access without weakening enforcement.
