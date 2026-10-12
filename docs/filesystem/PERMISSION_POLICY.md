# SigmaOS Permission Engine Policy

## Overview

The permission manager (`PermissionEngine`, `ProcessCredentials`, `AccessMode`) enforces POSIX filesystem permission semantics and Linux capability overrides across all VFS file access operations.

## Permission Model & Semantics

1. **Permission Representation**: File permissions are stored as standard POSIX octal mode bitmasks (e.g., `0rwxrwxrwx`).
2. **Access Evaluation Order**:
   - **Superuser Override**: Process with `uid == 0` or holding `CAP_DAC_OVERRIDE` bypasses read, write, and execute restrictions. Holding `CAP_DAC_READ_SEARCH` bypasses read and search (directory execute) restrictions.
   - **Owner Matching**: If process `euid` matches file `uid`, owner permission bits (bits 8..6) determine access.
   - **Group Matching**: If process `egid` or any supplemental group in `groups` matches file `gid`, group permission bits (bits 5..3) determine access.
   - **Other Matching**: Otherwise, world/other permission bits (bits 2..0) determine access.
3. **Capability Checks**:
   - `CAP_FOWNER`: Allows `chmod` / `chown` operations without owning the file or matching process UID.
4. **Error Semantics**: Unauthorized file access attempts return `FsError::PermissionDenied`.
