# Permission Management Policy (PERMISSION_POLICY)

## Core Model
SigmaOS file access control is evaluated by `PermissionEngine` (`src/fs/permission.rs`) using subject `ProcessCredentials` (`uid`, `gid`, effective groups, and POSIX capabilities) and target `InodeMetadata` mode bits.

## Permission Representation
- Standard 9-bit POSIX mode bitmask:
  - Owner: `rwx` (bits 8..6, e.g. `0o700`)
  - Group: `rwx` (bits 5..3, e.g. `0o070`)
  - Other: `rwx` (bits 2..0, e.g. `0o007`)
- Access Modes: `AccessMode::Read` (`0o4`), `AccessMode::Write` (`0o2`), `AccessMode::Execute` (`0o1`).

## Access Control Evaluation Hierarchy
1. **Root Privilege**: If `creds.uid == 0` (Superuser / Root), read and write access are unconditionally granted. Execute access requires at least one execute bit set in mode.
2. **Capability Overrides**:
   - `CAP_DAC_OVERRIDE` (bit 1): Bypasses all read, write, and execute permission checks.
   - `CAP_DAC_READ_SEARCH` (bit 2): Bypasses read and directory search permission checks.
   - `CAP_FOWNER` (bit 3): Overrides file ownership restrictions for `chmod`/`chown` operations.
3. **Owner Check**: If `creds.euid == metadata.uid`, check owner mode bits.
4. **Group Check**: If `creds.egid == metadata.gid` or `creds.groups` contains `metadata.gid`, check group mode bits.
5. **Other Check**: Otherwise, check other mode bits.

## Error Policy
- Denied access returns `Err(FsError::PermissionDenied)`.
