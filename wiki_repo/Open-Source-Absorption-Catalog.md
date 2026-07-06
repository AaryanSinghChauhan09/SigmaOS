# SigmaOS Open-Source Project Absorption Catalog

## Overview

This catalog catalogs 110+ open-source projects that SigmaOS can absorb, adapt, or reimplement to accelerate development and create consolidated alternatives. Each entry includes license analysis, technical feasibility, and repository mapping.

## License Compatibility Guide

### Permissive Licenses (Easy to Integrate)

- **MIT/BSD**: Can be integrated with minimal restrictions

- **Apache 2.0**: Patent protection, can be integrated

- **ISC**: Similar to MIT, very permissive

### Copyleft Licenses (Require Care)

- **GPL v2/v3**: Must keep derivative works under GPL

- **AGPL**: Requires source disclosure for network use

- **LGPL**: Can link statically with restrictions

### Strategy

- **Permissive**: Vendor and adapt with attribution

- **GPL**: Reimplement in Rust/Nim or use as reference only

- **Mixed**: Create interop/shims to keep projects separate

## Priority 1: Immediate Priority (Score 12-15)

| Project | License | Technical | Strategic | Total | Effort | Recommendation |
|---------|---------|-----------|-----------|-------|--------|----------------|
| Wasmtime | 4 | 5 | 5 | 14 | 2 | **Integrate directly** |
| Wasmer | 5 | 5 | 5 | 15 | 2 | **Integrate directly** |
| smoltcp | 5 | 5 | 5 | 15 | 1 | **Integrate directly** |
| libsodium | 5 | 5 | 5 | 15 | 1 | **Integrate directly** |
| wlroots | 5 | 4 | 5 | 14 | 3 | **Integrate directly** |
| Tokio | 5 | 5 | 4 | 14 | 1 | **Integrate directly** |
| SQLite | 5 | 5 | 5 | 15 | 1 | **Integrate directly** |
| Prometheus | 3 | 5 | 4 | 12 | 1 | **Integrate directly** |
| OpenTelemetry | 3 | 4 | 4 | 11 | 2 | **Integrate directly** |
| Sigstore/Cosign | 3 | 4 | 5 | 12 | 2 | **Integrate directly** |
| Firecracker | 3 | 4 | 4 | 11 | 3 | **Integrate directly** |
| BoringSSL | 3 | 4 | 5 | 12 | 2 | **Integrate directly** |
| Caddy | 3 | 5 | 4 | 12 | 1 | **Integrate directly** |
| Redis | 5 | 5 | 4 | 14 | 1 | **Integrate directly** |
| Homebrew | 5 | 4 | 4 | 13 | 2 | **Use as reference** |
| tmux | 5 | 5 | 3 | 13 | 1 | **Integrate directly** |
| dash | 5 | 5 | 4 | 14 | 1 | **Integrate directly** |
| TrustedFirmware-A | 5 | 4 | 4 | 13 | 2 | **Integrate directly** |
| rump kernels | 5 | 4 | 4 | 13 | 2 | **Integrate directly** |
| LK (Little Kernel) | 5 | 4 | 3 | 12 | 2 | **Integrate directly** |

## Implementation Phases

### Phase 1: Foundation (Weeks 1-12)

**Goal**: Establish core capabilities through direct integration

**Projects**:

1. Wasmtime/Wasmer - WASM runtime

2. smoltcp - Network stack

3. libsodium - Crypto primitives

4. Tokio - Async runtime

5. SQLite - Embedded database

6. wlroots - Wayland compositor

7. Prometheus - Metrics

8. OpenTelemetry - Tracing

9. Sigstore/Cosign - Signing

10. BoringSSL - TLS stack

### Phase 2: Expansion (Weeks 13-24)

**Goal**: Expand capabilities through selective integration

**Projects**:

1. Firecracker - MicroVM runtime

2. containerd/runc - Container runtime

3. gVisor - Sandbox

4. Caddy - Web server

5. Redis - Caching

6. Postgres - Database

7. CoreDNS - DNS resolution

8. quinn - QUIC protocol

9. libinput - Input handling

10. Mesa KMS - GPU modesetting

## Summary Statistics

### License Distribution

- **MIT/BSD**: 45 projects (40%)

- **Apache-2.0**: 30 projects (27%)

- **GPL**: 25 projects (22%)

- **LGPL**: 8 projects (7%)

- **Other**: 4 projects (4%)

### Feasibility Distribution

- **Very High** (permissive): 55 projects (49%)

- **High** (permissive with attribution): 30 projects (27%)

- **Medium** (copyleft/reference): 25 projects (22%)

- **Low** (incompatible): 2 projects (2%)

---

**Last Updated**: 2026-07-05
**Maintained by**: SigmaOS Core Team
