# Filesystem Permission Policy

## Overview
SigmaOS permission evaluation engine (`src/fs/permission.rs`) governs access control across files and directories.

## Specifications
1. **Permission Representation**: Represented as standard POSIX octal mode flags (`rwxrwxrwx`), setuid (`04000`), setgid (`02000`), and sticky bit (`01000`).
2. **Access Evaluation**: Evaluated against process effective UID/GID, process capability flags (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`), and path sandbox filters (Landlock/Unveil).
3. **Typed Error Codes**: Unpermitted access yields `FsError::PermissionDenied` (`EACCES` or `EPERM`).
4. **Exhaustion & Bounds**: Access decisions require zero heap allocations in critical evaluation loops.
