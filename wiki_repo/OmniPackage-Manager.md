# 📦 OmniPackage Manager

> "End dependency hell by isolating every package into its own containerized shard."

Linux package management often suffers from library conflicts in `/usr/lib`. SigmaOS solves this fundamentally.

## 1. Transactional Installations
OmniPackage does not dump files into a global filesystem. Instead, `sigma_pkg_install()` creates an isolated container path:
`/sigma/pkgs/[AppName]`

## 2. Zero-Dependency Ecosystem
Since SigmaOS applications cannot rely on standard libraries (like libc, libpng, etc.), they must be statically compiled with everything they need (or use the Sovereign HAL). 

This means OmniPackages (`.spk`) never conflict with one another.

## 3. Execution
When the shell invokes an application, it runs it strictly within the scope of its OmniPackage directory.
