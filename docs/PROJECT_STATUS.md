# SigmaOS Project Status

**Release status: experimental development project. Not ready for general installation.** The repository contains a broad set of Rust modules and unit-tested subsystem models. Those artifacts do not, by themselves, show that the boot, install, hardware, or desktop paths work end to end.

## Verified in the current development environment

The following checks passed on 2026-10-04:

| Check | Result | What it establishes |
|---|---|---|
| `cargo check --lib` | Passed | The hosted library target compiles. It does not build a bootable kernel image. |
| `cargo test --lib` | 6,349 passed; 66 ignored | The library's in-process unit tests pass. Ignored tests did not run. |
| `./run_sigma_tests.sh` | Passed | The repository's standalone test runner completed. Some test sources emitted compiler warnings. |
| QEMU boot | Not run | `qemu-system-x86_64` was unavailable in the environment. |

Test outcomes describe the tested code and configuration, not general hardware support or release readiness.

## Known release blockers

- There is no verified end-to-end path from firmware/bootloader through kernel initialization to a usable user session. The current `sigma_kernel` entry point still includes hosted `std` dependencies and is not validated as a bare-metal artifact.
- The ISO build scripts previously emitted empty or simulated artifacts when prerequisites failed. They now fail without writing an image until a real kernel, initramfs, bootloader configuration, and validation path exist.
- The QEMU smoke-test helpers previously simulated a successful run. They now reject missing/invalid images and do not report success without a runtime-ready signal.
- The current ISO/installer and recovery workflow has not been validated on disposable virtual disks and supported hardware.
- Credential verification and cryptographically secure randomness are not integrated. XorShift-backed random-byte paths remain in security prototype code, so encryption, VPN, TPM, and stack-canary features that consume them are not suitable for production use.
- Hardware compatibility needs a device-by-device test record; source modules are not a support matrix.
- Desktop and package-management workflows need validation from a clean installation, including error handling and recovery.

## Status vocabulary

- **Proposed:** design work only; no implementation is implied.
- **Prototype:** code or a model exists, but runtime integration or failure handling is incomplete.
- **Integrated:** connected to its intended runtime path and validated in the documented environment.
- **Supported:** repeatably tested on named configurations, with limitations and recovery documented.

Use [FEATURE_STATUS.toml](../FEATURE_STATUS.toml) as the component inventory, and the owning [GitHub Wiki component page](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki) for source links, references, test evidence, limitations, and roadmap. A `working` module test status must not be interpreted as `supported` system behavior.

## Next release milestones

1. Build the kernel for the intended bare-metal target and boot it in QEMU with serial logs.
2. Reach a minimal shell through the real process and syscall paths.
3. Install to a disposable virtual disk, interrupt installation at controlled points, and verify recovery.
4. Publish a supported device matrix based on actual boot and operation tests.
5. Validate update rollback and a first-session desktop workflow before publishing a general-use image.
