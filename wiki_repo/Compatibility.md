# Compatibility Layers

SigmaOS can run software built for other operating systems via its native compatibility subsystem (`src/compatibility/`).

## Linux Compatibility (S-COSMOS)

The **S-COSMOS syscall emulation matrix** translates Linux system calls to SigmaOS-native equivalents.

| Linux syscall group | Status | File |
|---|---|---|
| `read`, `write`, `open`, `close` | ✅ | `src/syscall/dispatcher.rs` |
| `fork`, `exec`, `wait`, `exit` | ✅ | `src/kernel/proc/` |
| `mmap`, `munmap`, `brk` | ✅ | `src/kernel/memory/pmm_vmm.rs` |
| `socket`, `bind`, `connect` | ✅ | `src/net/tcpip_stack.rs` |
| `ioctl` (device control) | 🔄 | `src/drivers/` |
| `ptrace` (debugger) | 🔄 | `src/debugger/` |

**ELF loader** (`src/compatibility/elf_execution.rs`): loads and executes Linux ELF binaries by mapping segments, resolving GOT/PLT, and starting at entry point.

## Windows Compatibility (SigmaWin / S-COSMOS)

Inspired by ReactOS and Wine, `src/compatibility/reactos.rs` and `src/compatibility/sigmawin.rs` provide:
- Win32 API surface (CreateWindow, MessageBox, WinSock2 …)
- NT kernel object model (handles, named objects, sections)
- PE/COFF executable loading
- Registry emulation

## macOS Compatibility

`src/compatibility/personality.rs` provides a Mach-port-inspired IPC model for macOS ABI compatibility.

## Distribution-Specific Layers

| Distro | Features | File |
|---|---|---|
| Arch Linux | pacman hooks, AUR concept | `src/compatibility/arch_linux.rs` |
| Fedora | RPM lifecycle, SELinux policy | `src/compatibility/fedora.rs` |
| Ubuntu/Canonical | Snap confinement, AppArmor | `src/compatibility/canonical.rs` |
| Mint Linux | Update stability tiers | `src/compatibility/mint_linux.rs` |
| Garuda Linux | Zen kernel scheduler hints | `src/compatibility/garuda_zen.rs` |
| CachyOS | BORE/EEVDF scheduler model | `src/compatibility/cachy_os.rs` |
| Bodhi Linux | Moksha desktop canvas | `src/compatibility/bodhi_moksha.rs` |
| EndeavourOS | Welcome wizard, community | `src/compatibility/endeavour.rs` |
| Chakra Linux | Half-rolling model | `src/compatibility/chakra.rs` |
| Chimera Linux | LLVM/musl base | `src/compatibility/chimera_linux.rs` |
| ReactOS | Win32 subsystem | `src/compatibility/reactos.rs` |
| FreeDOS | DOS compatibility | `src/compatibility/freedos.rs` |
| TempleOS | Bare-metal simplicity | `src/compatibility/templeos.rs` |
| SSSD | Offline credential caching | `src/compatibility/sssd.rs` |
| Historic Linux | Kernel 0.01 to 1.0 compat | `src/compatibility/historic_linux.rs` |

## WASM Sandbox

`src/compatibility/wasm_sandbox.rs` runs WebAssembly modules in an isolated sandbox, enabling:
- Portable plugin system
- Safe execution of untrusted code
- Web app compatibility

## Constellation Mesh

`src/compatibility/constellation_mesh.rs` provides inter-node compatibility for SigmaOS running across multiple machines (cluster computing, edge nodes).
