# sigma-monitor Manual

## NAME

`sigma-monitor` — SigmaOS real-time system monitor

## SYNOPSIS

```
sigma-monitor [mode] [--interval <sec>] [--count <n>] [--json]
sigma-monitor --version
sigma-monitor --help
```

## DESCRIPTION

`sigma-monitor` is a standalone system monitor for SigmaOS. It reads live metrics from `/proc/stat`, `/proc/meminfo`, and the kernel metrics interface at `/proc/sigma/*`. On non-SigmaOS platforms, it falls back to simulated data for demonstration.

Output supports both human-readable terminal display and `--json` for CI/monitoring pipelines.

## MODES

| Mode | Description |
|------|-------------|
| `cpu` | CPU usage per core with bar chart, colour-coded by threshold |
| `mem` | RAM + swap usage, used/total/percentage |
| `net` | Network I/O (rx/tx bytes) per interface |
| `disk` | Disk I/O and filesystem utilisation |
| `proc` | Top processes by CPU (reads `/proc/sigma/proclist`) |
| `all` | All metrics in a single snapshot (default) |
| `watch` | Continuous refresh loop, clears screen each iteration |

## OPTIONS

| Flag | Description |
|------|-------------|
| `--interval <sec>` | Refresh interval for watch mode (default: 2) |
| `--count <n>` | Stop after N samples (default: infinite in watch, 1 otherwise) |
| `--json` | Emit JSON lines instead of ANSI terminal output |
| `--version`, `-V` | Print version |
| `--help`, `-h` | Show help |

## OUTPUT COLOUR CODING

CPU usage bar changes colour based on load:

| Threshold | Colour |
|-----------|--------|
| < 50% | Green |
| 50–80% | Yellow |
| > 80% | Red |

## EXAMPLES

```bash

# Single snapshot, all metrics

sigma-monitor

# Watch CPU only, refresh every second

sigma-monitor cpu --interval 1

# Collect 10 samples of memory stats in JSON

sigma-monitor mem --count 10 --json

# Run continuously and pipe to a log

sigma-monitor all --json >> /var/log/sigma-metrics.jsonl
```

## PLUGIN USAGE

When `sigma-monitor` is on `PATH`, it can be invoked as a `sigma` plugin:

```bash
sigma monitor watch --interval 2
```

## VERSION

sigma-monitor 1.0.0

## SEE ALSO

`sigma_top(1)`, `sigma_diagnostics(1)`, `sigma-trace(1)`
