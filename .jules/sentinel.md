# Sentinel 🛡️ Critical Learning Journal

## 2026-10-22 - Path Traversal Protection in UDF Package Execution Sandbox
**Learning:** Raw scriptlet paths in maintainer package scripts can attempt relative directory traversal (`../`) during package extraction. Enforcing canonical chroot/Landlock isolation boundaries prevents unauthorized file system access outside the container root.
**Action:** Strictly validate canonical paths against isolated jail roots before unpacking maintainer scriptlets.
