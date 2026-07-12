# sigma-log Manual

## NAME

`sigma-log` — SigmaOS unified log viewer and anomaly detector

## SYNOPSIS

```
sigma-log <command> [options]
sigma-log --version
sigma-log --help
```

## DESCRIPTION

`sigma-log` is the primary log management interface for SigmaOS. It reads from the kernel journal socket at `/run/sigma/journal.sock` (or simulates sample data when the socket is unavailable) and provides real-time tailing, full-text search, statistics, anomaly detection, and multi-format export.

## COMMANDS

### `tail [--lines <n>] [--source <s>] [--level <l>]`

Show the most recent log entries:

```bash
sigma-log tail                           # last 20 lines, all sources

sigma-log tail --lines 50                # last 50 lines

sigma-log tail --source sigma-net        # filter by source

sigma-log tail --level warn              # only WARN and above

sigma-log tail --json                    # JSON output for parsing

```

### `follow [--source <s>]`

Stream logs in real time (like `tail -f`):

```bash
sigma-log follow
sigma-log follow --source sigma-security
```

### `search --query <q> [--level <l>]`

Full-text search across log entries. The matched term is highlighted in the output:

```bash
sigma-log search --query "OOM"
sigma-log search --query "ssh" --level error
sigma-log search --query "timeout" --json
```

### `dump [--output <file>]`

Dump all log entries to stdout or a file:

```bash
sigma-log dump
sigma-log dump --output /var/log/sigma-full.log
```

### `stats`

Log level distribution with bar charts:

```bash
sigma-log stats
sigma-log stats --json
```

### `anomaly [--threshold <n>]`

Detect spikes and anomalies in the log stream. The threshold controls sensitivity (default: 3):

```bash
sigma-log anomaly
sigma-log anomaly --threshold 2    # more sensitive

sigma-log anomaly --json
```

### `export --format <fmt> [--output <file>]`

Export logs in a specific format:

```bash
sigma-log export --format json   --output logs.json
sigma-log export --format csv    --output logs.csv
sigma-log export --format syslog --output syslog.log
```

## OPTIONS

| Flag | Description |
|------|-------------|
| `--lines <n>` | Number of lines to show (default: 20) |
| `--source <name>` | Filter by log source (e.g. `sigma-net`) |
| `--level <l>` | Minimum log level: `trace\|debug\|info\|warn\|error\|critical` |
| `--query <q>` | Search query string |
| `--threshold <n>` | Anomaly detection sensitivity (default: 3) |
| `--format <fmt>` | Export format: `json\|csv\|syslog` |
| `--output <file>` | Write output to file |
| `--no-color` | Disable ANSI colour codes |
| `--json` | Machine-readable JSON output |
| `--version`, `-V` | Print version |
| `--help`, `-h` | Show help |

## LOG LEVELS

| Level | Colour | Description |
|-------|--------|-------------|
| TRACE | dim | Extremely verbose, kernel internals |
| DEBUG | dim | Debugging information |
| INFO  | green | Normal operational messages |
| WARN  | yellow | Non-fatal issues worth investigating |
| ERROR | red | Errors that affect functionality |
| CRIT  | red bg | Critical: system stability at risk |

## EXAMPLES

```bash

# Follow all logs above WARN in real time

sigma-log follow --level warn

# Search for OOM events and export to JSON

sigma-log search --query OOM --json | jq '.results[].msg'

# Detect anomalies and pipe to sigma-fix

sigma-log anomaly --json | sigma-fix scan --stdin

# Export last 1000 entries as CSV

sigma-log dump --output /tmp/sigma-dump.log
sigma-log export --format csv --output /tmp/sigma.csv
```

## VERSION

sigma-log 1.0.0

## SEE ALSO

`sigma-debug(1)`, `sigma-fix(1)`, `sigma_diagnostics(1)`
