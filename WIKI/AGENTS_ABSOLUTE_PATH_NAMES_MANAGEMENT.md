# AI Agent Guidelines: Absolute Path Names Operation Management in SigmaOS

## Overview
This document defines guidelines for AI agents working on **Absolute Path Names Operation Management**, VFS path resolution, canonical path normalization, directory traversal prevention (`..`), cross-distro path translation, OpenBSD `unveil(2)` path restriction rules, and Linux Landlock v5 file path access control in SigmaOS.

SigmaOS enforces strict absolute path validation (`/path/to/target`) across all 21 supported Linux and BSD operating modes to prevent C-string NUL byte injection, directory traversal attacks, and symlink-based privilege escalation.

---

## 1. Absolute Path Management Subsystems

AI agents interacting with file paths in SigmaOS must interface with the following core security and VFS modules:

| Subsystem / Module | Location | Description |
| :--- | :--- | :--- |
| **Path Input Validation (`validate_path`)** | `src/security/input_validation.rs` | Length-bounded (`MAX_PATH_LEN = 4096`), NUL-byte checked, and path-traversal (`..`) validated raw byte slice validator. |
| **OpenBSD Unveil Manager (`UnveilManager`)** | `src/security/unveil.rs` | Enforces path-based access permissions (`r`, `w`, `x`, `c`) via prefix matching and strict path validation (`validate_path`). |
| **Universal VFS Path Translator** | `src/distro/linux_bsd_inspirations.rs` | Translates foreign Linux/BSD absolute paths (`/usr/ports`, `/etc/rc.d`, `/nix/store`) into canonical SigmaOS VFS locations. |
| **Landlock v5 Path Access Guard** | `src/distro/sovereign_nextgen_distro_leap.rs` | Landlock v5 file hierarchy access rules restricting file creation, deletion, and directory reads. |

---

## 2. Absolute Path Validation Rules

When handling filesystem paths, AI agents must enforce the following validation standards:

```
                          Path Input Slice (&[u8])
                                     |
                                     v
                  +-------------------------------------+
                  | 1. Non-Empty & Length <= 4096 Bytes|
                  +-------------------------------------+
                                     |
                                     v
                  +-------------------------------------+
                  | 2. Reject NUL Bytes (b'\0')         |
                  +-------------------------------------+
                                     |
                                     v
                  +-------------------------------------+
                  | 3. Reject Traversal Sequences ('..')|
                  +-------------------------------------+
                                     |
                                     v
                  +-------------------------------------+
                  | 4. Canonical Absolute Resolution    |
                  +-------------------------------------+
```

### Path Validation Standard (`validate_path` in `src/security/input_validation.rs`)
1. **Length Limit:** Path length MUST NOT exceed `MAX_PATH_LEN` (4096 bytes).
2. **NUL Byte Rejection:** Reject any path containing embedded NUL bytes (`0x00`) to prevent C-string truncation attacks.
3. **Directory Traversal Rejection:** Reject relative path components (`..`) separated by `/`, `\`, or boundary markers.

```rust
// Standard path validation in SigmaOS
pub fn validate_path(path: &[u8]) -> Result<(), ValidationError> {
    if path.is_empty() { return Err(ValidationError::EmptyInput); }
    if path.len() > MAX_PATH_LEN { return Err(ValidationError::TooLong); }
    for &b in path { if b == 0 { return Err(ValidationError::NullByte); } }
    // Check for directory traversal '..'
    // ...
    Ok(())
}
```

---

## 3. OpenBSD Unveil(2) Path Restriction Protocols

`UnveilManager` (`src/security/unveil.rs`) enforces granular path visibility:

```rust
// Validating unveil permissions before file access
let mut manager = UnveilManager::new();
manager.unveil("/var/log", UnveilPermission::Read)?;
manager.unveil("/tmp", UnveilPermission::Create)?;

// Path access checks
assert!(manager.validate_path("/var/log/syslog", UnveilPermission::Read).is_ok());
assert!(manager.validate_path("/etc/passwd", UnveilPermission::Read).is_err());
```

1. **Exact & Prefix Matching:** Path permission checks evaluate against unveiled directory hierarchies. Un-unveiled absolute paths are invisible (`Err(SigmaError::PermissionDenied)`).
2. **Path Hardening:** Unveil restrictions cannot be lifted once locked via `unveil(null, null)`.

---

## 4. AI Agent Self-Assessment Checklist

Before finalizing file system or path handling changes:

- [ ] Are all input paths validated via `validate_path` before opening or creating files?
- [ ] Are path traversal sequences (`..`) and NUL bytes strictly rejected?
- [ ] Is `MAX_PATH_LEN` (4096) respected across all VFS path allocations?
- [ ] Has `./run_sigma_tests.sh` been executed and confirmed passing with 0 failures?
