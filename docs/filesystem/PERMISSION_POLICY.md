# File System Permission Policy

## 1. Overview

SigmaOS enforces a POSIX-compatible Discretionary Access Control (DAC) permission system augmented with process capability vectors.

---

## 2. Credentials & Capability Representation

### Process Credentials (`ProcessCredentials`)

- Real UID / GID (`uid`, `gid`).
- Effective UID / GID (`euid`, `egid`).
- Supplementary GIDs (`supplementary_gids: Vec<u32>`).
- Capability bitmask (`capabilities: u64`).

### Core Capabilities

- `CAP_DAC_OVERRIDE` (`1 << 1`): Overrides file read, write, and execute mode bit restrictions.
- `CAP_DAC_READ_SEARCH` (`1 << 2`): Overrides file read and directory search mode bit restrictions.
- `CAP_FOWNER` (`1 << 3`): Overrides file ownership checks for `chmod` and timestamp modifications.

---

## 3. Access Permission Evaluation Engine (`PermissionEngine`)

Access evaluation determines permission for `AccessMode` (`Read`, `Write`, `Execute`, `ReadWrite`):

1. **Root & Override Check:** If `euid == 0` or process possesses `CAP_DAC_OVERRIDE`, access is granted immediately.
2. **Owner Evaluation:** If `euid == file_uid`, check user mode bits (`S_IRUSR`, `S_IWUSR`, `S_IXUSR`).
3. **Group Evaluation:** Else if `egid == file_gid` or `supplementary_gids.contains(&file_gid)`, check group mode bits (`S_IRGRP`, `S_IWGRP`, `S_IXGRP`).
4. **Other Evaluation:** Else check other mode bits (`S_IROTH`, `S_IWOTH`, `S_IXOTH`).
5. **Read Override Check:** If read access is denied by mode bits, check if process holds `CAP_DAC_READ_SEARCH`.
6. **Error Return:** Returns `Ok(())` on success, or `Err(FsError::PermissionDenied)` on authorization failure.
