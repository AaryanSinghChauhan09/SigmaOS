# ADR-001: Snapshot Extraction of SigmaOS-network

| Field       | Value                          |
|-------------|--------------------------------|
| Status      | Accepted                       |
| Date        | 2025-07-22                     |
| Deciders    | SigmaOS core team              |
| Supersedes  | —                              |

## Context

SigmaOS originally lived as a monolithic Cargo crate in the main repository.
The network subsystem (TCP/IP stack, TLS context, DNS resolver, mDNS announcer)
grew to the point where it deserved an independently versioned, separately
testable component — following the *multiple-repository* and *component
isolation* principles.

The extraction had to preserve the existing public API surface exactly (stable
APIs principle) and remove all cross-crate references that would create
a compile-time dependency on the full monorepo.

## Decision

Extract the network subsystem into a standalone crate `sigmaos-network` with
the following design constraints:

1. **Layered design** — public types map to OSI/TCP-IP layers:
   `TcpStack` (IP/transport), `TcpConnection` (transport), `TlsContext`
   (session), `DnsResolver` / `MdnsAnnouncer` (application/policy).

2. **Separation of mechanism and policy** — `TcpConnection` owns byte
   transport (mechanism); `TlsContext` owns cipher selection, certificate
   verification, and SNI (policy). These are separate types rather than
   mixed flags on a single struct.

3. **Stable public API** — all public types are exported from `lib.rs`
   directly; internal helpers remain private. The crate version follows
   semver: breaking changes require a major version bump.

4. **Zero-copy receive path** — `TcpConnection::recv` drains the internal
   ring buffer into the caller-supplied slice; no intermediate allocation.

5. **Race-free design** — the current stub is single-threaded; when async
   I/O is introduced the connection table will be guarded by an explicit
   lock (not global mutable state) to avoid data races.

6. **Memory safety** — the public API surface contains zero `unsafe` blocks;
   unsafe may appear in the future low-level device driver layer, isolated
   behind a `mod unsafe_hal` boundary.

7. **Minimal dependencies** — only `thiserror` for ergonomic error types.
   No runtime, no OS-specific calls in the crate root.

## Consequences

### Positive
- Independent release cycle for the network stack.
- Faster `cargo check` for contributors working only on network code.
- Clear ownership boundary; other SigmaOS crates depend on a semver-pinned
  `sigmaos-network` rather than importing internal modules.

### Negative / Trade-offs
- An additional repository to maintain.
- The stub implementation must be replaced with real TCP/IP machinery before
  production use; the stubs exist solely to satisfy the stable-API contract
  during the extraction phase (Gall's Law: build the simple system first).

## Alternatives Considered

| Option | Reason rejected |
|--------|-----------------|
| Keep in monorepo as a `mod network` | Does not achieve independent versioning or CI isolation |
| Use a Cargo workspace | Adds workspace complexity before the component is mature enough to warrant it |
| Extract with async runtime (tokio) | Premature — introduces a heavy dependency before the synchronous API is stable (YAGNI) |
