# SigmaOS Next Steps Guidelines

## Guidelines for Packaging and Distro Interop
1. Always maintain canonical mapping to `sovereign-*` system packages.
2. Sandboxing must enforce least privilege across all UDF hooks.
3. Every new distro format addition must be validated with zero-copy inspection tests and integration suites.
