# Permission Policy (PERMISSION_POLICY.md)

## Overview
The Permission Engine in SigmaOS implements POSIX-compatible DAC (Discretionary Access Control) evaluation and Linux capability checks.

## Specification & Semantics
- **Permission Bit Representation**: Standard POSIX permission mode bits (`S_IRUSR`, `S_IWUSR`, `S_IXUSR`, `S_IRGRP`, `S_IWGRP`, `S_IXGRP`, `S_IROTH`, `S_IWOTH`, `S_IXOTH`).
- **Access Evaluation Rules**:
  1. Root check (`euid == 0`): Standard access permitted, subject to execute bit checks for executable files.
  2. Owner check (`euid == inode.uid`): Applies user permission bits.
  3. Group check (`egid == inode.gid` or matching supplementary GIDs): Applies group permission bits.
  4. Others check: Applies other permission bits.
  5. Capabilities: `CAP_DAC_OVERRIDE` bypasses read/write/execute checks; `CAP_DAC_READ_SEARCH` bypasses read/directory search checks; `CAP_FOWNER` bypasses ownership checks for metadata modifications.
- **Error Semantics**: Unauthorized access attempts return `FsError::PermissionDenied`.
- **Limits & Hotplug**: Capability bitmask fits in a 64-bit integer, and credentials remain isolated per thread/process state across device hotplug events.
