# SigmaOS System Daemons

All SigmaOS daemons live in `sigmad/` and expose HTTP APIs on Unix domain sockets. They run in Ring 3 under strict `sigma_pledge` + `sigma_unveil` restrictions.

---

## Daemon Architecture

```
sigma_init (PID 1)
    │  reads /etc/sigma/services/*.xml
    │  starts daemons in dependency order
    ▼
sigmad-health    sigmad-pkg    sigmad-netd    ...
    │                │              │
    │  Unix socket   │  Unix socket │  Unix socket
    ▼                ▼              ▼
/run/sigma/      /run/sigma/    /run/sigma/
  healthd.sock     pkg.sock       netd.sock
```

Every daemon: `curl --unix-socket /run/sigma/<name>.sock /health`

---

## Core Daemons

### sigmad-health
- **Purpose**: Structured health monitoring (CoreOS-inspired)
- **Socket**: `/run/sigma/healthd.sock`
- **Endpoints**: `GET /health`, `GET /metrics`, `GET /readyz`
- **pledge**: `stdio inet rpath`

### sigmad-watchdog
- **Purpose**: Hardware WDT + daemon liveness
- **Action**: Restarts crashed daemons; triggers kernel WDT if unresponsive
- **pledge**: `stdio proc`

### sigmad-metrics
- **Purpose**: Prometheus-compatible metrics endpoint
- **Socket**: `/run/sigma/metrics.sock`
- **Endpoint**: `GET /metrics` — exposes CPU, RAM, I/O, network counters
- **pledge**: `stdio rpath`

### sigmad-telemetry
- **Purpose**: Opt-in, PII-scrubbed telemetry
- **Privacy**: All data anonymised before transmission; user can disable
- **pledge**: `stdio inet`

### sigmad-cloudsync
- **Purpose**: End-to-end encrypted cloud sync
- **Crypto**: AES-256-GCM + Argon2id key derivation
- **pledge**: `stdio inet rpath wpath`

### sigmad-netd
- **Purpose**: Network interface management
- **Features**: DHCP, routing table, DNS configuration
- **pledge**: `stdio inet rpath wpath`

### sigmad-pkg
- **Purpose**: Package manager daemon
- **Features**: Install, update, remove `.spkg` packages; repo management
- **pledge**: `stdio rpath wpath cpath inet exec`

### sigmad-vault
- **Purpose**: Secrets and key management (TPM2-backed)
- **Features**: Seal/unseal secrets via TPM2 PCR, DID-based identity
- **pledge**: `stdio rpath wpath`

### sigmad-power
- **Purpose**: Power management (ACPI P/C-states)
- **Features**: Battery monitoring, frequency scaling, thermal throttle
- **pledge**: `stdio rpath`

### sigmad-notify
- **Purpose**: Desktop notification delivery
- **pledge**: `stdio`

### sigmad-indexd
- **Purpose**: Full-text file indexer for sigma-search
- **pledge**: `stdio rpath`

### sigmad-timed
- **Purpose**: NTP time synchronisation
- **pledge**: `stdio inet`

### sigmad-update
- **Purpose**: OS update management (OSTree A/B)
- **pledge**: `stdio rpath wpath inet exec`

### sigmad-heal
- **Purpose**: Self-healing daemon — detects kernel faults, triggers recovery
- **pledge**: `stdio proc rpath`

---

## Interacting with Daemons

```bash
# Check health
curl --unix-socket /run/sigma/healthd.sock /health

# Install a package
sigma-pkg install firefox

# Behind the scenes:
curl --unix-socket /run/sigma/pkg.sock /install \
  -d '{"name":"firefox","version":"latest"}'

# Prometheus metrics
curl --unix-socket /run/sigma/metrics.sock /metrics
```

---

## Daemon Configuration

Daemons are configured via `/etc/sigma/services/*.xml`:

```xml
<service name="sigmad-netd">
  <exec>/usr/bin/sigmad-netd</exec>
  <pledge>stdio inet rpath wpath</pledge>
  <restart>on-failure</restart>
  <after>sigmad-health</after>
</service>
```

---

## Status

| Daemon | Status |
|--------|--------|
| sigmad-health | ✅ Implemented |
| sigmad-watchdog | ✅ Implemented |
| sigmad-metrics | ✅ Implemented |
| sigmad-telemetry | ✅ Implemented |
| sigmad-cloudsync | ✅ Implemented |
| sigmad-netd | 🔄 Partial |
| sigmad-pkg | 🔄 Partial |
| sigmad-vault | 🔄 Partial |
| sigmad-power | 🔄 Partial |
| sigmad-update | ⬜ Phase G |
| sigmad-heal | 🔄 Framework done |

---

*See also: [Kernel](Kernel) · [Networking](Networking) · [Architecture-Overview](Architecture-Overview)*
