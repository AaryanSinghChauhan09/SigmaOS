# System Daemons

SigmaOS daemons are Go-language services that bridge the kernel to the browser and to userland applications. Each daemon exposes an **HTTP API on a Unix socket** — no TCP port exposure, no root required to query them.

---

## Design Principles

**Unix socket HTTP** — every daemon listens on `/run/sigma/<name>.sock`. You can query any daemon with `curl --unix-socket /run/sigma/healthd.sock http://localhost/health`. No special client library needed.

**Pledge + unveil** — every daemon calls `sigma_pledge` and `sigma_unveil` after startup to restrict its own syscall surface and filesystem visibility.

**Watchdog registration** — every daemon registers with `sigma-watchdog` at startup and sends a heartbeat every 30 seconds. A daemon that misses 3 heartbeats is restarted automatically.

**Health reporting** — every daemon exposes `/health` and reports to `sigma-healthd`. `sigmactl health` shows real-time status of all subsystems.

---

## Complete Daemon Reference

### sigma-busd — IPC Message Bus
**Socket**: `/run/sigma/busd.sock`  
**Source**: `sigmad/busd/main.go`  
**Inspired by**: Fuchsia FIDL, D-Bus (but capability-gated)

Replaces D-Bus with a simpler, capability-gated message bus. Every IPC route requires an explicit capability token — a process can't accidentally reach a daemon it wasn't granted access to.

| Endpoint | Method | Description |
|---|---|---|
| `/bus/emit` | POST | Emit a message on an interface |
| `/bus/subscribe` | POST | Subscribe to messages on an interface |
| `/bus/interfaces` | GET | List all registered interfaces |

```bash
# Emit a notification via sigma-bus
curl -s --unix-socket /run/sigma/busd.sock \
  -X POST http://localhost/bus/emit \
  -d '{"interface":"sigma.Notifications","signal":"Notify","body":"{\"title\":\"Hello\"}"}'
```

---

### sigma-healthd — Structured Health Monitor
**Socket**: `/run/sigma/healthd.sock`  
**Source**: `sigmad/healthd/main.go`  
**Inspired by**: CoreOS health endpoints, Flatpak runtime

The system's conscience. Every subsystem reports its status here. Running `sigmactl health` shows which parts are real implementations and which are stubs.

```
✓ zero-trust    ok      0 violations in 24h
✓ pledge        ok      3412 calls enforced
✓ cryptfs       ok      AES-256-GCM + TPM2 key (Issue #44 fixed)
✓ net-firewall  ok      142 active flows
✗ nvidia-driver FAILED  driver not loaded — GPU not available
```

| Endpoint | Method | Description |
|---|---|---|
| `/health` | GET | Overall system health (JSON) |
| `/health/<subsystem>` | GET | Per-subsystem detailed status |
| `/health/history` | GET | Last 100 health events |

---

### sigma-apid — gRPC Management API
**Socket**: `/run/sigma/apid.sock`  
**Source**: `api/sigma.proto`  
**Inspired by**: Talos Linux management API

Full remote management over gRPC + mTLS. Used by the `sigmactl` CLI and the Zenith admin panel.

```bash
# Via sigmactl
sigmactl health          # system health
sigmactl pkg list        # installed packages
sigmactl pkg install vim # install a package
sigmactl sysctl get kernel.sched.rt_threshold
sigmactl sysctl set security.aslr.enabled=1
sigmactl audit stream    # live audit event stream
```

---

### sigma-watchdog — Hardware & Software Watchdog
**Socket**: `/run/sigma/watchdog.sock`  
**Source**: `sigmad/watchdog/main.go`  
**Inspired by**: Linux watchdog(8), systemd-watchdog

Pets `/dev/watchdog` every 15 seconds to prevent hardware reset. Monitors all registered daemons for missed heartbeats and restarts them via `sigmactl restart <name>` if they go silent.

Pre-registered critical daemons: `sigma-healthd`, `sigma-busd`, `sigma-trustd`, `sigma-netd`.

| Endpoint | Method | Description |
|---|---|---|
| `/watchdog/status` | GET | WDT last pet time + all watched daemon statuses |
| `/watchdog/register` | POST | Register a daemon for monitoring |
| `/watchdog/heartbeat` | POST | Daemon sends its own heartbeat |
| `/watchdog/unregister` | POST | Remove a daemon from monitoring |

```bash
# Check watchdog status
curl -s --unix-socket /run/sigma/watchdog.sock http://localhost/watchdog/status | jq .

# Register your daemon
curl -s --unix-socket /run/sigma/watchdog.sock \
  -X POST http://localhost/watchdog/register \
  -d '{"name":"my-daemon","max_interval_sec":60,"restart_cmd":"sigmactl restart my-daemon"}'
```

---

### sigma-metrics — Prometheus Exporter
**Socket**: `/run/sigma/metrics.sock`  
**Source**: `sigmad/metrics/main.go`  
**Inspired by**: Prometheus node_exporter

Exports system metrics in Prometheus text format. Optionally binds TCP `:9100` for Prometheus scraping by setting `SIGMA_METRICS_TCP=1`.

