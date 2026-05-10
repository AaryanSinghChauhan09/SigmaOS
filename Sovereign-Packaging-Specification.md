1


The **Sovereign Packaging Specification** defines the industrial standard for creating, signing, and distributing shards within the SigmaOS lattice.


1


A SigmaOS package is a PQC-sealed archive containing:



1. **manifest.json**: Shard metadata, dependencies, and capability requirements.
2. **binary.wasm / source.cpp**: The functional payload.



3. **resources/**: Assets, icons, and localized strings.
4. **signature.sig**: Dilithium-based PQC signature.


1



1


{
  "shard_id": "com.sigma.browser.firefox",
  "version": "128.0.1-sov",
  "name": "Firefox Sovereign",
  "capabilities": ["NET_ACCESS", "FS_USER_READ", "GPU_ACCEL"],
  "dependencies": ["libsigma-ui-v2", "pqc-toolkit-v0.9"],
  "maintainer": "Mozilla Shard Masters"
}


1



1


Packages must specify their isolation level:


1



1



1




1. The `SovereignMarketplace` fetches the `.spkg`.
2. `SovereignAttestation` verifies the PQC signature against the developer's public key.



3. `SovereignShardManager` staged the binary and executes it within a `SovereignSandbox` matching the requested capabilities.

---


1

