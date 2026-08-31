# Σ security/isolation — Sovereign Process Isolation (S-Sandbox)

Provides **zero-trust, capability-gated execution environments** for all
applications running on SigmaOS. No process can exceed its declared capability
set — not even with a kernel exploit.

## Source Files

| File | Description |
|---|---|
| `sandbox.rs` | Shard sandbox: spawn, constrain, monitor, destroy |

## Isolation Model

```
Untrusted App (WASM / ELF)
   └─ sigma_sandbox_create()         ← assigns capability token
         └─ Shard Boundary (hardware ring separation)
               ├─ FS namespace  (only allowed paths visible)
               ├─ Net namespace (only declared ports reachable)
               ├─ IPC namespace (only approved shard IDs)
               └─ Memory domain (IOMMU-enforced, no cross-shard DMA)
```

## API Interface

```c
typedef struct {
    sigma_u32 container_id;
    bool      network_access;
    bool      fs_access;
    sigma_u32 memory_limit_mb;
    char     *allowed_paths[16];
    uint16_t  allowed_ports[16];
} sigma_sandbox_config_t;

// Create a new sandboxed container
sigma_u32 sandbox_create(const sigma_sandbox_config_t *cfg);

// Execute a binary inside the sandbox
int sandbox_execute(sigma_u32 id, const char *binary_path);

// Check if a syscall is allowed inside this sandbox
int sandbox_check_syscall(sigma_u32 id, sigma_u32 syscall_nr);

// Destroy a sandbox and reclaim resources
void sandbox_destroy(sigma_u32 id);

// Validate a MAC policy decision for this sandbox
int sandbox_validate_mac(sigma_u32 id, const char *subj, const char *obj, const char *action);

// Initialise the sandbox subsystem
void init_security_isolation(void);
```

## Syscall Allowlist

Each sandbox defines an explicit **syscall allowlist**. Any call not on the list
is blocked at the dispatcher and logged to the audit chain:

```json
{
  "syscall_allowlist": ["sigma_vfs_read", "sigma_vfs_write", "sigma_net_send"],
  "syscall_denylist":  ["sigma_exec", "sigma_ptrace"]
}
```

## Roadmap

- [x] Sandbox create / execute / destroy lifecycle (`sandbox.rs`)

- [x] Syscall allowlist enforcement

- [ ] IOMMU-enforced DMA isolation

- [ ] seccomp-BPF equivalent for Sovereign ABI

- [ ] Sandbox live introspection API

- [ ] Escape detection via invariant checking

- [ ] Formal proof: sandbox capability confinement (Isabelle/HOL)

## Related Modules

- [`modules/security/access_control`](../access_control/README.md) — MAC policies

- [`modules/ext/plugins`](../../ext/plugins/README.md) — Plugin capsule sandboxing
