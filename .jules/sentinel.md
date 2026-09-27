## 2026-03-31 - Environment Sanitization Gaps in Privilege Elevation
**Vulnerability:** Privilege escalation modules (`SovereignSudoEngine` and `GksuSecurityGuard`) allowed environment variables like `GCONV_PATH`, `LD_AUDIT`, `DYLD_INSERT_LIBRARIES`, `BASH_ENV`, `IFS` to pass through during elevation.
**Learning:** Only filtering `LD_PRELOAD` and `LD_LIBRARY_PATH` leaves privilege escalation utilities vulnerable to PwnKit (CVE-2021-4034) and glibc audit / shell injection attacks.
**Prevention:** Always maintain a comprehensive denylist of dynamic linker, loader, locale, and shell execution variables across all privilege escalation and sudo/doas engines.
