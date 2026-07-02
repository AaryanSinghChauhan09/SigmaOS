# sigma-shard — Shard System Specification

**Status:** Draft · Target: v0.1
**Owner:** shard/ipc team
**Canonical source:** `suites/`, `kernel/shard/`, `userland/sigma-bus/`

---

## Overview

SigmaOS organises its codebase into 600+ independently-loadable shards. Each shard is an atomic unit of functionality identified by `S<N>_<Name>`. Shards communicate via sigma-bus IPC, declare capabilities, support hot-reload, and are supervised by a watchdog daemon for crash recovery.

## Goals

- Each shard testable in isolation: no implicit global state
- Hot-reload: update a shard binary without rebooting (replace `.sigpkg` + signal bus)
- Crash recovery: supervisor restarts failed shards within 100 ms
- Capability tokens: a shard declares required capabilities; bus enforces them
- Packaging: each shard ships as a `.sigpkg` with its own `manifest.toml`

---

## Shard Lifecycle

```
UNREGISTERED
     │ sigma-bus discovers .sigpkg or compiled-in shard
     ▼
  PROBING  ── probe() returns error ──► UNAVAILABLE
     │ probe() OK
     ▼
INITIALISING ── init() error ──────────► FAILED
     │ init() OK
     ▼
  RUNNING ◄────────────────────────────────────────
     │ tick(Δt) called on each scheduler quantum  │
     │ events dispatched from sigma-bus            │
     ▼                                             │
  PAUSING ← pause requested (e.g. hot-reload)     │
     │                                             │
  PAUSED ───────────────────────────────────────► │
     │ resume() OR new binary installed           │
     ▼
  SHUTDOWN ── shutdown() called
     │
UNREGISTERED
```

### Lifecycle Callbacks

```c
typedef struct {
    const char *name;           // e.g. "S034_AI"
    const char *version;        // semver
    int  (*probe)   (void);     // check hardware/deps present; return 0=ok
    int  (*init)    (void);     // allocate resources, open IPC sockets
    void (*tick)    (uint64_t delta_us); // periodic update
    void (*shutdown)(void);     // free resources
    void (*on_event)(SigmaEvent *ev);   // async event handler
} SigmaShardDesc;
```

Registration macro: `SIGMA_SHARD_REGISTER(desc)` — linker section `__shards` collects all.

---

## sigma-bus IPC

### Sync Request/Response

```c
// Caller
SigmaBusMsg req = { .dst = "S034_AI", .op = "infer", .payload = json_buf };
SigmaBusMsg resp;
int rc = sigma_bus_call(&req, &resp, timeout_ms);

// Handler (inside shard)
void on_event(SigmaEvent *ev) {
    if (strcmp(ev->op, "infer") == 0) { ... sigma_bus_reply(ev, &resp); }
}
```

### Async Publish/Subscribe

```c
sigma_bus_subscribe("S07_Net/link_up", my_callback);
sigma_bus_publish("S034_AI/model_loaded", &payload);
```

### IPC Transport

- In-process (same address space): direct function call via vtable, zero copy
- Cross-process: UNIX domain socket at `/run/sigma-bus/<shard>.sock`; `msghdr` with `SCM_RIGHTS` for fd passing
- Kernel shards: syscall `sigma_bus_ioctl(SIGMA_BUS_CALL, &req)` for Ring-0 ↔ Ring-3

---

## Capability Tokens

Each shard declares required capabilities in its `manifest.toml`:

```toml
[capabilities]
required = ["net_socket", "proc_exec"]
optional = ["ai", "vault"]
```

sigma-bus verifies that the requesting process holds the declared tokens (via sigma_pledge check) before routing messages. Denied messages logged to audit.

---

## Hot-Reload Mechanism

1. Install new shard `.sigpkg`: `sigma-pkg install S034-AI-1.1.0`
2. Bus notifies supervisor: `ShardUpdateAvailable(S034_AI, "1.1.0")`
3. Supervisor calls `shard->pause()` → state serialised to shared memory buffer
4. Old shard binary unmapped; new binary `dlopen()`'d (or new process forked)
5. New shard calls `restore_state(buf)` if implemented, else cold `init()`
6. Supervisor calls `resume()` → shard returns to RUNNING
7. Total downtime: < 100 ms target; in-flight requests queued by bus

---

## Health Heartbeat

- Each running shard calls `sigma_bus_heartbeat()` at least once per 5 seconds
- Supervisor maintains `last_heartbeat[shard_id]` timestamp
- If `now - last_heartbeat > 15s`: shard marked UNHEALTHY
- Supervisor sends SIGTERM; if no exit within 2s: SIGKILL; then restart
- sigma-monitor polls `sigma-bus://monitor/shards` to display health

---

## Crash Recovery

```
shard crashes (SIGSEGV / SIGABRT / exit non-zero)
   → supervisor receives SIGCHLD / process exit event
   → wait(2) to collect exit status
   → log crash with shard name, PID, exit code, timestamp
   → back-off: restart after 0s, 1s, 5s, 30s (exponential, cap 30s)
   → after 5 consecutive crashes within 60s: shard marked DISABLED, alert sent
```

---

## Shard Registry Manifest

Global registry at `/etc/sigma-shards/registry.toml`:

```toml
[[shard]]
id      = "S034_AI"
version = "1.0.0"
binary  = "/usr/lib/sigma-shards/S034_AI.so"
autostart = true
capabilities_required = ["ai"]

[[shard]]
id      = "S07_Net"
version = "0.9.0"
binary  = "/usr/lib/sigma-shards/S07_Net.so"
autostart = true
capabilities_required = ["net_socket"]
```

---

## sigpkg Packaging for Shards

```
S034-AI-1.0.0-x86_64.sigpkg
├── META/manifest.toml
│     type = "shard"
│     shard_id = "S034_AI"
│     capabilities_required = ["ai"]
├── META/checksums.b3
├── META/signature.dil5
└── payload/
    └── usr/lib/sigma-shards/S034_AI.so
        etc/sigma-shards/S034_AI.toml   ← registry entry fragment
```

---

## Implementation Plan

- [ ] 1. Shard descriptor struct + `SIGMA_SHARD_REGISTER` macro
- [ ] 2. Shard registry loader (parse `/etc/sigma-shards/registry.toml`)
- [ ] 3. sigma-bus UNIX socket transport layer
- [ ] 4. Sync call/reply + async pub/sub dispatch
- [ ] 5. Capability token check in bus router
- [ ] 6. Heartbeat monitor + supervisor restart logic
- [ ] 7. Hot-reload protocol (pause → state serialise → reload → restore)
- [ ] 8. Crash recovery with exponential back-off
- [ ] 9. sigpkg registry fragment installer
- [ ] 10. Tests: lifecycle transitions, crash recovery, hot-reload, capability gate

---

## Status

| Feature | State |
|---------|-------|
| Shard lifecycle | ⬜ Not started |
| sigma-bus IPC | ⬜ Not started |
| Capability tokens | ⬜ Not started |
| Hot-reload | ⬜ Not started |
| Health heartbeat | ⬜ Not started |
| Crash recovery | ⬜ Not started |
