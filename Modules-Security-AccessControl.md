# Σ security/access_control — Mandatory Access Control & Audit

Sovereign alternative to SELinux and AppArmor with **deterministic, lattice-based
policy evaluation** and an immutable audit chain.

## Source Files

| File | Description |
|---|---|
| `audit_chain.rs` | Append-only, BLAKE3-chained audit log of all policy decisions |

## Access Control Model

SigmaOS uses a **Lattice-Based Access Control** (LBAC) model:

- Every **subject** (shard) carries a clearance label `{confidentiality, integrity}`.
- Every **object** (file, socket, IPC endpoint) carries a sensitivity label.
- Operations are permitted only when the lattice partial order is satisfied.

```
ALLOW browser_shard READ  /media          # read public data
DENY  browser_shard ANY   /sys            # no kernel inspection
ALLOW ssh_daemon    BIND  net:22          # bind privileged port
```

## Audit Chain

Every policy decision is appended to a BLAKE3-linked chain:

```
Entry N: { timestamp, subject, object, action, decision, hash(Entry N-1) }
```

This makes the audit log **tamper-evident** — any modification breaks the hash
chain and is detected at verification time.

## API Interface

```c
// Check if a shard is allowed to perform an action on an object
int sigma_mac_check(shard_id_t subj, object_id_t obj, mac_action_t action);

// Append an audit entry (called automatically by mac_check)
void sigma_audit_log(const sigma_audit_entry_t *entry);

// Verify integrity of the entire audit chain
int sigma_audit_verify(void);

// Load a MAC policy file
int sigma_mac_load_policy(const char *policy_path);

// Initialise MAC subsystem
void init_security_access_control(void);
```

## Policy Language

```
# Allow app_shard to read /home, write /tmp
ALLOW app_shard   READ  /home
ALLOW app_shard   WRITE /tmp
DENY  app_shard   ANY   /etc/shadow

# Network policies
ALLOW web_shard   CONNECT net:443
DENY  web_shard   BIND    net:*
```

## Roadmap

- [x] Audit chain with BLAKE3 linking (`audit_chain.rs`)
- [ ] Policy compiler (text → binary rule table)
- [ ] Kernel enforcement hook in syscall dispatcher
- [ ] Label assignment to all shards at boot
- [ ] Policy hot-reload (without reboot)
- [ ] GUI policy editor for Zenith Desktop

## Related Modules

- [`modules/security/isolation`](../isolation/README.md) — Process isolation
- [`modules/core/kernel`](../../core/kernel/README.md) — Syscall enforcement hooks
