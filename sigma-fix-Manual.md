# sigma-fix Manual

## NAME

`sigma-fix` — AI-guided patch suggestion and auto-repair CLI

## SYNOPSIS

```
sigma-fix <command> [options]
sigma-fix --version
sigma-fix --help
```

## DESCRIPTION

`sigma-fix` analyses kernel logs, security audit results, configuration files, and system state to identify fixable issues. For each issue it generates a targeted diff patch with root-cause explanation and applies it with optional confirmation. All applied fixes are snapshotted for rollback.

## COMMANDS

### `scan [--path <dir>]`

Scan the system for fixable issues. Without `--path` it scans the whole filesystem:

```bash
sigma-fix scan
sigma-fix scan --path /etc
sigma-fix scan --json
```

Output shows each issue with ID, severity, category, and title.

### `suggest --id <fix-id>`

Show the AI-generated diff patch for a specific fix, with rationale:

```bash
sigma-fix suggest --id FIX-0001
sigma-fix suggest --id FIX-0007 --json
```

Output includes a coloured diff (green = added, red = removed) and a plain-English explanation of why the change is necessary.

### `apply --id <fix-id> [--auto] [--dry-run]`

Apply a suggested fix:

```bash
sigma-fix apply --id FIX-0001              # interactive confirmation
sigma-fix apply --id FIX-0001 --auto       # apply without prompt
sigma-fix apply --id FIX-0003 --dry-run    # show what would change
```

`--dry-run` shows the target file and change without modifying anything.

### `rollback --id <fix-id>`

Undo an applied fix by restoring the original file from the snapshot:

```bash
sigma-fix rollback --id FIX-0001
```

### `explain --id <fix-id>`

Detailed root-cause analysis for an issue and the rationale behind the fix:

```bash
sigma-fix explain --id FIX-0001   # SSH root login
sigma-fix explain --id FIX-0003   # PQC key generation
sigma-fix explain --id FIX-0007   # kptr_restrict
```

### `list`

Show all available and applied fixes:

```bash
sigma-fix list
sigma-fix list --json
```

## OPTIONS

| Flag | Description |
|------|-------------|
| `--id <fix-id>` | Fix identifier (e.g. `FIX-0042`) |
| `--path <dir>` | Directory to scan (default: `/`) |
| `--auto` | Apply without confirmation |
| `--dry-run` | Show what would change without writing |
| `--json` | Machine-readable JSON output |
| `--version`, `-V` | Print version |
| `--help`, `-h` | Show help |

## BUILT-IN FIX DATABASE

| ID | Severity | Category | Title |
|----|----------|----------|-------|
| FIX-0001 | CRITICAL | security | SSH root login enabled (`PermitRootLogin yes`) |
| FIX-0002 | HIGH | security | 3 SUID binaries with unexpected permissions |
| FIX-0003 | HIGH | pqc | Dilithium-5 keys missing — system using Ed25519 fallback |
| FIX-0004 | MEDIUM | config | sigma.toml missing mandatory [network] section |
| FIX-0005 | MEDIUM | kernel | GPU shard suspend timeout — driver version mismatch |
| FIX-0006 | LOW | perf | Transparent huge pages disabled — performance suboptimal |
| FIX-0007 | LOW | security | kernel.kptr_restrict not set (information disclosure risk) |

## CI INTEGRATION

```bash
# Fail CI if any CRITICAL issues are unfixed
sigma-fix scan --json | jq -e '.scan.fixes[] | select(.severity=="CRITICAL")' && exit 1

# Auto-apply all LOW severity fixes
sigma-fix scan --json | jq -r '.scan.fixes[] | select(.severity=="LOW") | .id' | \
  xargs -I{} sigma-fix apply --id {} --auto

# Full auto-repair in CI
sigma-fix scan --json | \
  jq -r '.scan.fixes[].id' | \
  xargs -I{} sigma-fix apply --id {} --auto
```

## EXAMPLES

```bash
# Find and review issues
sigma-fix scan
sigma-fix list

# Review before applying
sigma-fix suggest --id FIX-0001
sigma-fix explain --id FIX-0001

# Preview without writing
sigma-fix apply --id FIX-0001 --dry-run

# Apply and verify
sigma-fix apply --id FIX-0001 --auto
sigma-fix list   # FIX-0001 should now show APPLIED

# Undo if needed
sigma-fix rollback --id FIX-0001
```

## VERSION

sigma-fix 1.0.0

## SEE ALSO

`sigma-secure(1)`, `sigma-log(1)`, `sigma_diagnostics(1)`, `sigma-debug(1)`
