# Contributing to SigmaOS Zenith

We welcome contributions to the SigmaOS microkernel. Follow these guidelines to build, test, and integrate new modules.

---

## 🛠️ Build Instructions for Modular Layers

The root `Makefile` lists targets for building the modular parts of the OS:

1. **Kernel Subsystems (`/kernel/`)**:
   Built as Ring-0 binaries. Avoid standard C library headers. Use `include/sigma_kernel_types.h` for freestanding declarations.

2. **System Libraries (`/lib/`)**:
   To add core C functions, edit `/lib/libc/sigma_libc.c` and declare prototypes in `sigma_libc.h`. 

3. **Filesystem Drivers (`/fs/`)**:
   Implement mock and real disk parsing code here. All routines must interface with `/fs/vfs.c` inodes.

4. **Network Drivers (`/net/`)**:
   Socket interfaces must follow the C prototypes in `/net/tcp_ip.c`.

5. **Userland CLI (`/usr/`)**:
   To add shell utilities, edit `/usr/sh.c`.

---

## 🧪 Running Validation Tests

Before submitting pull requests:

1. Run the local automated test suites using the command prompt:
   ```bash
   cmd /c npm run test
   ```
2. Verify all 45 test files return success status.
3. Keep the directory junctions (`userland` -> `usr`, `networking` -> `net`) untracked to avoid polluting git commits.
