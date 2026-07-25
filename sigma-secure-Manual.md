# sigma-secure Manual

## NAME

`sigma-secure` — SigmaOS security hardening & audit CLI

## SYNOPSIS

```
sigma-secure <command> [options]
sigma-secure --version
sigma-secure --help
```

## DESCRIPTION

`sigma-secure` is the primary security management interface for SigmaOS. It performs system security audits, applies hardening profiles, manages post-quantum cryptography keys, verifies TPM attestation chains, and generates signed security reports.

All operations support `--json` for CI integration.

## COMMANDS

### `audit [--fix]`

Run a full system security audit covering 10 check categories:

- Secure Boot state
- Kernel hardening flags (SMEP, SMAP, stack protector)
- Unexpected SUID binaries
- SSH root login policy
- Firewall status
- Disk encryption
- IMA integrity policy
- PQC key presence
- CVE database scan
- Audit log daemon

`--fix` automatically remediates any fixable findings.

### `harden [--profile <name>]`

Apply a system hardening profile. Available profiles:

| Profile | Description |
|---------|-------------|
| `sovereign` | SigmaOS baseline hardening (default) |
| `cis` | CIS Benchmark Level 2 |
| `nist` | NIST SP 800-123 guidelines |
| `stig` | DISA STIG controls |

### `pqc <gen|list|verify>`

Manage post-quantum cryptography keys (CRYSTALS-Dilithium Level 5, NIST FIPS 204):

```bash
sigma-secure pqc gen           # Generate Dilithium-5 keypair
sigma-secure pqc list          # List keys in /etc/sigma/pqc/
sigma-secure pqc verify        # Verify boot signature chain
```

### `attest`

Verify TPM 2.0 attestation chain:

- Checks TPM chip presence
- Verifies PCR[0] (boot) and PCR[7] (Secure Boot)
- Validates quote signed by AIK

### `policy <list|set|export>`

Manage system security policies:

```bash
sigma-secure policy list                  # Show active policies
sigma-secure policy set cis-level2        # Apply a policy
sigma-secure policy export                # Export policy as JSON
```

### `report [--output <file>]`

Generate a signed security report with all findings. Without `--output`, prints to stdout.

## OPTIONS

| Flag | Description |
|------|-------------|
| `--profile <name>` | Hardening profile (sovereign\|cis\|nist\|stig) |
| `--output <file>` | Write report to file |
| `--fix` | Auto-remediate fixable audit findings |
| `--json` | Machine-readable JSON output |
| `--version`, `-V` | Print version |
| `--help`, `-h` | Show help |

## EXAMPLES

```bash
# Full audit and auto-fix
sigma-secure audit --fix

# Apply CIS hardening
sigma-secure harden --profile cis

# Generate and provision PQC keys
sigma-secure pqc gen

# Verify TPM attestation in JSON format
sigma-secure attest --json

# Export security report to HTML
sigma-secure report --output /tmp/sec-report.html

# CI: audit and fail on any FAIL-level findings
sigma-secure audit --json | jq '.audit.fail > 0'
```

## VERSION

sigma-secure 1.0.0

## SEE ALSO

`sigma-fix(1)`, `sigma-forensics(1)`, `sigma_diagnostics(1)`, `sigma-recover(1)`
