# SigmaOS Sovereign Landlock v5 + Capsicum + OpenBSD Unveil

## Overview

SigmaOS implements a **tri-OS sandboxing engine** in 100% safe Rust combining:

1. **Linux Landlock v5** (mainlined Linux 5.13, June 2021) — filesystem access control
2. **OpenBSD `unveil(2)`** — path-based permission restriction
3. **FreeBSD Capsicum** — capability-based security for file descriptors

All three in one unified `SovereignLandlockV5Guard` struct. No external dependencies.

## Why This Combination?

| Mechanism | OS Origin | Strength |
|-----------|-----------|----------|
| Landlock | Linux 5.13+ | Per-path filesystem rights bitmask |
| unveil | OpenBSD | Simple r/w/x/c path restriction |
| Capsicum | FreeBSD | FD capability mode — blocks all ambient authority |
| pledge | OpenBSD | Promise-based syscall restriction |

Together they provide **defense-in-depth** that exceeds any single OS's sandboxing.

## Landlock Access Rights Bitmask

Mirrors `LANDLOCK_ACCESS_FS_*` constants from Linux uapi:

| Bit | Right | Description |
|-----|-------|-------------|
| 0 | `EXECUTE` | Execute files |
| 1 | `WRITE_FILE` | Write to files |
| 2 | `READ_FILE` | Read files |
| 3 | `READ_DIR` | List directories |
| 4 | `REMOVE_DIR` | Remove directories |
| 5 | `REMOVE_FILE` | Unlink files |
| 13 | `REFER` | Create hardlinks across directories |
| 14 | `TRUNCATE` | Truncate files |
| 15 | `IOCTL_DEV` | ioctl on device files |
| 16 | `BIND_TCP` | Bind TCP socket (Landlock v5) |
| 17 | `CONNECT_TCP` | Connect TCP socket (Landlock v5) |

## Usage

```rust
let mut guard = SovereignLandlockV5Guard::new(5);

// Add path rules
guard.add_rule(LandlockPathRule::new("/etc", LandlockFsRights::READ_ONLY, true));
guard.add_rule(LandlockPathRule::new("/tmp", LandlockFsRights::READ_WRITE, true));
guard.add_rule(LandlockPathRule::new("/var/log/app", LandlockFsRights::WRITE_FILE, false));

// Add OpenBSD-style unveil entries
guard.unveil("/home/user", "rw");   // read + write
guard.unveil("/usr/lib",   "r");    // read-only

// Enforce sandbox (no new rules after this — matches real Landlock behavior)
guard.enforce(true); // true = no_new_privs (like prctl(PR_SET_NO_NEW_PRIVS))

// Check accesses — default-deny
assert!(guard.check_fs_access("/etc/hostname", LandlockFsRights::READ_FILE));
assert!(!guard.check_fs_access("/etc/shadow",  LandlockFsRights::READ_FILE)); // DENY

// Violation logging
println!("{}", guard.violation_summary());
```

## Capsicum FD Capability Mode

```rust
let mut fd = CapsicumFdDescriptor::new(
    3, // file descriptor number
    CapsicumRights(CapsicumRights::CAP_READ.0 | CapsicumRights::CAP_FSTAT.0)
);

// Before cap_enter(): all operations allowed (ambient authority)
assert!(fd.check(CapsicumRights::CAP_WRITE)); // true

fd.enter_capability_mode(); // cap_enter() equivalent

// After: only rights in the descriptor are allowed
assert!(fd.check(CapsicumRights::CAP_READ));   // true
assert!(!fd.check(CapsicumRights::CAP_WRITE)); // false — not in rights set

// Rights can only be restricted, never expanded
fd.limit_rights(CapsicumRights::CAP_READ); // Remove CAP_FSTAT too
```

## Security Model

### Default-Deny Principle

Without any rules, `check_fs_access()` returns **false** for all paths. This is the correct zero-trust default — you explicitly grant access, never revoke it.

### Violation Recording

Every denied access is logged (up to 256 entries):
```
LANDLOCK_DENY path=/etc/shadow rights=0x00000004
```

### Sandbox States

```
Building → enforce() → Enforced
                           ↓ (on violation)
                        Violated
```

## Linux/BSD Parity

| Linux | OpenBSD | FreeBSD | SigmaOS |
|-------|---------|---------|---------|
| Landlock ruleset | unveil paths | capsicum rights | `SovereignLandlockV5Guard` |
| `landlock_create_ruleset()` | — | — | `::new(version)` |
| `landlock_add_rule()` | `unveil(path, perms)` | `cap_rights_limit()` | `.add_rule() / .unveil()` |
| `landlock_restrict_self()` | — | `cap_enter()` | `.enforce(no_new_privs)` |

## Tests

6 unit tests, all passing:

- `test_landlock_path_rule_matching` — recursive and exact matching
- `test_landlock_sandbox_enforce_and_check` — read-only /etc, read-write /tmp
- `test_landlock_violation_logging` — denied accesses are logged
- `test_unveil_permissions` — OpenBSD-style rwxc parsing
- `test_capsicum_capability_mode` — pre/post cap_enter behavior
- `test_rights_restriction_monotonic` — rights can only decrease

## Source

[`src/security/landlock_sovereign.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/security/landlock_sovereign.rs)
