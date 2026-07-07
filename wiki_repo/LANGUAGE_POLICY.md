# Language Policy for SigmaOS

## Permitted Languages (production code only)

| Language     | Scope                                          | Rationale                                      |
| :----------- | :--------------------------------------------- | :--------------------------------------------- |
| **C**        | Kernel, drivers, boot, POSIX tools             | No runtime, bare-metal, deterministic ABI      |
| **Rust**     | Userland daemons, sigpkg, SigmaAI engine       | Memory-safe, `no_std` capable, zero-cost       |
| **NASM/AT&T ASM** | CPU-specific boot stubs, ISR entry points | Required for x86_64/AArch64 bare-metal startup |
| **POSIX sh** | Build scripts, CI hooks                        | Universal, no runtime dependency               |
| **Batch**    | Windows host-side build helpers only           | Native CMD, no PowerShell/Python required      |

## Banned Languages (production code)

| Language       | Status    | Migration path                          |
| :------------- | :-------- | :-------------------------------------- |
| **JavaScript** | REMOVED   | No replacement in core OS              |
| **HTML**       | REMOVED   | Desktop UI = Wayland native C/Rust     |
| **Python**     | REMOVED   | Replaced with C tools or Rust          |
| **Go**         | REMOVED   | Replaced with Rust daemons             |
| **PowerShell** | REMOVED   | Replaced with POSIX sh or Batch        |
| **PHP**        | FORBIDDEN | Not applicable                         |
| **Ruby**       | FORBIDDEN | Not applicable                         |

## Exceptions

- **Test harnesses**: POSIX sh test scripts only.
- **CI pipeline**: GitHub Actions YAML (declarative, not executable code).
- **Documentation**: Markdown only (`.md`).

> [!IMPORTANT]
> Any PR introducing a new file with a banned extension will be rejected automatically by CI.
