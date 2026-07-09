# SigmaOS Cybersecurity & Forensics Suite

## Overview
SigmaOS incorporates a complete, sandboxed cybersecurity and systems forensics environment designed to match and exceed platforms like Kali Linux and Qubes OS. Isolation is achieved via microVMs (Firecracker/gVisor), while local audits and network protection are managed via Suricata/Snort integrations and GnuPG/Vault secret managers.

## Security & Sandboxing Architecture
The system enforces tight Mandatory Access Control (MAC) sandboxing using gVisor/Firecracker, separating running user applications from the host kernel.

```
       [Zenith Application (e.g., Browser)]
                        │
                        ▼
   [gVisor Sentry (Intercepts Syscalls)]
                        │
                        ▼
   [Gofer (Filesystem sandbox daemon)]
                        │
                        ▼
    [SigmaOS Host Kernel (capability-token limited)]
```

## Network Protection (IDS/IPS)
Network interfaces run telemetry pipelines feeding raw frames directly to specialized userland Suricata and Snort daemons. System anomalies trigger automated packet filters and connection drops.

Example security monitor rules:
```toml
[firewall]
mode = "nftables"
block_p2p = true
allow_trusted_dids = true

[ids]
engine = "suricata"
rules_update_cron = "0 2 * * *"
action_on_threat = "quarantine_ip"
```

## Technical Implementation
The sandboxing engine wraps process execution using LSM (Linux Security Module) interfaces and capability tokens.

```rust
// sigmad-sandbox/src/main.rs
pub struct SandboxContext {
    pub container_id: String,
    pub capabilities: Vec<CapabilityToken>,
    pub runc_spec: PathBuf,
}

impl SandboxContext {
    pub fn enforce_isolation(&self) -> Result<(), SandboxError> {
        // Enforce Landlock and seccomp filters on the current process
        apply_seccomp_rules(&self.capabilities)?;
        apply_landlock_restrictions()?;
        Ok(())
    }
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: gVisor wrapper setup and Landlock file restriction controls.
- **Phase 2 (Months 3-6)**: Suricata network daemon integration with Zenith network monitor UI.
- **Phase 3 (Months 6-9)**: Incident response and forensics tools (Volatility, Autopsy, Sleuth Kit) pre-packaged.
- **Phase 4 (Months 9-12)**: End-to-end secrets management integrating TPM2 and KeePassXC-compatible vaults.
