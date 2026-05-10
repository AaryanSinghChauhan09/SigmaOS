# SigmaOS Sovereign Packaging Specification (SPS)

The **Sovereign Packaging Specification** defines the industrial standard for creating, signing, and distributing shards within the SigmaOS lattice.

## ðŸ“¦ Package Structure (.spkg)

A SigmaOS package is a PQC-sealed archive containing:

1. **manifest.json**: Shard metadata, dependencies, and capability requirements.
2. **binary.wasm / source.cpp**: The functional payload.
3. **resources/**: Assets, icons, and localized strings.
4. **signature.sig**: Dilithium-based PQC signature.

## ðŸ“ manifest.json Example

```json
{
  "shard_id": "com.sigma.browser.firefox",
  "version": "128.0.1-sov",
  "name": "Firefox Sovereign",
  "capabilities": ["NET_ACCESS", "FS_USER_READ", "GPU_ACCEL"],
  "dependencies": ["libsigma-ui-v2", "pqc-toolkit-v0.9"],
  "maintainer": "Mozilla Shard Masters"
}
```

## ðŸ›¡ï¸ Sandboxing Policies

Packages must specify their isolation level:

- **STRICT**: No hardware access; amnesic memory only.
- **SYSTEM**: Access to kernel bridges (requires attestation).
- **TRUSTED**: Full hardware access (Official shards only).

## ðŸ–‹ï¸ Verification Flow

1. The `SovereignMarketplace` fetches the `.spkg`.
2. `SovereignAttestation` verifies the PQC signature against the developer's public key.
3. `SovereignShardManager` staged the binary and executes it within a `SovereignSandbox` matching the requested capabilities.

---

### For development tools, see [Getting Started](Getting-Started.md)
