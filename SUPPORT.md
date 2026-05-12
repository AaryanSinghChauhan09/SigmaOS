# Sovereign Support Nexus

Welcome to the SigmaOS Support Nexus. As an industrial-grade operating system, SigmaOS provides several tiers of support for the Sovereign Lattice.

## 🛠 Self-Service Support

### 1. The Sovereign Wiki
The [GitHub Wiki](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) is the primary source of truth for all 600 shards, API references, and industrial strategy.

### 2. S-LOG Telemetry
If you encounter a shard failure, check the internal journal logs:
```bash
# View real-time kernel telemetry
sigma-cli log --follow
```

### 3. S-AUTO Self-Healing
SigmaOS is designed to automatically recover from shard-level corruption. If a component fails, the `SovereignRollbackShard` will attempt to restore the last stable PQC-attested snapshot.

## 🤝 Community & Industrial Support

### GitHub Issues
For bug reports and architectural suggestions, please use the [Issue Tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues). Ensure you attach the relevant `S-LOG` artifacts.

### Professional Tiers
For mission-critical industrial deployments (Bio-Fab, Aerospace, Defense), please refer to the `Sovereign-Industrial-Contract.md` in the `docs/` directory for SLA-backed support.

## 🔍 Frequently Asked Questions (FAQ)

**Q: How do I resolve include path errors?**
A: SigmaOS enforces root-relative addressing. Ensure your compiler search path includes the project root.

**Q: My PQC keys are not synchronizing.**
A: Verify that the `SovereignPQCEngine` is initialized and the hardware RNG (RDRAND) is accessible.

**Q: Can I run Linux applications?**
A: Yes, via the `S-PROTON` bridge, which provides OCI-compliant sharding for mainstream binaries.
