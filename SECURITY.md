# Σ SigmaOS Security Model

SigmaOS is designed for the post-quantum era, absorbing strengths from Whonix, Tails, and Fedora CoreOS.

## 🔐 Core Security Features
*   **Post-Quantum Crypto**: `SovereignPQC` (Kyber/FIPS-203) integrated at Ring 0.
*   **Mandatory Access Control**: `SovereignAppArmor` capability-based isolation.
*   **Hardened Networking**: `SovereignWhonixTor` privacy-first routing.
*   **Reproducible Builds**: SHA256 parity for all system-critical shards.

## 🚨 CVE Disclosure Process
Follow the official [**CVE Triage Pipeline**](CVE_TRIAGE) for vulnerability reporting and disclosure.

## 🛡️ Reproducibility
Every SigmaOS shard is built deterministically. Use `reproducible_build.ps1` to verify your local binaries against the official marketplace signatures.
