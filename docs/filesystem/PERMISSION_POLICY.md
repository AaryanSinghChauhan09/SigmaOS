# Permission Policy (PERMISSION_POLICY)

## Representation
- File permissions are represented by a 9-bit mode field:
  - Owner: `S_IRUSR` (0o400), `S_IWUSR` (0o200), `S_IXUSR` (0o100)
  - Group: `S_IRGRP` (0o040), `S_IWGRP` (0o020), `S_IXGRP` (0o010)
  - Other: `S_IROTH` (0o004), `S_IWOTH` (0o002), `S_IXOTH` (0o001)
- Process identity is represented by `ProcessCredentials` (`uid`, `gid`, `euid`, `egid`, `capabilities`).

## Security Verification Rules
1. Root user (`euid == 0`) or processes with `CAP_DAC_OVERRIDE` bypass standard mode checks.
2. If `process.euid == file.uid`, Owner bits are checked.
3. Else if `process.egid == file.gid` or matching supplemental groups, Group bits are checked.
4. Else, Other bits are checked.
5. `CAP_DAC_READ_SEARCH` allows read access or directory traversal even if standard read/execute mode bits are clear.
6. Access denial returns `FsError::PermissionDenied`.
