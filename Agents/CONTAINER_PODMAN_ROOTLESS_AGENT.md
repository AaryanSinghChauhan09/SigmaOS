# Rootless OCI Container & Podman Engine Component Agent

## Component Overview
Governs daemonless, unprivileged rootless OCI containers, user namespace UID/GID sub-mapping (subuid/subgid), slirp4netns unprivileged networking, and quadlet systemd integration.

## Linux Inspiration
Podman rootless containers, crun lightweight OCI runtime, buildah, and slirp4netns.

## BSD Inspiration
FreeBSD unprivileged Jails (prison_create), OpenBSD pledge-confined micro-containers.

## Current SigmaOS Status
User namespace mapping and rootless container isolation primitives functional in container runtime engine.

## Critical Missing Features
Slirp4netns-compatible unprivileged virtual network bridge, and OCI image rootfs layer extraction directly into user home dirs.

## Implementation Priority
HIGH (Phase 2)

## Key Files to Create/Improve
src/container/runtime.rs, src/container/namespace.rs, src/security/sandbox.rs

## Testing Strategy
Rootless container breakout prevention tests, network packet routing through unprivileged tap devices.

## Dependencies
CONTAINER_RUNTIME_AGENT, SECURITY_AGENT, FILESYSTEM_VFS_AGENT

## Success Criteria
Zero root privileges required to run full Linux distro containers, sub-15ms container startup latency.

## Open Source Competitors Analysis
Eliminates the dangerous root-running Docker daemon architecture entirely, ensuring complete security isolation.

## Future Enhancements
WebAssembly (WASM/WASI) lightweight container runtimes running microservices in sub-millisecond execution sandboxes.
