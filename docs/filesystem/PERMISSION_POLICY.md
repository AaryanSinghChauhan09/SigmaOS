# Permission Management Policy

## Overview
This document specifies the representation, check semantics, capability overrides, and error handling for file permissions in SigmaOS.

## Permission Representation
File permissions are represented using POSIX-standard 9-bit octal permission masks (`0o777`):
- Owner permissions: Read (`S_IRUSR = 0o400`), Write (`S_IWUSR = 0o200`), Execute (`S_IXUSR = 0o100`).
- Group permissions: Read (`S_IRGRP = 0o040`), Write (`S_IWGRP = 0o020`), Execute (`S_IXGRP = 0o010`).
- Other permissions: Read (`S_IROTH = 0o004`), Write (`S_IWOTH = 0o002`), Execute (`S_IXOTH = 0o001`).

## Capability Overrides
Process credentials (`ProcessCredentials`) contain bitfield capability flags that bypass standard mode checks:
- `CAP_DAC_OVERRIDE` (`1 << 1`): Overrides all read, write, and execute permission checks.
- `CAP_DAC_READ_SEARCH` (`1 << 2`): Overrides read and directory search checks.
- `CAP_FOWNER` (`1 << 3`): Overrides owner restriction checks for operations like `chmod` and `utimes`.

## Access Semantics and Evaluation Order
When a process requests access to a file inode:
1. Root user (`euid == 0`) or process with `CAP_DAC_OVERRIDE` is granted access immediately.
2. If `euid == file_uid`, access is granted if the requested permission bits match `owner` mode bits.
3. Else if `egid == file_gid` or `egid` is in process supplementary groups, access is granted if requested bits match `group` mode bits.
4. Else, access is granted if requested bits match `other` mode bits.
5. If access is denied, `FsError::PermissionDenied` is returned.
