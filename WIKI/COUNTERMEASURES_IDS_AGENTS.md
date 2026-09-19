# AI Agent Development Instructions for Countermeasures, IDS/IPS, Firewalls & Authentication (`src/security/`, `src/auth/`, `src/network/`)

This document details directives for Intrusion Detection/Prevention Systems (IDS/IPS), AI anomaly detection, stateful packet filter firewalls (OpenBSD PF / Linux nftables), authentication pipelines, PAM, and automated security countermeasures in SigmaOS.

## Subsystem Architecture & Directives

1. **Intrusion Detection Systems (IDS) & Threat Rules (`src/security/ids_rule_parser.rs` & `intrusion.rs`)**
   - Parse Snort/Suricata style IDS rules (`alert tcp $EXTERNAL_NET any -> $HOME_NET 22 (msg:"SSH Brute Force"; ...)`).
   - Evaluate network packet headers and payload signatures asynchronously using eBPF packet hooks (`seccomp_ebpf.rs`).

2. **AI Anomaly Detection & Threat Countermeasures (`src/security/ai_anomaly_detection.rs` & `malware.rs`)**
   - Monitor system call frequency, anomalous process spawning, and uncharacteristic outbound socket connections.
   - Automatically trigger defensive countermeasures (`BlockIp`, `KillProcess`, `IsolateSandbox`, `RevokeToken`) when threat anomaly scores exceed threshold (`> 0.90`).

3. **Stateful Firewalls & Packet Filtering (`src/network/pf_firewall.rs`, `nftables.rs`, `npf_firewall.rs`)**
   - OpenBSD PF & Linux nftables engine parity: maintain connection state tables (`PfStateTable`) supporting SYN flood protection (`synproxy`) and dynamic anchor rules (`pfctl` parity).
   - Default deny inbound traffic unless explicitly permitted by firewall rule tables.

4. **Multi-Factor Authentication & Access Control (`src/auth/authentication_pipeline.rs` & `pam.rs`)**
   - Enforce Pluggable Authentication Modules (PAM) architecture supporting multi-factor authentication (TOTP, WebAuthn, FIDO2, biometric fingerprint scans).
   - Apply exponential rate-limiting backoffs on failed authentication attempts to prevent brute-force attacks.

5. **Verification**
   - Verify security countermeasure logic using `cargo check --lib`.
