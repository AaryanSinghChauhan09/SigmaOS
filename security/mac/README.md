# Mandatory Access Control (MAC)

Sovereign alternative to SELinux and AppArmor with **deterministic policy
evaluation** (no probabilistic caching, no race conditions).

## Model
SigmaOS uses a **Lattice-Based Access Control** model where every subject
(process shard) and object (resource) has a clearance label. Operations are
only permitted when the lattice partial order is satisfied.

## Policy Language
```
# Example: allow browser shard to read /media, deny /sys
ALLOW browser_shard READ /media
DENY  browser_shard ANY  /sys
```

## Roadmap
- [ ] Label assignment to all shards at boot
- [ ] Policy compiler (text → binary rule table)
- [ ] Kernel enforcement hook in syscall dispatcher
