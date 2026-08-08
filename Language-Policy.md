# Language Policy for SigmaOS

## Approved Languages (production code only)

| Language               | Scope                                             | Rationale                                                          |
| :--------------------- | :------------------------------------------------ | :----------------------------------------------------------------- |
| **C (C11)**            | Kernel, drivers, boot, POSIX tools                | No runtime, bare-metal, deterministic ABI                          |
| **Rust (no_std)**      | Kernel subsystems, userland daemons, sigpkg       | Memory-safe, zero-cost, no GC, no runtime                         |
| **Nim (--gc:none)**    | CLI frontends, build tools, wiki sync utilities   | Compiles to C then native binary; fast, ergonomic, zero-GC mode   |
| **Zig**                | Low-level hardware drivers, cross-compilation     | Comptime, zero hidden allocations, C-interop                       |
| **NASM / AT&T ASM**    | CPU boot stubs, ISR entry points                  | Required for x86_64/AArch64 bare-metal startup                    |
| **POSIX sh**           | Build/CI scripts                                  | Universal, no runtime dependency                                   |
| **Batch (.bat)**       | Windows host-side helpers only                    | Native CMD, no interpreter required                                |

## Banned Languages (production code)

| Language       | Status    | Migration Path                             |
| :------------- | :-------- | :----------------------------------------- |
| **JavaScript** | REMOVED   | No desktop UI role; Wayland compositor = C/Rust |
| **HTML**       | REMOVED   | Desktop = native Wayland                   |
| **Python**     | REMOVED   | Replaced with Nim CLI or C tools           |
| **Go**         | REMOVED   | Replaced with Rust daemons                 |
| **PowerShell** | REMOVED   | Replaced with Nim or POSIX sh or Batch     |
| **PHP**        | FORBIDDEN | Not applicable to an OS                    |
| **Ruby**       | FORBIDDEN | Not applicable to an OS                    |
| **Java**       | FORBIDDEN | JVM runtime is antithetical to zero-bloat  |
| **C#**         | FORBIDDEN | CLR runtime dependency                     |
| **TypeScript** | FORBIDDEN | Transpiles to JS; same issues as JS        |

## Enforcement

> [!IMPORTANT]
> Any PR introducing a new file with a banned extension (`.js`, `.py`, `.go`, `.ps1`,
> `.html`, `.ts`, `.rb`, `.php`, `.java`, `.cs`) is rejected automatically by the
> GitHub Actions CI pipeline.

## Rationale for Nim and Zig

- **Nim** compiles via a C backend, producing auditable, minimal C that then goes through
  our standard LLVM/GCC toolchain. With `--gc:none` and `--mm:none` it produces
  completely allocation-free binaries comparable to handwritten C.
- **Zig** provides comptime evaluation, no hidden allocations, and direct C interop
  without a header layer, making it ideal for writing drivers and HAL components with
  compile-time verified memory layouts.
