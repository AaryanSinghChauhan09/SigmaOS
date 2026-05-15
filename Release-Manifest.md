# SigmaOS: Zenith v15.0 Release Manifest

SigmaOS is available in multiple sovereign formats to ensure total industrial parity and portability.

## ?? Deployment Formats Status

| Format | Status | Branch | Primary Shards |
| :--- | :--- | :--- | :--- |
| **Standalone (Bare Metal)** | [STABLE] | elease/standalone | SovereignInstaller, SovereignRecovery |
| **Dual Boot (Interop)** | [STABLE] | elease/dual-boot | SovereignCompatibility, SovereignPartition |
| **Browser-Based (Web)** | [BETA] | elease/browser | SovereignWASM, ZenithWebUI |
| **App-Based (Mobile/VM)** | [BETA] | elease/app | SovereignMobile, SovereignSDK |

## ?? Build Instructions
To build a specific format, switch to the corresponding branch and run:
`make zenith-<format>-iso`

Example:
`git checkout release/standalone`
`make zenith-standalone-iso`
