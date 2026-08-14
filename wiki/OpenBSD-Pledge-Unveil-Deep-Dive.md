# OpenBSD pledge(2) and unveil(2) Deep Dive for SigmaOS

## Overview

SigmaOS implements the OpenBSD security sandbox model with `pledge()` and `unveil()` syscalls, providing fine-grained process privilege restriction without requiring a traditional MAC framework.

## pledge(2) - Promise-Based Syscall Restriction

### How It Works

```
Process calls pledge("stdio rpath", nullptr)
         │
         ▼
    PledgeManager
         │
         ├─ Parse promise string into CapabilitySet
         │
         ├─ Apply restrictions (one-way ratchet)
         │
         └─ Store in process's security context
                  │
                  ▼
         On syscall entry: check against promise
                  │
        ┌─────────┴─────────┐
        │ ALLOWED            │ DENIED
        ▼                   ▼
  Continue               SIGABRT
```

### Promise String Parsing

| Promise Token | Allowed Syscalls |
|--------------|-----------------|
| `stdio` | read, write, close, fstat, stat, getdents |
| `rpath` | open(O_RDONLY), stat, lstat, readlink |
| `wpath` | open(O_WRONLY), create, truncate |
| `cpath` | mkdir, rename, link, symlink |
| `dpath` | mknod, mkfifo, socket |
| `exec` | execve, execvp |
| `inet` | socket(AF_INET), connect, listen, accept |
| `dns` | getaddrinfo, getnameinfo |
| `proc` | fork, waitpid, kill |
| `id` | setuid, setgid, getuid, getgid |
| `tty` | TIOCGWINSZ, TIOCSWINSZ, tcsetattr |

### SigmaOS Implementation

```rust
// src/security/pledge.rs
pub struct PledgeManager {
    pledge: Option<PledgePromise>,
    gate: CapabilityGate,
    unveiled_paths: Vec<UnveilEntry>,
}

impl PledgeManager {
    /// Apply pledge restriction (one-way ratchet - cannot expand promises)
    pub fn apply_pledge(
        &mut self,
        promises: &str,
        exec_promises: Option<&str>,
    ) -> Result<(), PledgeError> {
        let new_permissions = Self::parse_promises(promises)?;
        
        if let Some(ref current) = self.pledge {
            // Can only restrict, never expand
            for perm in &new_permissions {
                if !current.allows(*perm) {
                    return Err(PledgeError::InvalidPermission);
                }
            }
        }
        
        self.pledge = Some(PledgePromise::new(new_permissions));
        Ok(())
    }
}
```

## unveil(2) - Filesystem Visibility Restriction

### How It Works

After `unveil()` is called, only explicitly unveiled paths are visible to the process. All other filesystem paths return `ENOENT`.

```
Process calls unveil("/etc", "r")
Process calls unveil("/tmp", "rw")
Process calls unveil("", "")      ← Lock the unveil list
         │
         ▼
    UnveilManager
         │
         ├─ Add /etc with READ permission
         ├─ Add /tmp with READ|WRITE permission
         └─ Lock (no more unveil calls allowed)
                  │
                  ▼
         On open("/usr/lib/libfoo.so", O_RDONLY):
                  │
         Check prefix against unveiled paths
                  │
        ┌─────────┴─────────┐
        │ MATCH (prefix)     │ NO MATCH
        ▼                   ▼
  Check permissions       Return ENOENT
```

### Directory Traversal Prevention

**Critical security property**: Paths like `../../etc/passwd` must not bypass unveil restrictions.

```rust
// src/security/unveil.rs - Path normalization
fn normalize_path(raw_path: &str) -> Result<String, SecurityError> {
    let mut components: Vec<&str> = Vec::new();
    
    for component in raw_path.split('/') {
        match component {
            "" | "." => continue,
            ".." => {
                // Prevent traversal above root
                components.pop(); // Safe: empty Vec.pop() is a no-op
            }
            c => components.push(c),
        }
    }
    
    let mut result = String::from("/");
    result.push_str(&components.join("/"));
    Ok(result)
}

/// Check if path is covered by any unveiled restriction
pub fn is_path_allowed(&self, path: &str, required: UnveilPermission) -> bool {
    let normalized = match normalize_path(path) {
        Ok(p) => p,
        Err(_) => return false, // Reject malformed paths
    };
    
    for restriction in &self.restrictions {
        // Must be exact match or subdirectory - NOT a prefix bypass
        let unveiled_normalized = normalize_path(&restriction.path).unwrap_or_default();
        
        if normalized == unveiled_normalized 
            || normalized.starts_with(&format!("{}/", unveiled_normalized)) {
            return restriction.permissions.contains(&required);
        }
    }
    
    false
}
```

### Prefix Bypass Prevention

**Vulnerability Example (FIXED)**:
```
Unveiled: /app/data
Attack path: /app/data-evil/../../etc/passwd
             ^^^^^^^^^^^^^^^^ This is /etc/passwd after normalization
```

The fix: always normalize BOTH the unveiled path AND the requested path before comparison, and use `/` suffix in prefix checks.

## Integration with Linux Security Modules (LSM)

SigmaOS bridges pledge/unveil with a Tomoyo/AppArmor-like path-based MAC:

```
┌─────────────────────────────────────────────────┐
│                  Syscall Entry                  │
├─────────────────────────────────────────────────┤
│  1. pledge check: is this syscall in promises?  │
├─────────────────────────────────────────────────┤
│  2. unveil check: is this path visible?         │
├─────────────────────────────────────────────────┤
│  3. capability check: does process have cap?    │
├─────────────────────────────────────────────────┤
│  4. SELinux label check (optional)              │
└─────────────────────────────────────────────────┘
```

## Differences from OpenBSD

| Feature | OpenBSD | SigmaOS |
|---------|---------|---------|
| Promise strings | `"stdio rpath inet"` | Same |
| Exec promises | Separate set for child | Same |
| Violation response | `SIGABRT` | Configurable (log, kill, sandbox) |
| Implementation | Kernel `pledge.c` | Rust `PledgeManager` |
| Unveil | Path-based | Path-based + inode cache |
| Inheritance | Fork inherits | Fork inherits |
| Thread safety | Per-process | Per-thread (atomic) |

## References

- OpenBSD `pledge(2)` manpage
- OpenBSD `unveil(2)` manpage
- [Security Architecture](Security-Architecture.md)
- `src/security/pledge.rs`
- `src/security/unveil.rs`
