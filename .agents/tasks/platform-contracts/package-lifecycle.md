# Package Lifecycle Contract

> **Shared SigmaOS platform contract.** Define the install/remove/update/rollback contract for all packages, backend-agnostic. The package-manager implements this; backend adapters (pacman/apt/dnf/pkg) provide the actual operations.

---

## Interface

Transaction { resolve(names) -> ordered_list; verify(name) -> bool; install(names) -> result; remove(names) -> result; sync() -> result; }

## Invariants

- Transactions are atomic: either all operations succeed or none do (rollback to pre-transaction state).
- Dependency resolution is topological: a package's dependencies are installed before the package.
- Signature verification is mandatory before install; unsigned packages require explicit --allow-unsigned.
- A transaction log records every operation for audit and rollback.
- Dry-run mode (--dry-run) resolves and verifies without modifying state.

## Linux Adapter

pacman (Arch), apt (Debian), dnf (Fedora) adapters. Each maps the generic Transaction to the backend's native commands.

## BSD Adapter

pkg (FreeBSD/DragonFly), pkg_add (OpenBSD) adapters. OpenBSD uses pledge/unveil for sandboxed package builds.

## Failure Modes and Recovery

Dependency conflict, signature failure, disk full, network interrupted. Failed transactions roll back via the snapshot taken at resolve time.

## Open Questions

Should partial upgrades be allowed (update one package without its deps)? Current contract: no — atomic transactions only.
