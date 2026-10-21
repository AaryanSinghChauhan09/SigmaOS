# SigmaOS Permission Policy (PERMISSION_POLICY)

## Core Objectives

- **Permission Representation:** Standard POSIX octal mode bit representation (`0rwx-rwx-rwx`, e.g., `0755`, `0644`).
- **Bitwise Evaluation:** Access checks evaluate requested read (`R_OK`), write (`W_OK`), or execute (`X_OK`) permissions against matching credential ownership bits.
- **Capability Overrides:** Superuser (`UID == 0`) and process capabilities (`CAP_DAC_OVERRIDE`, `CAP_FOWNER`) override mode restrictions deterministically.
- **Error Semantics:** Permission failures return `FsError::PermissionDenied`.

---

## Permission Evaluation Algorithm

When a process with `ProcessCredentials { uid, gid, supplementary_gids, capabilities }` accesses an inode with `(file_uid, file_gid, file_mode)`:

1. **Superuser / Capability Check:**
   - If `uid == 0` or process possesses `CAP_DAC_OVERRIDE`, requested read/write access is permitted.
   - Execute access for `CAP_DAC_OVERRIDE` requires at least one execute bit (`0111`) set on `file_mode`.

2. **Owner Matching:**
   - If `uid == file_uid`:
     - Test requested access bits against owner mode mask `(file_mode >> 6) & 07`.

3. **Group Matching:**
   - Else if `gid == file_gid` or `supplementary_gids` contains `file_gid`:
     - Test requested access bits against group mode mask `(file_mode >> 3) & 07`.

4. **Other Matching:**
   - Else:
     - Test requested access bits against other mode mask `file_mode & 07`.

If the required mask bits are present, return `Ok(())`; otherwise, return `Err(FsError::PermissionDenied)`.
