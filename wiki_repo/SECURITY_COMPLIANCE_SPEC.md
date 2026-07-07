# Security, Privacy, and Compliance Framework

## 1. Zero Trust and Least Privilege
SigmaOS is built on a "Default Deny" philosophy.
- **Root/Admin Access:** The traditional `root` user is entirely disabled. Administrative escalation (`sudo`) requires cryptographic attestation via TPM2 or a hardware security key (YubiKey/FIDO2).
- **Mandatory Access Control (MAC):** A native, lightweight MAC engine strictly governs file access and syscall permissions. This replaces the complex configuration requirements of AppArmor and SELinux.

## 2. Supply Chain Integrity
- **Kernel & Artifact Signing:** Every kernel image, bootloader, and `.sigpkg` package is signed using Ed25519. Unsigned binaries cannot execute in Ring 0 or as background system services.
- **SBOM Generation:** The CI/CD pipeline enforces the generation of an SPDX SBOM for every build.

## 3. Privacy Controls and Legal Forensics
- **Telemetry:** All system telemetry is explicitly opt-in.
- **AI Provenance Logs:** When the local SigmaAI runtime suggests or executes a command, it is permanently logged in a secure audit trail, noting the AI origin to maintain human accountability.
- **Forensics Mode:** SigmaOS includes a dedicated boot target (`BootTarget::Forensics`) that mounts all filesystems read-only, captures physical memory dumps, and integrates tools like Volatility and Sleuth Kit directly into the rescue shell.
