# Mandatory Access Control (MAC)

Sovereign alternative to SELinux and AppArmor with **deterministic policy
evaluation** (no probabilistic caching, no race conditions).

## Model

SigmaOS uses a **Lattice-Based Access Control** model where every subject
(process shard) and object (resource) has a clearance label. Operations are
only permitted when the lattice partial order is satisfied.

## Architecture

```
Subject (Shard)
   └─ Capability Token (clearance label)
         └─ MAC Policy Engine
               ├─ Policy Compiler (text → binary)
               ├─ Policy Evaluator (lattice check)
               └─ Audit Chain (BLAKE3-linked log)
                     └─ Object (Resource)
```

## Lattice Structure

The security lattice is defined as a partial order `(L, ≤)` where:

- **Subjects** have clearance labels `{confidentiality, integrity}`
- **Objects** have sensitivity labels
- **Operations** are permitted when `subject_label ≥ object_label`

```
Top (SYSTEM_ADMIN)
├── High (CONFIDENTIAL, HIGH_INTEGRITY)
├── Medium (INTERNAL, MEDIUM_INTEGRITY)
└── Low (PUBLIC, LOW_INTEGRITY)
```

## Policy Language

```
# Example: allow browser shard to read /media, deny /sys

ALLOW browser_shard READ /media
DENY  browser_shard ANY  /sys

# Network policies
ALLOW web_shard   CONNECT net:443
DENY  web_shard   BIND    net:*

# Filesystem policies
ALLOW app_shard   READ  /home
ALLOW app_shard   WRITE /tmp
DENY  app_shard   ANY   /etc/shadow
```

## API Interface

```c
// Load a MAC policy file
int sigma_mac_load_policy(const char *policy_path);

// Check if a subject is allowed to perform an action on an object
int sigma_mac_check(shard_id_t subj, object_id_t obj, mac_action_t action);

// Assign a clearance label to a subject
int sigma_mac_assign_subject_label(shard_id_t subj, const char *label);

// Assign a sensitivity label to an object
int sigma_mac_assign_object_label(object_id_t obj, const char *label);

// Get the current policy decision for a subject-object-action triple
mac_decision_t sigma_mac_get_decision(shard_id_t subj, object_id_t obj, mac_action_t action);

// Initialize MAC subsystem
void init_security_mac(void);
```

## Policy Compiler

The policy compiler transforms human-readable policy rules into an efficient binary format:

```
Policy File (text)
   └─ Policy Compiler
         ├─ Lexical Analysis
         ├─ Syntax Parsing
         ├─ Semantic Analysis (lattice validation)
         └─ Binary Generation
               └─ Policy Binary (O(1) lookup table)
```

## Enforcement Points

MAC policies are enforced at multiple kernel entry points:

| Enforcement Point | Description |
|---|---|
| Syscall Dispatcher | Checks file/network operations |
| VFS Layer | Validates file access permissions |
| Network Stack | Enforces socket binding/connecting rules |
| IPC Layer | Validates inter-shard communication |

## Roadmap

- [x] Basic MAC policy language
- [ ] Label assignment to all shards at boot
- [ ] Policy compiler (text → binary rule table)
- [ ] Kernel enforcement hook in syscall dispatcher
- [ ] VFS layer integration
- [ ] Network stack integration
- [ ] GUI policy editor for Zenith Desktop
- [ ] Policy hot-reload (without reboot)
- [ ] Formal verification of policy correctness
- [ ] SELinux policy import tool

## Related Modules

- [`modules/security/access_control`](../modules/security/access_control/README.md) — Runtime MAC & Audit
- [`modules/security/isolation`](../modules/security/isolation/README.md) — Process isolation
