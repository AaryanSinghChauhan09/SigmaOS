# Permission Management Policy

## Overview

The permission manager (`src/fs/permission.rs`) evaluates process credentials (`ProcessCredentials`) against inode access modes (`AccessMode`: Read, Write, Execute) and ownership bits.

## Key Rules & Invariants

1. **Credential Structure**: Process credentials include `uid` (user ID), `gid` (group ID), `effective_uid`, `effective_gid`, and process capabilities (`capabilities: u64`).
2. **Permission Evaluation Hierarchy**:
   - **Root Override**: Root user (`uid == 0` or `effective_uid == 0`) or processes with `CAP_DAC_OVERRIDE` bypass standard mode bit checks.
   - **Owner Match**: If `process.euid == file.uid`, permission bits `0700` apply.
   - **Group Match**: If `process.egid == file.gid`, permission bits `0070` apply.
   - **Other Match**: Otherwise, permission bits `0007` apply.
3. **Capability Overrides**:
   - `CAP_DAC_OVERRIDE` (1 << 1): Overrides DAC read, write, and execute restrictions.
   - `CAP_DAC_READ_SEARCH` (1 << 2): Overrides DAC read restrictions and directory search restrictions.
   - `CAP_FOWNER` (1 << 3): Overrides permission checks on operations that normally require file ownership (e.g. `chmod`).
4. **Error Semantics**:
   - Unmet permission requirements fail deterministically returning `FsError::PermissionDenied`.

## Testing & Verification

Permission checks and capability overrides are tested in `tests/filesystem/permission.rs`.
