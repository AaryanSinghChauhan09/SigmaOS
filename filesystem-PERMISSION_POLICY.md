# SigmaOS Permission Subsystem Policy

## Objective

Govern permission checks, mode bits, process credentials, capability overrides, and DAC (Discretionary Access Control) evaluation in `src/fs/permission.rs`.

## Requirements & Rules

1. **Permission Representation**:
   - File modes follow standard POSIX octal bits: Owner Read/Write/Execute (`0u32` octal `0700`), Group Read/Write/Execute (`0070`), Other Read/Write/Execute (`0007`).
   - `AccessMode`: `Read = 4`, `Write = 2`, `Execute = 1`.

2. **Process Credentials (`ProcessCredentials`)**:
   - Captures real/effective `uid`, `gid`, `supp_groups` (supplementary group list), and capability bitmask `cap_mask`.

3. **Capability Overrides**:
   - `CAP_DAC_OVERRIDE` (`1 << 1`): Bypasses DAC read, write, and execute restrictions.
   - `CAP_DAC_READ_SEARCH` (`1 << 2`): Bypasses DAC read and directory search restrictions.
   - `CAP_FOWNER` (`1 << 3`): Bypasses file owner check for operations such as `chmod` and `utimes`.

4. **Access Verification Lifecycle**:
   - Root user (`uid == 0`) or process with `CAP_DAC_OVERRIDE` bypasses DAC checks.
   - Owner match (`creds.uid == file_uid`): evaluate owner permission bits against `mode`.
   - Group match (`creds.gid == file_gid` or `supp_groups.contains(&file_gid)`): evaluate group permission bits.
   - Otherwise: evaluate other permission bits.
   - Access denied returns `Err(FsError::PermissionDenied)`.

5. **Exhaustion & Safety**:
   - Supplementary group list is bounded to prevent unbounded linear scans during credential evaluation.
