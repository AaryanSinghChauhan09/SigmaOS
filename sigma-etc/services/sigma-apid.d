# /sigma/etc/services/sigma-apid.d — dinit-style service definition (Chimera Linux)
# sigma-init reads these files and builds a dependency graph before starting services.
#
# Dependency types (dinit semantics):
#   needs     — hard: if dep fails to start, this service also fails
#   waits-for — soft: wait for dep but start even if it's missing/failed
#   before    — ordering only, no dependency
#
# This is more expressive than systemd Requires= / Wants= because:
#   - 'needs' propagates failure UP the dependency chain
#   - 'waits-for' allows degraded starts without cascading failure
#   - 'before' handles pure ordering without creating a dependency

type              = process
command           = /sigma/sbin/sigma-apid
smooth-recovery   = true
restart           = true

# ── Hard dependencies — sigma-apid CANNOT start if these fail ──────────────
# sigma-apid needs mTLS certificates from trustd before it can accept connections
needs             = sigma-trustd
# sigma-apid uses sigma-ds for service discovery
needs             = sigma-ds

# ── Soft dependencies — start even if these are absent ────────────────────
# healthd is nice to have but not required for apid to function
waits-for         = sigma-healthd
# vault provides secrets but apid can still start and block on vault
waits-for         = sigma-vault

# ── Ordering — start apid before these (but don't depend on them) ─────────
before            = sigma-lb

# ── Readiness notification — write byte to fd 3 when ready (s6 protocol) ──
ready-notification = pipefd:3

# ── Timing ─────────────────────────────────────────────────────────────────
# Wait up to 10s for hard deps before declaring startup failure
start-timeout     = 10
# If apid doesn't become ready within 15s, treat as crash
ready-timeout     = 15

# ── Fault contract (Round 8 SMF-inspired) ──────────────────────────────────
max-restarts      = 5
restart-window-s  = 60
on-give-up        = kill-contract