```
# HELP sigma_cpu_usage_percent CPU usage percent per core
# TYPE sigma_cpu_usage_percent gauge
sigma_cpu_usage_percent{core="0"} 12.4
sigma_cpu_usage_percent{core="1"} 8.1

# HELP sigma_mem_total_bytes Total memory
sigma_mem_total_bytes 8589934592
sigma_mem_available_bytes 6204416000

# HELP sigma_uptime_seconds System uptime seconds
sigma_uptime_seconds 3847.2
```

| Endpoint | Method | Description |
|---|---|---|
| `/metrics` | GET | All metrics (Prometheus format) |
| `/metrics/cpu` | GET | CPU metrics only |
| `/metrics/mem` | GET | Memory metrics only |

---

### sigma-power — Power Management
**Socket**: `/run/sigma/power.sock`  
**Source**: `sigmad/power/main.go`  
**Inspired by**: systemd-logind, UPower

Monitors battery, handles lid close/open events, manages screen dim on idle, triggers suspend/hibernate. Emits events on the `sigma.Power` bus interface.

| Endpoint | Method | Description |
|---|---|---|
| `/power/status` | GET | Battery %, AC plugged, lid state, idle seconds |
| `/power/suspend` | POST | Trigger immediate suspend |
| `/power/hibernate` | POST | Trigger hibernate to disk |
| `/power/activity` | POST | Reset idle timer (user is active) |

```bash
# Check battery status
curl -s --unix-socket /run/sigma/power.sock http://localhost/power/status
# {"battery_percent":73,"ac_plugged":false,"lid_open":true,"state":"active","idle_seconds":45}
```

---

### sigma-netd — Network Namespace Daemon
**Socket**: `/run/sigma/netd.sock`  
**Source**: `sigmad/netd/main.go`  
**Inspired by**: Android netd

Creates and manages per-process network namespaces. Associates veth pairs for processes that declare `net:host` capability. Configures per-namespace firewall rules.

---

### sigma-telemetry — Privacy-Respecting Telemetry
**Socket**: `/run/sigma/telemetry.sock`  
**Source**: `sigmad/telemetry/main.go`  
**Inspired by**: Ubuntu apport (but opt-in and transparent)

All telemetry is **OFF by default**. Every report is shown to the user before transmission. PII (hostnames, usernames, IP addresses, local paths) is scrubbed automatically before any data leaves the device. Data is sent over TLS 1.3 to `telemetry.sigma-os.dev`.

| Endpoint | Method | Description |
|---|---|---|
| `/telemetry/status` | GET | Opt-in status + report count |
| `/telemetry/optin` | POST | Enable telemetry |
| `/telemetry/optout` | POST | Disable + purge local ledger |
| `/telemetry/report` | POST | Submit an event report |

---

### sigma-cloudsync — E2E Encrypted Cloud Sync
**Socket**: `/run/sigma/cloudsync.sock`  
**Source**: `sigmad/cloudsync/main.go`  
**Inspired by**: Nextcloud sync client, Syncthing

All data is encrypted client-side with AES-256-GCM **before** upload. The encryption key is derived from the user's passphrase via Argon2id — the server never sees the key. Files are chunked to 4 MiB and deduplicated by BLAKE2b hash.

| Endpoint | Method | Description |
|---|---|---|
| `/sync/login` | POST | Derive key from passphrase (key stays in memory only) |
| `/sync/start` | POST | Begin sync of a folder |
| `/sync/stop` | POST | Pause sync |
| `/sync/status` | GET | Files queued, bytes uploaded, last sync time |
| `/sync/logout` | POST | Wipe encryption key from memory |

---

### sigma-trustd — Certificate Authority & Attestation
**Socket**: `/run/sigma/trustd.sock`

Issues Dilithium3-signed certificates for workload identities (SPIFFE URIs). Uses Kyber-1024 for the key exchange establishing the mTLS session between nodes. Enforces the correct separation: Kyber for key exchange, Dilithium for signing.

---

### sigma-vault — Secrets Manager
**Socket**: `/run/sigma/vault.sock`  
**Source**: `sigmad/vault/main.go`  
**Inspired by**: HashiCorp Vault, macOS Keychain

AES-256-GCM encrypted secret store with the master key sealed to TPM2. Secrets are bound to the boot measurement (PCR values) — if the boot chain changes (kernel replaced, BIOS tampered), the vault cannot be unsealed.

---

## Adding a New Daemon

Minimal template:

```go
// sigmad/mydaemon/main.go
package main

import (
    "fmt"
    "net"
    "net/http"
    "os"
)

func main() {
    // 1. Remove stale socket
    sockPath := "/run/sigma/mydaemon.sock"
    os.Remove(sockPath)

    // 2. Register with watchdog
    // (HTTP POST to /watchdog/register)

    // 3. Set up routes
    mux := http.NewServeMux()
    mux.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
        fmt.Fprintln(w, `{"status":"ok"}`)
    })

    // 4. Listen
    ln, _ := net.Listen("unix", sockPath)
    fmt.Println("[sigma-mydaemon] listening on", sockPath)
    http.Serve(ln, mux)
}
```

Then add a dinit service file at `sigma-etc/services/sigma-mydaemon.d` so the init system starts it automatically.

---

*See also: [Architecture Overview](Architecture-Overview) · [Kernel Architecture](Kernel) · [Security Model](Security-Model) · [Building from Source](Building-from-Source)*
