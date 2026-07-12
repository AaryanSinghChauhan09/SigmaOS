# SigmaOS vs Windows — Sovereignty, Open Source, and CLI Design

## Overview

Windows remains a closed-source, proprietary operating system. While Microsoft has open-sourced certain tools (PowerShell, VS Code, .NET Core, Windows Terminal), the Windows kernel (NT), core OS components, Win32 APIs, most device drivers, and the UI shell remain fully proprietary.

SigmaOS is designed from first principles to be the antithesis of this model: every line of the kernel, every CLI tool, every driver framework, and every userspace utility is open-source under GPL-2.0-or-later or MIT.

---

## What Windows Keeps Closed vs What SigmaOS Opens

| Component | Windows | SigmaOS |
|-----------|---------|---------|
| Kernel | Closed (NT kernel) | Open — `kernel/` (Rust, no_std) |
| Device drivers | Vendor proprietary + WHQL | Open SDF — `drivers/` (Rust + Zig) |
| Shell | Closed (Explorer, cmd.exe) | Open — `sigma-sh` (Rust, full scripting) |
| CLI tools | Closed (most) / Partial (PowerShell) | Fully open — 30+ tools in `tools/` |
| File system | NTFS proprietary | Open SigmaFS — `fs/` (Zig + Rust) |
| Package manager | Closed (Windows Update, winget partial) | Open `sigma-pkg` (Nim + Rust) |
| Build toolchain | MSVC proprietary | Open — Cargo + LLVM + Zig cc |
| Security model | Proprietary ACLs, BitLocker | Open PQC + MAC — `security/` |
| Update system | Closed Windows Update | Open A/B OTA — `tools/sigma-cli.rs` |
| Hypervisor | Hyper-V closed | Open `sigma-hypervisor` (Rust) |
| Telemetry | Mandatory, opaque | Optional, transparent `sigma-telemetry` |
| Source availability | Partial tools only | 100% of every commit |

---

## CLI Design Philosophy: Open vs Closed

### Windows CLI limitations

- `cmd.exe` is a legacy, proprietary shell with no published specification

- PowerShell is open source but the Windows APIs it calls are not

- WinAPI, registry access, and driver interfaces are undocumented or opaque

- No standardised way to install, update, or audit system tools

- No JSON output standard across system tools

### SigmaOS CLI design principles

Every CLI tool in SigmaOS follows the same contract:

```
sigma-<tool> <verb> [options]
  --help        Show usage (always available)
  --json        Machine-readable output (always available)
  --dry-run     Safe preview mode (available where applicable)
  --version     Print version
```

This means every tool is:

- **Scriptable** — `--json` output for automation pipelines

- **Auditable** — source is in the same repo, no black boxes

- **Composable** — outputs pipe cleanly to `jq`, `sigma-fix`, etc.

- **Self-documenting** — `sigma help <command>` and man pages

---

## Driver Model: Community vs Vendor Control

Windows relies on hardware vendors to write and sign closed-source WHQL-certified drivers. When a vendor stops supporting hardware, the driver is simply abandoned.

SigmaOS uses the **Sovereign Driver Framework (SDF)**:

- All drivers are open-source Rust or Zig

- Drivers are signed with Dilithium-5 (post-quantum) instead of WHQL

- Any contributor can write, audit, or fix a driver

- Driver hot-reload: `sigma shard reload <driver>` without rebooting

- See: [Driver Framework](Driver-Framework) · [Hardware Support](Hardware-Support)

---

## Reproducible Builds

Windows binaries are not reproducibly buildable — the same source produces different binaries depending on build environment and timestamp.

SigmaOS enforces reproducible builds:
```bash
sigma build --release --profile standalone

# SHA-256 of output is deterministic given same source commit

```

See: [Reproducible Builds Guide](Reproducibility-Guide)

---

## Security Model Comparison

| Feature | Windows | SigmaOS |
|---------|---------|---------|
| Secure Boot | UEFI (Microsoft key required) | UEFI + Dilithium-5 (own keys) |
| Driver signing | WHQL (Microsoft controls) | Dilithium-5 (self-sovereign) |
| Encryption | BitLocker (closed) | AES-256-XTS + CryptFS (open) |
| Post-quantum | Not yet deployed broadly | Dilithium-5 + Kyber (shipped) |
| Attestation | TPM (closed Windows stack) | Open TPM2 + sigma-attest |
| Audit logs | Event Viewer (proprietary) | sigma-log + OpenTelemetry |
| Vulnerability scan | Windows Defender (closed) | sigma-fix + sigma-secure audit |

---

## The Sovereignty Argument

Microsoft keeps Windows closed for three main reasons — revenue, vendor lock-in, and control over backward compatibility. These are business reasons, not technical ones.

SigmaOS makes the opposite bet: **sovereignty over software creates a more secure, trustworthy, and long-lived system** because:

1. **No hidden telemetry** — every data flow is auditable in source

2. **No forced obsolescence** — you own the driver, you maintain it

3. **No vendor gatekeeping** — any organisation can fork, audit, or deploy

4. **Cryptographic trust** — PQC signatures instead of corporate certificate hierarchies

5. **Reproducibility** — the binary you run matches the source you read

---

## Running Windows Apps on SigmaOS

SigmaOS does not exclude Windows compatibility — it absorbs it. The `sigma-wine` subsystem runs Windows EXEs via a cleanroom PE loader + NT API translation layer:

```bash
sigma-wine exec notepad.exe
sigma-wine info myapp.exe    # PE header analysis

sigma-wine prefix create ~/win-prefix --arch win64
```

See: [Windows Parity Roadmap](Windows-Parity-Roadmap) · [Win32 Compatibility](Win32-Compatibility)

---

## Summary

| | Windows | SigmaOS |
|-|---------|---------|
| Kernel source | ❌ Closed | ✅ GPL-2.0 open |
| CLI tools source | Partial | ✅ All 30+ open |
| Driver model | Vendor proprietary | ✅ Community SDF |
| Build reproducibility | ❌ No | ✅ Yes |
| Post-quantum crypto | Roadmap only | ✅ Shipped (Dilithium-5) |
| Audit trail | Opaque | ✅ Full OpenTelemetry |
| Package manager | Partial (winget) | ✅ sigma-pkg (fully open) |
| Sovereignty | ❌ Microsoft-controlled | ✅ Self-sovereign |

*See also: [SigmaOS vs Linux](SigmaOS-vs-Linux) · [OSS Absorption Strategy](OSS-Absorption-Strategy) · [Security Model](Security-Model)*
