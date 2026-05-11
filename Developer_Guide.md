1


Welcome to the SigmaOS Sovereign Repository.


1


SigmaOS abandons legacy `make` pipelines in favor of a mathematically deterministic build matrix written in Python.


1


To cross-compile the 600-shard C++ kernel for all supported Sovereign Architectures (x86_64, ARM64, RISC-V):


1


python3 tools/sigma-build.py


1


This will automatically resolve dependencies, parse the `SovereignEnclave` constraints, and output a bootable ISO.


1


SigmaOS supports unmodified Linux ELF binaries through the `SovereignCompat` shard. You do not need to recompile userland Linux apps; simply drop the binary onto the `SovereignVFS` and the BIT engine will dynamically translate the POSIX syscalls into native Sovereign API hooks.

