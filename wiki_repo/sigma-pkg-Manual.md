# sigma-pkg Manual

## NAME

`sigma-pkg` — SigmaOS Sovereign Package Manager

## SYNOPSIS

```
sigma-pkg <command> [options] [packages...]
sigma-pkg --version
sigma-pkg --help
```

## DESCRIPTION

`sigma-pkg` is the native package manager for SigmaOS. Packages are cryptographically attested `.spkg` bundles signed with Dilithium-5 (NIST PQC). The implementation is in `pkg/sigma_pkg_cli.nim` (Nim, stdlib only).

All packages in the Sigma Store registry have:

- Ed25519 or Dilithium-5 signatures

- Declared capability requirements

- A dependency graph

- Atomic install/rollback via A/B staging

## COMMANDS

### `install <pkg...>`

Install one or more packages:

```bash
sigma-pkg install zenith-desktop
sigma-pkg install sigma-browser sigma-notes sigma-agent
sigma-pkg install sigma-ml --dry-run    # preview without installing

```

Options: `--dry-run`, `--force`, `--version <x.y.z>`

### `remove <pkg...>`

Remove installed packages:

```bash
sigma-pkg remove sigma-browser
sigma-pkg remove sigma-notes --purge    # also remove config files

```

### `search <query>`

Search the registry by name or description:

```bash
sigma-pkg search vr
sigma-pkg search "machine learning"
sigma-pkg search sigma --json
```

### `list [--filter <s>]`

List installed packages:

```bash
sigma-pkg list
sigma-pkg list --filter sigma
sigma-pkg list --json | jq '.[].name'
```

Output shows name, version, size, and install date.

### `update [pkg...]`

Check for updates. Without arguments, checks all installed packages:

```bash
sigma-pkg update                # check all

sigma-pkg update sigma-core    # check one package

sigma-pkg update --dry-run      # preview what would update

```

### `audit`

Scan all installed packages for known CVEs:

```bash
sigma-pkg audit
sigma-pkg audit --json | jq '.vulnerabilities'
```

### `info <pkg>`

Show detailed package metadata:

```bash
sigma-pkg info sigma-agent
sigma-pkg info zenith-desktop --json
```

Output includes version, description, size, dependencies, installed date, and signature status.

### `clean`

Remove orphaned packages and clear package cache:

```bash
sigma-pkg clean
```

### `pin <pkg>`

Prevent a package from being auto-updated:

```bash
sigma-pkg pin sigma-core      # protect core shard

sigma-pkg pin sigma-gpu-hal   # protect tested driver version

```

### `unpin <pkg>`

Re-enable auto-updates for a pinned package:

```bash
sigma-pkg unpin sigma-gpu-hal
```

### `export [--output <file>]`

Export the list of installed packages:

```bash
sigma-pkg export                          # print to stdout

sigma-pkg export --output packages.txt   # save to file

```

The export format is `name==version` per line — compatible with `sigma-pkg install` as input.

## OPTIONS

| Flag | Description |
|------|-------------|
| `--dry-run` | Show what would happen without making changes |
| `--force` | Override safety checks |
| `--purge` | Remove package and all associated config/data |
| `--filter <s>` | Filter `list` output by name substring |
| `--output <file>` | Write export to file |
| `--json` | Machine-readable JSON output |
| `--version`, `-V` | Print version |
| `--help`, `-h` | Show help |

## INSTALLED PACKAGES

Core packages that ship with every SigmaOS installation:

| Package | Version | Description |
|---------|---------|-------------|
| `sigma-core` | 15.0.0 | Kernel core shards |
| `sigma-sh` | 0.3.0 | Sovereign interactive shell |
| `sigma-net` | 2.1.0 | Networking stack shard |
| `sigma-vfs` | 3.0.0 | VFS and sigma-fs driver |
| `sigma-pqc` | 1.0.0 | Post-quantum cryptography |
| `sigma-hal` | 1.0.0 | Hardware abstraction layer |
| `sigma-agent` | 2.0.0 | AI-native system agent daemon |

## SECURITY

Every package in the Sigma Store is:

1. Built reproducibly — bit-for-bit identical on all hardware

2. Signed with Dilithium-5 (NIST FIPS 204) by the publisher

3. Counter-signed by the Sigma Store registry CA

4. Verified against the TPM PCR chain on install

To manually verify a package signature:
```bash
sigma-secure pqc verify sigma-core
sigma-pkg info sigma-core --json | jq '.signature'
```

## EXAMPLES

```bash

# Install the full desktop environment

sigma-pkg install zenith-desktop sigma-browser sigma-notes

# Check for and apply updates

sigma-pkg update
sigma-pkg update --dry-run   # preview first

# Search and install a specific package

sigma-pkg search vr
sigma-pkg install sigma-vr-studio

# Audit for vulnerabilities

sigma-pkg audit

# Export package list for reproducible deployments

sigma-pkg export --output /etc/sigma/packages.txt

# Re-install from an exported list

cat /etc/sigma/packages.txt | xargs sigma-pkg install
```

## VERSION

sigma-pkg 1.0.0

## SEE ALSO

`sigma pkg(1)` (via main sigma CLI), `sigma-secure(1)`, `sigma shard(1)`, `sigma-fix(1)`
